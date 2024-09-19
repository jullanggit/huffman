use std::collections::HashMap;

use crate::bitvec::{BitVec, Read, Write};

#[derive(Debug, Clone)]
struct Node {
    freq: u32,
    message: Option<usize>,
    children: Option<Box<[Node; 2]>>,
}
impl Node {
    /// Fills the given `HashMap` according the huffman tree
    fn get_map(&self, values: &mut HashMap<usize, (usize, u8)>, cur_value: usize, cur_len: u8) {
        match self.children {
            Some(ref boxed_nodes) => {
                let nodes = boxed_nodes;
                nodes[0].get_map(values, cur_value * 2, cur_len + 1);
                nodes[1].get_map(values, cur_value * 2 + 1, cur_len + 1);
            }
            None => {
                values.insert(self.message.unwrap(), (cur_value, cur_len));
            }
        }
    }
}

pub fn encode(input_data: Vec<u8>, orig_symbol_size: u8, print: bool) -> Vec<u8> {
    if input_data.is_empty() {
        return input_data;
    }

    let mut input_data = BitVec::from_data(input_data);

    let (symbol_nodes, num_symbols) = get_symbol_nodes(&mut input_data, orig_symbol_size);
    let huffman_root = construct_huffman_tree(symbol_nodes);

    // Map from original symbol to (encoded_symbol, encoded_symbol_len)
    let mut map = HashMap::new();
    huffman_root.get_map(&mut map, 0, 1);

    // The length (in bits) of the longest symbol
    let longest_symbol_length = map.values().map(|&(_, len)| len).max().unwrap();
    let encoded_symbol_size_size = u8::BITS - longest_symbol_length.leading_zeros();

    let mut output_data = BitVec::default();

    write_header(
        &mut output_data,
        num_symbols,
        orig_symbol_size,
        encoded_symbol_size_size as usize,
    );

    // Write original to encoded symbol map
    for (original_symbol, &(encoded_symbol, encoded_symbol_len)) in &map {
        debug_assert!(encoded_symbol_len != 0);

        output_data.write(encoded_symbol_len, encoded_symbol_size_size as u8);
        output_data.write(*original_symbol, orig_symbol_size);
        output_data.write(encoded_symbol, encoded_symbol_len);
    }
    // Write delimiter
    output_data.write(0_u8, encoded_symbol_size_size as u8);

    let header_len = output_data.bits();

    input_data.reset();

    // Write data
    while let Some(original_symbol) = input_data.read(orig_symbol_size) {
        let (encoded_symbol, encoded_symbol_len) = map[&original_symbol];
        output_data.write(encoded_symbol, encoded_symbol_len);
    }

    if print {
        let data_len = output_data.bits() - header_len;

        println!("Original len: {}b", input_data.bits());
        println!(
            "Compressed len: {}b (header: {}b, data: {}b)",
            output_data.bits(),
            header_len,
            data_len
        );
    }
    output_data.data()
}

/// Returns a Node for each unique symbol in the original message, containing its frequency, and
/// the amount of symbols in the message
fn get_symbol_nodes(message: &mut BitVec<Read>, symbol_len: u8) -> (Vec<Node>, usize) {
    let mut freqs = HashMap::new();

    let mut symbols = 0;
    // Populate Hashmap
    while let Some(symbol) = message.read(symbol_len) {
        symbols += 1;

        let freq = freqs.entry(symbol).or_insert(0);
        *freq += 1;
    }
    // Convert to Vec
    let nodes: Vec<_> = freqs
        .into_iter()
        .map(|(key, value)| Node {
            freq: value,
            message: Some(key),
            children: None,
        })
        .collect();

    (nodes, symbols)
}

/// Takes a Vec nodes and constructs a huffman tree from it
fn construct_huffman_tree(mut nodes: Vec<Node>) -> Node {
    // Sort from biggest to smallest
    nodes.sort_unstable_by(|a, b| b.freq.cmp(&a.freq));
    while nodes.len() > 1 {
        // Get the two smallest nodes
        let a = nodes.pop().unwrap();
        let b = nodes.pop().unwrap();

        // Combine them into a new node
        let new_node = Node {
            freq: a.freq + b.freq,
            message: None,
            children: Some(Box::new([a, b])),
        };

        // Get the right index (so the vec remains sorted)
        let index = nodes
            .binary_search_by_key(&new_node.freq, |node| node.freq)
            .unwrap_or_else(|e| e);
        // Insert the new node
        nodes.insert(index, new_node);
    }
    nodes[0].clone()
}

fn write_header(
    data: &mut BitVec<Write>,
    num_symbols: usize,
    orig_symbol_size: u8,
    encoded_symbol_size_size: usize,
) {
    // amount of bites used for specifying the number of symbols
    let num_symbols_size = usize::BITS - num_symbols.leading_zeros();
    data.write(num_symbols_size as usize, 5);

    // Number of unique symbols
    data.write(num_symbols, num_symbols_size as u8);

    // Size of the original symbols
    data.write(orig_symbol_size, 4);

    // Size of the longest encoded symbol
    data.write(encoded_symbol_size_size, 5);
}

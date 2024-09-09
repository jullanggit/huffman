use std::collections::HashMap;

use crate::bitvec::BitVec;

#[expect(clippy::cast_possible_truncation)]
pub fn decode(input_data: Vec<u8>) -> Vec<u8> {
    if input_data.is_empty() {
        return input_data;
    }

    let mut input_data = BitVec::from_data(input_data);

    let num_symbols_size = input_data.read(5).unwrap() as u8;
    let num_symbols = input_data.read(num_symbols_size).unwrap();

    let orig_symbol_size = input_data.read(4).unwrap() as u8;

    let encoded_symbol_size_size = input_data.read(5).unwrap() as u8;

    // Maps (encoded symbol, len) to original symbol
    let mut map = HashMap::new();

    loop {
        let encoded_symbol_size = input_data.read(encoded_symbol_size_size).unwrap();

        if encoded_symbol_size == 0 {
            break;
        }

        let original_symbol = input_data.read(orig_symbol_size).unwrap();
        let encoded_symbol = input_data.read(encoded_symbol_size as u8).unwrap();

        map.insert((encoded_symbol, encoded_symbol_size), original_symbol);
    }

    let mut output_data = BitVec::default();

    for _ in 0..num_symbols {
        let mut cur_len = 0;
        let mut cur_data = 0;

        loop {
            cur_len += 1;
            cur_data = (cur_data << 1) | input_data.read(1).unwrap();

            if let Some(orig_symbol) = map.get(&(cur_data, cur_len)) {
                output_data.write(*orig_symbol, orig_symbol_size);

                break;
            }
        }
    }
    output_data.data()
}

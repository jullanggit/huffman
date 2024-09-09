#![no_main]

use huffman::{decode::decode, encode::encode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let encoded_data = encode(data.to_vec(), 8, false);
    let decoded_data = decode(encoded_data);

    assert_eq!(data, decoded_data);
});

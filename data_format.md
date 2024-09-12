
## Header:
- 5 bits: num_symbols_size (amount of bits used for specifying the number of symbols)
- [num_symbols_size] bits: number of symbols
- 4 bits: size of original symbols
- 5 bits: the size of the bits indicating how long the encoded symbol is
- Symbol structure:
    - [encoded_symbol_size_size] bits: length of the encoded symbol (will never be 0)
    - [orig_symbol_size] bits: original symbol
    - [encoded_symbol_size] bits: encoded symbol
## Data:
- Raw data

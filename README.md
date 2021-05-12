[![crates.io](https://img.shields.io/crates/v/huff-tree-tap.svg)](https://crates.io/crates/huff-tree-tap)
# huff-tree-tap
Huffman Encoder and Decoder Library


# Example usage
Cargo.toml:
```toml
[dependencies]
huff_tree_tap = "*" # You can specify a version here if you want
```

```rust
extern crate huff_tree_tap;
use  huff_tree_tap::*;

let data: Vec<u8> = "this is a test string!".to_string().into_bytes();
let huffman_data: HuffmanData = huffman_encode(&data);
let encoded_data: Vec<u8> = huffman_data.encoded_data; // The given data encoded
let encoding_map: HashMap<u8,String> = huffman_data.encoding_map; // The encoding map required to decode the data

```

[![Crate][crate_img]][crate]
[![License][license_img]][license_file]
[![Crate Downloads][downloads_img]][crate]
[![Crate Size][loc_img]][loc]
[![Build][actions]][loc]

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
use std::collections::HashMap;

let data: Vec<u8> = "this is a test string!".to_string().into_bytes();
let huffman_data: HuffmanData = HuffmanData::new(&data).unwrap();
let decoded_data: Vec<u8> = huffman_data.decode().unwrap();
assert_eq!(decoded_data,data);

```

<!-- Badges -->
[crate]: https://crates.io/crates/huff-tree-tap "Crate Link"
[crate_img]: https://img.shields.io/crates/v/huff-tree-tap.svg?logo=rust "Crate Page"
[downloads_img]: https://img.shields.io/crates/dv/huff-tree-tap.svg?logo=rust "Crate Downloads"
[license_file]: https://github.com/chimbosonic/huff-tree-tap/blob/master/LICENSE "License File"
[license_img]: https://img.shields.io/crates/l/huff-tree-tap.svg "License Display"
[loc]: https://github.com/chimbosonic/huff-tree-tap "Repository"
[loc_img]: https://tokei.rs/b1/github/chimbosonic/huff-tree-tap?category=code "Repository Size"
[actions]: https://github.com/chimbosonic/huff-tree-tap/actions/workflows/build.yml/badge.svg
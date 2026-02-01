<div align="center">
 
 # PNG You! 🦀

[![Crates.io](https://img.shields.io/crates/v/pngyou.svg)](https://crates.io/crates/pngyou)
[![Docs.rs](https://img.shields.io/docsrs/pngyou)](https://docs.rs/pngyou)
[![License](https://img.shields.io/crates/l/pngyou)]()

</div>

`pngyou` is a Rust library and CLI tool for PNG steganography for embedding, extracting, and managing hidden messages in PNG chunks.

## ⚡️ Features

### Library
* PNG-spec compliant parsing and chunk validation
* Strongly-typed `Chunk` and `ChunkType`
* Safe insertion, removal, and modification of chunks
* Automatic CRC computation for integrity
* Built with extensibility in mind for tooling beyond steganography

### CLI tool
* Encode secret messages into PNG chunk types
* Decode hidden messages by chunk type
* Remove custom chunks from a PNG file
* View PNG file bytes

---

## 🚀 Installation

### For installing lib,

Add this to your `Cargo.toml`:

```toml
[dependencies]
pngyou = "0.1"
```

OR,

```sh
cargo add pngyou
```

### For installing CLI tool

```sh
cargo install pngyou
```

## 💫 Features
- [x] **Encode:** Hide messages in PNG chunks.
- [x] **Decode:** Retrieve messages by chunk type.
- [x] **Remove:** Strip custom chunks from files.
- [ ] **Format Support:** Support for other chunk-based formats.
- [ ] **Detection:** Automated scanning for hidden data.
- [ ] **URL Inputs:** Support for automatically fetching an image from the internet.
- [ ] **Obfuscation:** Automatically obfuscate your messages before encoding.

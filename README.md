# rust-edf
A library written in Rust for reading EDF file localy

It uses [this library](https://github.com/mleprince/rust-edf-reader) under the hood. I didn't put this code in this library because I wanted to use the library in WebAssembly and WebAssembly [does not support std lib](https://rustwasm.github.io/docs/book/reference/add-wasm-support-to-crate.html#avoid-performing-io-directly).

[![](https://img.shields.io/crates/v/local-edf-reader.svg)](https://crates.io/crates/edf-reader)

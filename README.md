[中文](README.zh.md)
# promptpro
This is a small tool that generates AI prompts. Currently, it supports bulk random generation for MidJourney.
Please place the config.toml in the same directory as the program's statistics directory
## About

## Usage
Build the tool
Copy code
```
cargo build
```
Run the tool
```
cargo run
```
To run it as a WebAssembly (wasm) module:
```
cargo install wasm-pack
//
asm-pack build --target web --dev
// or wasm-pack build --release --target web
// then
python3 -m http.server
```
Access the page at http://localhost:8000






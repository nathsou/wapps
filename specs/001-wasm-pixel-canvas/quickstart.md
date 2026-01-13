# Quickstart: Portable WASM Apps

**Feature**: `001-wasm-pixel-canvas`

## Prerequisites
- Rust 1.75+
- SDL2 Development Libraries (install via system package manager)
- `wasm32-wasi` target: `rustup target add wasm32-wasi`

## 1. Building the Host

The Host is the runner application.

```bash
cd host
cargo run --release -- ../examples/demo.wapp
```

## 2. Building a Guest App

A Guest App is a Rust project targeting `wasm32-wasi`.

### `Cargo.toml`
```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# No special dependencies required for raw FFI, 
# but a 'wapps-guest' helper crate is recommended later.
```

### `src/lib.rs`
```rust
#[link(wasm_import_module = "wapps")]
extern "C" {
    fn update_frame(width: i32, height: i32, pixels: *const u8);
}

#[no_mangle]
pub extern "C" fn update(dt: f64) {
    // 1. Draw to a static buffer
    // 2. Call unsafe { update_frame(...) }
}
```

### 3. Packaging

To create a `.wapp` file, you must prepend the header to the WASM binary.

```bash
# Build WASM
cargo build --target wasm32-wasi --release

# Package (using a helper script or tool)
printf "WAPP\x01" > demo.wapp
cat target/wasm32-wasi/release/demo.wasm >> demo.wapp
```

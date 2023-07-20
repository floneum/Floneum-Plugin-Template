# Floneum Plugin Template

> A template to get you started creating a [Floneum](https://floneum.com) plugin

## Usage

Before you build your plugin, you must install [rust](https://rustup.rs/) and the `wasm32-wasi` toolchain.

```sh
rustup target add wasm32-wasi
```

You can build the project with:

```sh
cargo build --release --target wasm32-wasi
```

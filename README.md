# Floneum Plugin Template

> A template to get you started creating a [Floneum](https://floneum.com) plugin

## Usage

Before you build your plugin, you must install [rust](https://rustup.rs/) and the `wasm32-wasi` toolchain.

```sh
rustup target add wasm32-wasi
```

And install the Floneum CLI:

```sh
cargo install --git https://github.com/floneum/floneum floneum-cli
```

You can build the project with:

```sh
floneum build --release
```

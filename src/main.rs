// Build with:
// rustup target add wasm32-wasi
// cargo build --release --target wasm32-wasi

use floneum_rust::*;

#[export_plugin]
/// Plugin description
fn plugin_name(
    /// Input description
    input: String
) -> String {
    Logger::new(log::Level::Info).register();
    log::info!("received: {}", template);
    input
}

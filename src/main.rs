// Build with:
// rustup target add wasm32-wasi
// cargo install --git https://github.com/floneum/floneum floneum-cli
// floneum build --release

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

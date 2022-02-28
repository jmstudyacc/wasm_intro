/*
Enabling WASM:
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve - starts a webserver to test the WASM on
*/

use seed::window;

fn main() {
    // assigning window() to window
    let window = window();
    window.alert_with_message("Hello World! This will appear in a window in the browser :)");
}

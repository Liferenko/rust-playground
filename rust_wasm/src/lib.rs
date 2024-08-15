// TODO: add entrypoint test: send req, send resp, show its content
//

use wasm_bindgen::prelude::*;
// use wasm_bindgen_test::console_log;
use web_sys::console;

// TODO: from websocket example

// pub struct Promise<T: Send, E: Send> {/* TODO: */}
// // https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Call you await
    // TODO: smells like poopy. Need to rework it
    let response = ws_ping("/snacks", "Spicy chips");

    console::log_1(&JsValue::from_str("Initialized"));

    console::log_1(&JsValue::from_str(&response));

    Ok(())
}

// ref - wsPing(endpoint: string, message: string): Promise<string>
// TODO:
// - result should be  Promise<string>
#[wasm_bindgen]
// pub async fn ws_ping(endpoint: &str, message: &str) -> String {
pub fn ws_ping(endpoint: &str, message: &str) -> String {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // TODO:
    alert(&format!("Sup, {} {}!", endpoint, message));

    format!("Sup, {} {}!", endpoint, message).to_string()
}

// TODO: REMOVE BEFORE FLIGHT!!!!!!
//
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // #[test]
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
//
//     #[test]
//     fn endpoint_test() {
//         assert_eq!(ws_ping("/snacks", "Spicy chips!"), "Sup, Spicy chips!");
//     }
// }

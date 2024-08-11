// TODO: add entrypoint test: send req, send resp, show its content
//

use wasm_bindgen::prelude::*;

// pub struct Promise<T: Send, E: Send> {/* TODO: */}
// // https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// ref - wsPing(endpoint: string, message: string): Promise<string>
// TODO:
// - result should be  Promise<string>
#[wasm_bindgen]
// pub async fn ws_ping(endpoint: &str, message: &str) -> String {
pub fn ws_ping(endpoint: &str, message: &str) -> String {
    // TODO:
    // alert(&format!("Sup, {} {}!", endpoint, message));

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
//     fn entrypoint_test() {
//         assert_eq!(greet("Spicy chips!"), "Sup, Spicy chips!");
//     }
// }

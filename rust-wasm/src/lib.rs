// TODO: add entrypoint test: send req, send resp, show its content
//

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Sup, {}!", name));
}

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

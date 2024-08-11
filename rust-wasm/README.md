## Simple web application: Wasm + web sockets.

- A Rust library that should be compiled to Wasm and export one function (Typescript syntax)
wsPing(endpoint: string, message: string): Promise<string>
- This function should establish a web socket connection to the "endpoint" and send the text message, receive a message, and return its content.
- Any UI will be sufficient, but a simple test executed by nodejs/deno/bun would be OK. 
- Rust + wasm-bindings + anything else you need.

##### TODO:
NB! is it a lib with macro? I mean Rust -> TS?
- [x] implement a dummy Rust lib
- [x] a simple test executed wasm-pack
- [ ] a simple test executed by nodejs/deno/bun (my note: I'd choose bun)
- [ ] export a wsPing method in JS file
- [ ] add a method `wsPing(endpoint: string, message: string): Promise<string>`
- [ ] establish a web socket connection to the "endpoint"
    - [ ] send the text message
    - [ ] receive a message,
    - [ ] and return its content

##### DoD:
- [x] Is it in Rust?
- [x] Is it a library?
- [ ] Does it compiles into Wasm?
- [ ] Does it export one function wsPing/2?
- [ ] Does exported function have Promise<string> as a return?
- [ ] Does the function establish a web socket connection to the "endpoint"?
- [ ] Does it send the text message? receive a message, and return its content?
- [ ] Does it receive a message? and return its content?
- [ ] Does it return its content?
- [ ] Is it possible to test it using `make bun_test`?
- [ ] Does the final stack contain `Rust + wasm-bindings + ...`?
- [ ] Did you remove all `// REMOVE BEFORE FLIGHT!!!!!!` and TODOs?
- [ ] Have you already sent this repo's link to this task?


#### How to use
- `make wasm_test`
- `make bun_test`







#### Sources:
- https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html

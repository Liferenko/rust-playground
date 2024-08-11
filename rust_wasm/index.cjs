// ref - few ways to import - https://modernjs.dev/en/guides/basic-features/wasm-assets.html
// const rust_wasm = import("./pkg/rust_wasm");
//
// rust_wasm.ws_ping("TODO endpoint", "message");
//
//
//

// old morning 11.08
import("./pkg/rust_wasm.js").then(rwasm => {
	rwasm.ws_ping("TODO endpoint", "message");
}).catch(error => {
	console.error("Failed to load WASM module:", error);
});

// const wasmModule = require("./pkg/rust_wasm.js");
// console.log("ws_ping", wasmModule.ws_ping("entrypoint", "msg"))

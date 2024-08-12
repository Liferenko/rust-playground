## Rust Wasm Tokio

#### Requirements:
The completed tasks should be sent by August 17, 00:01 a.m.
The acceptable format of the tasks:
- Github repositories;
- Clean carefully written code;
- Well-commented code;
- Readme files.

---

### Description
There are 3 test challenges. You could take at least one.


#### 1: ZK knowledge proof

- Please find the attached Python source file. Please implement the same primitive in Rust. 
- Please use any suitable from RustCrypto. 

##### TODO:
- [ ] decompose what py-script is doing
- [ ] get the idea what ZK Proof means in this task

- [ ] test it out:
    - [ ] add dummy test for `pytest dlog_proof.py`
    - [ ] add dummy test for `cargo test`
- [] RustCrypto - check what it can do 
- [] draft:
    - [ ] py class DLogProof -> RS struct and impl
    - [ ] py class DLogProofField -> RS impl with supertrait SerializerField and inherited trait DLogProof (it uses `.to_dict()`)
        - [ ] inherit `dlog_proof:to_dict()`
        - [ ] inherit `dlog_proof:from_dict(data)`

#### 2: Simple web application: Wasm + web sockets.

- A Rust library that should be compiled to Wasm and export one function (Typescript syntax)
wsPing(endpoint: string, message: string): Promise<string>
- This function should establish a web socket connection to the "endpoint" and send the text message, receive a message, and return its content.
- Any UI will be sufficient, but a simple test executed by nodejs/deno/bun would be OK. 
- Rust + wasm-bindings + anything else you need.

##### TODO:
NB! is it a lib with macro? I mean Rust -> TS?
- [ ] implement a dummy Rust lib
- [ ] a simple test executed by nodejs/deno/bun (my note: I'd choose bun)
- [ ] add a method `wsPing(endpoint: string, message: string): Promise<string>`
- [ ] establish a web socket connection to the "endpoint"
    - [ ] send the text message
    - [ ] receive a message,
    - [ ] and return its content
---
Resources: 
- how to import async wasm - https://modernjs.dev/en/guides/basic-features/wasm-assets.html

#### 3: Cloud sync-point.

- Small web service with one endpoint: /wait-for-second-party/:unique-id
- This endpoint allows two parties to sync. When one party makes a POST request, the response will be delayed until the second party requests the same URL. In other words, the first party is blocked until the second party arrives or a timeout occurs (let it be 10 seconds).
- Rust: tokio + anything else you need.

##### TODO:
- [ ] implement a dummy web service with ping-pong
- [ ] add a test for ping-pong to check it
- [ ] add a test for an endpoint `/wait-for-second-party/:unique-id`
    - [ ] run test `cargo test`
    - [ ] add test case with 1st party request and no response
    - [ ] add test case with 1st party request and 2nd party 
- [ ] handle 1st party's POST request
- [ ] wait for 2nd party's POST request
- [ ] handle a response when 2nd party's request appears
- [ ] add 10 sec response timeout
    - [ ] handle 10+ seconds no-response case

---

#### Current task's stack:
- https://tokio.rs/


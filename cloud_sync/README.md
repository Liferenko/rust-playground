## Cloud sync point

- Small web service with one endpoint: /wait-for-second-party/:unique-id
- This endpoint allows two parties to sync. When one party makes a POST request, the response will be delayed until the second party requests the same URL. In other words, the first party is blocked until the second party arrives or a timeout occurs (let it be 10 seconds).
- Rust: tokio + anything else you need.

##### TODO:
- [x] implement dummy tokio client
- [x] implement dummy tokio server
- [ ] implement a dummy web service with ping-pong
    - [x] make dummy ping-pong
    - [x] make dummy ping-pong from cli args
    - [x] make dummy ping-pong with N sec delay
    - [ ] make dummy ping-pong when 2nd party appears
    - [x] remove mini_redis from main.rs and replace it with generic one
    - [x] donno how exactly. Maybe after Tokio tutorial it will be better
- [ ] add a test for ping-pong to check it
- [ ] add a test for an endpoint `/wait-for-second-party/:unique-id`
    - [x] run test `cargo test`
    - [ ] add test case with 1st party request and no response
    - [ ] add test case with 1st party request and 2nd party 
- [x] add endpoint `/wait-for-second-party/:unique-id` in Tokio's router
- [x] handle 1st party's POST request
- [x] wait for 2nd party's POST request
- [x] handle a response when 2nd party's request appears
- [x] add 10 sec response timeout
    - [x] handle 10+ seconds no-response case
- [x] add "How to use" in README 


#### How to use
- run server app - `cargo run`
- run client app - `cargo run --example client`
- test - `cargo test`

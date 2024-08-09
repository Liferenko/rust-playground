## Cloud sync point

- Small web service with one endpoint: /wait-for-second-party/:unique-id
- This endpoint allows two parties to sync. When one party makes a POST request, the response will be delayed until the second party requests the same URL. In other words, the first party is blocked until the second party arrives or a timeout occurs (let it be 10 seconds).
- Rust: tokio + anything else you need.

##### TODO:
- [ ] implement a dummy web service with ping-pong
- [ ] implement dummy tokio client and server
    - [ ] donno how exactly. Maybe after Tokio tutorial it will be better
- [ ] add a test for ping-pong to check it
- [ ] add a test for an endpoint `/wait-for-second-party/:unique-id`
    - [x] run test `cargo test`
    - [ ] add test case with 1st party request and no response
    - [ ] add test case with 1st party request and 2nd party 
- [ ] handle 1st party's POST request
- [ ] wait for 2nd party's POST request
- [ ] handle a response when 2nd party's request appears
- [ ] add 10 sec response timeout
    - [ ] handle 10+ seconds no-response case


#### Test
`cargo test`


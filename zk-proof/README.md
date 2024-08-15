#### 1: ZK knowledge proof

- Please find the attached Python source file. Please implement the same primitive in Rust. 
- Please use any suitable from RustCrypto. 

##### TODO:
- [ ] decompose what py-script is doing
- [ ] get the idea what ZK Proof means in this task

- [ ] test it out:
    - [ ] add dummy test for `pytest dlog_proof.py`
        - [ ] resolve an issue with htss_ecdsa lib 
    - [ ] add dummy test for `cargo test`
- [] RustCrypto - check what it can do 
- [] draft:
    - [ ] py class DLogProof -> RS struct and impl
    - [ ] py class DLogProofField -> RS impl with supertrait SerializerField and inherited trait DLogProof (it uses `.to_dict()`)
        - [ ] inherit `dlog_proof:to_dict()`
        - [ ] inherit `dlog_proof:from_dict(data)`

#### Glossari:
- ZK - zero-knowledge
- DLOG - discrete log

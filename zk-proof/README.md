#### 1: ZK knowledge proof

- Please find the attached Python source file. Please implement the same primitive in Rust. 
- Please use any suitable from RustCrypto. 

##### TODO:
- [x] decompose what py-script is doing
- [x] get the idea what ZK Proof means in this task

- [ ] test it out:
    - [ ] add dummy test for `pytest dlog_proof.py`
        - [ ] resolve an issue with htss_ecdsa lib 
        - [x] add curve = todo!("ecdsa::p256");
        - [x] add generator = todo!("curve.generator");
        - [x] add q = todo!("curve.order");
- [x] RustCrypto - check what it can do 
- [x] draft:
    - [x] py class DLogProof -> RS struct and impl
    - [x] py class DLogProofField -> RS impl with supertrait SerializerField and inherited trait DLogProof (it uses `.to_dict()`)
        - [x] inherit `dlog_proof:to_dict()`
        - [ ] inherit `dlog_proof:from_dict(data)`

#### Glossari:
- ZK - zero-knowledge (read more (paywall alarm) - https://rabmcmenemy.medium.com/zero-knowledge-proofs-zk-proofs-in-rust-2f65c43d3458 - )
- DLOG - discrete log
- ECDSA - Elliptic Curve Digital Signature Algorithm

#### Resourses:
- https://github.com/RustCrypto

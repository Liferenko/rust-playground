// use json; // TODO find in rs
// use secrets; // TODO find in rs
// use core::time;
use chrono::Utc;
use std::fmt::format;
use std::collections::HashMap;
use getrandom;

use crypto_bigint::{U256, Encoding};
use ecdsa;
use ecdsa::elliptic_curve; // TODO find PointJacobi
use std::hash::Hash;
use sha256::digest;



static curve = todo!("ecdsa::p256");
static generator = todo!("curve.generator");
static q = todo!("curve.order");



fn generate_random_number() -> Uint {
    U256::from_le_bytes(getrandom::getrandom(&mut [0u8; 32])?)
}

#[derive(PartialEq)]
struct DLogProof {
    fn 

    fn _hash_points(sid: str, pid: u8, points: [PointJacobi]) -> U256  // TODO replace PointJacobi

    fn prove(sid: &str, pid: u8, x: u128, y: PointJacobi, base_point: PointJacobi = G) -> boolean,

    fn verify(self, sid: &str, pid: u8, y: PointJacobi, base_point: PointJacobi = G) -> boolean,

    fn from_dict(data: HashMap<K, V>),
    fn to_dict(self) -> HashMap<K, V>,
    fn to_str(self) -> str

}

impl DLogProof {
    let curve = todo!("ecdsa::p256");
    let generator = todo!("curve.generator");
    let q = todo!("curve.order");


    // TODO: find out how to impl __init__

    fn prove(sid: &str, pid: u8, x: u128, y: PointJacobi, base_point: PointJacobi = G) {
        let r = generate_random_number();
        let t = r * base_point;
        let c = DLogProof._hash_points(sid, pid, [base_point, y, t]);
        let s = (r + c * x) % DLogProof.q;

        DLogProof(t, s);
        todo!()
    }

    fn verify(self, sid: &str, pid: u8, y: PointJacobi, base_point: PointJacobi = G) {
        let c = self._hash_points(sid, pid, [base_point, y, t]);
        let lhs = self.s * base_point;
        let rhs = self.t + (y * c);
        lhs == rhs;
        todo!()
    }

    fn to_dict(self) {
        HashMap{
            "t": todo!(),
            "s": todo!()
        }
        todo!()
    }

    fn to_str(self) {
        json::dumps(self.to_dict());
        todo!()
    }
    
    fn from_dict(data: HashMap<K, V>) {
        todo!()
    }
}

// TODO: find out how to impl __eq__
// TODO: find out how to impl __ne__
impl PartialEq for DLogProof {
    fn eq(&self, other) {
        assert!(type(other), DLogProofField, "Can only compare DLogProofs") // TODO:
        // doublecheck `type(other)` and assert!/3 with msg as 3rd arg
    }
} 


struct DLogProofField {
    fn serialize(self, dlog_proof) -> HashMap<K, V>;
    fn deserialize(self, data: HashMap<K, V>) -> DLogProof;
}

impl SerializerField from DLogProofField {
    fn serialize(self, dlog_proof) {
        dlog_proof.to_dict()
    };

    fn deserialize(self, data: HashMap) {
        DLogProof.from_dict(data)
    };
}

fn main() {
    let sid = "sid";
    let pid = 1;

    let x = generate_random_number();
    println!(x);
    let y = x * generator; // TODO:
    

    let start_proof = Utc::now();
    let dlog_proof = DLogProof.prove(sid, pid, x, y);

    println!("Proof computation time: {} ms", format!((Utc::now() - start_proof) * 1000) );

    println!("");
    println!(dlog_proof.t.x(), dlog_proof.t.y());
    println!(dlog_proof.s);

    let start_verify = Utc::now();
    let result = dlog_proof.verify(sid, pid, y);

    println!("Verify computation time: {} ms", format!((Utc::now() - start_verify) * 1000));

    if result {
        println!("DLog proof is correct");
    } else {
        println!("DLog proof is not correct");
    }
}

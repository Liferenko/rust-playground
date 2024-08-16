// use json; // TODO find in rs
// use secrets; // TODO find in rs
// use core::time;
use chrono::Utc;
use crypto_bigint::consts::U256;
use core::str;
use std::fmt::format;
use std::collections::HashMap;
use getrandom;

use crypto_bigint::{U256, Encoding};
use ecdsa::{self, ECDSA_SHA224_OID};
use ecdsa::elliptic_curve::{self, point}; // TODO find PointJacobi
use std::hash::Hash;
use sha256::digest;



const curve = todo!("ecdsa::p256"); //TODO: shall I use static instead?
const G = todo!("curve.generator");
const Q = todo!("curve.order");



fn generate_random_number() -> U256 {
    U256::from_le_bytes(getrandom::getrandom(&mut [0u8; 32])?)
}

// TODO:
// - define PointJacobi
#[derive(PartialEq)]
struct DLogProof {
    fn 

    fn _hash_points(sid: &str, pid: u8, points: [PointJacobi]) -> U256  // TODO replace PointJacobi

    fn prove(sid: &str, pid: u8, x: u128, y: PointJacobi, base_point: PointJacobi = G) -> boolean,

    fn verify(self, sid: &str, pid: u8, y: PointJacobi, base_point: PointJacobi = G) -> boolean,

    fn from_dict(data: HashMap<K, V>),
    fn to_dict(self) -> HashMap<K, V>,
    fn to_str(self) -> str

}

impl DLogProof {
    // TODO: find out how to impl __init__
    fn new(self, t: PointJacobi, s: u8) {
        self.t = t;
        self.s = s;
        // TODO looks bad. Need to double-check those self.* entities
    }
    
    fn _hash_points(self, sid: &str, pid: u8, points: Vec<PointJacobi>) {
        let string_field = ""; // TODO: StringField
        let bigint_field = U256::new();
        let pount_field = ECDSA_SHA224_OID; // TODO:

        let mut digest_data = String::new();
        digest_data.push_str(string_field.to_bytes(sid)); // TODO: replace .to_bytes(sid)
        digest_data.push(bigint_field.to_bytes(pid)); // TODO: replace .to_bytes(sid) && .push or
        // .push_str?
        
        for point in points {
            digest_data.push(pount_field.to_bytes(point));
        }

        let digest = sha256::digest(digest_data);

        U256::from(digest);
    }


    fn prove(self, sid: &str, pid: u8, x: u128, y: PointJacobi, base_point: PointJacobi = G) {
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
// TODO: find out how to impl __ne__  - check this one https://docs.rs/ec_core/latest/ec_core/elliptic_curve/struct.EllipticCurve.html#method.ne
impl PartialEq<EllipticCurve> for DLogProof {
    fn eq(&self, other) -> bool {
        assert!(type(other), DLogProofField, "Can only compare DLogProofs") // TODO:
        // doublecheck `type(other)` and assert!/3 with msg as 3rd arg
    }

    fn ne(&self, other: &EllipticCurve) -> bool {
        todo!()
        // ref - https://docs.rs/ec_core/latest/ec_core/elliptic_curve/struct.EllipticCurve.html#method.ne
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

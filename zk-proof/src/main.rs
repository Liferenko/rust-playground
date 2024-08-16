use chrono::Utc;
use crypto_bigint::Encoding;
use core::str;
use std::fmt::format;
use std::collections::HashMap;
use getrandom;

use k256::{elliptic_curve::Field, ProjectivePoint, Scalar, U256, sha2::Sha256};

//# TODO: REMOVE BEFORE FLIGHT!!!!!!
// use crypto_bigint::{U256, Encoding};
// use crypto_bigint::consts::U256;
// use ecdsa::{self, ECDSA_SHA224_OID};
// use ecdsa::elliptic_curve::{self, point, ScalarPrimitive}; // TODO find ProjectivePoint
use sha256::digest;



const G = ProjectivePoint::GENERATOR;
const Q = todo!() // TODO: define which order shoyld be in use here



fn generate_random_number() -> U256 {
    U256::from_le_bytes(getrandom::getrandom(&mut [0u8; 32])?)
}

// TODO:
// - define ProjectivePoint
#[derive(PartialEq)]
struct DLogProof {
    t: ProjectivePoint,
    s: Scalar,

    // Sidenote: seems like these pre-def methods are not required. Need to doublecheck
    // fn new(t: ProjectivePoint, s: Scalar) -> Self,
    //
    // fn _hash_points(sid: &str, pid: u8, points: [ProjectivePoint]) -> U256  // TODO replace ProjectivePoint
    //
    // fn prove(sid: &str, pid: u8, x: u128, y: ProjectivePoint, base_point: ProjectivePoint = G) -> boolean,
    //
    // fn verify(self, sid: &str, pid: u8, y: ProjectivePoint, base_point: ProjectivePoint = G) -> boolean,
    //
    // fn from_dict(data: HashMap<K, V>),
    // fn to_dict(self) -> HashMap<K, V>,
    // fn to_str(self) -> str
    //
}

impl DLogProof {
    fn new(t: ProjectivePoint, s: Scalar) -> Self {
        DLogProof{t, s};
    }
    
    fn _hash_points(self, sid: &str, pid: u8, points: [ProjectivePoint]) -> Scalar {
        let string_field = ""; // TODO: StringField
        let bigint_field = U256::new(32);
        let point_field = ProjectivePoint; // TODO:

        let mut digest_data = Sha256::new();
        digest_data.update(string_field.to_bytes(sid));
        digest_data.update(bigint_field.to_bytes(&pid));
        
        for point in points {
            digest_data.update(point_field.to_bytes(point));
        }

        let digest = sha256::digest(digest_data);

        U256::from_be_bytes(digest);
    }


    fn prove(self, sid: &str, pid: u8, x: u128, y: ProjectivePoint) -> DLogProof { // TODO: G?
        let r = generate_random_number();
        let t = r * G;
        let c = self._hash_points(sid, pid, [G, y, t]);
        let s = (r + c * x) % Q;

        DLogProof::new(t, s)
    }

    fn verify(self, sid: &str, pid: u8, y: ProjectivePoint) -> bool {
        let c = self._hash_points(sid, pid, [G, y, self.t]);
        let lhs = self.s * G;
        let rhs = self.t + (y * c);

        lhs == rhs
    }

    fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap()
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

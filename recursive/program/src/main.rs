#![no_main]
sp1_zkvm::entrypoint!(main);

use gnark_bn254_verifier::{PlonkVerifier, Verifier};
use substrate_bn::Fr;

pub fn main() {
    let proof = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vk = sp1_zkvm::io::read_vec()[8..].to_vec();
    let vkey_hash = sp1_zkvm::io::read_vec()[8..].to_vec();
    let commitment_values_digest = sp1_zkvm::io::read_vec()[8..].to_vec();

    let proof = proof;
    let vk = vk;
    let vkey_hash = Fr::from_slice(&vkey_hash).expect("Unable to read vkey_hash");
    let committed_values_digest = Fr::from_slice(&commitment_values_digest[8..])
        .expect("Unable to read committed_values_digest");
    println!(
        "[READ] commited_values_digest: {:?}",
        committed_values_digest
    );

    println!("cycle-tracker-start: setup");
    if PlonkVerifier::verify(&proof, &vk, &[vkey_hash, committed_values_digest]) {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }
    println!("cycle-tracker-end: setup");
}

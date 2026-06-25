//This code is written to check the deterministic behaviour of the Crate. If we provide the same seed and same message, the public key and the signature should be same every time. This property is required in many circumstances for example Blockchain. Because the signatre size is huge for slh-dsa, the the code display the sha2 hash of signature for comparison purpose. 


#![allow(unused_imports)]
#[allow(dead_code)]

use purecrypto::hash::{Sha256, Digest};
use purecrypto::slhdsa::{ParamSet, PrivateKey};
use std::io::{self, Write};


// Normalizing seed to 32 bytes. If smaller, append 'a's and if larger, trim
fn normalize_seed(mut seed: Vec<u8>) -> [u8; 32] {
    const LEN: usize = 32;

    if seed.len() < LEN {
        seed.resize(LEN, b'a');
    } else if seed.len() > LEN {
        seed.truncate(LEN);
    }

    let mut out = [0u8; 32];
    out.copy_from_slice(&seed);
    out
}


fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    let result = h.finalize();

    result.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn get_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    println!("SLH-DSA Signature Determinism Checker...\n");

    let seed_input = get_input("Enter your seed string: ");
    let msg_input = get_input("Enter your message for signing: ");

    let seed_bytes = normalize_seed(seed_input.into_bytes());
    let msg = msg_input.as_bytes();

    println!("\n--- Running all 12 SLH-DSA variants ---\n");

    let variants = [
        ParamSet::Sha2_128s,
        ParamSet::Sha2_128f,
        ParamSet::Sha2_192s,
        ParamSet::Sha2_192f,
        ParamSet::Sha2_256s,
        ParamSet::Sha2_256f,
        ParamSet::Shake_128s,
        ParamSet::Shake_128f,
        ParamSet::Shake_192s,
        ParamSet::Shake_192f,
        ParamSet::Shake_256s,
        ParamSet::Shake_256f,
    ];

    for v in variants {
        let sk_seed = &seed_bytes;
        let sk_prf = &seed_bytes;
        let pk_seed = &seed_bytes;

        let (sk, pk) = PrivateKey::from_seeds(v, sk_seed, sk_prf, pk_seed);

        let sig = sk
            .sign_deterministic(msg, b"")
            .expect("sign failed");

        let sig_hash = sha256_hex(&sig);

        println!("====================================================");
        println!("SLH-DSA Variant: {:?}", v);

        println!("Public Key (hex):");
        println!("{}", hex(pk.to_bytes()));

        println!("\nSignature SHA-256:");
        println!("{}", sig_hash);

        println!("Signature size: {} bytes", sig.len());
    }
}


#![allow(unused_imports)]
#[allow(dead_code)]


/// This use all variants of SLH-DSA to sign a message. 
/// Input: This code ask a seed of 32 bytes from user. If the seed is less than 32 bytes, append 'a' to complete 32 bytes or if larger than 32 bytes, trim to make 32 bytes. The code also ask a message to sign.
///Output: Key generation, signing and verification using all 12 variants for benchmarking
///Output: Public key size, singature size, key generation latency, signing time latency and verification result



use purecrypto::slhdsa::{ParamSet, PrivateKey, PublicKey};
use sha2::{Sha256, Digest};

use std::io::{self, Write};
use std::time::Instant;


/// Normalize seed to exactly 32 bytes, is smaller, append 'a's or trim if larger
fn normalize_seed(input: &str) -> [u8; 32] {
    let mut out = [b'a'; 32];
    let bytes = input.as_bytes();
    let len = bytes.len().min(32);
    out[..len].copy_from_slice(&bytes[..len]);
    out
}

/// Deterministic seed expansion
fn derive_seed(base: &[u8; 32], label: u8, out_len: usize) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(base);
    h.update([label]);

    let digest = h.finalize();

    let mut out = vec![0u8; out_len];
    out.copy_from_slice(&digest[..out_len]);
    out
}


fn get_n(set: ParamSet) -> usize {
    set.signature_size(); 
    match set {
        ParamSet::Sha2_128s | ParamSet::Sha2_128f | ParamSet::Shake_128s | ParamSet::Shake_128f => 16,
        ParamSet::Sha2_192s | ParamSet::Sha2_192f | ParamSet::Shake_192s | ParamSet::Shake_192f => 24,
        ParamSet::Sha2_256s | ParamSet::Sha2_256f | ParamSet::Shake_256s | ParamSet::Shake_256f => 32,
    }
}

fn run_variant(set: ParamSet, seed32: &[u8; 32], msg: &[u8]) {
    let n = get_n(set);

    let sk_seed = derive_seed(seed32, 0, n);
    let sk_prf  = derive_seed(seed32, 1, n);
    let pk_seed = derive_seed(seed32, 2, n);

    // KEYGEN functionality
    let t0 = Instant::now();
    let (sk, pk) = PrivateKey::from_seeds(set, &sk_seed, &sk_prf, &pk_seed);
    let keygen_time = t0.elapsed();

    // SIGN functionality
    let t1 = Instant::now();
    let sig = sk.sign_deterministic(msg, b"").unwrap();
    let sign_time = t1.elapsed();

    // VERIFY functionality
    let ok = pk.verify(&sig, msg, b"");

    println!("==============================");
    println!("SLH-DSA Variant       : {:?}", set);
    println!("Security parameter n             : {}", n);
    println!("PK size       : {} bytes", pk.to_bytes().len());
    println!("Sig size      : {} bytes", sig.len());
    println!("Keygen time   : {:?}", keygen_time);
    println!("Sign time     : {:?}", sign_time);
    println!("Verify result : {}", ok);
}

fn main() {

    print!("Enter your seed string: ");
    io::stdout().flush().unwrap();

    let mut seed_input = String::new();
    io::stdin().read_line(&mut seed_input).unwrap();

    print!("Enter your message for signing: ");
    io::stdout().flush().unwrap();

    let mut msg_input = String::new();
    io::stdin().read_line(&mut msg_input).unwrap();

    let seed32 = normalize_seed(seed_input.trim());
    let msg = msg_input.trim().as_bytes();

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
        run_variant(v, &seed32, msg);
    }
}

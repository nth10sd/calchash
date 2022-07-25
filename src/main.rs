use sha2;
use sha3::{Digest, Sha3_256};
use std::fs;
use std::io;
fn main() {
    let mut hasher_b2 = blake3::Hasher::new();
    let mut file_b2 = fs::File::open("Cargo.toml").unwrap();

    io::copy(&mut file_b2, &mut hasher_b2).unwrap();
    let hash_bytes_b2 = hasher_b2.finalize();
    println!("\nBlake3 Hash: {:?}", hash_bytes_b2);

    let mut hasher = sha2::Sha256::new();
    let mut file = fs::File::open("Cargo.toml").unwrap();

    io::copy(&mut file, &mut hasher).unwrap();
    let hash_bytes = hasher.finalize();
    println!("\nSHA-256 Hash: {:?}", hex::encode(&hash_bytes));

    let mut hasher3 = Sha3_256::new();
    let mut file3 = fs::File::open("Cargo.toml").unwrap();

    io::copy(&mut file3, &mut hasher3).unwrap();
    let hash_bytes3 = hasher3.finalize();
    let hex_string3 = hex::encode(&hash_bytes3);
    println!("\nSHA3-256 Hash: {:?}", hex_string3);
}

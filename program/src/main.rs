#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;

sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use sp1_zkvm::io;

pub fn main() {
    let message: Vec<u8> = io::read_vec();

    // Hash the message using SHA-256 with precompiles
    let hash = sha256_with_precompile(&message);

    // Commit the hash so it becomes part of the public output
    io::commit(&hash);
}

fn sha256_with_precompile(data: &[u8]) -> [u8; 32] {
    // Use the patched sha2 crate that leverages precompiles
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
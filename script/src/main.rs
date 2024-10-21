use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::time::Instant;

fn main() {
    utils::setup_logger();

    // Include the compiled ELF file
    let elf = include_bytes!("/Users/uttam/workspace/hash_com/program/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program");

    // Input message to hash
    let message = b"Hello, SP1 with precompiles!";

    // Prepare the input
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(message);

    // Create a ProverClient
    let client = ProverClient::new();

    // Execute the program without generating a proof
    let start = Instant::now();
    let (public_values, report) = client.execute(elf, stdin).run().unwrap();
    let duration = start.elapsed();

    println!("Execution Time: {:?}", duration);
    println!("Total Cycles: {}", report.total_instruction_count());

    // Read and display the hash output
    let mut output = public_values.as_slice();
    let hash: [u8; 32] = bincode::deserialize_from(&mut output).unwrap();

    println!("SHA-256 Hash: {:?}", hex::encode(hash));
}
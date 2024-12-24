use common::{Input};
use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use num_bigint::BigUint;
use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo, Receipt};
use std::time::Instant;

fn main() {
    let (base, modulus, range, result) = setup_inputs();

    let env = setup_env(&base, &modulus, &range, &result);

    // Generate proof and get receipt
    let receipt = generate_proof(env).receipt;

    let input: Input = receipt.journal.decode().unwrap();

    println!("Host::Base: {}", input.base);
    println!("Host::Modulus: {}", input.modulus);
    println!("Host::Range: {}", input.range);
    println!("Host::Result: {}", input.result);

    // Verify the proof
    verify_proof(&receipt);
}

fn setup_inputs() -> (BigUint, BigUint, BigUint, BigUint) {
    let input = Input::default();

    println!("Host::Initial Base: {}", input.base);
    println!("Host::Initial Modulus: {}", input.modulus);
    println!("Host::Initial Range: {}", input.range);
    println!(
        "Host::Initial Result of base^exponent % modulus: {}",
        input.result
    );

    (input.base, input.modulus, input.range, input.result)
}

fn setup_env<'a>(
    base: &'a BigUint,
    modulus: &'a BigUint,
    range: &'a BigUint,
    result: &'a BigUint,
) -> ExecutorEnv<'a> {
    let input = Input {
        base: base.clone(),
        modulus: modulus.clone(),
        range: range.clone(),
        result: result.clone(),
    };

    ExecutorEnv::builder()
        .write_slice(&bincode::serialize(&input).unwrap())
        .build()
        .unwrap()
}

fn generate_proof(env: ExecutorEnv) -> ProveInfo {
    println!("Starting proof generation...");
    let proof_start = Instant::now();
    let prove_info = default_prover().prove(env, RANGE_PROOF_ELF).unwrap();
    let proof_duration = proof_start.elapsed();
    println!("Proof generation completed in {:?}", proof_duration);
    prove_info
}

fn verify_proof(receipt: &Receipt) {
    println!("Starting verification...");
    let verify_start = Instant::now();

    receipt.verify(RANGE_PROOF_ID).unwrap();

    let verify_duration = verify_start.elapsed();
    println!("Verification completed in {:?}", verify_duration);

    println!("Verified");
}

use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use common::{BASE, MODULUS, RANGE, EXPONENT};
use num_bigint::BigUint;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, ProveInfo};
use std::str::FromStr;
use std::time::Instant;
use risc0_zkvm::serde::to_vec;



fn main() {
    let (base, modulus, range, result) = setup_inputs();
    
    let env = setup_env(&base, &modulus, &range, &result);

    // Generate proof and get receipt
    let receipt = generate_proof(env).receipt;

    // Verify the proof
    verify_proof(&receipt);
}

fn setup_inputs() -> (BigUint, BigUint, BigUint, BigUint) {
    let base = BigUint::from_str(BASE).expect("Invalid number for Base");
    let modulus = BigUint::from_str(MODULUS).expect("Invalid number for Modulus");
    let range = BigUint::from_str(RANGE).expect("Invalid number for Range");
    // exponent would not be provided
    let exponent = BigUint::from_str(EXPONENT).expect("Invalid number for Exponent");
    let result = base.modpow(&exponent, &modulus);
    (base, modulus, range, result)
}

fn setup_env<'a>(
    base: &'a BigUint, 
    modulus: &'a BigUint, 
    range: &'a BigUint, 
    result: &'a BigUint, 
) -> ExecutorEnv<'a> {
    ExecutorEnv::builder()
        .write(&to_vec(base).unwrap())
        .unwrap()
        .write(&to_vec(modulus).unwrap())
        .unwrap()
        .write(&to_vec(range).unwrap())
        .unwrap()
        .write(&to_vec(result).unwrap())
        .unwrap()
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

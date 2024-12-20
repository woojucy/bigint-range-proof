use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use num_bigint::BigUint;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, ProveInfo};
use std::str::FromStr;
use std::time::Instant;
use risc0_zkvm::serde::to_vec;

pub const MODULUS: &str = "8155133734070055735139271277173718200941522166153710213522626777763679009805792017274916613411023848268056376687809186180768200590914945958831360737612803";
pub const BASE: &str = "4";
pub const RANGE: &str = "4";
pub const EXPONENT: &str = "4";

fn main() {
    let (base, modulus, range, exponent) = setup_inputs();
    
    let env = setup_env(&base, &modulus, &range, &exponent);

    // Generate proof and get receipt
    let receipt = generate_proof(env).receipt;

    // Verify the proof
    verify_proof(&receipt);
}

fn setup_inputs() -> (BigUint, BigUint, BigUint, BigUint) {
    let base = BigUint::from_str(BASE).expect("Invalid number for Base");
    let modulus = BigUint::from_str(MODULUS).expect("Invalid number for Modulus");
    let range = BigUint::from_str(RANGE).expect("Invalid number for Range");
    let exponent = BigUint::from_str(EXPONENT).expect("Invalid number for Exponent");
    (base, modulus,range, exponent)
}

fn setup_env<'a>(
    base: &'a BigUint, 
    modulus: &'a BigUint, 
    range: &'a BigUint, 
    exponent: &'a BigUint
) -> ExecutorEnv<'a> {
    ExecutorEnv::builder()
        .write(&to_vec(base).unwrap())
        .unwrap()
        .write(&to_vec(modulus).unwrap())
        .unwrap()
        .write(&to_vec(range).unwrap())
        .unwrap()
        .write(&to_vec(exponent).unwrap())
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

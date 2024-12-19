use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use num_bigint::BigUint;
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::str::FromStr;
use std::time::Instant;
use risc0_zkvm::serde::to_vec;

pub const PRIME_P: &str = "8155133734070055735139271277173718200941522166153710213522626777763679009805792017274916613411023848268056376687809186180768200590914945958831360737612803";
pub const PRIME_Q: &str = "13379153270147861840625872456862185586039997603014979833900847304743997773803109864546170215161716700184487787472783869920830925415022501258643369350348243";
pub const GENERATOR: &str = "4";

fn main() {
    let p = BigUint::from_str(PRIME_P).expect("Invalid number for P");
    let q = BigUint::from_str(PRIME_Q).expect("Invalid number for Q");
    let generator = BigUint::from_str(GENERATOR).expect("Invalid number for Generator");

    let modulus = p * q;

    let range: BigUint = BigUint::parse_bytes(b"5000", 10).unwrap();
    let exp: BigUint = BigUint::parse_bytes(b"5000", 10).unwrap();

    // Start timing for proof generation
    let proof_start = Instant::now();

    // let input_data = bincode::serialize(&(&generator, &modulus, &range, &exp)).unwrap();
    let env = ExecutorEnv::builder()
        .write(&to_vec(&generator.clone()).unwrap())
        .unwrap()
        .write(&to_vec(&modulus.clone()).unwrap())
        .unwrap()
        .write(&to_vec(&range.clone()).unwrap())
        .unwrap()
        .write(&to_vec(&exp.clone()).unwrap())
        .unwrap()
        .build()
        .unwrap();
    //     .write_slice(&input_data)
    //     .build()
    //     .unwrap();

    let prover = default_prover().prove(env, RANGE_PROOF_ELF).unwrap();
    let receipt = prover.receipt;

    let proof_duration = proof_start.elapsed();
    println!("Proof generation completed in {:?}", proof_duration);

    // Start timing for verification
    let verify_start = Instant::now();

    receipt.verify(RANGE_PROOF_ID).unwrap();

    let verify_duration = verify_start.elapsed();

    println!("Verification completed in {:?}", verify_duration);

    println!("Verified");
}

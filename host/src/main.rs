use common::{print_input, Input};
use methods::{RANGE_PROOF_ELF, RANGE_PROOF_ID};
use num_bigint::BigUint;
use risc0_zkvm::{
    get_prover_server, ExecutorEnv, ExecutorImpl, ProveInfo, ProverOpts, Receipt, VerifierContext,
};
use std::time::Instant;

fn main() {
    let (base, modulus, range, result) = setup_inputs();
    let env = setup_env(&base, &modulus, &range, &result);

    // Generate proof and get receipt
    let receipt = generate_proof(env).receipt;
    let input: Input = receipt.journal.decode().unwrap();
    print_input("Host", &input);

    // Verify the proof
    verify_proof(&receipt);
}

fn setup_inputs() -> (BigUint, BigUint, BigUint, BigUint) {
    let input = Input::default();
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
    let mut exec = ExecutorImpl::from_elf(env, RANGE_PROOF_ELF).unwrap();
    let exec_start = Instant::now();
    let session = exec.run().unwrap();
    let exec_duration = exec_start.elapsed();
    println!("✅ Session execution completed in {:?}", exec_duration);

    let prover = get_prover_server(&ProverOpts::fast()).unwrap();
    let ctx = VerifierContext::default();

    println!("⏳ Starting proof generation...");
    let proof_start = Instant::now();
    let prove_info = prover.prove_session(&ctx, &session).unwrap();
    let proof_duration = proof_start.elapsed();
    println!("✅ Proof generation completed in {:?}", proof_duration);
    prove_info
}

fn verify_proof(receipt: &Receipt) {
    println!("⏳ Starting verification...");
    let verify_start = Instant::now();

    receipt.verify(RANGE_PROOF_ID).unwrap();

    let verify_duration = verify_start.elapsed();
    println!("✅ Verification completed in {:?}", verify_duration);
}

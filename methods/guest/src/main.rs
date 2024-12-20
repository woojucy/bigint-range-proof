use num_bigint::BigUint;
use risc0_zkvm::guest::env;
use std::str::FromStr;
use common::EXPONENT;

// pub const EXPONENT: &str = "8";
pub fn main() {
    let base: BigUint = env::read();
    let modulus: BigUint = env::read();
    let range: BigUint = env::read();
    let result: BigUint = env::read();

    let exponent = BigUint::from_str(EXPONENT).expect("Invalid number for Exponent");
    env::write(&exponent);

    if exponent > range {
        panic!("Range proof generation failed: Exponent ({}) is out of range ({})", exponent, range);
    }

    let calculation = base.modpow(&exponent, &modulus);

    if result != calculation {
        panic!("Range proof generation failed: Result ({}) does not match the calculation ({})", result, calculation);
    }

    env::commit(&(base, modulus, range, calculation));
}
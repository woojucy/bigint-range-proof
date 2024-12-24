use num_bigint::BigUint;
use risc0_zkvm::guest::env;
use std::str::FromStr;
use common::{EXPONENT, Input};
use std::{vec::Vec, io::Read};

pub fn main() {
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = bincode::deserialize(&input_bytes).unwrap();
    let base = input.base;
    let modulus = input.modulus;
    let range = input.range;
    let result = input.result;

    let exponent = BigUint::from_str(EXPONENT).expect("Guest::Invalid number for Exponent");
    env::write(&exponent);

    if exponent > range {
        panic!("Guest::Range proof generation failed: Exponent ({}) is out of range ({})", exponent, range);
    }

    let calculation = base.modpow(&exponent, &modulus);

    println!("Guest::Base: {}", base);
    println!("Guest::Modulus: {}", modulus);
    println!("Guest::Range: {}", range);
    println!("Guest::Calculation: {}", calculation);

    if result != calculation {
        panic!("Guest::Range proof generation failed: Result ({}) does not match the calculation ({})", result, calculation);
    }

    env::commit(&(base, modulus, range, calculation));
}
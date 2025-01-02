use num_bigint::BigUint;
use risc0_zkvm::guest::env;
use std::str::FromStr;
use common::{EXPONENT, Input, print_input};
use std::{vec::Vec, io::Read};

pub fn main() {
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = bincode::deserialize(&input_bytes).unwrap();
    print_input("Guest", &input);

    let base = input.base;
    let modulus = input.modulus;
    let range = input.range;
    let result = input.result;

    let exponent = BigUint::from_str(EXPONENT).expect("🚨 Invalid number for Exponent");
    env::write(&exponent);

    if exponent > range {
        panic!("🚨 Range proof generation failed: Exponent ({}) is out of range ({})", exponent, range);
    }

    let calculation = base.modpow(&exponent, &modulus);
    println!("✅ Calculation Completed: {}", calculation);

    if result != calculation {
        panic!("🚨 Range proof generation failed: Result ({}) does not match the calculation ({})", result, calculation);
    }

    env::commit(&(base, modulus, range, calculation));
}
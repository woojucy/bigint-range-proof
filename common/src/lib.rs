use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::str::FromStr;

pub const MODULUS: &str = "8155133734070055735139271277173718200941522166153710213522626777763679009805792017274916613411023848268056376687809186180768200590914945958831360737612803";
pub const BASE: &str = "4";
pub const RANGE: &str = "8";
pub const EXPONENT: &str = "2";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub base: BigUint,
    pub modulus: BigUint,
    pub range: BigUint,
    pub result: BigUint, // the result is base^exponent under modulus
}

impl Input {
    // Constructor using default constants
    pub fn default() -> Self {
        Self::new(BASE, MODULUS, RANGE)
    }

    // Constructor that allows custom input
    pub fn new(base_str: &str, modulus_str: &str, range_str: &str) -> Self {
        let base = BigUint::from_str(base_str).expect("Invalid number for Base");
        let modulus = BigUint::from_str(modulus_str).expect("Invalid number for Modulus");
        let range = BigUint::from_str(range_str).expect("Invalid number for Range");

        let result = if modulus.is_zero() {
            BigUint::zero()
        } else {
            Self::calculate_private_modular_exponentiation(&base, &modulus)
        };
        
        println!("Initial parameter settings");
        println!("Base: {}", base);
        println!("Modulus: {}", modulus);
        println!("Range: {}", range);
        println!("Result of base^exponent % modulus: {}", result);

        Input {
            base,
            modulus,
            range,
            result,
        }
    }

    pub fn calculate_private_modular_exponentiation(base: &BigUint, modulus: &BigUint) -> BigUint {
        let exponent = BigUint::from_str(EXPONENT).expect("Invalid number for Exponent");
        if modulus.is_zero() {
            BigUint::zero()
        } else {
            base.modpow(&exponent, modulus)
        }
    }
}

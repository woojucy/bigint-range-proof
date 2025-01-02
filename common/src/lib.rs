use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::str::FromStr;

pub const MODULUS: &str = "8019834928465827340298510934701928311";
pub const BASE: &str = "4";
pub const RANGE: &str = "801983492846582734029851093470192831";
pub const EXPONENT: &str = "325926";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub base: BigUint,
    pub modulus: BigUint,
    pub range: BigUint,
    pub result: BigUint, // the result is base^exponent under modulus
}

impl Input {
    // Constructor using default constants
    #[warn(clippy::should_implement_trait)]
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

        println!("✅ Initial parameter setting finished");

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

pub fn print_input(label: &str, input: &Input) {
    println!("\n----------------------------------------------------");
    println!("🔎 {}::Show Parameters", label);
    println!("====================================================");
    println!("1️⃣ Base:     {}", input.base);
    println!("2️⃣ Modulus:  {}", input.modulus);
    println!("3️⃣ Range:    {}", input.range);
    println!("4️⃣ Result:   {}", input.result);
    println!("----------------------------------------------------\n");
}

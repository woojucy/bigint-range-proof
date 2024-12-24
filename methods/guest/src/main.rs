use num_bigint::BigUint;
use risc0_zkvm::guest::env;

pub fn main() {
    let base: BigUint = env::read();
    let modulus: BigUint = env::read();
    let range: BigUint = env::read();
    let exp: BigUint = env::read();

    if exp > range {
        panic!("Exp is not in range");
    }

    let result = base.modpow(&exp, &modulus);

    env::commit(&(base, modulus, range, result));
}
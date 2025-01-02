# bigint-range-proof
This repository implements range proofs for big integers using the zkVM (zero-knowledge virtual machine).


## How to Run the Code
To run this project, you will need Rust installed on your system. If Rust is not installed, you can install it from [the official Rust website](https://rust-lang.org).


### Running the project:
   ```bash
   cargo run --release
   ```

### Run the host application with printed output:
   ```bash
   cargo run --bin host -- --nocapture
   ```

## Common Module Descruption

### Constants Description

The `common` module defines several constants used for modular exponentiation in big integer calculations:

- **`MODULUS`**: A large prime number used as the modulus for all modular arithmetic operations. It provides a secure, finite field for cryptographic calculations. It supports 2048-bit bit integer.
- **`BASE`**: The base number from which exponentiation starts, serving as the initial value in the exponentiation process.
- **`RANGE`**: Specifies the range within which results are considered valid, helping to define the bounds for proof validation.
- **`EXPONENT`**: The exponent used in the modular exponentiation, determining the power to which the base is raised under the modulus.

### Input Struct Code Description

The `Input` struct is central to managing parameters and results for modular exponentiation within the zkVM (zero-knowledge virtual machine). Below is the detailed implementation and usage of the `Input` struct:

```rust
// Struct to hold the input parameters for modular exponentiation calculations.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub base: BigUint,      // Base value for exponentiation
    pub modulus: BigUint,   // Modulus value under which all operations are performed
    pub range: BigUint,     // Range within which the result should fall
    pub result: BigUint,    // Result of base^exponent % modulus, computed during initialization
}
```

This setup ensures that all necessary cryptographic operations are secure and consistent across different parts of the project, and that the `Input` struct efficiently encapsulates all required functionality for modular exponentiation.

## Structure
```plaintext
bigint-range-proof
│ 
├── common
│   ├── src    
│   │   └── lib.rs           
│   └── Cargo.toml     
│
├── host
│   ├── src    
│   │   └── main.rs           
│   └── Cargo.toml            
│ 
├── methods
│   ├── guest
│   │   ├── src
│   │   │   └── main.rs  
│   │   └── Cargo.toml  
│   ├── src
│   │   └── lib.rs  
│   ├── build.rs           
│   └── Cargo.toml             
│ 
├── README.md                 
└── rust-toolchain.toml       
```
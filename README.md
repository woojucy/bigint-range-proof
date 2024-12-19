# bigint-range-proof
This repository implements range proofs for big integers using the zkVM (zero-knowledge virtual machine).

## Structure
```plaintext
BIGINTRANGEPROOF
├── host
│   ├── src
│   │   ├── bin
│   │   │   └── prove.rs      
│   │   └── main.rs           
│   ├── Cargo.toml             
│   └── Cargo.lock             
├── methods
│   ├── guest
│   │   ├── src
│   │   │   ├── build.rs       
│   │   │   └── Cargo.toml     
│   │   └── Cargo.lock         
│   └── Cargo.toml             
├── target                     
│   ├── Cargo.lock
│   └── Cargo.toml
├── LICENSE                    
├── README.md                 
└── rust-toolchain.toml       
```

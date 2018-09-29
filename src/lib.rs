//! This library provides common cryptographic functionality for working within
//! the exponential ElGamal cryptosystem.

extern crate num;
extern crate rand;
extern crate sha2;

#[macro_use]
extern crate serde_derive;
extern crate serde;

/// Adds support for modular arithmetic within a cyclic field of integers.
pub mod arithmetic;

/// Adds a universal cast-as-intended proof for a particular ElGamal ciphertext.
pub mod cai;

/// Adds support for encrypting and decrypting messages in the exponential ElGamal
/// cryptosystem, applying homomorphic addition on the ciphertexts. In addition, membership
/// proofs can be generated for a ciphertext, ensuring that the encrypted plain-text message
/// is within a particular bound.
pub mod el_gamal;

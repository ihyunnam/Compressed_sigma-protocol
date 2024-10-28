#![allow(non_snake_case)]
#![feature(test)]
#![deny(missing_docs)]

#![doc(include = "../README.md")]

extern crate curve25519_dalek;
extern crate digest;
extern crate merlin;
extern crate rand;
extern crate sha3;

pub mod commitments;
pub mod errors;
pub mod group;
pub mod random;
pub mod transcript;
pub mod math;

pub mod scalar;
pub mod sigma_protocol;
pub mod nizk;
pub mod polynomial;


#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

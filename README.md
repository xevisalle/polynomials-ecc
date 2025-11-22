# Polynomials ECC

[![Crates.io](https://img.shields.io/crates/v/polynomials-ecc.svg)](https://crates.io/crates/polynomials-ecc)
[![GitHub](https://img.shields.io/badge/GitHub-Repository-blue?logo=github)](https://github.com/xevisalle/polynomials-ecc)

This library allows to operate on polynomials over the BLS12-381. In particular, it allows to perform evaluations and interpolations using FFTs. It also offers tools such as efficient multi scalar multiplication, using the [blst](https://crates.io/crates/blst) library as a backend.

**DISCLAIMER:** the code in this repository has NOT went through an exhaustive security review. Use at your own risk.

## Example

```rust
use polynomials_ecc::{Domain, Polynomial};
use bls12_381::Scalar;
use rand::rngs::OsRng;
use ff::Field;

// Define our domain of size n = 2^14
let domain = Domain::new(14);

// Create an array of random coefficients
let mut coeffs = vec![];
for _ in 0..10000 {
    coeffs.push(Scalar::random(&mut OsRng));
}

// Define our polynomial
let polynomial = Polynomial::new(coeffs);

// Evaluate using the FFT
let evaluations = polynomial.evaluate(&domain);

// Interpolate using the IFFT
let polynomial_p = evaluations.interpolate(&domain);
```

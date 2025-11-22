//! # polynomials-ecc
//!
//! This library is meant to perform FFTs and IFFTs to evaluate and
//! interpolate polynomials, where the domain is defined over the
//! BLS12-381 elliptic curve. It also offers other tools such as
//! efficient multi scalar multiplication.

#![deny(missing_docs)]

mod domain;
pub use domain::Domain;

mod polynomials;
pub use polynomials::{Evaluations, Polynomial};

mod fourier;
use fourier::{fft, ifft};

mod utils;
pub use utils::msm;

mod error;
use error::Error;

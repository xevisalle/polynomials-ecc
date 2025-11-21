use crate::{Domain, fft, ifft};
use bls12_381::Scalar;

/// The coefficients of a polynomial
#[derive(Debug, PartialEq)]
pub struct Polynomial(Vec<Scalar>);

impl Polynomial {
    /// Creates a new Polynomial given its coefficients
    pub fn new(coeffs: Vec<Scalar>) -> Self {
        Polynomial(coeffs)
    }

    /// Evaluates the polynomial at a given domain
    pub fn evaluate(&self, domain: &Domain) -> Evaluations {
        let mut evaluations = Evaluations(vec![Scalar::zero(); domain.len()]);
        evaluations.0[..self.len()].copy_from_slice(self.0.as_slice());

        fft(&mut evaluations.0, &domain.roots(), domain.len());
        evaluations
    }

    /// Returns the length of the polynomial
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the polynomial is empty
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

/// The evaluations of a polynomial over a given domain
#[derive(Debug)]
pub struct Evaluations(Vec<Scalar>);

impl Evaluations {
    /// Interpolates the evaluations over a given domain to get a polynomial
    pub fn interpolate(&self, domain: &Domain) -> Polynomial {
        let mut polynomial = Polynomial(self.0.clone());

        ifft(&mut polynomial.0, &domain.roots());

        while polynomial.0.last() == Some(&Scalar::zero()) {
            polynomial.0.pop();
        }

        polynomial
    }

    /// Returns the length of the evaluations
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the evaluations are empty
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

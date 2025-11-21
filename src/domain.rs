use bls12_381::Scalar;

// The generator of the field to compute the primitive root of unity
const FIELD_GEN: usize = 7;

/// The power-of-2-sized domain to evaluate / interpolate
pub struct Domain(Vec<Scalar>);

impl Domain {
    /// Returns a new domain given the exponent of a power of 2
    pub fn new(e: u32) -> Domain {
        let n = 2usize.pow(e);
        let mut roots = vec![Scalar::zero(); n];

        // r = k · n + 1
        let k_bytes = (-Scalar::one() * Scalar::from(n as u64).invert().unwrap()).to_bytes();
        let mut k = [0u64; 4];

        for i in 0..4 {
            k[i] = u64::from_le_bytes(k_bytes[i * 8..i * 8 + 8].try_into().unwrap());
        }

        // w = g^k
        let w = Scalar::from(FIELD_GEN as u64).pow(&k);

        roots[0] = Scalar::one();
        for i in 1..n {
            roots[i] = roots[i - 1] * w;
        }

        Domain(roots)
    }

    /// Returns the roots of unity
    pub fn roots(&self) -> Vec<Scalar> {
        self.0.clone()
    }

    /// Returns the length of the domain
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the domain is empty
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

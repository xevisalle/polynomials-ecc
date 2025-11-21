use bls12_381::Scalar;
use ff::Field;
use polynomials_ecc::{Domain, Polynomial};
use rand::rngs::OsRng;

#[test]
fn test_evaluate_interpolate() {
    let domain = Domain::new(14);

    let mut coeffs = vec![Scalar::random(&mut OsRng)];
    for _ in 0..10000 {
        coeffs.push(Scalar::random(&mut OsRng));
    }

    let polynomial = Polynomial::new(coeffs);

    let evaluations = polynomial.evaluate(&domain);
    let polynomial_p = evaluations.interpolate(&domain);

    assert_eq!(polynomial, polynomial_p);
}

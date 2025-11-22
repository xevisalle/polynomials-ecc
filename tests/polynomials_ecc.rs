use bls12_381::{G1Projective, Scalar};
use ff::Field;
use polynomials_ecc::{Domain, Polynomial, msm};
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

#[test]
fn test_msm() {
    let mut scalars = vec![];
    let mut points = vec![];

    for _ in 0..10 {
        scalars.push(Scalar::random(&mut OsRng));
        points.push(G1Projective::generator() * Scalar::random(&mut OsRng));
    }

    let mut naive_mult = G1Projective::identity();
    for i in 0..10 {
        naive_mult += points[i] * scalars[i];
    }

    let pipp_mult = msm(&points, &scalars).expect("MSM succeeded.");
    assert_eq!(naive_mult, pipp_mult);
}

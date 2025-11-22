use bls12_381::{G1Affine, G1Projective, Scalar};
use bytemuck::cast_slice;

use crate::Error;
use blst::{
    blst_p1, blst_p1_affine, blst_p1_affine_compress, blst_p1_deserialize, blst_p1_from_affine,
    blst_p1_to_affine, p1_affines,
};

// Converts a G1Projective to a blst_p1
fn g1proj_to_blst_p1(point: &G1Projective) -> blst_p1 {
    let mut blst_affine = blst_p1_affine::default();
    unsafe {
        blst_p1_deserialize(
            &mut blst_affine,
            G1Affine::from(point).to_compressed().as_ptr(),
        );
    }

    let mut blst_projective = blst_p1::default();
    unsafe {
        blst_p1_from_affine(&mut blst_projective, &blst_affine);
    }

    blst_projective
}

/// Performs efficient multi scalar multiplication using the Pippenger's algorithm over
/// a given set of scalars and points.
pub fn msm(points: &[G1Projective], scalars: &[Scalar]) -> Result<G1Projective, Error> {
    // Both sets need to have the same length
    if points.len() != scalars.len() {
        return Err(Error::InvalidLength);
    }

    // Convert scalars to bytes
    let mut scalar_bytes = vec![];
    for scalar in scalars {
        scalar_bytes.push(scalar.to_bytes());
    }
    let scalar_bytes = cast_slice(scalar_bytes.as_slice());

    // Convert points to p1_affines
    let mut blst_points = vec![];
    for point in points {
        blst_points.push(g1proj_to_blst_p1(point));
    }
    let blst_points = p1_affines::from(&blst_points);

    // Perform the MSM
    let result = blst_points.mult(scalar_bytes, 255);

    // Convert the result to bytes
    let mut out = blst_p1_affine::default();
    unsafe {
        blst_p1_to_affine(&mut out, &result);
    }
    let mut out_compressed = [0u8; 48];
    unsafe { blst_p1_affine_compress(out_compressed.as_mut_ptr(), &out) }

    // Return the result as G1Projective
    Ok(G1Projective::from(
        G1Affine::from_compressed(&out_compressed).unwrap(),
    ))
}

use bls12_381::Scalar;

pub(crate) fn fft(polynomial: &mut [Scalar], domain: &[Scalar], mut len: usize) {
    if len != 1 {
        len /= 2;

        let mut odd_dom = vec![Scalar::zero(); len];
        let mut odd_vals = vec![Scalar::zero(); len];
        let mut even_vals = vec![Scalar::zero(); len];

        for i in 0..len {
            odd_dom[i] = domain[2 * i];
            odd_vals[i] = polynomial[2 * i];
            even_vals[i] = polynomial[(2 * i) + 1];
        }

        fft(&mut odd_vals, &odd_dom, len);
        fft(&mut even_vals, &odd_dom, len);

        for i in 0..len {
            odd_dom[i] = even_vals[i] * domain[i];
            polynomial[i] = odd_vals[i] + odd_dom[i];
            polynomial[i + len] = odd_vals[i] - odd_dom[i];
        }
    }
}

pub(crate) fn ifft(polynomial: &mut [Scalar], domain: &[Scalar]) {
    fft(polynomial, domain, domain.len());

    let out = polynomial.to_owned();
    let len_inv = Scalar::from(domain.len() as u64).invert().unwrap();

    polynomial[0] *= len_inv;
    for i in 1..domain.len() {
        polynomial[i] = out[domain.len() - i] * len_inv;
    }
}

rust
pub fn poly_st<F, const N: usize>(z: &Vec<F, N>) -> Vec<F, { N + 1 }>
where
    F: ComplexField,
    [(); { N + 1 }]: Sized,
{
    let mut a: Vec<F, { N + 1 }> = Vec::new();
    a.push(F::one());

    const KER: usize = 2;
    for zi in z {
        let mut b = Vec::new();
        b.resize(a.len() + 1, F::zero());
        let k = [F::one(), -zi.clone()];
        for i in 0..a.len() + KER - 1 {
            let u_i = if i > a.len() { i - KER } else { 0 };
            let u_f = i.min(a.len() - 1);
            if u_i == u_f {
                b[i] += a[u_i].clone() * k[i - u_i].clone();
            } else {
                for u in u_i..(u_f + 1) {
                    if i - u < KER {
                        b[i] += a[u].clone() * k[i - u].clone();
                    }
                }
            }
        }
        a = b;
    }

    let mut roots = z.clone();
    sort_cplx_st(&mut roots);
    let mut root_conjs = z
        .iter()
        .map(|zi| zi.clone().conjugate())
        .collect::<Vec<_, N>>();
    sort_cplx_st(&mut root_conjs);
    if roots.into_iter().zip(root_conjs).all(|(a, b)| a == b) {
        a = a
            .into_iter()
            .map(|ai| ComplexField::from_real(ai.real()))
            .collect();
    }

    a
}

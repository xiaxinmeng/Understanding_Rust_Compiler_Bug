rust
pub fn zpk2tf_st<C, F, const N: usize>(
    z: &Vec<C, N>,
    p: &Vec<C, N>,
    k: F,
) -> BaFormatFilter<F, { N + 1 }>
where
    C: ComplexField<RealField = F>,
    F: Float + RealField,
    [(); { N + 1 }]: Sized,
{
    let b = poly_st(z)
        .into_iter()
        .map(|bi| <C as ComplexField>::from_real(k) * bi)
        .collect::<Vec<_, _>>();
    let a = poly_st(p);

    // lots of math not shown
    todo!()
}

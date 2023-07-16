 rust
pub trait Matrix
<
    S: Field,
    RV: VectorSpace<S>,
    CV: VectorSpace<S>,
    MT: Matrix<S, CV, RV, Self>
>
:   Ring
{
    fn transpose(&self) -> MT;
}

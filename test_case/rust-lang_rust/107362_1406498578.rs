rust
pub enum Value<F>
where
    F: Functor,
{
    SignedInt(F::With<i64>),
    Array(Box<Value<Compose<F, Vec<()>, ()>>>),

}

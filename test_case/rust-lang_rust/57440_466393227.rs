rust
pub struct S1<T, A = <T as Trait>::Assoc>
where
    T: Trait<Assoc = A>,
{
    pub t: T,
    pub a: A,
}

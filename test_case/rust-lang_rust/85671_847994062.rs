rust
pub fn as_ref_a<Coef>(&self) -> A<&[Coef]>
where
    Self: AsSlice<Element = Coef>,

rust
fn discr<T, U>(v: T, value: U)
where
    <T as DiscriminantKind>::Discriminant: PartialEq<U>,
{
    assert!(discriminant_value(&v) == value);
}

rust
impl<'lhs1, 'rhs1> PartialEq<Type<'rhs1>> for Type<'lhs1>
where
    Field1<'lhs1>: PartialEq<Field1<'rhs1>>,
    Field2<'lhs1>: PartialEq<Field2<'rhs1>>,
    Field3<'lhs1>: PartialEq<Field3<'rhs1>>,
{
    // ...
}

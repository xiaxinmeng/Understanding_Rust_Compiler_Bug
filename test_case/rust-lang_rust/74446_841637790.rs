rust
/// SAFETY constraint: all values of this type must be convertible to valtrees such
/// that the valtrees are equal if and only if the values compare equal with `Eq::eq`.
unsafe trait StructuralEq: Eq {}

rust
/// If this is a field projection, and the field is being projected from a closure type,
/// then returns the index of the field being projected. Note that this closure will always
/// be `self` in the current MIR, because that is the only time we directly access the fields
/// of a closure type.
fn is_upvar_field_projection(lvalue: Lvalue<'tcx>) -> Option<usize> { ... }

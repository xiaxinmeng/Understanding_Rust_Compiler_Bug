 rust
// the type &'static u8 is a subtype of &'a u8:
fn basic_subtyping<'a>(f: &'static u8) -> &'a u8 {
    f
}

// the & type constructor is covariant in the type position:
fn covariant_in_type_position<'a>(f: &'static (&'static u8)) -> &'static (&'a u8) {
    f
}

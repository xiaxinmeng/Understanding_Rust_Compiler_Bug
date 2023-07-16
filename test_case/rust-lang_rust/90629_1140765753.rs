rust
struct UsesRef<'a> {
    actual_ref: &'a (),
    in_anon_const: [u8; std::mem::size_of::<&'a ()>()],
}

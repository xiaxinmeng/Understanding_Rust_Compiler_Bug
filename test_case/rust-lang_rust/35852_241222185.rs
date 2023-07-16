 Rust
fn example<'a>(data: &'a u32) {
    let foo_impl = FooImpl { inside: &data };
    let carrier: Carrier<FooImpl<'a>> = foo_impl.through_carrier();
    must_be_static(carrier);
}

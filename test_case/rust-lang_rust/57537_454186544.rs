rust
fn foo(s: &mut String) {
    // parsed as `ArgumentIs(0)`
    write!(s, "{0}", "foo");
    // parsed as `ArgumentImplicitlyIs(0)`
    write!(s, "{}", "foo");
}

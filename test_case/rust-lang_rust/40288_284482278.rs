rust
fn lifetime_transmute<'a, 'b, T: ?Sized>(x: &'a T, y: &'b T) -> &'a T {
    let mut out = [x];
    // Neither 'a nor 'b work here, they both error correctly.
    (&mut out as &mut [&'b T])[0] = y;
    out[0]
}

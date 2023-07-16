rust
fn prove_static<T: 'static + ?Sized>(_: &'static T) {}

fn lifetime_transmute<'a, T: ?Sized>(x: &'a T, y: &T) -> &'a T {
    let mut out = [x];
    (&mut out as &mut [_])[0] = y;
    out[0]
}

fn main() {
    prove_static(lifetime_transmute("", &String::from("foo")));
}

rust
fn main() {
    let c = |v: &()| {};
    x(c);
    y(c);
    for _ in 0..1 {
        let s = ();
        c(&s);
    }
}

fn x<F>(_: F) where for<'a> F: FnMut(&'a ()) {}
fn y<'a, F>(_: F) where F: FnMut(&'a ()) {}

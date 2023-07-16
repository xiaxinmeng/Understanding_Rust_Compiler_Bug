 rust
fn small<'a>(x: &'a mut ()) -> &'a mut () {
    let f = || &mut *x;
    f()
}

fn main() { }

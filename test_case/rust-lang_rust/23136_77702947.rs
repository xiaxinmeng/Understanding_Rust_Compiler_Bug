 rust
fn f<'a>(_: &'a u8) -> &'static &'a () { loop { } }

fn g<'a>(_: &'a u8, _: fn(&'a u8) -> &'static &'a ()) { }

fn main() {
    let x = 0;
    g(&x, f);
}

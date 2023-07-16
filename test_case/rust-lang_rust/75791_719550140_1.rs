rust
fn thing(x: impl FnOnce(&u32)) {}

fn main() {
    let f = |_: &_| ();
    thing(f);
}

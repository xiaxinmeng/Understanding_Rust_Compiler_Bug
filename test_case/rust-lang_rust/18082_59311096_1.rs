 rust
// app.rs
extern crate opaque;

fn main() {
    let he_who_can_not_be_typed: _ = opaque::new_thing();  // what's the type of this?
    he_who_can_not_be_typed.kaboom();
}

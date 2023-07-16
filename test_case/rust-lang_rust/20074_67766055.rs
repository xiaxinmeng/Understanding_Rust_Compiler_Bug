 rust
enum E {
    F(i64),
    C(i32, i32, i32, i32)
}

static T: &'static [E] = &[
    E::C(0,0,0,0)
];

fn main() { }

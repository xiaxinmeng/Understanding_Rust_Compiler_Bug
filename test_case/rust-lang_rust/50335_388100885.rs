rust

#[derive(Debug)]
pub enum E {
    A(u8, u8),
    B,
}

let foo = E::B;
assert_match!(E::A(_, _), foo)

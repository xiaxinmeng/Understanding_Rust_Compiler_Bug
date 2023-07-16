rust
pub fn foo_if(foo: bool) -> u32 {
    if foo {
        1
    } else {
        2
    }
}

pub fn foo_match(foo: bool) -> u32 {
    match foo {
        true => 1,
        false => 2,
    }
}

rust
#![feature(const_generics)]

fn break_me<T, const N: usize>() -> [T; N]
where
    [T; N]: Default 
{
    Default::default()
}
fn main() {
    let _: [u8; 35] = break_me();
}

rust
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum TwoVal {
    One = 1,
    Two = 2,
}

union Foo {
    bar: u8,
    oneval: TwoVal
}

fn main() {
    match (Foo { bar: 42 }) {
        Foo { oneval: TwoVal::One | TwoVal::Two } => { //~ ERROR access to union field is unsafe
            println!("reached!");
        },
    }
}

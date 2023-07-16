rust
#[derive(Copy, Clone)]
#[repr(u8)]
enum OneVal {
    One = 1,
}

union Foo {
    bar: u8,
    oneval: OneVal
}

fn main() {
    match (Foo { bar: 42 }) {
        Foo { oneval: OneVal::One } => {
            println!("reached!"); // always prints
        },
    }
}

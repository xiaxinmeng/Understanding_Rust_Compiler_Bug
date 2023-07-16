rust
#![feature(inline_const)]

fn main() {
    let value: &[u8] = b"Any";
    match value {
        const { stringify!(Any).as_bytes() } => println!("Yo!"),
        _ => println!("Boo!")
    }
}

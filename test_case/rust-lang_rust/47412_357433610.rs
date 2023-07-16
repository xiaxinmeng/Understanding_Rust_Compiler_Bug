Rust
fn main() {
    union A { a: u8, v: u16 }
    let a = A { a: 1 };
    match a.v {
        _ => println!("Congrats, it's a u16!")
    }
}

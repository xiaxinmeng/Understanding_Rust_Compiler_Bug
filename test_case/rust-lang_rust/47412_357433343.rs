
fn main() {
    union A {
        a: i8,
        b: u8,
    }
    let a = A { a: -2 };
    match a.b {
        _ => { println!("should not be allowed") }
    }
}

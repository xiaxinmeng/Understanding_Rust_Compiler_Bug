rust
// src/main.rs
fn main() {
    let func = |a| { a; };
    let mut what: u32 = 2;
    loop {
        func(&mut what);
    }
}

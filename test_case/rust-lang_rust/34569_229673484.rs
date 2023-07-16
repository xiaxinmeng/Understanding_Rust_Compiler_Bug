 rust
fn main() {
    match 0 {
        e if (|| { e == 0 })() => unimplemented!(),
        1 => unimplemented!(),
        _ => unimplemented!()
    }
}

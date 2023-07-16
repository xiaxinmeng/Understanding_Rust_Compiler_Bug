 rust
fn main() {
    static A: i32 = 0;
    static B: i32 = *&A;
    println!("{:?}", B);
}

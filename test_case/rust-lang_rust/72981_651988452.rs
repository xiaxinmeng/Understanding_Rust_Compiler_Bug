rust
fn main() {
    let b = Backtrace::capture();
    println!("{:#?}", b);
}

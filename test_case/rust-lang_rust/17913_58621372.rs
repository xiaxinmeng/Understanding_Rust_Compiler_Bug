 rust
fn main() {
    let n = 0u;
    let a = box [&n,..0xF000000000000000u];
    println!("{}", a[0xFFFFFFu]);
}

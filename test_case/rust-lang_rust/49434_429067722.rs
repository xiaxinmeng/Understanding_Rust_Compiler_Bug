rust
fn main() {
    let mut x = vec![];
    x.push(x.len());
    x.get_mut(x.len());
    println!("Hello, world!");
}


pub fn main() {
    let a = include_str!("foo.txt");
    let b = include_str!("bar.txt");
    println!("a = {}, b = {}", a, b);
}

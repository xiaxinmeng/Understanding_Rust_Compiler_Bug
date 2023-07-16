 rust
fn main() {
    let mut v = Vec::<(i32, f32)>::with_capacity(2);
    v.push((1, 2.5));
    v.push((2, 7.0));
    for &(a, b) in v.iter() {
        println!("a: {}  b: {}", a, b);
    }
}

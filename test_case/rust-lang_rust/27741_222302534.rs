 rust
fn main() {
    let v: Vec<_> = (0..5).map(|x| x*2).collect();
    println!("list: {:?}", v);
    for v in (0..10).map(|x| x as f32 / 10.0) {
        println!("v: {}", v);
    }
}

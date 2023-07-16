
fn main() {
    println!("{}", range(4194303.8f32, 4194305f32).collect::<Vec<f32>>());
    println!("{}", range(4194303.8f32, 4194305f32).count());
    println!("{}", range(4194303.8f32, 4194305f32).size_hint());
}


fn main() {
    println!("{}", range(0f32, 1.5f32).collect::<Vec<f32>>());
    println!("{}", range(0f32, 1.5f32).count());
    println!("{}", range(0f32, 1.5f32).size_hint());
}

rust
fn main() {
    let floaty: f32 = 1.222222; // 6 decimal points
    println!("{:.32}", floaty);
    println!("{:.32}", floaty / 2.0);
    println!("{:.32}", (floaty / 2.0).tan());
    println!("{:.32}", ((floaty / 2.0) as f64).tan() as f32);
}

 rust
fn bug(x: f32) -> f32 {
    let r = if x > 0.0 {
        1.0
    } else {
        2.0
    } * 6.0;
    return r;
}

fn main() {
    println!("{:?}", bug(0.5));
}

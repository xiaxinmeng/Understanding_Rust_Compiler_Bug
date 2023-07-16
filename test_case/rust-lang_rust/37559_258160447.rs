 rust
#[inline(never)]
fn foo() -> f32 {
    12345.0f32.powi(0)
}

fn main() {
    println!("{}", foo());
}

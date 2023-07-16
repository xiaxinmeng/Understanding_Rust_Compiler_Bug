 rust
#[inline(never)]
fn foo(i: i32) -> f32 {
    12345.0f32.powi(i)
}

fn main() {
    let s = String::new();

    println!("{}", foo(s.len() as i32));
}

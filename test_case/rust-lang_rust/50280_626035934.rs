
fn show(a: fn () -> f32, b: fn (&Vec<i8>)->f32) {
    println!("the two pointers: {:p} {:p}", a, b);
}

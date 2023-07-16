rust
fn my_function(_callback: impl FnMut(&mut i32, &mut i32)) {}

fn main() {
    let closure_consumer = |_a: &mut i32, _b| {};
    my_function(closure_consumer);
}

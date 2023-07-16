rust
fn int() -> impl Send {
    7
}

fn main() {
    let _x: i32 = int().wrapping_add(7);
}

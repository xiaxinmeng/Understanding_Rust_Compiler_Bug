 rust
#[no_mangle]
pub extern "C" fn sum_point(p: Point) -> f32 {
    p.x + p.y
}

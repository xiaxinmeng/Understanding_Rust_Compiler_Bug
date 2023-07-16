rust
unsafe fn is_nan(x: f32) -> bool {
    let val: u32 = mem::transmute(x);
    ....
}

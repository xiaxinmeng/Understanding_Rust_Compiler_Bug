 rust
#[rustc_mir]
fn foo(x: &[i32; 3]) -> &[i32] {
    let y: &[i32] = x;
    y
}

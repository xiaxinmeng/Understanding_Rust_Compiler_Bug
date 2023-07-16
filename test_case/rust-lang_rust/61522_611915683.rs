
fn foo<const N: usize>() -> [i32; N + 1] {
    let x: [i32; N + 1] = [0; N + 1];
    x
}

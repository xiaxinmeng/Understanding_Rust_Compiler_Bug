
fn main() {
    let x = vec![1i32];
    let value: i32 = match &x[..] {
        &[v] => v,
        _ => panic!(),
    };
}

rust
fn main() {
    let v = vec![1, 2, 3, 4];

    let idx = v.len();

    println!("v[{}] = {}", idx, unsafe { *v.get_unchecked(idx) });
}

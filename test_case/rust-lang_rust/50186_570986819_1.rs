rust
fn dup<T: Copy>(x: T) -> (T, T) { (x, x) }
let i = i.update2(|i| (i + 1, i)); // i++
let i = i.update2(|i| dup(i + 1)); // ++i

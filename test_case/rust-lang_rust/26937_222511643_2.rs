 rust
fn printer(a: &i32) { println!("{}", a) }
let f = |a| printer(a);
caller(f)

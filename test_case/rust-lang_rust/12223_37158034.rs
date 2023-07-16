 rust
fn main() {
    let a = ~"";
    let b: Vec<&str> = a.lines().collect();  // note: borrow of `a[..]` occurs here
    drop(a);    // error: cannot move out of `a` because it is borrowed
}

rust
fn accept_closure<F>(cb: F) where F: FnOnce(&i32) {
    todo!()
}

fn main() {
    // Adding the type annotation &i32 to x makes it compile successfully
    // Inlining the closure (with no type annotation) also makes it compile successfully
    let cb = |x| {};
    accept_closure(cb);
}

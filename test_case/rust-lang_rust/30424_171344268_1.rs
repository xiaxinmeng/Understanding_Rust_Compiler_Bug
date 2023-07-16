 rust
fn caller(x: &mut T) {
    caller(x, y); // what lifetime is inferred for the (implicit) coercion? does it cover the call?
}
fn callee(x: *mut T, y: *mut T) {
}

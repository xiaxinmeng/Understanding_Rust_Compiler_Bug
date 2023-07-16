 rust
type FnMoo = FnMut(i32) + 'static;

fn foo(mut c: Box<FnMoo>) -> (*mut FnMoo, Box<FnMoo>) {
    (&mut *c as *mut _, c)
}

fn main() {
    // Add code here
}

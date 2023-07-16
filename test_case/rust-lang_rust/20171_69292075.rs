 rust
fn f0<R, F> (_: F) where F: Fn<(), R> {
}
fn f1<A, R, F> (_: F) where F: Fn<(A,), R> {
}
fn f<A, R, F> (_: F) where F: Fn<A, R> {
}
fn g<A, R, F> (_: F) where F: FnOnce<A, R> {
}
fn h<A, R, F> (_: F) where F: FnMut<A, R> {
}

fn main() {
    f0(|| ());
    f1(|_: i32| ());
    f(|&: _: i32| ());
    f(|&: _: i32, _: i32| ());
    g(|: _: i32| ());
    g(|: _: i32, _: i32| ());
    h(|&mut: _: i32| ());
    h(|&mut: _: i32, _: i32| ());
}

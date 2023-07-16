rust
#![feature(unsized_fn_params)]
fn f(x: [i32], _: ()) {
    assert_eq!([1], x);
}
fn main() {
    let mut x: Box<[i32]> = Box::new([1]);
    f(*x, x[0] = 2);
}

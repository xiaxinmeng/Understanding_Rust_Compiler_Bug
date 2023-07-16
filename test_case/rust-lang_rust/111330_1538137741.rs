rust
#![feature(unsized_fn_params)]
fn f(x: [i32], _: ()) {
    assert_eq!(&x, [1]);
}
fn main() {
    let mut x: Box<[i32]> = Box::new([1]);
    f(*x, {x = Box::new([2]) as Box<[i32]>; let _ = x;});
}

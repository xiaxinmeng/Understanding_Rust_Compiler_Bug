rs
#![feature(non_lifetime_binders)]

fn take<F>(_: F) where F: for<T> FnOnce(T) -> T {}

fn main() {
    take(|x| x)
}

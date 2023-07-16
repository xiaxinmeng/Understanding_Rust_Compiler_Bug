
#![feature(unboxed_closures)]

use std::future::Future;

fn less_than<'a>(number: &'a i32) -> impl FnOnce<(i32,), Output = impl Future<Output = bool> + 'a> {
    move |n: i32| async move { n < *number }
}

fn main() {
    let zero = 0;
    let less_than_zero = less_than(&zero);
}

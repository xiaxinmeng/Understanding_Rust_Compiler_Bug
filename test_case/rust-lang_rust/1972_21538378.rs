
% cat /tmp/foo.rs
pub fn access<T:Clone>(a: &[T], i: uint) -> T {
    a[i].clone()
}
% cat /tmp/bar.rs
extern mod foo;
use foo::*;

pub fn main() {
    let arr = [1, 2, 3];
    let a_4 = access(arr, 4);
    println(fmt!("arr[4]: %?", a_4));
}
% rustc --lib /tmp/foo.rs && rustc -L/tmp /tmp/bar.rs && /tmp/bar
warning: missing crate link meta `name`, using `foo` as default
warning: missing crate link meta `vers`, using `0.0` as default
warning: no debug symbols in executable (-arch x86_64)
warning: no debug symbols in executable (-arch x86_64)
rust: task failed at 'index out of bounds: the len is 3 but the index is 4', /tmp/bar.rs:1
rust: domain main @0x7fd1e8814c10 root task failed
% 

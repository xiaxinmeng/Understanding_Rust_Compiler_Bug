console
$ cat a.rs 
#![feature(box_syntax)]
#[inline]
fn f() -> Box<i32> { box 0 }
fn main() { box f(); }
$ rustc +3930b75493eb88cd5431103dbdbc47eb760e9d9b a.rs -Zinline-mir
thread 'rustc' panicked at '(*(*_7))', compiler/rustc_const_eval/src/transform/validate.rs:314:13

 rust
$ rustc foo.rs
foo.rs:3:20: 3:24 warning: struct field is never used: `v`, #[warn(dead_code)] on by default
foo.rs:3 struct MyType<T> { v: T }
                            ^~~~
$ cat foo.rs
#![feature(unsafe_destructor)]
struct MyType<T> { v: T }

#[unsafe_destructor]
impl Drop for MyType<u8> {
    fn drop(&mut self) { () }
}

fn main() {
    let _foo = MyType{v:3u};
}

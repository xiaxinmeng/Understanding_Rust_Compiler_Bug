 rust
$ rustc foo.rs
$ cat foo.rs
#![feature(unsafe_destructor)]
struct Test<T>(T);

#[unsafe_destructor]
impl <T: Iterator> Drop for Test<T> {
    fn drop(&mut self) {  }
}

fn main() {
    let x = Test(2i);
    let _ = x;
}

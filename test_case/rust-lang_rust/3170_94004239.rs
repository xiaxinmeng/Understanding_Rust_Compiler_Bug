 shell
$ cat foo.rs
#![allow(dead_code)]
const A: &'static [usize] = &[];
const B: usize = A[1];

fn main() {
    println!("B={}", B);
}
$ rustc foo.rs
foo.rs:3:18: 3:22 error: const index-expr is out of bounds
foo.rs:3 const B: usize = A[1];
                          ^~~~
error: aborting due to previous error
$ rustc --version
rustc 1.0.0-dev (a691f1eef 2015-04-15) (built 2015-04-15)

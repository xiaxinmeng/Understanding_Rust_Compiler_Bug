
$ cat test.rs
use foo::bar;

fn main() {
    bar();
}

$ x86_64-apple-darwin/stage1/bin/rustc test.rs
test.rs:1:5: 1:8 error: unresolved import `foo::bar`. Maybe a missing `extern crate foo`? [E0432]
test.rs:1 use foo::bar;
              ^~~
test.rs:1:5: 1:8 help: run `rustc --explain E0432` to see a detailed explanation
error: aborting due to previous error

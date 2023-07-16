shell
$ cat src/main.rs
#![feature(marker_trait_attr)]

#[marker]
trait A {}

fn main() {}
$ cargo check
    Checking unused v0.1.0 (/../unused)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
$ touch src/main.rs
$ cargo check
    Checking unused v0.1.0 (/../unused)
warning: unused attribute
 --> src/main.rs:3:1
  |
3 | #[marker]
  | ^^^^^^^^^
  |
  = note: `#[warn(unused_attributes)]` on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.25s

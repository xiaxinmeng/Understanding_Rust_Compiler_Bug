 rust
/home/nmatsakis/tmp/x.rs:1:1: 4:2 error: recursive type `Recursive` has infinite size [E0072]
/home/nmatsakis/tmp/x.rs:1 struct Recursive {
/home/nmatsakis/tmp/x.rs:2     this: Recursive,
/home/nmatsakis/tmp/x.rs:3     last: u32
/home/nmatsakis/tmp/x.rs:4 }
/home/nmatsakis/tmp/x.rs:1:1: 4:2 help: run `rustc --explain E0072` to see a detailed explanation
/home/nmatsakis/tmp/x.rs:1:1: 4:2 help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `Recursive` representable
error: aborting due to previous error

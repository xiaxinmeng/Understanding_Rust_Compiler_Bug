 fish
/tmp $ echo 'fn main() { let shortty = "test"; shortty(); }' > hello.rs
/tmp $ rustc hello.rs
hello.rs:1:35: 1:44 error: expected function, found `&str`
hello.rs:1 fn main() { let shortty = "test"; shortty(); }
                                             ^~~~~~~~~
error: aborting due to previous error

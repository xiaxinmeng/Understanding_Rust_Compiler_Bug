
+ rm -rf a b c
+ cargo new --lib a
     Created library `a` project
+ cargo new --lib b
     Created library `b` project
+ cargo new --bin c
     Created binary (application) `c` project
+ echo 'pub trait A { fn a(&self) {} } impl A for () {}'
+ echo 'a = { path = "../a" }'
+ echo 'pub extern crate a;'
+ echo 'b = { path = "../b" }'
+ echo 'extern crate b; fn main() { ().a(); }'
+ cargo build --manifest-path c/Cargo.toml
   Compiling a v0.1.0 (file:///Users/kennytm/Downloads/a/a)
   Compiling b v0.1.0 (file:///Users/kennytm/Downloads/a/b)
   Compiling c v0.1.0 (file:///Users/kennytm/Downloads/a/c)
error[E0599]: no method named `a` found for type `()` in the current scope
 --> c/src/main.rs:1:32
  |
1 | extern crate b; fn main() { ().a(); }
  |                                ^
  |
  = help: items from traits can only be used if the trait is in scope
  = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
          candidate #1: `use b::<unnamed>::A;`

error: aborting due to previous error

error: Could not compile `c`.

To learn more, run the command again with --verbose.

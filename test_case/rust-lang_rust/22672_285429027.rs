text
C:\Users\steve\tmp> rustc .\foo.rs
error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<U as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T as Circular>::T`
 --> .\foo.rs:5:23
  |
5 | impl<X, U : Circular> TypeTrans<X> for U {
  |                       ^^^^^^^^^^^^
  |
  = help: consider adding a `#![recursion_limit="128"]` attribute to your crate

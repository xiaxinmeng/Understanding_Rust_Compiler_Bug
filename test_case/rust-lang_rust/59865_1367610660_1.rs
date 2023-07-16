
error[[E0195]](https://doc.rust-lang.org/nightly/error-index.html#E0195): lifetime parameters or bounds on method `next` do not match the trait declaration
  --> src/lib.rs:11:12
   |
11 |     fn next<'b: 'a>(&'b mut self) -> Option<Self::Item> {
   |            ^^^^^^^^ lifetimes do not match method in trait

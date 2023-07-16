
error[E0623]: lifetime mismatch
  --> src/main.rs:11:21
   |
7  |     fn bar(&self, other: Foo) -> Foo {
   |                          ---     ---
   |                          |
   |                          this parameter and the return type are declared with different lifetimes...
...
11 |                     other
   |                     ^^^^^ ...but data from `other` is returned here

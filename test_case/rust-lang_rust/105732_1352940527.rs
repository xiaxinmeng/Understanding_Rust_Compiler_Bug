console
cat@LAPTOP-V6U0QKD4:~/code/rust$ j dev         src/test/ui/methods/issues/issue-105732.rs
rustc +dev src/test/ui/methods/issues/issue-105732.rs
error[E0380]: auto traits cannot have associated items
 --> src/test/ui/methods/issues/issue-105732.rs:4:8
  |
3 | auto trait Foo {
  |            --- auto trait cannot have associated items
4 |     fn g(&self); //~ ERROR auto traits cannot have associated items
  |     ---^-------- help: remove these associated items

error[E0599]: the method `g` exists for reference `&Self`, but its trait bounds were not satisfied
 --> src/test/ui/methods/issues/issue-105732.rs:9:14
  |
9 |         self.g();
  |              ^
  |
  = note: the following trait bounds were not satisfied:
          `Self: Foo`
          which is required by `&Self: Foo`
          `&Self: Foo`
  = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `g`, perhaps you need to add a supertrait for it:
  |
7 | trait Bar: Foo {
  |          +++++

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0380, E0599.
For more information about an error, try `rustc --explain E0380`.

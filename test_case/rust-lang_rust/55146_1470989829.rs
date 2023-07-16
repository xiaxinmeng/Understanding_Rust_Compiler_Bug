
error[E0373]: closure may outlive the current function, but it borrows `books`, which is owned by the current function
  --> $DIR/issue-55146.rs:8:10
   |
LL | #[derive(Foo)]
   |          ^^^
   |          |
   |          `books` is borrowed here
   |          may outlive borrowed value `books`
   |
note: closure is returned here
  --> $DIR/issue-55146.rs:8:10
   |
LL |   #[derive(Foo)]
   |            ^--
   |            |
   |  __________in this derive macro expansion
   | |
LL | | struct Bar;
LL | |
LL | | fn main() {}
...  |
   = note: this error originates in the derive macro `Foo` (in Nightly builds, run with -Z macro-backtrace for more info)
help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
   |
LL | #[derive(move Foo)]
   |          ++++


error[E0433]: failed to resolve: use of undeclared crate or module `my_core`
  --> $DIR/extern-prelude-from-opaque-fail.rs:24:14
   |
LL |     fn f() { my_core::mem::drop(0); }
   |              ^^^^^^^
   |              |
   |              use of undeclared crate or module `my_core`
   |              help: a similar crate or module exists: `my_core`

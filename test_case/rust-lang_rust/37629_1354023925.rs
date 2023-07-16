
error[E0603]: trait `RLINQ` is private
  --> src/lib.rs:14:16
   |
14 |     [use rlinq::RLINQ;](https://play.rust-lang.org/?gist=a296dc331e14c85c860158172a765497&version=nightly&mode=debug#)
   |                ^^^^^ private trait
   |
note: the trait `RLINQ` is defined here
  --> src/lib.rs:2:5
   |
2  |     trait RLINQ {
   |     ^^^^^^^^^^^

error[E0599]: no method named `is_where` found for struct `Vec<String>` in the current scope
  --> src/lib.rs:18:11
   |
18 |         x.is_where();
   |           ^^^^^^^^ method not found in `Vec<String>`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `RLINQ` defines an item `is_where`, perhaps you need to implement it
  --> src/lib.rs:2:5
   |
2  |     trait RLINQ {
   |     ^^^^^^^^^^^

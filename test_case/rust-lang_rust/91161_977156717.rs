
error[E0603]: unit variant `B` is private
 --> src/main.rs:4:19
  |
4 |     let _x = Foo::B as u8;
  |                   ^ private unit variant
  |
note: the unit variant `B` is defined here
 --> /tmp/issue-91161-dep/src/lib.rs:5:5
  |
5 |     B,
  |     ^

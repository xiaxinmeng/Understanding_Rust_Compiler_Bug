
error[E0308]: arguments to this function are incorrect
  --> $DIR/two-mismatch-notes.rs:10:5
   |
LL |     foo(f, w);
   |     ^^^
   |
note: expected `i32`, found `u32`
  --> $DIR/two-mismatch-notes.rs:10:9
   |
LL |     foo(f, w);
   |         ^
   = note: expected fn pointer `fn(i32)`
                 found fn item `fn(u32) {f}`
note: expected `i32`, found `isize`
  --> $DIR/two-mismatch-notes.rs:10:12
   |
LL |     foo(f, w);
   |            ^
   = note: expected struct `Wrapper<i32>`
              found struct `Wrapper<isize>`
note: function defined here
  --> $DIR/two-mismatch-notes.rs:4:4
   |
LL | fn foo(_: fn(i32), _: Wrapper<i32>) {}
   |    ^^^ ----------  ---------------

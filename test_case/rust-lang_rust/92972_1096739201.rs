
error: field is never read: `1`
  --> $DIR/tuple-struct-field.rs:6:21
   |
LL | struct Wrapper(i32, [u8; LEN], String);
   |                     ^^^^^^^^^
   |
help: change the field to unit type to suppress this warning while preserving the field numbering
   |
LL | struct Wrapper(i32, (), String);
   |                     ~~

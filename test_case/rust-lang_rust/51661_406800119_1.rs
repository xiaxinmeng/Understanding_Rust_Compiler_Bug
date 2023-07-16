
error: constant `foo` should have an upper case name such as `FOO`
 --> a.rs:3:1
  |
3 | pub const foo: u8 = 0;
  | ^^^^^^^^^^^^^^^^^^^^^^
  |
note: lint level defined here
 --> a.rs:1:9
  |
1 | #![deny(non_upper_case_globals)]
  |         ^^^^^^^^^^^^^^^^^^^^^^

error: Compilation failed, aborting rustdoc

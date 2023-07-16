
error[E0659]: `allow` is ambiguous (built-in attribute vs any other name)
 --> src/lib.rs:3:3
  |
3 | #[allow(unused)]
  |   ^^^^^ ambiguous name
  |
  = note: `allow` could refer to a built-in attribute
note: `allow` could also refer to the attribute macro imported here
 --> src/lib.rs:1:5
  |
1 | use global_allocator as allow;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  = help: use `crate::allow` to refer to this attribute macro unambiguously

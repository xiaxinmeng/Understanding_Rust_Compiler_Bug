
error: unexpected token: `include_str`
 --> library/std/src/macros.rs:7:34
  |
7 | #[cfg_attr(not(bootstrap), doc = include_str!("../../core/src/macros/panic.md"))]
  |                                  ^^^^^^^^^^^
  |
  = help: the valid syntax is `#[cfg_attr(condition, attribute, other_attribute, ...)]`
  = note: for more information, visit <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute>

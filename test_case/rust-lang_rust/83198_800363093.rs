
warning: unnecessary braces around const expression
  --> src/main.rs:20:31
   |
20 |     print_const_generic_arg::<{expand_to_usize!()}>();
   |                               ^^^^^^^^^^^^^^^^^^^^ help: remove these braces
   |
   = note: `#[warn(unused_braces)]` on by default

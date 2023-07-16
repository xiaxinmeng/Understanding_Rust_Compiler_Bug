
rustc 1.17.0-nightly (b1e31766d 2017-03-03)
error: no method named `write_fmt` found for type `&mut std::vec::Vec<_>` in the current scope
 --> <anon>:3:5
  |
3 |     writeln!(&mut buffer, "foo").unwrap();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::io::Write;`
  = note: this error originates in a macro outside of the current crate

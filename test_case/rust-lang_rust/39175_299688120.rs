rust
error[E0432]: unresolved import `std::os::ext::ffi::OsStrExt`
  --> src/main.rs:44:5
   |
44 | use std::os::ext::ffi::OsStrExt;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ Could not find `ext` in `os`

error: no method named `as_bytes` found for type `&std::ffi::OsStr` in the current scope
  --> src/main.rs:51:41
   |
51 | 		.filter(|n| *(n.unwrap().file_name()).as_bytes() == b".dsc") {
   | 		                                      ^^^^^^^^
   |
   = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
   = help: candidate #1: `use std::os::ext::ffi::OsStrExt;`

error: aborting due to previous error

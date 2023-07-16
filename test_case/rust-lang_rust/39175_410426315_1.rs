
error[E0433]: failed to resolve. Could not find `ext` in `os`
 --> src\main.rs:2:14
  |
2 | use std::os::ext::ffi::OsStrExt;
  |              ^^^ Could not find `ext` in `os`

warning: unused import: `std::os::ext::ffi::OsStrExt`
 --> src\main.rs:2:5
  |
2 | use std::os::ext::ffi::OsStrExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error[E0599]: no method named `encode_wide` found for type `&std::ffi::OsStr` in the current scope
 --> src\main.rs:5:24
  |
5 |     OsStr::new("1234").encode_wide();
  |                        ^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use std::os::ext::ffi::OsStrExt;
  |

error: aborting due to 2 previous errors

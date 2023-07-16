
error[E0433]: failed to resolve. Could not find `ext` in `os`
 --> src/main.rs:3:14
  |
3 | use std::os::ext::fs::MetadataExt;
  |              ^^^ Could not find `ext` in `os`

warning: unused import: `std::os::ext::fs::MetadataExt`
 --> src/main.rs:3:5
  |
3 | use std::os::ext::fs::MetadataExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error[E0599]: no method named `mtime` found for type `std::fs::Metadata` in the current scope
 --> src/main.rs:6:35
  |
6 |     fs::metadata("/usr").unwrap().mtime()
  |                                   ^^^^^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use std::os::ext::fs::MetadataExt;
  |

error: aborting due to 2 previous errors

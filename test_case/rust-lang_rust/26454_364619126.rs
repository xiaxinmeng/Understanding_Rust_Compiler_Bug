
error[E0599]: no method named `mtime` found for type `std::fs::Metadata` in the current scope
  --> src/main.rs:17:15
   |
17 |     let m = m.mtime();
   |               ^^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
   |
1  | use std::os::ext::fs::MetadataExt;
   |

text
[01:20:32] ---- os/linux/fs.rs - os::linux::fs::MetadataExt::st_size (line 179) stdout ----
[01:20:32] 	error: unused import: `std::os::unix::prelude::*;`
[01:20:32]  --> os/linux/fs.rs:5:5
[01:20:32]   |
[01:20:32] 5 | use std::os::unix::prelude::*;
[01:20:32]   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:20:32]   |
[01:20:32] note: lint level defined here
[01:20:32]  --> os/linux/fs.rs:1:9
[01:20:32]   |
[01:20:32] 1 | #![deny(warnings)]
[01:20:32]   |         ^^^^^^^^
[01:20:32]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:20:32] 
[01:20:32] error[E0599]: no method named `st_size` found for type `std::fs::Metadata` in the current scope
[01:20:32]   --> os/linux/fs.rs:10:21
[01:20:32]    |
[01:20:32] 10 | println!("{}", meta.st_size());
[01:20:32]    |                     ^^^^^^^
[01:20:32]    |
[01:20:32]    = help: items from traits can only be used if the trait is in scope
[01:20:32]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:20:32]            candidate #1: `use std::os::linux::fs::MetadataExt;`

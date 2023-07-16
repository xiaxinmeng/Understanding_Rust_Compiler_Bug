
error: enum variants on type aliases are experimental
  --> src/main.rs:13:13
   |
13 |             Self::Init => 1,
   |             ^^^^^^^^^^
   |
   = help: add `#![feature(type_alias_enum_variants)]` to the crate attributes to enable

error: enum variants on type aliases are experimental
  --> src/main.rs:14:13
   |
14 |             Self::NewProject | Self::NewTask | Self::NewEvent => 2,
   |             ^^^^^^^^^^^^^^^^
   |
   = help: add `#![feature(type_alias_enum_variants)]` to the crate attributes to enable

error: enum variants on type aliases are experimental
  --> src/main.rs:14:32
   |
14 |             Self::NewProject | Self::NewTask | Self::NewEvent => 2,
   |                                ^^^^^^^^^^^^^
   |
   = help: add `#![feature(type_alias_enum_variants)]` to the crate attributes to enable

error: enum variants on type aliases are experimental
  --> src/main.rs:14:48
   |
14 |             Self::NewProject | Self::NewTask | Self::NewEvent => 2,
   |                                                ^^^^^^^^^^^^^^
   |
   = help: add `#![feature(type_alias_enum_variants)]` to the crate attributes to enable

shell
warning: `Self::IDENT` is both an enum and an enum
  --> src/main.rs:11:7
   |
11 | /// [`Self::IDENT`]
   |       ^^^^^^^^^^^ ambiguous link
   |
help: to link to the enum, prefix with `enum@`
   |
11 | /// [`enum@Self::IDENT`]
   |       +++++
help: to link to the enum, prefix with `enum@`
   |
11 | /// [`enum@Self::IDENT`]
   |       +++++

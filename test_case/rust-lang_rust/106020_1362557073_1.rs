rust
error: expected `;`, found `#`
  --> src\engine\renderer\context\instance.rs:44:73
   |
44 |             [debug_utils_ext, (*surface_extensions).to_owned()].concat()
   |                                                                         ^ help: add `;` here
45 |
46 |             #[cfg(not(feature = "validation"))]
   |             - unexpected token

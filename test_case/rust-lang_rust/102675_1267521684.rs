plain
    |
494 | ...                   (CastKind::IntToInt
    |                       ^
...
499 | ...                   | CastKind::PtrToPtr),
    |
    |
    = note: `-D unused-parens` implied by `-D warnings`
help: remove these parentheses
494 ~                                     CastKind::IntToInt
495 |                                     | CastKind::FloatToFloat
  ...
  ...
498 |                                     | CastKind::FnPtrToPtr
499 ~                                     | CastKind::PtrToPtr,

error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:04:07

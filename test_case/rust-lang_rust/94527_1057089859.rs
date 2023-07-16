plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0532]: expected unit struct, unit variant or constant, found struct variant `Primitive::F32`
   --> src/common.rs:32:9
    |
32  |         Primitive::F32 => types::F32,
    |
   ::: /checkout/compiler/rustc_target/src/abi/mod.rs:726:5
    |
726 |     F32 {
726 |     F32 {
    |     --- `Primitive::F32` defined here
help: use struct pattern syntax instead
    |
    |
32  |         Primitive::F32 { /* fields */ } => types::F32,
help: consider importing one of these items instead
    |
1   | use cranelift_codegen::ir::types::F32;
    |
---

error[E0532]: expected unit struct, unit variant or constant, found struct variant `Primitive::F64`
   --> src/common.rs:33:9
    |
33  |         Primitive::F64 => types::F64,
    |
   ::: /checkout/compiler/rustc_target/src/abi/mod.rs:729:5
    |
729 |     F64 {
729 |     F64 {
    |     --- `Primitive::F64` defined here
help: use struct pattern syntax instead
    |
    |
33  |         Primitive::F64 { /* fields */ } => types::F64,
help: consider importing one of these items instead
    |
1   | use cranelift_codegen::ir::types::F64;
    |
---

error[E0532]: expected unit struct, unit variant or constant, found struct variant `Primitive::Pointer`
   --> src/common.rs:34:9
    |
34  |         Primitive::Pointer => pointer_ty(tcx),
    |
   ::: /checkout/compiler/rustc_target/src/abi/mod.rs:732:5
    |
732 |     Pointer {
732 |     Pointer {
    |     ------- `Primitive::Pointer` defined here
    |
help: use struct pattern syntax instead
    |
34  |         Primitive::Pointer { /* fields */ } => pointer_ty(tcx),
help: consider importing one of these items instead
    |
    |
1   | use rustc_codegen_ssa::common::TypeKind::Pointer;
1   | use rustc_span::sym::Pointer;
    |

error[E0308]: `match` arms have incompatible types
error[E0308]: `match` arms have incompatible types
   --> src/discriminant.rs:111:22
    |
109 |               let signed = match tag_scalar.value {
    |  __________________________-
110 | |                 Int(_, signed) => signed,
    | |                                   ------ this is found to be of type `IntFlags`
111 | |                 _ => false,
    | |                      ^^^^^ expected struct `IntFlags`, found `bool`
    | |_____________- `match` arms have incompatible types

Some errors have detailed explanations: E0308, E0532.
For more information about an error, try `rustc --explain E0308`.

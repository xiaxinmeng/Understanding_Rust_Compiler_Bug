plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0425]: cannot find function `message` in this scope
  --> compiler/rustc_codegen_ssa/src/back/archive.rs:62:30
   |
62 |                 .map_err(|e| message("failed to extract section", &e))?;
   |
help: consider importing this constant
   |
1  | use rustc_span::sym::message;

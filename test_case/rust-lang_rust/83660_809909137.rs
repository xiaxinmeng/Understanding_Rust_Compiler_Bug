plain
    Checking cranelift-frontend v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-jit v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking cranelift-object v0.72.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#8e43e964)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `iter` found for struct `Statements<'_>` in the current scope
  --> src/analyze.rs:30:35
   |
30 |         for stmt in bb.statements.iter() {
   |                                   ^^^^ method not found in `Statements<'_>`

error[E0277]: `&Statements<'_>` is not an iterator
    |
    |
214 |         for stmt in &bb_data.statements {
    |                     ^^^^^^^^^^^^^^^^^^^ `&Statements<'_>` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&Statements<'_>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&Statements<'_>`


error[E0277]: `&Statements<'_>` is not an iterator
    |
    |
232 |                     for stmt in &fx.mir[*target].statements {
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&Statements<'_>` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `&Statements<'_>`
    = note: required because of the requirements on the impl of `IntoIterator` for `&Statements<'_>`


error[E0609]: no field `source_info` on type `&rustc_middle::mir::Statement<'tcx>`
    |
    |
430 |     fx.set_debug_loc(stmt.source_info);
    |
    = note: available fields are: `kind`


error[E0609]: no field `source_info` on type `&rustc_middle::mir::Statement<'tcx>`
    |
    |
725 | ...                   .is_sized(fx.tcx.at(stmt.source_info.span), ParamEnv::reveal_all())
    |
    = note: available fields are: `kind`


error[E0609]: no field `source_info` on type `&rustc_middle::mir::Statement<'tcx>`
    |
    |
831 |                     .span_fatal(stmt.source_info.span, "Inline assembly is not supported"),
    |
    = note: available fields are: `kind`

error: aborting due to 6 previous errors

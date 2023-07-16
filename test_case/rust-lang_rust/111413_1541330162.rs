plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0277]: the trait bound `StreamingBuffer<BufWriter<std::fs::File>>: object::write::util::WritableBuffer` is not satisfied
   --> compiler/rustc_codegen_ssa/src/back/link.rs:651:32
    |
651 |         package.finish()?.emit(&mut output_stream)?;
    |                                ^^^^^^^^^^^^^^^^^^ the trait `object::write::util::WritableBuffer` is not implemented for `StreamingBuffer<BufWriter<std::fs::File>>`
    = help: the following other types implement trait `object::write::util::WritableBuffer`:
              Vec<u8>
              object::write::util::StreamingBuffer<W>
              object::write::util::StreamingBuffer<W>
    = note: required for the cast from `StreamingBuffer<BufWriter<std::fs::File>>` to the object type `dyn object::write::util::WritableBuffer`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_codegen_ssa` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_ssa` (lib) due to previous error

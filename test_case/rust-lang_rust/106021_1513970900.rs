
error[E0277]: the trait bound `StreamingBuffer<BufWriter<std::fs::File>>: object::write::util::WritableBuffer` is not satisfied
   --> compiler/rustc_codegen_ssa/src/back/link.rs:649:32
    |
649 |         package.finish()?.emit(&mut output_stream)?;
    |                                ^^^^^^^^^^^^^^^^^^ the trait `object::write::util::WritableBuffer` is not implemented for `StreamingBuffer<BufWriter<std::fs::File>>`
    |
    = help: the following other types implement trait `object::write::util::WritableBuffer`:
              Vec<u8>
              object::write::util::StreamingBuffer<W>
    = note: required for the cast from `StreamingBuffer<BufWriter<std::fs::File>>` to the object type `dyn object::write::util::WritableBuffer`

   Compiling rustc_transmute v0.1.0 (/Volumes/QuickMac/rust/compiler/rustc_transmute)
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:50

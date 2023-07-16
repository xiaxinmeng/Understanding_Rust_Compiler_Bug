
<backtrace>


   Compiling main v0.1.0 (/usr1/w00620137/rust_dev/test/main)
error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:12:16
   |
12 |     IN_STP                                      // 寄存器保护
   |                ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:18:22
   |
18 |     LOAD_HASH_TABLE                            // 加载g
   |                      ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:23:27
   |
23 |     LOAD_HASH_TABLE                                                // load hash table
   |                           ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:29:12
   |
29 |     b.le .LEnc_end
   |            ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:33:37
   |
33 |     ENC1_LOOP                         // 处理64字节
   |                                     ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:39:28
   |
39 |     b.le .LEnc_end                          // <= 0
   |                            ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:44:33
   |
44 |     b.le .LEnc_end                          // <= 0
   |                                 ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:53:5
   |
53 | .size SOME_FUNC_A,.-SOME_FUNC_A
   |     ^

error: unrecognized instruction mnemonic
   |
note: instantiated into assembly here
  --> <inline asm>:53:21
   |
53 | .size SOME_FUNC_A,.-SOME_FUNC_A
   |                     ^

error: symbol 'SOME_FUNC_A' is already defined
   |
note: instantiated into assembly here
  --> <inline asm>:62:48
   |
62 |     IN_STP                                      // 寄存器保护
   |                                                ^

thread 'rustc' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', compiler/rustc_span/src/lib.rs:1761:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/b628260df0587ae559253d8640ecb8738d3de613/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/b628260df0587ae559253d8640ecb8738d3de613/library/core/src/panicking.rs:67:14
   2: core::panicking::panic
             at /rustc/b628260df0587ae559253d8640ecb8738d3de613/library/core/src/panicking.rs:117:5
   3: <rustc_span::SourceFile>::lookup_file_pos
   4: <rustc_span::SourceFile>::lookup_file_pos_with_col_display
   5: <rustc_span::source_map::SourceMap>::lookup_char_pos
   6: <rustc_errors::emitter::EmitterWriter>::get_multispan_max_line_num
   7: <rustc_errors::emitter::EmitterWriter>::emit_messages_default
   8: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
   9: <rustc_errors::json::Diagnostic>::from_errors_diagnostic
  10: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
  11: <rustc_errors::HandlerInner>::emit_diagnostic::{closure#2}
  12: <rustc_errors::HandlerInner>::emit_diagnostic
  13: <rustc_errors::diagnostic_builder::Noted as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  14: <rustc_codegen_ssa::back::write::SharedEmitterMain>::check
  15: <rustc_codegen_ssa::back::write::OngoingCodegen<rustc_codegen_llvm::LlvmCodegenBackend>>::join
  16: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::join_codegen
  17: <rustc_interface::queries::Linker>::link
  18: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  19: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (b628260df 2023-04-22) running on aarch64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `main` (bin "main") due to 10 previous errors

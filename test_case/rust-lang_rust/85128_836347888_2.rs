shell
stack backtrace:
   0:     0x7ffd960b5d2e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h65f771762210e3e5
   1:     0x7ffd960de84c - core::fmt::write::h3d73561f801c6731
   2:     0x7ffd960a9888 - <std::io::IoSlice as core::fmt::Debug>::fmt::h6fd08a0ca4059bc8
   3:     0x7ffd960b9de2 - std::panicking::take_hook::ha49dee799706a2d8
   4:     0x7ffd960b98d4 - std::panicking::take_hook::ha49dee799706a2d8
   5:     0x7ffd7303be57 - rustc_driver::report_ice::h85377e4dc53c89a8
   6:     0x7ffd960ba5c5 - std::panicking::rust_panic_with_hook::h9e7cc259d68b1de2
   7:     0x7ffd960ba1a1 - rust_begin_unwind
   8:     0x7ffd960b667f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h65f771762210e3e5
   9:     0x7ffd960ba0f9 - rust_begin_unwind
  10:     0x7ffd9611201c - std::panicking::begin_panic_fmt::h6be4566f1d66379f
  11:     0x7ffd765a9140 - rustc_codegen_ssa::coverageinfo::map::FunctionCoverage::get_expressions_and_counter_regions::h5459bf7c73688deb
  12:     0x7ffd73256072 - <rustc_codegen_llvm::llvm_::ObjectFile as core::ops::drop::Drop>::drop::hfb1ea7ee5c5da4b8
  13:     0x7ffd73302a75 - <rustc_codegen_llvm::base::ValueIter as core::iter::traits::iterator::Iterator>::next::ha01d740fab69f46b
  14:     0x7ffd732ba777 - <rustc_codegen_llvm::back::lto::ThinLTOKeysMap as core::fmt::Debug>::fmt::hc8b79cef3f320fc8
  15:     0x7ffd73302211 - <rustc_codegen_llvm::base::ValueIter as core::iter::traits::iterator::Iterator>::next::ha01d740fab69f46b
  16:     0x7ffd732b406e - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h1fc5f21611e540fb
  17:     0x7ffd731761a2 - rustc_interface::passes::BoxedResolver::to_resolver_outputs::hd59e13590b9f5648
  18:     0x7ffd7318ec56 - rustc_interface::queries::Queries::ongoing_codegen::h7c88f490e7bc85a6
  19:     0x7ffd73046627 - rustc_driver::pretty::print_after_hir_lowering::h7bca709356dc8324
  20:     0x7ffd7303eb6c - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hf06cd1d1b5fda44d
  21:     0x7ffd73047906 - rustc_driver::pretty::print_after_hir_lowering::h7bca709356dc8324
  22:     0x7ffd7306d9f4 - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h2eaf6a8c0589bd7e
  23:     0x7ffd730702bf - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h2eaf6a8c0589bd7e
  24:     0x7ffd7307a8fd - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h2eaf6a8c0589bd7e
  25:     0x7ffd960c7f8a - std::sys::windows::thread::Thread::new::h35832c8baf769397
  26:     0x7ffdfc8f7c24 - BaseThreadInitThunk
  27:     0x7ffdfd40d721 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (ca82264ec 2021-05-09) running on x86_64-pc-windows-msvc

note: compiler flags: -Z instrument-coverage -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `tokio-util`

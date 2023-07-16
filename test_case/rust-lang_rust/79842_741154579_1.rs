rust
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler\rustc_middle\src\ty\query\mod.rs:235:5
stack backtrace:
   0:     0x7ffbcd57b865 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h106ca6f8fb6a294d
   1:     0x7ffbcd5a727b - core::fmt::write::h6390b994d95aebdb
   2:     0x7ffbcd56cefd - <std::io::IoSlice as core::fmt::Debug>::fmt::hd713332ae62141f3
   3:     0x7ffbcd57fbdd - std::panicking::take_hook::h78f8a454f3a4df82
   4:     0x7ffbcd57f66a - std::panicking::take_hook::h78f8a454f3a4df82
   5:     0x7ffba5661127 - rustc_driver::report_ice::h4bc3647b6f621de8
   6:     0x7ffbcd580730 - std::panicking::rust_panic_with_hook::h931fd6f26be82856
   7:     0x7ffbcd5801d3 - rust_begin_unwind
   8:     0x7ffbcd57c20f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h106ca6f8fb6a294d
   9:     0x7ffbcd580159 - rust_begin_unwind
  10:     0x7ffbcd5a32f0 - core::panicking::panic_fmt::h45da916c710b88f7
  11:     0x7ffbcd5a323c - core::panicking::panic::h3a7a516f2be28c18
  12:     0x7ffba9902a70 - rustc_middle::ty::query::force_from_dep_node::h841e1a791cab1f4a
  13:     0x7ffba8d0a65a - <rustc_incremental::assert_dep_graph::GraphvizDepGraph as rustc_graphviz::Labeller>::node_label::h0377fe93b350976d
  14:     0x7ffba8d21410 - rustc_incremental::persist::fs::garbage_collect_session_directories::h8752c03e5176082c
  15:     0x7ffba8cd84d9 - rustc_incremental::persist::save::save_work_product_index::h9f355c816532590a
  16:     0x7ffba8cd4634 - punycode::encode::h3996edd3fcfd4e50
  17:     0x7ffba8d20ca9 - rustc_incremental::persist::fs::garbage_collect_session_directories::h8752c03e5176082c
  18:     0x7ffba8cd7715 - rustc_incremental::persist::save::save_dep_graph::h454bb9997b91eed7
  19:     0x7ffba8bf5790 - rustc_codegen_ssa::base::finalize_tcx::h0bd1f35d020d2bd3
  20:     0x7ffba5a74565 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h409f60e0ae4aaf13
  21:     0x7ffba5858ede - rustc_interface::interface::parse_cfgspecs::he4e421c50de55949
  22:     0x7ffba588d3da - rustc_interface::passes::BoxedResolver::to_resolver_outputs::hb096f93f8cf6df4c
  23:     0x7ffba58d5a46 - rustc_interface::queries::Queries::ongoing_codegen::ha9e2915cfb307d3a
  24:     0x7ffba5608b74 - <rustc_codegen_ssa::back::linker::MsvcLinker as rustc_codegen_ssa::back::linker::Linker>::no_crt_objects::h47dd01e667a15f55
  25:     0x7ffba5699aac - <rustc_driver::args::Error as core::fmt::Debug>::fmt::hb838ed2d57a61e3c
  26:     0x7ffba5609bfc - <rustc_codegen_ssa::back::linker::MsvcLinker as rustc_codegen_ssa::back::linker::Linker>::no_crt_objects::h47dd01e667a15f55
  27:     0x7ffba56b0f84 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::hb838ed2d57a61e3c
  28:     0x7ffba56b8270 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h0da38a96661182e5
  29:     0x7ffba5612a3d - <rustc_codegen_ssa::back::linker::MsvcLinker as rustc_codegen_ssa::back::linker::Linker>::no_crt_objects::h47dd01e667a15f55
  30:     0x7ffbcd58f4f7 - std::sys::windows::thread::Thread::new::h05d768f6146b61f1
  31:     0x7ffc07127034 - BaseThreadInitThunk
  32:     0x7ffc07e9cec1 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (3d6705aa5 2020-12-07) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: 2 warnings emitted

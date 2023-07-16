
thread 'rustc' panicked at 'Box<Any>', C:\bot\slave\nightly-dist-rustc-win-msvc-64\build\src\librustc_errors\lib.rs:423
stack backtrace:
   0:     0x7ffd4f0dff30 - std::panicking::Location::line::h8c528bbdf4eef8a3
   1:     0x7ffd4f0df3b2 - std::panicking::Location::line::h8c528bbdf4eef8a3
   2:     0x7ffd4f0e2dfd - std::panicking::rust_panic_with_hook::h7abc6e334345e341
   3:     0x7ffd5c1429ea - <unknown>
   4:     0x7ffd5c15db8e - rustc_errors::Handler::bug::hbd513abe0d1e3f18
   5:     0x7ffd4e3cd065 - rustc::session::bug_fmt::hcff0bf8ae86d3c88
   6:     0x7ffd4e3cca72 - rustc::session::bug_fmt::hcff0bf8ae86d3c88
   7:     0x7ffd4e3cc18c - rustc::session::bug_fmt::hcff0bf8ae86d3c88
   8:     0x7ffd4eebc6f8 - rustc_trans::context::CrateContext::layout_of::he1b6401d700095e0
   9:     0x7ffd4ee794b2 - rustc_trans::abi::FnType::apply_attrs_callsite::h73e6feb1527ce239
  10:     0x7ffd4ef25f4c - rustc_trans::type_::Type::from_primitive::h9e7efe5b450d9e05
  11:     0x7ffd4ef25e46 - rustc_trans::type_::Type::from_primitive::h9e7efe5b450d9e05
  12:     0x7ffd4ee74ff3 - rustc_trans::abi::FnType::unadjusted::h9efa81225506b54d
  13:     0x7ffd4ee745ab - rustc_trans::abi::FnType::unadjusted::h9efa81225506b54d
  14:     0x7ffd4eeda76f - rustc_trans::debuginfo::CrateDebugContext::new::h155d50bc9ace152f
  15:     0x7ffd4ef1ebdd - rustc_trans::trans_item::TransItem::predefine::h059b29bceadef9e0
  16:     0x7ffd4ee845a0 - rustc_trans::base::trans_crate::h7630c9da869079ce
  17:     0x7ffd61efa922 - rustc_driver::driver::phase_4_translate_to_llvm::h7a1e00c44f844853
  18:     0x7ffd61ecbe45 - rustc_driver::driver::compile_input::h44e43be0fc34a1a1
  19:     0x7ffd61ef04ff - rustc_driver::driver::count_nodes::h761a1dba824a4f1e
  20:     0x7ffd61e2c51b - <unknown>
  21:     0x7ffd61ec7f4c - rustc_driver::driver::compile_input::h44e43be0fc34a1a1
  22:     0x7ffd61f19917 - rustc_driver::run_compiler::h5e34a930c0929e5c
  23:     0x7ffd61def0de - <unknown>
  24:     0x7ffd4f0e5e31 - _rust_maybe_catch_panic
  25:     0x7ffd61e1dec6 - <unknown>
  26:     0x7ffd4f0dd42e - std::sys::imp::thread::Thread::new::h7a1b2b1b6bab390a
  27:     0x7ffd92828363 - BaseThreadInitThunk


stack backtrace:
     0: std::panicking::default_hook::{{closure}}
     1: std::panicking::default_hook
     2: rustc::util::common::panic_hook
     3: std::panicking::rust_panic_with_hook
     4: std::panicking::continue_panic_fmt
     5: rust_begin_unwind
     6: core::panicking::panic_fmt
     7: core::panicking::panic
     8: <rustc_codegen_llvm::back::archive::LlvmArchiveBuilder as rustc_codegen_ssa::back::archive::ArchiveBuilder>::build
     9: rustc_codegen_ssa::back::link::link_binary
    10: rustc::util::common::time
    11: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
    12: rustc_interface::queries::Query<T>::compute
    13: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::link
    14: rustc_interface::interface::run_compiler_in_existing_thread_pool
    15: std::thread::local::LocalKey<T>::with
     16: scoped_tls::ScopedKey<T>::set
     17: syntax::with_globals
     note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     query stack during panic:
     end of query stack

     error: internal compiler error: unexpected panic

**BACKTRACE=full:**
     stack backtrace:
     0:        0x10550c312 - std::panicking::default_hook::{{closure}}::h6e0c5011619dccf8
   1:        0x10550bfdb - std::panicking::default_hook::h673b9cd39a829714
   2:        0x1044a34a3 - rustc::util::common::panic_hook::h84287c71f722b4de
   3:        0x10550cbe1 - std::panicking::rust_panic_with_hook::h7d6a669f1a899680
   4:        0x10550c62d - std::panicking::continue_panic_fmt::h682bc3f8dac9c5a1
   5:        0x10550c519 - rust_begin_unwind
   6:        0x1055383c2 - core::panicking::panic_fmt::he11aece98ed722cd
   7:        0x105538307 - core::panicking::panic::h89241d71a860ed98
   8:        0x1060408b4 - <rustc_codegen_llvm::back::archive::LlvmArchiveBuilder as rustc_codegen_ssa::back::archive::ArchiveBuilder>::build::hb83f4d41f3bb743f
   9:        0x106029121 - rustc_codegen_ssa::back::link::link_binary::h5ac4a21e99cf8e2d
  10:        0x1060764f5 - rustc::util::common::time::hb999715119d81bd3
  11:        0x1061225ed - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link::h56979a2f10dbb1ea
  12:        0x1014fb632 - rustc_interface::queries::Query<T>::compute::h28c05c945908a803
  13:        0x101589c2c - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::link::h13b90e3c9c77b107
  14:        0x101382ca2 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hd51cad8f0ae43601
  15:        0x1013a0534 - std::thread::local::LocalKey<T>::with::h9299c21742d55aa6
  16:        0x1013b1a22 - scoped_tls::ScopedKey<T>::set::head1877c75a7c8b4
  17:        0x1013c9135 - syntax::with_globals::h8e099f446704231c
  18:        0x10137345a - std::sys_common::backtrace::__rust_begin_short_backtrace::hdebc92dc94cbb432
  19:        0x10551bf3f - __rust_maybe_catch_panic
  20:        0x101384187 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h35ffbfb717702997
  21:        0x1054efc6e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::ha44ca87baccc45af
  22:        0x10551ad4e - std::sys::unix::thread::Thread::new::thread_start::hceb9fd3b79bf176c
  23:     0x7fff6db1b2eb - _pthread_body
  24:     0x7fff6db1e249 - _pthread_start
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

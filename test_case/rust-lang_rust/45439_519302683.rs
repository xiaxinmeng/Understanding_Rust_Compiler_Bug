
$ ./mach build
 0:00.28 Clobber not needed.
 0:00.28 Adding make options from /fast/ben/tb/mozilla/mozconfig
    MOZ_OBJDIR=/fast/ben/tb/mozilla/obj-x86_64-pc-linux-gnu
    OBJDIR=/fast/ben/tb/mozilla/obj-x86_64-pc-linux-gnu
    FOUND_MOZCONFIG=/fast/ben/tb/mozilla/mozconfig
    export FOUND_MOZCONFIG
 0:00.29 /usr/bin/make -f client.mk -s
 0:00.46 Elapsed: 0.00s; From dist/public: Kept 0 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.46 Elapsed: 0.00s; From dist/private: Kept 0 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.48 Elapsed: 0.01s; From dist/xpi-stage: Kept 58 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.57 Elapsed: 0.12s; From _tests: Kept 1473 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.63 Elapsed: 0.17s; From dist/bin: Kept 2715 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.77 Elapsed: 0.29s; From dist/include: Kept 6258 existing; Added/updated 0; Removed 0 files and 0 directories.
 0:00.78 ./buildid.h.stub
 0:00.86 ./source-repo.h.stub
 0:01.24 build/application.ini.stub
 0:01.34 build/application.ini.h.stub
 0:01.44 js/src/frontend/binast/force-cargo-host-program-build
 0:01.44 toolkit/library/rust/force-cargo-library-build
 0:01.44 js/src/rust/force-cargo-library-build
 0:01.44 toolkit/crashreporter/rust/force-cargo-host-library-build
 0:01.44 testing/geckodriver/force-cargo-program-build
 0:01.49     Blocking    Blocking waiting for file lock on package cache lock
 0:01.49  waiting for file lock on package cache lock
 0:01.49     Blocking waiting for file lock on package cache lock
 0:01.51     Blocking waiting for file lock on package cache lock
 0:01.95     Blocking waiting for file lock on package cache lock
 0:02.36     Blocking waiting for file lock on package cache lock
 0:02.74     Blocking waiting for file lock on package cache lock
 0:02.76     Blocking waiting for file lock on package cache lock
 0:02.80     Blocking waiting for file lock on package cache lock
 0:03.06 comm/mail/app
 0:03.23     Blocking waiting for file lock on package cache lock
 0:03.50     Blocking waiting for file lock on package cache lock
 0:03.52     Blocking waiting for file lock on package cache lock
 0:03.52     Blocking waiting for file lock on package cache lock
 0:03.53     Blocking waiting for file lock on package cache lock
 0:03.53     Blocking  waiting for file lock on build directory
 0:03.54   Blocking waiting for file lock on package cache lock
 0:03.54     Finished dev [optimized + debuginfo] target(s) in 2.10s
 0:03.54     Blocking waiting for file lock on package cache lock
 0:03.55     Blocking waiting for file lock on build directory
 0:03.55     Finished dev [optimized + debuginfo] target(s) in 2.11s
 0:03.61    Compiling style v0.0.1 (/fast/ben/tb/mozilla/servo/components/style)
 0:04.04 comm/mail/app/thunderbird
 1:32.24 thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
 1:32.24 stack backtrace:
 1:32.48    0:     0x7fea577c4823 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h6485381528590a55
 1:32.49                                at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
 1:32.49    1:     0x7fea577bc4cb - std::sys_common::backtrace::_print::h49a82ae9552e35c7
 1:32.49                                at src/libstd/sys_common/backtrace.rs:71
 1:32.49    2:     0x7fea577c0a56 - std::panicking::default_hook::{{closure}}::he20974adbefcc046
 1:32.50                                at src/libstd/sys_common/backtrace.rs:59
 1:32.50                                at src/libstd/panicking.rs:197
 1:32.50    3:     0x7fea577c07e9 - std::panicking::default_hook::he4af6af4ac7fef7b
 1:32.50                                at src/libstd/panicking.rs:211
 1:32.50    4:     0x7fea554f9680 - rustc::util::common::panic_hook::h47e7e1f47d58fee8
 1:32.50    5:     0x7fea577c1248 - std::panicking::rust_panic_with_hook::h057ff03eb4c8000f
 1:32.50                                at src/libstd/panicking.rs:478
 1:32.50    6:     0x7fea577c0ce1 - std::panicking::continue_panic_fmt::ha6d6ae144369025b
 1:32.50                                at src/libstd/panicking.rs:381
 1:32.50    7:     0x7fea577c0bc5 - rust_begin_unwind
 1:32.50                                at src/libstd/panicking.rs:308
 1:32.50    8:     0x7fea577e9d9c - core::panicking::panic_fmt::hc4f83bfed80aeabd
 1:32.50                                at src/libcore/panicking.rs:85
 1:32.50    9:     0x7fea577e9cdb - core::panicking::panic::h62fdcfa056e70982
 1:32.50                                at src/libcore/panicking.rs:49
 1:32.50   10:     0x7fea55594b07 - rustc::ty::query::plumbing::<impl rustc::dep_graph::dep_node::DepNode>::cache_on_disk::h235d90ce08144d33
 1:32.50   11:     0x7fea551387e0 - <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::h04e9bd0a956328c6
 1:32.50   12:     0x7fea555dfeea - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter::h6fb1ae03745b62d7
 1:32.50   13:     0x7fea5504b536 - rustc::dep_graph::graph::DepGraph::exec_cache_promotions::h81051aba84c9e86e
 1:32.50   14:     0x7fea519771ab - rustc::dep_graph::graph::DepGraph::with_ignore::h2f171eb9b4dc3a0d
 1:32.50   15:     0x7fea5193ed85 - rustc::util::common::time::h35b9392ec92eda4d
 1:32.50   16:     0x7fea51980dc0 - rustc_incremental::persist::save::save_in::hb558fe5b34642efc
 1:32.50   17:     0x7fea5193eee9 - rustc::util::common::time::h59c1f39bdab7372f
 1:32.50   18:     0x7fea5197fec2 - rustc_incremental::persist::save::save_dep_graph::hff2b87f9f7880928
 1:32.50   19:     0x7fea51c4ecc5 - rustc::util::common::time::he70dede8e4b5d29b
 1:32.50   20:     0x7fea51c73a1e - rustc_codegen_ssa::base::assert_and_save_dep_graph::h1e531df11c3f2d92
 1:32.50   21:     0x7fea4d5ced36 - rustc_codegen_ssa::base::codegen_crate::h62f831815cdf11fd
 1:32.50   22:     0x7fea4d624539 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::hef05d07cb707dcde
 1:32.50   23:     0x7fea571cdde6 - rustc::util::common::time::h0fbcc227f41e7f6f
 1:32.50   24:     0x7fea57193c3a - rustc_interface::passes::start_codegen::hb08d4fff6b46f5df
 1:32.50   25:     0x7fea57176ac6 - rustc::ty::context::tls::enter_global::h979075051b657993
 1:32.50   26:     0x7fea57194430 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::hfb73e196d3ef6f7e
 1:32.50   27:     0x7fea57233974 - rustc_interface::passes::create_global_ctxt::{{closure}}::h5bc249452380bb5b
 1:32.51   28:     0x7fea57192f85 - rustc_interface::passes::BoxedGlobalCtxt::enter::h2f56c115baeac6e1
 1:32.51   29:     0x7fea5717bb6f - rustc_interface::queries::Query<T>::compute::h137166c00d774605
 1:32.51   30:     0x7fea57283683 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen::h74b31a771a891ecd
 1:32.51   31:     0x7fea57aabe5e - rustc_interface::interface::run_compiler_in_existing_thread_pool::h01017a6213536cb6
 1:32.51   32:     0x7fea57b29da5 - std::thread::local::LocalKey<T>::with::h57e4e9c51937ccb9
 1:32.51   33:     0x7fea57acceb4 - scoped_tls::ScopedKey<T>::set::h581b66761e86975e
 1:32.51   34:     0x7fea57b18801 - syntax::with_globals::h7a79417414defd1b
 1:32.51   35:     0x7fea57a7a414 - std::sys_common::backtrace::__rust_begin_short_backtrace::h81c9d6a59d8e80cf
 1:32.51   36:     0x7fea577d2169 - __rust_maybe_catch_panic
 1:32.51                                at src/libpanic_unwind/lib.rs:85
 1:32.51   37:     0x7fea57aa5e58 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd1b720554dd4145a
 1:32.51   38:     0x7fea577a336e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h805c3cc89d534c05
 1:32.51                                at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/liballoc/boxed.rs:704
 1:32.51   39:     0x7fea577d0ddf - std::sys::unix::thread::Thread::new::thread_start::h6f10b78f26c98dc6
 1:32.51                                at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/liballoc/boxed.rs:704
 1:32.51                                at src/libstd/sys_common/thread.rs:13
 1:32.51                                at src/libstd/sys/unix/thread.rs:79
 1:32.51   40:     0x7fea5770dfa2 - start_thread
 1:32.51   41:     0x7fea5762d88e - __clone
 1:32.51   42:                0x0 - <unknown>
 1:32.51 query stack during panic:
 1:32.51 end of query stack
 1:33.07 thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
 1:33.07   left: `LLVMing`,
 1:33.07  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1509:21
 1:33.07 stack backtrace:
 1:33.07    0:     0x7fea577c4823 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h6485381528590a55
 1:33.07                                at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
 1:33.07    1:     0x7fea577bc4cb - std::sys_common::backtrace::_print::h49a82ae9552e35c7
 1:33.07                                at src/libstd/sys_common/backtrace.rs:71
 1:33.07    2:     0x7fea577c0a56 - std::panicking::default_hook::{{closure}}::he20974adbefcc046
 1:33.07                                at src/libstd/sys_common/backtrace.rs:59
 1:33.07                                at src/libstd/panicking.rs:197
 1:33.07    3:     0x7fea577c07e9 - std::panicking::default_hook::he4af6af4ac7fef7b
 1:33.07                                at src/libstd/panicking.rs:211
 1:33.07    4:     0x7fea554f9680 - rustc::util::common::panic_hook::h47e7e1f47d58fee8
 1:33.07    5:     0x7fea577c1248 - std::panicking::rust_panic_with_hook::h057ff03eb4c8000f
 1:33.08                                at src/libstd/panicking.rs:478
 1:33.08    6:     0x7fea577c0ce1 - std::panicking::continue_panic_fmt::ha6d6ae144369025b
 1:33.08                                at src/libstd/panicking.rs:381
 1:33.08    7:     0x7fea577c0c2e - std::panicking::begin_panic_fmt::he54eae869ed71eb1
 1:33.08                                at src/libstd/panicking.rs:336
 1:33.08    8:     0x7fea4d4f79aa - std::sys_common::backtrace::__rust_begin_short_backtrace::hfd4f61cbe896c449
 1:33.08    9:     0x7fea4d547f1b - std::panicking::try::do_call::hca59485ee8276411
 1:33.08   10:     0x7fea577d2169 - __rust_maybe_catch_panic
 1:33.08                                at src/libpanic_unwind/lib.rs:85
 1:33.08   11:     0x7fea4d5138c3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hfebbca42beb71568
 1:33.08   12:     0x7fea577a336e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h805c3cc89d534c05
 1:33.08                                at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/liballoc/boxed.rs:704
 1:33.08   13:     0x7fea577d0ddf - std::sys::unix::thread::Thread::new::thread_start::h6f10b78f26c98dc6
 1:33.08                                at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/liballoc/boxed.rs:704
 1:33.08                                at src/libstd/sys_common/thread.rs:13
 1:33.08                                at src/libstd/sys/unix/thread.rs:79
 1:33.08   14:     0x7fea5770dfa2 - start_thread
 1:33.08   15:     0x7fea5762d88e - __clone
 1:33.08   16:                0x0 - <unknown>
 1:33.08 query stack during panic:
 1:33.08 end of query stack
 1:33.41 error: internal compiler error: unexpected panic
 1:33.41 note: the compiler unexpectedly panicked. this is a bug.
 1:33.41 note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
 1:33.41 note: rustc 1.36.0 (a53f9df32 2019-07-03) running on x86_64-unknown-linux-gnu
 1:33.41 note: compiler flags: -C opt-level=1 -C panic=abort -C debuginfo=2 -C debug-assertions=on -C linker=/fast/ben/tb/mozilla/build/cargo-linker -C incremental -C opt-level=1 -C debuginfo=2 -C force-frame-pointers=yes --crate-type lib
 1:33.41 note: some of the compiler flags provided by cargo are hidden
 1:33.56     Finished dev [optimized + debuginfo] target(s) in 1m 32s
 1:33.58     Finished dev [optimized + debuginfo] target(s) in 1m 32s
 1:33.59 error: Could not compile `style`.
 1:33.59 To learn more, run the command again with --verbose.
 1:33.59 make[4]: *** [/fast/ben/tb/mozilla/config/makefiles/rust.mk:245: force-cargo-library-build] Error 101
 1:33.59 make[3]: *** [/fast/ben/tb/mozilla/config/recurse.mk:74: toolkit/library/rust/target] Error 2
 1:33.59 make[3]: *** Waiting for unfinished jobs....
 1:33.61 make[2]: *** [/fast/ben/tb/mozilla/config/recurse.mk:34: compile] Error 2
 1:33.61 make[1]: *** [/fast/ben/tb/mozilla/config/rules.mk:391: default] Error 2
 1:33.61 make: *** [client.mk:125: build] Error 2
 1:33.62 536 compiler warnings present.


RandomInsano@chip-04:~/Documents/Code/pscontroller-rs/src$ rustc --version
rustc 1.26.0-nightly (55c984ee5 2018-03-16)
RandomInsano@chip-04:~/Documents/Code/pscontroller-rs/src$ cargo clean
RandomInsano@chip-04:~/Documents/Code/pscontroller-rs/src$ CARGO_INCREMENTAL=0 RUST_BACKTRACE=full cargo test --verbose
   Compiling unicode-xid v0.1.0
     Running `rustc --crate-name unicode_xid /home/RandomInsano/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.1.0/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="default"' -C metadata=1991908c4612c1c0 -C extra-filename=-1991908c4612c1c0 --out-dir /home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps -L dependency=/home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps --cap-lints allow`
   Compiling cc v1.0.8
     Running `rustc --crate-name cc /home/RandomInsano/.cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.8/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=c045fdd855277bee -C extra-filename=-c045fdd855277bee --out-dir /home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps -L dependency=/home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps --cap-lints allow`
thread 'rustc' panicked at 'index out of bounds: the len is 4847291 but the index is 4847291', /checkout/src/libserialize/opaque.rs:259:21
stack backtrace:
   0: 0xb6c80667 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h93b2fefcbf6912f6
                       at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: 0xb6c4f1eb - std::sys_common::backtrace::print::h8275cc4a0228025e
                       at libstd/sys_common/backtrace.rs:71
                       at libstd/sys_common/backtrace.rs:59
   2: 0xb6c6a457 - std::panicking::default_hook::{{closure}}::ha990f106491019b1
                       at libstd/panicking.rs:207
   3: 0xb6c6a0bf - std::panicking::default_hook::h1ab2d07a5322573d
                       at libstd/panicking.rs:223
   4: 0xb5a18137 - core::ops::function::Fn::call::hb910eaef0aecb065
   5: 0xb6c6a9cb - std::panicking::rust_panic_with_hook::h06fc1d87327d7e52
                       at libstd/panicking.rs:403
   6: 0xb6c6a7f3 - std::panicking::begin_panic_fmt::h1b61dff2ddab2600
                       at libstd/panicking.rs:349
   7: 0xb6c6a74b - rust_begin_unwind
                       at libstd/panicking.rs:325
   8: 0xb6cdab6f - core::panicking::panic_fmt::h67a7a8328d952f02
                       at libcore/panicking.rs:72
   9: 0xb6cdab23 - core::panicking::panic_bounds_check::h434bc36e56d90864
                       at libcore/panicking.rs:58
  10: 0xb6494b43 - serialize::serialize::Decoder::read_seq::hfd5771196e2a4d15
  11: 0xb6488767 - serialize::serialize::Decoder::read_struct::hf10cf5868951fa01
  12: 0xb64a8c63 - <rustc_metadata::decoder::DecodeContext<'a, 'tcx> as serialize::serialize::SpecializedDecoder<rustc::mir::interpret::AllocId>>::specialized_decode::h511731b3428098a3
  13: 0xb6477efb - serialize::serialize::Decoder::read_struct::h4ac607f9629c6909
  14: 0xb64a2463 - serialize::serialize::Decoder::read_enum::hbf203143f32403f7
  15: 0xb64a1757 - serialize::serialize::Decoder::read_enum::ha59c8e856fabf996
  16: 0xb649632f - serialize::serialize::Decoder::read_enum::h0ced95c27abbb786
  17: 0xb6497f9f - serialize::serialize::Decoder::read_enum::h349e7636a72a1bf6
  18: 0xb647508f - serialize::serialize::Decoder::read_struct::h2cc6da3b554b474e
  19: 0xb642eecf - <alloc::boxed::Box<T> as serialize::serialize::Decodable>::decode::h82bbbf14f32bf6fb
  20: 0xb64a4cbf - serialize::serialize::Decoder::read_enum::hdea19f36b5c31d62
  21: 0xb648eab3 - serialize::serialize::Decoder::read_seq::h42ae64e41bddab1e
  22: 0xb64a2f1f - serialize::serialize::Decoder::read_enum::hd21ad6388c600ca6
  23: 0xb6494ea7 - serialize::serialize::Decoder::read_enum::h04162e21b57d971a
  24: 0xb6483a7f - serialize::serialize::Decoder::read_struct::hc1701c750f0aab71
  25: 0xb648ee5b - serialize::serialize::Decoder::read_seq::h56e0898483a14ca7
  26: 0xb647bb1b - serialize::serialize::Decoder::read_struct::h75818a2537dc5cf5
  27: 0xb6494107 - serialize::serialize::Decoder::read_seq::heac0b0a9d8bae61b
  28: 0xb646d153 - <rustc::mir::Mir<'tcx> as serialize::serialize::Decodable>::decode::{{closure}}::h6a7f2382a57ec903
  29: 0xb6491cd7 - serialize::serialize::Decoder::read_seq::ha5867f7bfb039a0b
  30: 0xb646d253 - <rustc::mir::Mir<'tcx> as serialize::serialize::Decodable>::decode::{{closure}}::h6a7f2382a57ec903
  31: 0xb64f734b - rustc_metadata::decoder::<impl rustc_metadata::schema::Lazy<T>>::decode::h4478f672edc8a274
  32: 0xb63ea67f - rustc_metadata::cstore_impl::provide_extern::optimized_mir::hef17fb39368def9d
  33: 0xb593f76f - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::compute_result::hc8153a167fd2cc80
  34: 0xb563bf23 - rustc::dep_graph::graph::DepGraph::with_task_impl::h1f17613b988fb4f7
  35: 0xb56b817b - rustc_errors::Handler::track_diagnostics::h6f907ce7a064e0ae
  36: 0xb58b8557 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h89642d6b285a296b
  37: 0xb593f7f3 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::force::hb839e3233a21d5c0
  38: 0xb593fd0f - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get::h02048f96cd2d3418
  39: 0xb5900de7 - rustc::ty::maps::TyCtxtAt::optimized_mir::hefe786359add3d19
  40: 0xb58f1ae7 - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir::h1f2f8c01d42aa611
  41: 0xb5f573cf - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  42: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  43: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  44: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  45: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  46: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  47: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  48: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  49: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  50: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  51: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  52: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  53: 0xb5f5703b - rustc_mir::monomorphize::collector::collect_items_rec::ha798000c0ba6a115
  54: 0xb5f561fb - rustc_mir::monomorphize::collector::collect_crate_mono_items::h42ba49e83cce04e8
  55: 0xb160b1f3 - rustc::util::common::time::h36e2915d760802d3
  56: 0xb157d5f3 - rustc_trans::base::collect_and_partition_translation_items::hb00fda54eb793e4e
  57: 0xb5659243 - rustc::dep_graph::graph::DepGraph::with_task_impl::h8bf405f93ff5c5e3
  58: 0xb56b4b1b - rustc_errors::Handler::track_diagnostics::h60114bf68b72fde4
  59: 0xb58c5637 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::hb32546aed983fc24
  60: 0xb59b0ccf - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force::ha60bc632a0148e93
  61: 0xb59b134f - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get::had2faba087de8a66
  62: 0xb5906137 - rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items::h68ae702fb87be5ef
  63: 0xb58ff5eb - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items::h20d5713291984f0b
  64: 0xb157be8b - rustc_trans::base::trans_crate::h1a33bde112087c12
  65: 0xb1597a5f - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h635b3e99aaa549d9
  66: 0xb6de4f07 - rustc::util::common::time::h181240aad27d5e8a
  67: 0xb6dc9637 - rustc_driver::driver::phase_4_translate_to_llvm::hc89bfbd9ca36d741
  68: 0xb6e4a187 - rustc_driver::driver::compile_input::{{closure}}::hfa6b2fbd25427f03
  69: 0xb6e48513 - <std::thread::local::LocalKey<T>>::with::hfe1ad4c1ecfc61bf
  70: 0xb6e4477b - <std::thread::local::LocalKey<T>>::with::h9ec517b779397ffc
  71: 0xb6eac32f - rustc::ty::context::TyCtxt::create_and_enter::ha1426d1d12735577
  72: 0xb6dc417b - rustc_driver::driver::compile_input::h738177a00b975227
  73: 0xb6e6ad4f - rustc_driver::run_compiler_impl::ha84cb5f01fd0dd97
  74: 0xb6da15d7 - syntax::with_globals::h35a730735981f17e
  75: 0xb6dee2cb - std::sys_common::backtrace::__rust_begin_short_backtrace::h941f7d5fb16dafc2
  76: 0xb6c8e81f - __rust_maybe_catch_panic
                       at libpanic_unwind/lib.rs:102
  77: 0xb6df881f - <F as alloc::boxed::FnBox<A>>::call_box::hf3c580d870bbed4a
  78: 0xb6c83abf - std::sys_common::thread::start_thread::hc9c9c8bccde31424
                       at /checkout/src/liballoc/boxed.rs:793
                       at libstd/sys_common/thread.rs:24
  79: 0xb6c8a597 - std::sys::unix::thread::Thread::new::thread_start::h86817e665e0cb8a0
                       at libstd/sys/unix/thread.rs:90

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (55c984ee5 2018-03-16) running on armv7-unknown-linux-gnueabihf

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `cc`.

Caused by:
  process didn't exit successfully: `rustc --crate-name cc /home/RandomInsano/.cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.8/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=c045fdd855277bee -C extra-filename=-c045fdd855277bee --out-dir /home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps -L dependency=/home/RandomInsano/Documents/Code/pscontroller-rs/target/debug/deps --cap-lints allow` (exit code: 101)

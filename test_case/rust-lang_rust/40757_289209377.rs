
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/option.rs:323
stack backtrace:
   1:     0x7f8643d0540c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f8643d139ae - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x7f8643d13553 - std::panicking::default_hook::h1670459d2f3f8843
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:     0x7f8643d13e4b - std::panicking::rust_panic_with_hook::hcf0ddb069e7beee7
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x7f8643d13ce4 - std::panicking::begin_panic::hd6eb68e27bdf6140
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:517
   6:     0x7f8643d13c09 - std::panicking::begin_panic_fmt::hfea5965948b877f8
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:501
   7:     0x7f8643d13b97 - rust_begin_unwind
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:477
   8:     0x7f8643d50edd - core::panicking::panic_fmt::hc0f6d7b2c300cdd9
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/panicking.rs:69
   9:     0x7f8643d50e14 - core::panicking::panic::h7abeb5b818ec354e
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/panicking.rs:49
  10:     0x7f86419d8876 - rustc_metadata::creader::CrateLoader::resolve_crate::h6e72600e2dd6f65b
  11:     0x7f86419df98f - <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::process_item::h3593d13570805f47
  12:     0x7f86411ff8b0 - rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::build_reduced_graph_for_item::h3cc2f08d7828c563
  13:     0x7f864120667f - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item::h775ca4c5a23a8667
  14:     0x7f8641206b7f - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item::h775ca4c5a23a8667
  15:     0x7f86411fa5bf - rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::visit_expansion::hc836a3e7268abdbf
  16:     0x7f863d69877a - syntax::ext::expand::MacroExpander::collect_invocations::h32417ac98c0c9be1
  17:     0x7f863d697140 - syntax::ext::expand::MacroExpander::expand::h695353fd58aabfd3
  18:     0x7f863d696ba5 - syntax::ext::expand::MacroExpander::expand_crate::h9811866201c8237a
  19:     0x7f86440a00dc - rustc_driver::driver::phase_2_configure_and_expand::{{closure}}::h231d639b5d33e30c
  20:     0x7f86440977f5 - rustc_driver::driver::phase_2_configure_and_expand::hd9965365e42d7a90
  21:     0x7f864409043a - rustc_driver::driver::compile_input::hd9f060ee16a643fb
  22:     0x7f86440dc844 - rustc_driver::run_compiler::h762802568c0e140e
  23:     0x7f8643fe8edb - std::panicking::try::do_call::h935e2f773deaf841
  24:     0x7f8643d1cc8a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  25:     0x7f8644011112 - <F as alloc::boxed::FnBox<A>>::call_box::he43811d1f6894655
  26:     0x7f8643d12804 - std::sys::imp::thread::Thread::new::thread_start::he668872ac11287ba
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/liballoc/boxed.rs:624
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  27:     0x7f863c7d0183 - start_thread
  28:     0x7f86439c037c - clone
  29:                0x0 - <unknown>

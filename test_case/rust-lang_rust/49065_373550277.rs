
thread 'rustc' panicked at 'index out of bounds: the len is 146464 but the index is 146464', /checkout/src/libcore/slice/mod.rs:871:14
stack backtrace:
   0:     0x7fba14b799a3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h6b1aa521439b5a8f
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fba14b70854 - std::sys_common::backtrace::_print::hf0c55570aef1b27d
                               at libstd/sys_common/backtrace.rs:71
   2:     0x7fba14b7623d - std::panicking::default_hook::{{closure}}::h488bb196ba72cdaa
                               at libstd/sys_common/backtrace.rs:59
                               at libstd/panicking.rs:207
   3:     0x7fba14b75f39 - std::panicking::default_hook::hef52c65948961f87
                               at libstd/panicking.rs:223
   4:     0x7fba115e5f8d - core::ops::function::Fn::call::h7157cbafb9ba04dd
   5:     0x7fba14b766d9 - std::panicking::rust_panic_with_hook::h8de03ed35cb0aca6
                               at libstd/panicking.rs:403
   6:     0x7fba14b764c2 - std::panicking::begin_panic_fmt::he59de9a058877729
                               at libstd/panicking.rs:349
   7:     0x7fba14b763f2 - rust_begin_unwind
                               at libstd/panicking.rs:325
   8:     0x7fba14bdbcb0 - core::panicking::panic_fmt::hcf48133a163446a9
                               at libcore/panicking.rs:72
   9:     0x7fba14bdbc53 - core::panicking::panic_bounds_check::had8878732a61d7c7
                               at libcore/panicking.rs:58
  10:     0x7fba1177bb46 - rustc::dep_graph::graph::DepGraph::with_task_impl::h69ea33b1a83370f9
  11:     0x7fba11b03486 - rustc::ty::maps::<impl rustc::ty::maps::queries::crate_disambiguator<'tcx>>::force::h4a442cc6872941ce
  12:     0x7fba11b04409 - rustc::ty::maps::<impl rustc::ty::maps::queries::crate_disambiguator<'tcx>>::try_get::h2e50dfe8a458fb39
  13:     0x7fba11b7d7e5 - rustc::ty::maps::TyCtxtAt::crate_disambiguator::hb8db33de2220679d
  14:     0x7fba11980911 - rustc::ty::maps::on_disk_cache::OnDiskCache::compute_cnum_map::{{closure}}::{{closure}}::h36dadc8e90e8c0c2
  15:     0x7fba115c4c63 - <std::collections::hash::map::HashMap<K, V, S> as core::iter::traits::Extend<(K, V)>>::extend::h44090189c5bea819
  16:     0x7fba1175f6b2 - rustc::dep_graph::graph::DepGraph::with_ignore::h204ae534ae08069c
  17:     0x7fba1197c624 - rustc::ty::maps::on_disk_cache::OnDiskCache::load_diagnostics::h87d8c2c7cdf16d7e
  18:     0x7fba1179ae41 - rustc::dep_graph::graph::DepGraph::try_mark_green::h6adb32782295b92c
  19:     0x7fba11962a88 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h9dd24a0a495e64bf
  20:     0x7fba11aa5763 - rustc::ty::maps::<impl rustc::ty::maps::queries::trans_fn_attrs<'tcx>>::try_get::h73a16b44c41157cd
  21:     0x7fba11b7b6cb - rustc::ty::maps::TyCtxtAt::trans_fn_attrs::hc9a60d17a8556b94
  22:     0x7fba1179f8fa - <rustc::hir::check_attr::CheckAttrVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h537d59fb9e638edf
  23:     0x7fba117a1907 - rustc::hir::check_attr::check_crate::h9223c79ba3d97259
  24:     0x7fba14ed35bb - rustc::ty::context::TyCtxt::create_and_enter::hb54e14a06694259e
  25:     0x7fba14f38321 - rustc_driver::driver::compile_input::h6560736e056a863b
  26:     0x7fba14f55435 - rustc_driver::run_compiler::h060b34b2732dceee
  27:     0x7fba14e7e200 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd18945a624b32a6b
  28:     0x7fba14b8f53e - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  29:     0x7fba14eb91d6 - <F as alloc::boxed::FnBox<A>>::call_box::h75b5bcc306bf3f92
  30:     0x7fba14b8679b - std::sys::unix::thread::Thread::new::thread_start::h3d29264a23eaa0fc
                               at /checkout/src/liballoc/boxed.rs:793
                               at libstd/sys_common/thread.rs:24
                               at libstd/sys/unix/thread.rs:90
  31:     0x7fba0fa61493 - start_thread
  32:     0x7fba1484fafe - __clone
  33:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (521d91c6b 2018-03-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `myci`.

Caused by:
  process didn't exit successfully: `rustc --crate-name myci src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=3b60f9127c64b658 -C extra-filename=-3b60f9127c64b658 --out-dir /home/james/code/myci/myci_rs/target/debug/deps -C incremental=/home/james/code/myci/myci_rs/target/debug/incremental -L dependency=/home/james/code/myci/myci_rs/target/debug/deps --extern uuid=/home/james/code/myci/myci_rs/target/debug/deps/libuuid-812570f1efa45dae.rlib --extern r2d2=/home/james/code/myci/myci_rs/target/debug/deps/libr2d2-b007805a486e80d1.rlib --extern unshare=/home/james/code/myci/myci_rs/target/debug/deps/libunshare-2ae1263adf91031c.rlib --extern futures=/home/james/code/myci/myci_rs/target/debug/deps/libfutures-e25fbc7f0146e954.rlib --extern websocket=/home/james/code/myci/myci_rs/target/debug/deps/libwebsocket-16323a0f8cd6c0ba.rlib --extern hyper=/home/james/code/myci/myci_rs/target/debug/deps/libhyper-c44559d6f78edfb7.rlib --extern chrono=/home/james/code/myci/myci_rs/target/debug/deps/libchrono-59e40df4a6365872.rlib --extern dotenv=/home/james/code/myci/myci_rs/target/debug/deps/libdotenv-3647f4459f633ef0.rlib --extern libmount=/home/james/code/myci/myci_rs/target/debug/deps/liblibmount-62a1ac17e6f4ea2a.rlib --extern base64=/home/james/code/myci/myci_rs/target/debug/deps/libbase64-c91850d8cd9276d4.rlib --extern ring=/home/james/code/myci/myci_rs/target/debug/deps/libring-197552645c135b9e.rlib --extern diesel=/home/james/code/myci/myci_rs/target/debug/deps/libdiesel-20dd4c1711b4fc1b.rlib -L native=/usr/lib/x86_64-linux-gnu -L native=/home/james/code/myci/myci_rs/target/debug/build/backtrace-sys-f7e191858d272cd3/out/.libs -L native=/home/james/code/myci/myci_rs/target/debug/build/ring-55fc3222616ce83e/out` (exit code: 101)

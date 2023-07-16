
ERROR:rbml::reader: failed to find block with tag 1
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librbml/lib.rs:253
stack backtrace:
   1:     0x7fe80c6489f0 - sys::backtrace::write::hce8e882332e546a7JGt
   2:     0x7fe80c669820 - failure::on_fail::h588dafd90e82b74cj7z
   3:     0x7fe80c5d9ff0 - rt::unwind::begin_unwind_inner::h80ab16382fe3341ajLz
   4:     0x7fe8063243e0 - rt::unwind::begin_unwind::h1631746081972254121
   5:     0x7fe806322b60 - reader::get_doc::h2922d64c0e3fa9de0La
   6:     0x7fe80a99e5c0 - metadata::decoder::item_name::h342ac23f34c6fc45aDg
   7:     0x7fe80a9aa970 - metadata::decoder::get_trait_item_name_and_kind::hc85cab121b9bcebaAfh
   8:     0x7fe80a9ca310 - metadata::csearch::get_trait_item_name_and_kind::h5e523f8f26b802252yj
   9:     0x7fe80b84bff0 - Resolver<$u{27}a$GT$::build_reduced_graph_for_external_crate_def::he6103e3a99036d22gTc
  10:     0x7fe80b8517d0 - Resolver<$u{27}a$GT$::populate_external_module::unboxed_closure.13677
  11:     0x7fe80b81c260 - Resolver<$u{27}a$GT$::populate_module_if_necessary::h13429a2c5fef87d3i7c
  12:     0x7fe80b8598e0 - Resolver<$u{27}a$GT$::resolve_single_import::h617f6a0cbefe0c03zzd
  13:     0x7fe80b854d40 - Resolver<$u{27}a$GT$::resolve_imports_for_module::h21760343ab6f5d103ld
  14:     0x7fe80b853f90 - Resolver<$u{27}a$GT$::resolve_imports_for_module_subtree::hcf21ceb4110b82efjjd
  15:     0x7fe80b889db0 - resolve_crate::h5275b8fb523748b5Zxi
  16:     0x7fe80cba6d70 - driver::phase_3_run_analysis_passes::hdab79b3989f72249Jta
  17:     0x7fe80cb890b0 - driver::compile_input::h136bd835ef2474a0wba
  18:     0x7fe80cd55df0 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h3869597535200102526
  19:     0x7fe80cd54500 - rt::unwind::try::try_fn::h14290739184301814176
  20:     0x7fe80c6ce4e0 - rust_try_inner
  21:     0x7fe80c6ce4d0 - rust_try
  22:     0x7fe80cd54810 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h14740898037462295535
  23:     0x7fe80c658d40 - sys::thread::thread_start::h208082a168773e0ahFw
  24:     0x7fe806e87250 - start_thread
  25:     0x7fe80c28e589 - clone
  26:                0x0 - <unknown>
Could not compile `pb`.
Caused by:
  Process didn't exit successfully: `rustc /home/kstep/git/rust-pb/src/bin/trans-done-pb.rs --crate-name trans-done-pb --crate-type bin -g --out-dir /home/kste
p/git/rust-pb/target --emit=dep-info,link -L /home/kstep/git/rust-pb/target -L /home/kstep/git/rust-pb/target/deps --extern url=/home/kstep/git/rust-pb/target/
deps/liburl-d8534f0d463d0fed.rlib --extern http=/home/kstep/git/rust-pb/target/deps/libhttp-cdd1854d276fc158.rlib --extern rustc-serialize=/home/kstep/git/rust
-pb/target/deps/librustc-serialize-bd3e38505da47309.rlib --extern pb=/home/kstep/git/rust-pb/target/libpb-6571e72a68dae90d.rlib -L /usr/lib -L /home/kstep/git/
rust-pb/target/build/time-2f77139443a72f65/out` (status=101)

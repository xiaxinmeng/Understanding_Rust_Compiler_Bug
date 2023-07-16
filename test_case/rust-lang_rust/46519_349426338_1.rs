rust
#7  0x00007ffff5a49a69 in rustc_trans::mir::trans_mir ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#8  0x00007ffff5a66c36 in rustc_trans::base::trans_instance ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#9  0x00007ffff5a97b75 in rustc_trans::trans_item::TransItemExt::define ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#10 0x00007ffff5a6afc4 in rustc_trans::base::compile_codegen_unit ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#11 0x00007ffff369af92 in rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute_result
    ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#12 0x00007ffff3885ea1 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#13 0x00007ffff34be2d2 in rustc_errors::Handler::track_diagnostics ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#14 0x00007ffff3a28240 in rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#15 0x00007ffff369b03a in rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::force ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#16 0x00007ffff369ba0f in rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::try_get ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#17 0x00007ffff37b04bd in rustc::ty::maps::TyCtxtAt::compile_codegen_unit ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#18 0x00007ffff3a4704a in rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc-d6aca8ae434d20ce.so
#19 0x00007ffff5a68f6e in rustc_trans::base::trans_crate ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#20 0x00007ffff5af88c7 in <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#21 0x00007ffff7a356d8 in rustc::util::common::time ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#22 0x00007ffff7a61fae in rustc_driver::driver::phase_4_translate_to_llvm ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#23 0x00007ffff7aa5213 in rustc_driver::driver::compile_input::{{closure}} ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#24 0x00007ffff7a9aaa2 in <std::thread::local::LocalKey<T>>::with ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#25 0x00007ffff7a9e407 in <std::thread::local::LocalKey<T>>::with ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#26 0x00007ffff7ae98a8 in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#27 0x00007ffff7a5d195 in rustc_driver::driver::compile_input ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#28 0x00007ffff7af6ca4 in rustc_driver::run_compiler ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#29 0x00007ffff7a3bed2 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#30 0x00007ffff76a94bf in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:101
#31 0x00007ffff7ab8823 in <F as alloc::boxed::FnBox<A>>::call_box ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/librustc_driver-33a33ebd533ddabe.so
#32 0x00007ffff766567c in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:827
#33 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:24
#34 std::sys::unix::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:90
#35 0x00007ffff166c7fc in start_thread (arg=0x7fffec1ff700) at pthread_create.c:465
#36 0x00007ffff734bb0f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95

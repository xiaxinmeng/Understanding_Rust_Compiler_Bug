
thread 'rustc' panicked at 'visit_mac disabled by default', src/libsyntax/visit.rs:102
stack backtrace:
   1:     0x7f6603f90d5e - std::sys::imp::backtrace::tracing::imp::write::h518610b6cdcee611
                        at /home/user/work/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f6603fc043c - std::panicking::default_hook::{{closure}}::h73d8acc8aa188e9c
                        at /home/user/work/rust/src/libstd/panicking.rs:351
   3:     0x7f6603fbff30 - std::panicking::default_hook::hb346e96e14e8e6ac
                        at /home/user/work/rust/src/libstd/panicking.rs:361
   4:     0x7f6603fc10d5 - std::panicking::rust_panic_with_hook::h4d032fa503172e37
                        at /home/user/work/rust/src/libstd/panicking.rs:555
   5:     0x7f6600b5cd83 - std::panicking::begin_panic::h6d4fab3dfce17e2b
                        at /home/user/work/rust/src/libstd/panicking.rs:517
   6:     0x7f6600c20dd9 - syntax::visit::Visitor::visit_mac::he1ff4bc18d7cfa28
                        at /home/user/work/rust/<panic macros>:3
   7:     0x7f6600c2204c - syntax::visit::walk_ty::hc237d852c17c3cad
                        at /home/user/work/rust/src/libsyntax/visit.rs:364
   8:     0x7f6600c20ccc - syntax::visit::Visitor::visit_ty::hbfa34222ed9ab474
                        at /home/user/work/rust/src/libsyntax/visit.rs:69
   9:     0x7f6600c1dba3 - syntax::visit::walk_struct_field::h2845befa07941a18
                        at /home/user/work/rust/src/libsyntax/visit.rs:611
  10:     0x7f6600c2028c - syntax::visit::Visitor::visit_struct_field::hb11317b31993e1d3
                        at /home/user/work/rust/src/libsyntax/visit.rs:87
  11:     0x7f6600c1bafb - syntax::visit::walk_struct_def::h460cba41f9913a70
                        at /home/user/work/rust/src/libsyntax/visit.rs:140
  12:     0x7f6600c202f7 - syntax::visit::Visitor::visit_variant_data::h34b9309f4a4656bf
                        at /home/user/work/rust/src/libsyntax/visit.rs:85
  13:     0x7f6600c2c1af - syntax::visit::walk_item::h952481656fd0fe0d
                        at /home/user/work/rust/src/libsyntax/visit.rs:286
  14:     0x7f6600c1f1ec - syntax::visit::Visitor::visit_item::h82930e5e5e342650
                        at /home/user/work/rust/src/libsyntax/visit.rs:61
  15:     0x7f6600cdb309 - <syntax_ext::deriving::custom::CustomDerive as syntax::ext::base::MultiItemModifier>::expand::hb85be6dadc698836
                        at /home/user/work/rust/src/libsyntax_ext/deriving/custom.rs:71
  16:     0x7f6600cfbf81 - syntax_ext::deriving::expand_derive::h11ea136a17126e4e
                        at /home/user/work/rust/src/libsyntax_ext/deriving/mod.rs:245
  17:     0x7f6600d0452f - core::ops::Fn::call::ha1fae1f0ef1c46d2
  18:     0x7f6600bba35c - <F as syntax::ext::base::MultiItemModifier>::expand::h32615659f51c84df
                        at /home/user/work/rust/src/libsyntax/ext/base.rs:135
  19:     0x7f65fa490a17 - syntax::ext::expand::MacroExpander::expand_attr_invoc::h41e5a9a709f86086
                        at /home/user/work/rust/src/libsyntax/ext/expand.rs:356
  20:     0x7f65fa49030b - syntax::ext::expand::MacroExpander::expand_invoc::hfd12226f9b1c5db7
                        at /home/user/work/rust/src/libsyntax/ext/expand.rs:332
  21:     0x7f65fa48ecc3 - syntax::ext::expand::MacroExpander::expand::h12c7b11a42c50ca2
                        at /home/user/work/rust/src/libsyntax/ext/expand.rs:269
  22:     0x7f65fa48dcd6 - syntax::ext::expand::MacroExpander::expand_crate::h0dc21b228744e473
                        at /home/user/work/rust/src/libsyntax/ext/expand.rs:208
  23:     0x7f6604991a0c - rustc_driver::driver::phase_2_configure_and_expand::{{closure}}::h9b9637cdecef8735
                        at /home/user/work/rust/src/librustc_driver/driver.rs:675
  24:     0x7f66046f068d - rustc::util::common::time::h18a389421cc20671
                        at /home/user/work/rust/src/librustc/util/common.rs:48
  25:     0x7f660498cd7e - rustc_driver::driver::phase_2_configure_and_expand::hbdde9dd6f937d0ab
                        at /home/user/work/rust/src/librustc_driver/driver.rs:638
  26:     0x7f660498673a - rustc_driver::driver::compile_input::h00c4724689831631
                        at /home/user/work/rust/src/librustc_driver/driver.rs:112
  27:     0x7f66049b2905 - rustc_driver::run_compiler::hc8108b737246cece
                        at /home/user/work/rust/src/librustc_driver/lib.rs:221
  28:     0x7f66049beda9 - rustc_driver::main::{{closure}}::hb94620e8836fbcdf
                        at /home/user/work/rust/src/librustc_driver/lib.rs:1118
  29:     0x7f66049b12ce - rustc_driver::run::{{closure}}::h1ecd8537d8480e64
                        at /home/user/work/rust/src/librustc_driver/lib.rs:137
  30:     0x7f66049bea82 - rustc_driver::monitor::{{closure}}::hbf80a57be51c8894
                        at /home/user/work/rust/src/librustc_driver/lib.rs:1052
  31:     0x7f6604941927 - <std::panic::AssertUnwindSafe<F> as core::ops::FnOnce<()>>::call_once::hd02e3f480f0a6eae
                        at /home/user/work/rust/src/libstd/panic.rs:296
  32:     0x7f66046193ca - std::panicking::try::do_call::h7674ae3dbd396508
                        at /home/user/work/rust/src/libstd/panicking.rs:460
  33:     0x7f6603fd2cb7 - __rust_try
  34:     0x7f6603fd2b37 - __rust_maybe_catch_panic
                        at /home/user/work/rust/src/libpanic_unwind/lib.rs:98
  35:     0x7f660461907f - std::panicking::try::h92a12de5d841affe
                        at /home/user/work/rust/src/libstd/panicking.rs:436
  36:     0x7f660461828a - std::panic::catch_unwind::hd118b8210d00de33
                        at /home/user/work/rust/src/libstd/panic.rs:361
  37:     0x7f6604618c93 - std::thread::Builder::spawn::{{closure}}::h64871368bcadb209
                        at /home/user/work/rust/src/libstd/thread/mod.rs:357
  38:     0x7f66046ac2ab - <F as alloc::boxed::FnBox<A>>::call_box::h24b488af099ea696
                        at /home/user/work/rust/src/liballoc/boxed.rs:605
  39:     0x7f6603ee82b9 - <Box<alloc::boxed::FnBox<A, Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..FnOnce$LT$A$GT$$GT$::call_once::hf18c24e265f2b0c0
                        at /home/user/work/rust/src/liballoc/boxed.rs:615
  40:     0x7f6603f83c44 - std::sys_common::thread::start_thread::h589b492354cb4567
                        at /home/user/work/rust/src/libstd/sys_common/thread.rs:21
  41:     0x7f6603fba49c - std::sys::imp::thread::Thread::new::thread_start::h1351221081b4f008
                        at /home/user/work/rust/src/libstd/sys/unix/thread.rs:84
  42:     0x7f65f91293a3 - <unknown>
  43:     0x7f6603bcbbdc - clone
  44:                0x0 - <unknown>


../../common/lib.rs:1:1: 1:1 error: internal compiler error: debuginfo::set_members_of_composite_type() - Already completed forward declaration re-encountered.
../../common/lib.rs:1 #![crate_id = "main#0.2.2"]
                      ^
[Switching to Thread 0x7ffff0bff480 (LWP 2173)]

Breakpoint 1, 0x00007ffff537f700 in rust_fail () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
(gdb) bt
#0  0x00007ffff537f700 in rust_fail () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#1  0x00007ffff5380037 in unwind::begin_unwind_inner::hfd37841716213d96x2c::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#2  0x00007ffff43dd4a7 in unwind::begin_unwind::h12950421085911345052::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-555559ea-0.11.0-pre.so
#3  0x00007ffff43dd428 in diagnostic::SpanHandler::span_bug::h48fc76835081112a0Xb::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libsyntax-555559ea-0.11.0-pre.so
#4  0x00007ffff5c959e7 in driver::session::Session::span_bug::hb8d7c368d1d97a6doKo::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#5  0x00007ffff5d9c7f3 in middle::trans::debuginfo::set_members_of_composite_type::h3805a45387885655QWC::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#6  0x00007ffff5d9ade8 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::h8a31e02455da0737xqC::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#7  0x00007ffff5d90fe6 in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#8  0x00007ffff5d9080b in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#9  0x00007ffff5da50aa in middle::trans::debuginfo::vec_slice_metadata::h101cbd830e2c51c10dD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#10 0x00007ffff5d911d5 in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#11 0x00007ffff5d98765 in middle::trans::debuginfo::MemberDescriptionFactory::create_member_descriptions::ha28510fe76d9a672qpC::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#12 0x00007ffff5d9adb3 in middle::trans::debuginfo::RecursiveTypeDescription::finalize::h8a31e02455da0737xqC::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#13 0x00007ffff5d90fe6 in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#14 0x00007ffff5d9080b in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#15 0x00007ffff5da5827 in middle::trans::debuginfo::subroutine_type_metadata::h6349c46bd350d435XkD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#16 0x00007ffff5d90019 in middle::trans::debuginfo::type_metadata::h3eb6bdcf2d1a89187oD::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#17 0x00007ffff5d28c51 in middle::trans::debuginfo::create_function_debug_context::h2420aabfb75dcf1ceJB::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#18 0x00007ffff5c90433 in middle::trans::base::new_fn_ctxt::hdc1078f52281090fo7p::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#19 0x00007ffff5d2d3b5 in middle::trans::base::trans_closure::h0dcff8693fb366e0tlq::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#20 0x00007ffff5c3f793 in middle::trans::base::trans_fn::h92a1c2c07b68000awtq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#21 0x00007ffff5d30e71 in middle::trans::foreign::trans_rust_fn_with_foreign_abi::h602b9f0d0017ed68S0y::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#22 0x00007ffff5c394e1 in middle::trans::base::trans_item::h8bd0be1e1511fc4dWJq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#23 0x00007ffff5c38e2a in middle::trans::base::trans_item::h8bd0be1e1511fc4dWJq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#24 0x00007ffff5c38e2a in middle::trans::base::trans_item::h8bd0be1e1511fc4dWJq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#25 0x00007ffff5c38e2a in middle::trans::base::trans_item::h8bd0be1e1511fc4dWJq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#26 0x00007ffff5c38e2a in middle::trans::base::trans_item::h8bd0be1e1511fc4dWJq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#27 0x00007ffff5d3d9d1 in middle::trans::base::trans_crate::h2f77fbab9c1e59223Cr::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#28 0x00007ffff64fca10 in driver::driver::phase_4_translate_to_llvm::h743c10cb074825937Un::v0.11.0.pre ()
   from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#29 0x00007ffff64f3062 in driver::driver::compile_input::h6483688c4f0613baICn::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#30 0x00007ffff65bc658 in driver::run_compiler::h8ac7207d0ef3107aphq::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#31 0x00007ffff65b9986 in driver::main_args::closure.94747 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#32 0x00007ffff65d425b in driver::monitor::closure.95809 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#33 0x00007ffff65cf478 in task::TaskBuilder::try::closure.95572 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc-d252d482-0.11.0-pre.so
#34 0x00007ffff56d5868 in task::spawn_opts::closure.7118 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/libnative-1fb5e2c0-0.11.0-pre.so
#35 0x00007ffff537d4b3 in task::Task::run::closure.5032 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#36 0x00007ffff53f074c in rust_try () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#37 0x00007ffff537f506 in unwind::try::h271ce1a7188be5b4TQc::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#38 0x00007ffff537d365 in task::Task::run::h4c021c9979341529qxc::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustrt-d8560cb2-0.11.0-pre.so
#39 0x00007ffff56d56db in task::spawn_opts::closure.7091 () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/libnative-1fb5e2c0-0.11.0-pre.so
#40 0x00007ffff2fdd024 in rt::thread::thread_start::h5953ff68110adc88jAs::v0.11.0.pre () from /home/piotr/Desktop/src/rust/x86_64-unknown-linux-gnu/stage1/bin/../lib/./libstd-59beb4f7-0.11.0-pre.so
#41 0x00007ffff2d13124 in start_thread () from /usr/lib/libpthread.so.0
#42 0x00007ffff50564bd in clone () from /usr/lib/libc.so.6

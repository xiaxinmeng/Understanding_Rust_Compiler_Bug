
#0  0x0000000000001000 in ?? ()
#1  0x00007f36a58f567a in backtrace_pcinfo (state=0x7f36a6359000, pc=139872682138457, 
    callback=0x7f36a5885a00 <sys_common::gnu::libbacktrace::print::pcinfo_cb::h29c7688dfe185d9fyht>, 
    error_callback=0x7f36a58859f0 <sys_common::gnu::libbacktrace::print::error_cb::h9d640c5e5585541eJgt>, data=0x7f369d7ed170)
    at /var/tmp/portage/dev-lang/rust-9999/work/rust-9999/src/libbacktrace/fileline.c:176
#2  0x00007f36a588610c in sys::backtrace::tracing::imp::write::trace_fn::h3227ea145b706b5bSst ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#3  0x00007f369f8b75c9 in _Unwind_Backtrace (
    trace=0x7f36a5885cb0 <sys::backtrace::tracing::imp::write::trace_fn::h3227ea145b706b5bSst>, trace_argument=0x7f369d7ed518)
    at /var/tmp/portage/sys-devel/gcc-5.2.0/work/gcc-5.2.0/libgcc/unwind.inc:295
#4  0x00007f36a5885b5a in sys::backtrace::tracing::imp::write::h82b779db8318b1850qt () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#5  0x00007f36a5883287 in panicking::on_panic::h5d10db3f19f75694zqx () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#6  0x00007f36a584fbdf in sys_common::unwind::begin_unwind_inner::h021af668201d101f3es ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#7  0x00007f36a5850619 in sys_common::unwind::begin_unwind_fmt::h6ae185d70e9749249ds ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#8  0x00007f36a34ae205 in front::map::Map$LT$$u27$ast$GT$::get_path_elem::hcac69fe7ff0a5591Sfb ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#9  0x00007f36a3768af8 in front::map::Map$LT$$u27$ast$GT$::with_path_next::h7677687426421862442 ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#10 0x00007f36a36070ea in middle::ty::ctxt$LT$$u27$tcx$GT$::item_path_str::h093d7dc118b8ded1nUf ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#11 0x00007f36a3605edc in middle::def_id::DefId.fmt..Debug::fmt::h084461e0056999ebtRo ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#12 0x00007f36a58eb226 in fmt::write::h4db63ca58df27bafp0W () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#13 0x00007f36a58c3e31 in fmt::builders::DebugTuple$LT$$u27$a$C$$u20$$u27$b$GT$::field::h434f658db5f3c7ecRnW ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#14 0x00007f36a35961b0 in middle::def::Def...std..fmt..Debug::fmt::h95a2b86eebd51a73Fon ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#15 0x00007f36a58eb226 in fmt::write::h4db63ca58df27bafp0W () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#16 0x00007f36a58c3905 in fmt::builders::DebugStruct$LT$$u27$a$C$$u20$$u27$b$GT$::field::he7df159292fb73ecXiW ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#17 0x00007f36a3606e12 in fmt::_$RF$$u27$a$u20$T.Debug::fmt::h9876525571199719911 () from /usr/lib64/rust-9999/librustc-gentoo-git.so
#18 0x00007f36a58eb226 in fmt::write::h4db63ca58df27bafp0W () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#19 0x00007f36a58505bb in sys_common::unwind::begin_unwind_fmt::h6ae185d70e9749249ds ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#20 0x00007f36a35a82b1 in middle::const_eval::eval_const_expr_partial::h77eb81c891e44397Nyk ()
   from /usr/lib64/rust-9999/librustc-gentoo-git.so
#21 0x00007f36a443099d in astconv::ast_ty_to_ty::h446730aafa9ba3b32ox () from /usr/lib64/rust-9999/librustc_typeck-gentoo-git.so
#22 0x00007f36a449c199 in collect::convert_field::h079a992ee3267235d5y () from /usr/lib64/rust-9999/librustc_typeck-gentoo-git.so
#23 0x00007f36a4488ab0 in collect::convert_item::h6dd288f7ad9918efTdz () from /usr/lib64/rust-9999/librustc_typeck-gentoo-git.so
#24 0x00007f36a4483a88 in collect::collect_item_types::hdc4b9b528a28e1a1ppy ()
   from /usr/lib64/rust-9999/librustc_typeck-gentoo-git.so
#25 0x00007f36a44d1b9c in check_crate::hc33dccfaaa753e70ZGE () from /usr/lib64/rust-9999/librustc_typeck-gentoo-git.so
#26 0x00007f36a5da2acf in driver::phase_3_run_analysis_passes::closure.21555 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#27 0x00007f36a5d851ed in middle::ty::context::ctxt$LT$$u27$tcx$GT$::create_and_enter::h14912410508915335040 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#28 0x00007f36a5d809a2 in driver::phase_3_run_analysis_passes::h6812549652698182288 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#29 0x00007f36a5d612f4 in driver::compile_input::h14af076c634191100ba () from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#30 0x00007f36a5ec6d3c in run_compiler::h89a832461de3e8ab3qc () from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#31 0x00007f36a5ec45a7 in boxed::F.FnBox$LT$A$GT$::call_box::h4415565791460127255 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#32 0x00007f36a5ec3eb5 in sys_common::unwind::try::try_fn::h17089024600160099748 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#33 0x00007f36a5882b89 in __rust_try () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#34 0x00007f36a58773cf in sys_common::unwind::try::inner_try::h9d46b930d0260ed7Dbs () from /usr/lib64/rust-9999/libstd-gentoo-git.so
#35 0x00007f36a5ec404f in boxed::F.FnBox$LT$A$GT$::call_box::h8089523330245650700 ()
   from /usr/lib64/rust-9999/librustc_driver-gentoo-git.so
#36 0x00007f36a588ae34 in sys::thread::Thread::new::thread_start::h0a7752ba7c557a49oJw ()
   from /usr/lib64/rust-9999/libstd-gentoo-git.so
#37 0x00007f369fdc2484 in start_thread (arg=0x7f369d7ff700) at pthread_create.c:334
#38 0x00007f36a55145cd in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109

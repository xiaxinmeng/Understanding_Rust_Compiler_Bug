
Breakpoint 1, 0x701c8ac0 in rust_fail () from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
(gdb) bt
#0  0x701c8ac0 in rust_fail () from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#1  0x701c95a6 in unwind::begin_unwind_inner::he0051bbad957d0946zd::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#2  0x701c8f08 in unwind::begin_unwind_fmt::h0d0f954482e3389ezxd::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#3  0x701c8cee in rust_begin_unwind () from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#4  0x70215898 in failure::begin_unwind::h1ee234800f8570f923v::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#5  0x65b97f39 in driver::driver::phase_2_configure_and_expand::h8d96f8c3f8496f2aQjv::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#6  0x65b911f5 in _fu2123___ZN9LOG_LEVEL20h1b062b54018c9f0ejia11v0.11.0.preE ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#7  0x65c4874a in driver::run_compiler::h10885a06abea3df3gXx::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#8  0x65c45e5d in driver::main_args::closure.98681 ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#9  0x65c5f851 in driver::monitor::closure.99774 ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#10 0x65c5af8e in task::TaskBuilder::try::closure.99534 ()
   from c:\Program Files (x86)\Rust\bin\rustc-d252d482-0.11.0-pre.dll
#11 0x641715e3 in task::spawn_opts::closure.7185 ()
   from c:\Program Files (x86)\Rust\bin\native-1fb5e2c0-0.11.0-pre.dll
#12 0x701c67bc in task::Task::run::closure.5421 ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#13 0x701c8ab5 in unwind::try::try_fn::haab7172debb57904Sqd::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#14 0x7022eca6 in rust_try () from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#15 0x701c8935 in unwind::try::hce2ffb3335cad393vod::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#16 0x701c657e in task::Task::run::he2dac0e0baab6665fTc::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#17 0x6417146c in task::spawn_opts::closure.7156 ()
   from c:\Program Files (x86)\Rust\bin\native-1fb5e2c0-0.11.0-pre.dll
#18 0x701c844f in thread::thread_start::h0d16693d392cfdf3Ead::v0.11.0.pre ()
   from c:\Program Files (x86)\Rust\bin\rustrt-d8560cb2-0.11.0-pre.dll
#19 0x7586338a in KERNEL32!BaseThreadInitThunk () from C:\Windows\syswow64\kernel32.dll
#20 0x77d99f72 in ntdll!RtlInitializeExceptionChain () from C:\Windows\system32\ntdll.dll
#21 0x77d99f45 in ntdll!RtlInitializeExceptionChain () from C:\Windows\system32\ntdll.dll
#22 0x00000000 in ?? ()

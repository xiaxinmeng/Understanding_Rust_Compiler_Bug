
Breakpoint 1, 0x61f882d0 in rust_fail () from c:\Rust\bin\rustrt-4e7c5e5c.dll
(gdb) bt
#0  0x61f882d0 in rust_fail () from c:\Rust\bin\rustrt-4e7c5e5c.dll
#1  0x61f88b61 in unwind::begin_unwind_inner::h010e59c18491af9ckJd ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#2  0x61f8866d in unwind::begin_unwind_fmt::hef720eb2a21d4618NGd ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#3  0x61f8849e in rust_begin_unwind () from c:\Rust\bin\rustrt-4e7c5e5c.dll
#4  0x61fd2658 in failure::begin_unwind::h952689cf05f74959Gsj ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#5  0x7130b59c in _fu8323___ZN6result23Result$LT$T$C$$x20E$GT$6unwrap15__STATIC_
FMTSTR20ha52c8aa3d1f66f95aEmE () from c:\Rust\bin\rustc-4e7c5e5c.dll
#6  0x7170c5c4 in _fu6362___ZN9LOG_LEVEL20ha26882c95fe69b47diaE ()
   from c:\Rust\bin\rustc-4e7c5e5c.dll
#7  0x717b6613 in _fu10225___ZN4cell32RefMut$LT$$x27b$C$$x20T$GT$.Drop4drop15__S
TATIC_FMTSTR20h79632b2a8020ae00K1iE () from c:\Rust\bin\rustc-4e7c5e5c.dll
#8  0x717b3b0d in driver::main_args::closure.$x22closure$x22$LP$134984$RP$ ()
   from c:\Rust\bin\rustc-4e7c5e5c.dll
#9  0x717c603e in task::TaskBuilder$LT$S$GT$::try_future::closure.$x22closure$x2
2$LP$136133$RP$ () from c:\Rust\bin\rustc-4e7c5e5c.dll
#10 0x717c5f5c in task::TaskBuilder$LT$S$GT$::spawn_internal::closure.$x22closur
e$x22$LP$136110$RP$ () from c:\Rust\bin\rustc-4e7c5e5c.dll
#11 0x70c33cf8 in task::spawn_opts::closure.$x22closure$x22$LP$8578$RP$ ()
   from c:\Rust\bin\native-4e7c5e5c.dll
#12 0x61f882c5 in unwind::try::try_fn::h20ffd20407168fa56zd ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#13 0x61fe7cf6 in rust_try () from c:\Rust\bin\rustrt-4e7c5e5c.dll
#14 0x61f86007 in unwind::try::hbad0e19adbb1bee3Jxd ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#15 0x61f85d9a in task::Task::run::h612833bf1f74f9919Xc ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#16 0x70c33b97 in task::spawn_opts::closure.$x22closure$x22$LP$8524$RP$ ()
   from c:\Rust\bin\native-4e7c5e5c.dll
#17 0x61f87e1f in thread::thread_start::h13b93ba6c58b755cHkd ()
   from c:\Rust\bin\rustrt-4e7c5e5c.dll
#18 0x764d33aa in KERNEL32!BaseThreadInitThunk ()
   from C:\Windows\syswow64\kernel32.dll
#19 0x77099ef2 in ntdll!RtlInitializeExceptionChain ()
   from C:\Windows\system32\ntdll.dll
#20 0x77099ec5 in ntdll!RtlInitializeExceptionChain ()
   from C:\Windows\system32\ntdll.dll
#21 0x00000000 in ?? ()
(gdb)

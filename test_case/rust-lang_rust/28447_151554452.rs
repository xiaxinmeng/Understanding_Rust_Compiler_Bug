
we@we-pc MINGW64 ~
$ gdb --args rust/x86_64-pc-windows-gnu/stage1/bin/rustc test.rs
GNU gdb (GDB) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from rust/x86_64-pc-windows-gnu/stage1/bin/rustc...done.
(gdb) run
Starting program: C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc.exe test.rs
[New Thread 16184.0x2e60]
[New Thread 16184.0x340c]
gdb: unknown target exception 0xe1525354 at 0x7ffa1f5d871c

Program received signal ?, Unknown signal.
[Switching to Thread 16184.0x340c]
0x00007ffa1f5d871c in RaiseException ()
   from C:\Windows\system32\KernelBase.dll
(gdb) bt
#0  0x00007ffa1f5d871c in RaiseException ()
   from C:\Windows\system32\KernelBase.dll
#1  0x000000006cd7e2dd in rt::unwind::imp::panic::h138fc8f543c5f59afHw ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#2  0x000000006cd7e729 in rust_panic ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#3  0x000000006cd44f56 in rt::unwind::begin_unwind_inner::h5fc0152d0aafc9b7qWw
    ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#4  0x000000006cd45711 in rt::unwind::begin_unwind_fmt::hfba13529a9c1d09ewVw
    ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#5  0x0000000069f97469 in middle::ty::ctxt$LT$$u27$tcx$GT$::item_variances::hd71df34ac43909d1D9b ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc-a5fc0d6c.dll
#6  0x0000000063caef9c in check::wf::CheckTypeWellFormedVisitor$LT$$u27$ccx$C$$u20$$u27$tcx$GT$::check_variances_for_type_defn::h70494ae83f1695746Dk ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#7  0x0000000063ca99ab in check::wf::CheckTypeWellFormedVisitor$LT$$u27$ccx$C$$u20$$u27$tcx$GT$::check_item_well_formed::h0cfd3f328e35e059hlk ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#8  0x0000000063cb2d20 in visit::walk_expr::h1538854274586118031 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#9  0x0000000063cb235b in visit::walk_item::h3466652534645482253 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#10 0x0000000063cdc3dd in check::check_wf_old::h5c158d441ae75523Psp ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#11 0x0000000063d752fa in check_crate::hcc545635754bdf91pDE ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_typeck-a5fc0d6c.dll
#12 0x000000006c660def in driver::phase_3_run_analysis_passes::closure.21717
    ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#13 0x000000006c65f8a0 in driver::phase_3_run_analysis_passes::h16750792111201762936 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#14 0x000000006c6435a5 in driver::compile_input::h7319900f2a3481bdYba ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#15 0x000000006c70ab66 in run_compiler::hcd12017dca535bb4eqc ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#16 0x000000006c7087d0 in boxed::F.FnBox$LT$A$GT$::call_box::h10755116803638224322 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#17 0x000000006c708249 in rt::unwind::try::try_fn::h152431577491858085 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#18 0x000000006cd6b14b in rt::unwind::try::inner_try::h8e2b93ec3e97781fjSw ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#19 0x000000006c7083d1 in boxed::F.FnBox$LT$A$GT$::call_box::h8720629305512333058 ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\rustc_driver-a5fc0d6c.dll
#20 0x000000006cd7cd25 in sys::thread::Thread::new::thread_start::h521642f36151b154A4v ()
   from C:\msys64\home\we\rust\x86_64-pc-windows-gnu\stage1\bin\std-a5fc0d6c.dll
#21 0x00007ffa1fa313d2 in KERNEL32!BaseThreadInitThunk ()
   from C:\Windows\system32\kernel32.dll
#22 0x00007ffa223c5454 in ntdll!RtlUserThreadStart ()
   from C:\Windows\SYSTEM32\ntdll.dll
#23 0x0000000000000000 in ?? ()
Backtrace stopped: previous frame inner to this frame (corrupt stack?)



thread '[codegen-units] codegen-units\partitioning\local-drop-glue.rs' panicked at 'explicit panic', src\tools\compilete
st\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\local-generic.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\local-generic.rs
" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Local\\
poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-generic\\a.exe" "-Crpath"
"-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-he
lpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/local-generic" "-L" "c:\\Local\\poligon\\github\
\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-generic\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn local_generic::generic[0]<&str> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<char> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<u32> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::generic[0]<u64> @@ local_generic.3a1fbbbh.volatile[External]
MONO_ITEM fn local_generic::mod1[0]::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM fn local_generic::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1[Internal]
MONO_ITEM fn local_generic::mod2[0]::user[0] @@ local_generic.3a1fbbbh-mod2[Internal]
MONO_ITEM fn local_generic::user[0] @@ local_generic.3a1fbbbh[Internal]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xaa4469d - _report_error
  40:          0xa8658c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z incremen
tal -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\local-generic.rs' panicked at 'explicit panic', src\tools\compiletest
\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\extern-drop-glue.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\extern-drop-glue
.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Loca
l\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\extern-drop-glue\\a.exe" "-Cr
path" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-t
est-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/extern-drop-glue" "-Zinline-in-all-cgus" "-
L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\extern-drop-glue\
\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue.3a1fbbbh[Internal]
 extern_drop_glue.3a1fbbbh-mod1[Internal]
MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh[Internal
]
MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh
-mod1[Internal]
MONO_ITEM fn extern_drop_glue::mod1[0]::user[0] @@ extern_drop_glue.3a1fbbbh-mod1[External]
MONO_ITEM fn extern_drop_glue::user[0] @@ extern_drop_glue.3a1fbbbh[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xaa2469d - _report_error
  40:          0xa8458c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\extern-drop-glue.rs' panicked at 'explicit panic', src\tools\compilet
est\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\local-inlining-but-not-all.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\local-inlining-b
ut-not-all.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o"
 "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-inlining-but-
not-all\\a.exe" "-Crpath" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\native\\rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining-but-not-
all" "-Zinline-in-all-cgus=no" "-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units
\\partitioning\\local-inlining-but-not-all\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn local_inlining_but_not_all::inline[0]::inlined_function[0] @@ local_inlining_but_not_all.3a1fbbbh-inline[Ex
ternal]
MONO_ITEM fn local_inlining_but_not_all::non_user[0]::baz[0] @@ local_inlining_but_not_all.3a1fbbbh-non_user[External]
MONO_ITEM fn local_inlining_but_not_all::user1[0]::foo[0] @@ local_inlining_but_not_all.3a1fbbbh-user1[External]
MONO_ITEM fn local_inlining_but_not_all::user2[0]::bar[0] @@ local_inlining_but_not_all.3a1fbbbh-user2[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xaa0469d - _report_error
  40:          0xa8258c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus=no -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\local-inlining-but-not-all.rs' panicked at 'explicit panic', src\tool
s\compiletest\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\inlining-from-extern-crate.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\inlining-from-ex
tern-crate.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o"
 "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\inlining-from-exter
n-crate\\a.exe" "-Crpath" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\native\\rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/inlining-from-extern-cr
ate" "-Zinline-in-all-cgus" "-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\p
artitioning\\inlining-from-extern-crate\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn cgu_explicit_inlining::always_inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_ext
ern_crate.3a1fbbbh-mod2[Internal]
MONO_ITEM fn cgu_explicit_inlining::inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_extern_cra
te.3a1fbbbh-mod1[Internal]
MONO_ITEM fn inlining_from_extern_crate::mod1[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod1[External]
MONO_ITEM fn inlining_from_extern_crate::mod2[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod2[External]
MONO_ITEM fn inlining_from_extern_crate::user[0] @@ inlining_from_extern_crate.3a1fbbbh[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xa94469d - _report_error
  40:          0xa7658c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\inlining-from-extern-crate.rs' panicked at 'explicit panic', src\tool
s\compiletest\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\local-inlining.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\local-inlining.r
s" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Local\
\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-inlining\\a.exe" "-Crpath
" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-
helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining" "-Zinline-in-all-cgus" "-L" "c:
\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-inlining\\auxilia
ry"
stdout:
------------------------------------------
MONO_ITEM fn local_inlining::inline[0]::inlined_function[0] @@ local_inlining.3a1fbbbh-user1[Internal] local_inlining.3a
1fbbbh-user2[Internal]
MONO_ITEM fn local_inlining::non_user[0]::baz[0] @@ local_inlining.3a1fbbbh-non_user[External]
MONO_ITEM fn local_inlining::user1[0]::foo[0] @@ local_inlining.3a1fbbbh-user1[External]
MONO_ITEM fn local_inlining::user2[0]::bar[0] @@ local_inlining.3a1fbbbh-user2[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xa96469d - _report_error
  40:          0xa7858c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\local-inlining.rs' panicked at 'explicit panic', src\tools\compiletes
t\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\local-transitive-inlining.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\local-transitive
-inlining.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o"
"c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\local-transitive-inl
ining\\a.exe" "-Crpath" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-g
nu\\native\\rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-transitive-inlining
" "-Zinline-in-all-cgus" "-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\part
itioning\\local-transitive-inlining\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn local_transitive_inlining::direct_user[0]::foo[0] @@ local_transitive_inlining.3a1fbbbh-indirect_user[Inter
nal]
MONO_ITEM fn local_transitive_inlining::indirect_user[0]::bar[0] @@ local_transitive_inlining.3a1fbbbh-indirect_user[Ext
ernal]
MONO_ITEM fn local_transitive_inlining::inline[0]::inlined_function[0] @@ local_transitive_inlining.3a1fbbbh-indirect_us
er[Internal]
MONO_ITEM fn local_transitive_inlining::non_user[0]::baz[0] @@ local_transitive_inlining.3a1fbbbh-non_user[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xab1469d - _report_error
  40:          0xa9358c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\local-transitive-inlining.rs' panicked at 'explicit panic', src\tools
\compiletest\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\regular-modules.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\regular-modules.
rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Local
\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\regular-modules\\a.exe" "-Crpa
th" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-tes
t-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/regular-modules" "-L" "c:\\Local\\poligon\\g
ithub\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\regular-modules\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn regular_modules::bar[0] @@ regular_modules.3a1fbbbh[Internal]
MONO_ITEM fn regular_modules::foo[0] @@ regular_modules.3a1fbbbh[Internal]
MONO_ITEM fn regular_modules::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
MONO_ITEM fn regular_modules::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
MONO_ITEM fn regular_modules::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
MONO_ITEM fn regular_modules::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]
MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]
MONO_ITEM static regular_modules::BAZ[0] @@ regular_modules.3a1fbbbh[Internal]
MONO_ITEM static regular_modules::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
MONO_ITEM static regular_modules::mod1[0]::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
MONO_ITEM static regular_modules::mod1[0]::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
MONO_ITEM static regular_modules::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
MONO_ITEM static regular_modules::mod2[0]::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
MONO_ITEM static regular_modules::mod2[0]::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xa9c469d - _report_error
  40:          0xa7e58c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z incremen
tal -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\regular-modules.rs' panicked at 'explicit panic', src\tools\compilete
st\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\statics.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\statics.rs" "-Zt
hreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Local\\poligo
n\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\statics\\a.exe" "-Crpath" "-O" "-Zunst
able-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-Zpr
int-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/statics" "-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_6
4-pc-windows-gnu\\test\\codegen-units\\partitioning\\statics\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn statics::function[0] @@ statics.3a1fbbbh[External]
MONO_ITEM fn statics::mod1[0]::function[0] @@ statics.3a1fbbbh-mod1[External]
MONO_ITEM static statics::BAR[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::FOO[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::function[0]::BAR[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::function[0]::FOO[0] @@ statics.3a1fbbbh[Internal]
MONO_ITEM static statics::mod1[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::function[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
MONO_ITEM static statics::mod1[0]::function[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]

------------------------------------------
stderr:
------------------------------------------
warning: static item is never used: `FOO`
 --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:9:1
  |
9 | static FOO: u32 = 0;
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: static item is never used: `BAR`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:12:1
   |
12 | static BAR: u32 = 0;
   | ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `FOO`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:17:5
   |
17 |     static FOO: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `BAR`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:20:5
   |
20 |     static BAR: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `FOO`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:25:5
   |
25 |     static FOO: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `BAR`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:28:5
   |
28 |     static BAR: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `FOO`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:33:9
   |
33 |         static FOO: u32 = 0;
   |         ^^^^^^^^^^^^^^^^^^^^

warning: static item is never used: `BAR`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\statics.rs:36:9
   |
36 |         static BAR: u32 = 0;
   |         ^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xab4469d - _report_error
  40:          0xa9658c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\statics.rs' panicked at 'explicit panic', src\tools\compiletest\src\r
untest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\vtable-through-const.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\vtable-through-c
onst.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\
Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\vtable-through-const\\a.e
xe" "-Crpath" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native
\\rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/vtable-through-const" "-Zinline-in-
all-cgus" "-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\vtabl
e-through-const\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<u32> @@ vtable_through_const.7rcbfp3g[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volati
le[External]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.v
olatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volati
le[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.v
olatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<char> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<i64> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volat
ile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.
volatile[External]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volat
ile[Internal]
MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.
volatile[Internal]
MONO_ITEM fn vtable_through_const::start[0] @@ vtable_through_const.7rcbfp3g[Internal]

------------------------------------------
stderr:
------------------------------------------
warning: constant item is never used: `TRAIT2_REF`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\vtable-through-const.rs:61:5
   |
61 |     pub const TRAIT2_REF: &'static Trait2 = &0u32 as &Trait2;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: constant item is never used: `TRAIT2_GEN_REF`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\vtable-through-const.rs:62:5
   |
62 |     pub const TRAIT2_GEN_REF: &'static Trait2Gen<u8> = &0u32 as &Trait2Gen<u8>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant item is never used: `ID_I64`
  --> c:\Local\poligon\github\rust\src/test\codegen-units\partitioning\vtable-through-const.rs:63:5
   |
63 |     pub const ID_I64: fn(i64) -> i64 = id::<i64>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xaa2469d - _report_error
  40:          0xa8458c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z increment
al -Z inline-in-all-cgus -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\vtable-through-const.rs' panicked at 'explicit panic', src\tools\comp
iletest\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search

---- [codegen-units] codegen-units\partitioning\shared-generics.rs stdout ----

error: compilation failed!
status: exit code: 101
command: PATH="c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage1\bin;c:\Local\poligon\github\rust\build\x8
6_64-pc-windows-gnu\llvm\build\bin;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_6
4-pc-windows-gnu\release\deps;c:\Local\poligon\github\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\windows\System32;C:
\windows;C:\windows\System32\Wbem;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\windows\System32\WindowsPo
werShell\v1.0\;c:\programdata\IPGM\OraPGM64\Oracle\Client11gR2\bin;c:\programdata\IPGM\OraPGM32\Oracle\Client11gR2\bin;C
:\Program Files (x86)\SafeCom\SafeComPrintClient;C:\Program Files (x86)\CheckPoint\Endpoint Security\Endpoint Common\bin
;C:\PROGRA~1\IBM\SQLLIB\BIN;C:\PROGRA~1\IBM\SQLLIB\FUNCTION;C:\Program Files\Git\cmd;C:\Program Files (x86)\GitExtension
s\;C:\windows\System32\WindowsPowerShell\v1.0\;C:\Local\poligon\.cargo\bin;C:\Program Files\Python2;C:\msys64\mingw64\bi
n;C:\msys64\usr\bin;C:\Users\c152216\AppData\Local\atom\bin" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows
-gnu\\stage1\\bin\\rustc.exe" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units\\partitioning\\shared-generics.
rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "c:\\Local
\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\shared-generics\\a.exe" "-Crpa
th" "-O" "-Zunstable-options" "-Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-tes
t-helpers" "-Zprint-mono-items=eager" "-Zshare-generics=yes" "-Zincremental=tmp/partitioning-tests/shared-generics-exe"
"-L" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\test\\codegen-units\\partitioning\\shared-generics
\\auxiliary"
stdout:
------------------------------------------
MONO_ITEM fn shared_generics::foo[0] @@ shared_generics.3a1fbbbh[External]
MONO_ITEM fn shared_generics_aux::generic_fn[0]<u16> @@ shared_generics_aux.3a1fbbbh-in-shared_generics.3a1fbbbh.volatil
e[External]

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src\librustc\ty\query\on_disk_cache.rs:949: Encoding DefIndex without context.

thread '<unnamed>' panicked at 'Box<Any>', src\librustc_errors\lib.rs:605:9
stack backtrace:
   0:         0x622a6ed5 - _report_error
   1:         0x6229e088 - _report_error
   2:         0x622bced5 - _report_error
   3:         0x622bcc12 - _report_error
   4:          0x1d90dcb - _report_error
   5:         0x622bd587 - _report_error
   6:         0x674491af - _report_error
   7:         0x67473c3e - _report_error
   8:          0x1861bbb - _report_error
   9:          0x18619ba - _report_error
  10:          0x1861a49 - _report_error
  11:          0x1862c3c - _report_error
  12:          0x1862ba2 - _report_error
  13:          0x42e2352 - _report_error
  14:          0x4347303 - _report_error
  15:          0x42e7b5d - _report_error
  16:          0x42f028f - _report_error
  17:          0x437df55 - _report_error
  18:          0x42e912b - _report_error
  19:          0x437faf9 - _report_error
  20:          0x43124aa - _report_error
  21:          0x433ec63 - _report_error
  22:          0x42e43e1 - _report_error
  23:          0x4310e98 - _report_error
  24:          0x430cf52 - _report_error
  25:          0x43956f5 - _report_error
  26:          0x433ce2f - _report_error
  27:          0x43c326c - _report_error
  28:          0x433d263 - _report_error
  29:          0x434df98 - _report_error
  30:          0x4390300 - _report_error
  31:         0x622c8fa8 - _report_error
  32:          0x4390184 - _report_error
  33:          0x434ea02 - _report_error
  34:          0x434f8f7 - _report_error
  35:          0x430e3dd - _report_error
  36:          0x439538a - _report_error
  37:          0x43c260c - _report_error
  38:         0x6c6d0985 - _report_error
  39:          0xa8a469d - _report_error
  40:          0xa6c58c3 - _report_error
  41:         0x639f5a2f - _report_error
  42:         0x63ad0c15 - _report_error
  43:         0x63a70823 - _report_error
  44:         0x63a3d0ef - _report_error
  45:         0x63a1168f - _report_error
  46:         0x639b5b22 - _report_error
  47:         0x63acb921 - _report_error
  48:         0x63a2b5ef - _report_error
  49:         0x63a77cf0 - _report_error
  50:         0x63a1554b - _report_error
  51:         0x622c8fa8 - _report_error
  52:         0x63a14edf - _report_error
  53:         0x6398e064 - _report_error
  54:         0x711c6cf4 - _report_error
  55:         0x711c9f34 - _report_error
  56:         0x639cd5ff - _report_error
  57:         0x63a12329 - _report_error
  58:         0x639cc6bc - _report_error
  59:         0x622c8fa8 - _report_error
  60:         0x711d0032 - _report_error
  61:         0x711c7a67 - _report_error
  62:         0x711d010e - _report_error
  63:         0x622c8fa8 - _report_error
  64:         0x711cff82 - _report_error
  65:         0x711c9cfb - _report_error
  66:         0x622b8e41 - _report_error
  67:         0x6229fd88 - _report_error
  68:         0x76d359cc - _report_error
  69:         0x76e9385c - _report_error
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-ge
nerics=yes -Z incremental -C prefer-dynamic -C rpath


------------------------------------------

thread '[codegen-units] codegen-units\partitioning\shared-generics.rs' panicked at 'explicit panic', src\tools\compilete
st\src\runtest.rs:3295:9
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618aab - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:209
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x427456 - compiletest::runtest::ProcRes::fatal::hbba49a63347d0dad
   6:           0x4221a1 - compiletest::runtest::TestCx::fatal_proc_rec::hb3753fd74fa1bd15
   7:           0x40e647 - compiletest::runtest::TestCx::run_revision::h4b5d05737daa9038
   8:           0x405da7 - compiletest::runtest::run::h9d16007c32a49185
   9:           0x467ea7 - <F as alloc::boxed::FnBox<A>>::call_box::h3efc0dcd86839534
  10:           0x4aadb4 - <F as alloc::boxed::FnBox<A>>::call_box::h03332f8886ebf2aa
                               at src\libtest/lib.rs:1468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  11:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  12:           0x4c9d11 - test::run_test::run_test_inner::{{closure}}::hfd7683d7173d3f0a
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at src\libtest/lib.rs:1430
  13:           0x4a32dd - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f710af7a73c0d09
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\sys_common/backtrace.rs:135

  14:           0x4a378d - std::panicking::try::do_call::h6beb9dc7329d2821
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:469
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:309
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:297
  15:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
  16:           0x4ab0ab - <F as alloc::boxed::FnBox<A>>::call_box::h9b90509d496c10fc
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd\thread/mod.rs:468
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  17:           0x627a64 - std::sys::windows::thread::Thread::new::thread_start::h3b918374d9ab9704
                               at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:744
                               at src\libstd\sys_common/thread.rs:14
                               at src\libstd\sys\windows/thread.rs:47
  18:         0x76d359cc - unit_addrs_search
  19:         0x76e9385c - unit_addrs_search


failures:
    [codegen-units] codegen-units\partitioning\extern-drop-glue.rs
    [codegen-units] codegen-units\partitioning\extern-generic.rs
    [codegen-units] codegen-units\partitioning\inlining-from-extern-crate.rs
    [codegen-units] codegen-units\partitioning\local-drop-glue.rs
    [codegen-units] codegen-units\partitioning\local-generic.rs
    [codegen-units] codegen-units\partitioning\local-inlining-but-not-all.rs
    [codegen-units] codegen-units\partitioning\local-inlining.rs
    [codegen-units] codegen-units\partitioning\local-transitive-inlining.rs
    [codegen-units] codegen-units\partitioning\regular-modules.rs
    [codegen-units] codegen-units\partitioning\shared-generics.rs
    [codegen-units] codegen-units\partitioning\statics.rs
    [codegen-units] codegen-units\partitioning\vtable-through-const.rs

test result: FAILED. 24 passed; 12 failed; 3 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:502:22
stack backtrace:
   0:           0x615ea4 - std::sys_common::backtrace::_print::haba1881f40f7b7cc
                               at src\libstd\sys\windows\backtrace/mod.rs:94
                               at src\libstd\sys\windows\backtrace/mod.rs:85
                               at src\libstd\sys_common/backtrace.rs:70
   1:           0x618de1 - std::panicking::default_hook::{{closure}}::h221f218b20abe005
                               at src\libstd\sys_common/backtrace.rs:58
                               at src\libstd/panicking.rs:200
   2:           0x618b23 - std::panicking::default_hook::hbae2b3160b35d096
                               at src\libstd/panicking.rs:215
   3:           0x619411 - std::panicking::rust_panic_with_hook::hd22db9dd24519846
                               at src\libstd/panicking.rs:478
   4:           0x4802b4 - std::panicking::begin_panic::hc3f8188c1f6ff183
   5:           0x469682 - compiletest::main::h3557e9b6f735d027
   6:           0x4758e5 - std::rt::lang_start::{{closure}}::haf3fd46702b7c468
   7:           0x618eb6 - std::panicking::try::do_call::h2488968fe0e5819c
                               at src\libstd/rt.rs:49
                               at src\libstd/panicking.rs:297
   8:           0x62a0c8 - _rust_maybe_catch_panic
                               at src\libpanic_unwind/lib.rs:92
   9:           0x61972e - std::rt::lang_start_internal::h3bbc40a34ccd0194
                               at src\libstd/panicking.rs:276
                               at src\libstd/panic.rs:388
                               at src\libstd/rt.rs:48
  10:           0x472217 - main
  11:           0x4013f7 - _tmainCRTStartup
  12:           0x40151a - mainCRTStartup
  13:         0x76d359cc - unit_addrs_search
  14:         0x76e9385c - unit_addrs_search


command did not execute successfully: "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\
\compiletest.exe" "--compile-lib-path" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\stage1\\bin" "--
run-lib-path" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\stage1\\lib\\rustlib\\x86_64-pc-windows-g
nu\\lib" "--rustc-path" "c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\stage1\\bin\\rustc.exe" "--src-
base" "c:\\Local\\poligon\\github\\rust\\src/test\\codegen-units" "--build-base" "c:\\Local\\poligon\\github\\rust\\buil
d\\x86_64-pc-windows-gnu\\test\\codegen-units" "--stage-id" "stage1-x86_64-pc-windows-gnu" "--mode" "codegen-units" "--t
arget" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "c:\\Local\\poligon\\github\\rust\\bu
ild\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnativ
e=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crp
ath -O -Zunstable-options  -Lnative=c:\\Local\\poligon\\github\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-he
lpers" "--docck-python" "C:\\Program Files\\Python2\\python.exe" "--lldb-python" "C:\\Program Files\\Python2\\python.exe
" "--quiet" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "-
-adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101

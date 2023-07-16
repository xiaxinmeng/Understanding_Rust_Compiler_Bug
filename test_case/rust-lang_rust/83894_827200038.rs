plain
---- [ui] ui\backtrace-debuginfo.rs stdout ----

error: test run failed!
status: exit code: 101
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29910\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.28.29910\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.5.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.0.5\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.282-8\x64\bin;C:\npm\prefix;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\backtrace-debuginfo\\a.exe"
------------------------------------------
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:183", "backtrace-debuginfo.rs:119", "backtrace-debuginfo.rs:82", "backtrace-debuginfo-aux.rs:6"]
--- stdout
backtrace-debuginfo-aux.rs:6
backtrace-debuginfo.rs:82
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:183

--- stderr
test case 2
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\backtrace-debuginfo.rs:83:9
stack backtrace:
   0:     0x7ffdcfa8845e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2acd651982b597b3
   1:     0x7ffdcfab41bd - core::fmt::write::hb03e13d59cf1fd5f
   2:     0x7ffdcfa7be18 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd2fcbf5878964876
   3:     0x7ffdcfa8c8f2 - std::panicking::take_hook::hec0a9630d503a271
   4:     0x7ffdcfa8c36b - std::panicking::take_hook::hec0a9630d503a271
   5:     0x7ffdcfa8d093 - std::panicking::rust_panic_with_hook::h53f2c9d7b3eaa7d2
   6:     0x7ff6374510a8 - std::panicking::begin_panic::{{closure}}<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:520
   7:     0x7ff637451032 - std::sys_common::backtrace::__rust_end_short_backtrace<closure-0,!>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:141
   8:     0x7ff63745581f - std::panicking::begin_panic<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:519
   9:     0x7ff637452a23 - backtrace_debuginfo::inner::{{closure}}
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:83
  10:     0x7ff637452a23 - backtrace_debuginfo::aux::callback<closure-0>
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo-aux.rs:7
  11:     0x7ff637452f06 - backtrace_debuginfo::inner
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:82
  12:     0x7ff637453280 - backtrace_debuginfo::outer
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:119
  13:     0x7ff637453674 - backtrace_debuginfo::main
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:183
  14:     0x7ff63745104c - core::ops::function::FnOnce::call_once
                               at D:\a\rust\rust\library\core\src\ops\function.rs:227
  15:     0x7ff63745104c - std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),tuple<>>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:125
  16:     0x7ff637451072 - std::rt::lang_start::{{closure}}<tuple<>>
                               at D:\a\rust\rust\library\std\src\rt.rs:49
  17:     0x7ffdcfa8d367 - std::rt::lang_start_internal::h2d1ffef100a23ae8
  18:     0x7ff637454b6b - main
  19:     0x7ff637454eb0 - invoke_main
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  20:     0x7ff637454eb0 - __scrt_common_main_seh
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  21:     0x7ffe012d7974 - BaseThreadInitThunk
  22:     0x7ffe01b0a2f1 - RtlUserThreadStart
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:183", "backtrace-debuginfo.rs:119", "backtrace-debuginfo.rs:85", "backtrace-debuginfo-aux.rs:13"]
--- stdout
backtrace-debuginfo-aux.rs:13
backtrace-debuginfo.rs:85
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:119
backtrace-debuginfo.rs:183

--- stderr
test case 3
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\backtrace-debuginfo.rs:86:9
stack backtrace:
   0:     0x7ffdcfa8845e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2acd651982b597b3
   1:     0x7ffdcfab41bd - core::fmt::write::hb03e13d59cf1fd5f
   2:     0x7ffdcfa7be18 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd2fcbf5878964876
   3:     0x7ffdcfa8c8f2 - std::panicking::take_hook::hec0a9630d503a271
   4:     0x7ffdcfa8c36b - std::panicking::take_hook::hec0a9630d503a271
   5:     0x7ffdcfa8d093 - std::panicking::rust_panic_with_hook::h53f2c9d7b3eaa7d2
   6:     0x7ff6374510a8 - std::panicking::begin_panic::{{closure}}<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:520
   7:     0x7ff637451032 - std::sys_common::backtrace::__rust_end_short_backtrace<closure-0,!>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:141
   8:     0x7ff63745581f - std::panicking::begin_panic<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:519
   9:     0x7ff637452ba3 - backtrace_debuginfo::inner::{{closure}}
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:86
  10:     0x7ff637452ba3 - backtrace_debuginfo::aux::callback_inlined<closure-1>
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo-aux.rs:14
  11:     0x7ff637452f36 - backtrace_debuginfo::inner
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:85
  12:     0x7ff637453280 - backtrace_debuginfo::outer
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:119
  13:     0x7ff637453674 - backtrace_debuginfo::main
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:183
  14:     0x7ff63745104c - core::ops::function::FnOnce::call_once
                               at D:\a\rust\rust\library\core\src\ops\function.rs:227
  15:     0x7ff63745104c - std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),tuple<>>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:125
  16:     0x7ff637451072 - std::rt::lang_start::{{closure}}<tuple<>>
                               at D:\a\rust\rust\library\std\src\rt.rs:49
  17:     0x7ffdcfa8d367 - std::rt::lang_start_internal::h2d1ffef100a23ae8
  18:     0x7ff637454b6b - main
  19:     0x7ff637454eb0 - invoke_main
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  20:     0x7ff637454eb0 - __scrt_common_main_seh
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  21:     0x7ffe012d7974 - BaseThreadInitThunk
  22:     0x7ffe01b0a2f1 - RtlUserThreadStart
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:183", "backtrace-debuginfo.rs:120", "backtrace-debuginfo.rs:105", "backtrace-debuginfo-aux.rs:6"]
--- stdout
backtrace-debuginfo-aux.rs:6
backtrace-debuginfo.rs:105
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:183

--- stderr
test case 7
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\backtrace-debuginfo.rs:106:9
stack backtrace:
   0:     0x7ffdcfa8845e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2acd651982b597b3
   1:     0x7ffdcfab41bd - core::fmt::write::hb03e13d59cf1fd5f
   2:     0x7ffdcfa7be18 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd2fcbf5878964876
   3:     0x7ffdcfa8c8f2 - std::panicking::take_hook::hec0a9630d503a271
   4:     0x7ffdcfa8c36b - std::panicking::take_hook::hec0a9630d503a271
   5:     0x7ffdcfa8d093 - std::panicking::rust_panic_with_hook::h53f2c9d7b3eaa7d2
   6:     0x7ff6374510a8 - std::panicking::begin_panic::{{closure}}<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:520
   7:     0x7ff637451032 - std::sys_common::backtrace::__rust_end_short_backtrace<closure-0,!>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:141
   8:     0x7ff63745581f - std::panicking::begin_panic<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:519
   9:     0x7ff637452ae3 - backtrace_debuginfo::inner_inlined::{{closure}}
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:106
  10:     0x7ff637452ae3 - backtrace_debuginfo::aux::callback<closure-0>
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo-aux.rs:7
  11:     0x7ff637453098 - backtrace_debuginfo::inner_inlined
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:105
  12:     0x7ff6374532b2 - backtrace_debuginfo::outer
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:120
  13:     0x7ff637453674 - backtrace_debuginfo::main
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:183
  14:     0x7ff63745104c - core::ops::function::FnOnce::call_once
                               at D:\a\rust\rust\library\core\src\ops\function.rs:227
  15:     0x7ff63745104c - std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),tuple<>>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:125
  16:     0x7ff637451072 - std::rt::lang_start::{{closure}}<tuple<>>
                               at D:\a\rust\rust\library\std\src\rt.rs:49
  17:     0x7ffdcfa8d367 - std::rt::lang_start_internal::h2d1ffef100a23ae8
  18:     0x7ff637454b6b - main
  19:     0x7ff637454eb0 - invoke_main
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  20:     0x7ff637454eb0 - __scrt_common_main_seh
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  21:     0x7ffe012d7974 - BaseThreadInitThunk
  22:     0x7ffe01b0a2f1 - RtlUserThreadStart
---------------------------------------
trace does not match position list
trace does not match position list
still need to find ["backtrace-debuginfo.rs:183", "backtrace-debuginfo.rs:120", "backtrace-debuginfo.rs:108", "backtrace-debuginfo-aux.rs:13"]
--- stdout
backtrace-debuginfo-aux.rs:13
backtrace-debuginfo.rs:108
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:120
backtrace-debuginfo.rs:183

--- stderr
test case 8
thread 'main' panicked at 'explicit panic', D:\a\rust\rust\src/test\ui\backtrace-debuginfo.rs:109:9
stack backtrace:
   0:     0x7ffdcfa8845e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2acd651982b597b3
   1:     0x7ffdcfab41bd - core::fmt::write::hb03e13d59cf1fd5f
   2:     0x7ffdcfa7be18 - <std::io::IoSlice as core::fmt::Debug>::fmt::hd2fcbf5878964876
   3:     0x7ffdcfa8c8f2 - std::panicking::take_hook::hec0a9630d503a271
   4:     0x7ffdcfa8c36b - std::panicking::take_hook::hec0a9630d503a271
   5:     0x7ffdcfa8d093 - std::panicking::rust_panic_with_hook::h53f2c9d7b3eaa7d2
   6:     0x7ff6374510a8 - std::panicking::begin_panic::{{closure}}<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:520
   7:     0x7ff637451032 - std::sys_common::backtrace::__rust_end_short_backtrace<closure-0,!>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:141
   8:     0x7ff63745581f - std::panicking::begin_panic<str>
                               at D:\a\rust\rust\library\std\src\panicking.rs:519
   9:     0x7ff637452c63 - backtrace_debuginfo::inner_inlined::{{closure}}
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:109
  10:     0x7ff637452c63 - backtrace_debuginfo::aux::callback_inlined<closure-1>
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo-aux.rs:14
  11:     0x7ff6374530c8 - backtrace_debuginfo::inner_inlined
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:108
  12:     0x7ff6374532b2 - backtrace_debuginfo::outer
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:120
  13:     0x7ff637453674 - backtrace_debuginfo::main
                               at D:\a\rust\rust\src\test\ui\backtrace-debuginfo.rs:183
  14:     0x7ff63745104c - core::ops::function::FnOnce::call_once
                               at D:\a\rust\rust\library\core\src\ops\function.rs:227
  15:     0x7ff63745104c - std::sys_common::backtrace::__rust_begin_short_backtrace<fn(),tuple<>>
                               at D:\a\rust\rust\library\std\src\sys_common\backtrace.rs:125
  16:     0x7ff637451072 - std::rt::lang_start::{{closure}}<tuple<>>
                               at D:\a\rust\rust\library\std\src\rt.rs:49
  17:     0x7ffdcfa8d367 - std::rt::lang_start_internal::h2d1ffef100a23ae8
  18:     0x7ff637454b6b - main
  19:     0x7ff637454eb0 - invoke_main
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  20:     0x7ff637454eb0 - __scrt_common_main_seh
                               at D:\a01\_work\26\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  21:     0x7ffe012d7974 - BaseThreadInitThunk
  22:     0x7ffe01b0a2f1 - RtlUserThreadStart

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
thread 'main' panicked at 'found some errors', D:\a\rust\rust\src/test\ui\backtrace-debuginfo.rs:173:9

------------------------------------------


---
test result: FAILED. 11670 passed; 1 failed; 132 ignored; 0 measured; 0 filtered out; finished in 368.22s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\ui" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--suite" "ui" "--mode" "ui" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.4\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.4\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 src/test/ui src/tools/linkchecker
Build completed unsuccessfully in 0:30:18
Build completed unsuccessfully in 0:30:18
make: *** [Makefile:78: ci-subset-2] Error 1

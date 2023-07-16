
failures:
---- [run-pass] run-pass\backtrace-debuginfo.rs stdout ----
	
error: test run failed!
status: exit code: 101
command: PATH="C:\projects\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x86;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\i686-pc-windows-msvc\stage0-tools\i686-pc-windows-msvc\release\deps;C:\projects\rust\build\i686-pc-windows-msvc\stage0-sysroot\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw32\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Tools\vcpkg;C:\Program Files (x86)\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\Tools\Binn;C:\Program Files\Microsoft SQL Server\140\DTS\Binn;C:\Program Files\PowerShell\6.0.0;C:\Program Files\erl9.2\bin;C:\Program Files (x86)\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\Yarn\bin;C:\Program Files (x86)\NSIS;C:\Tools\Octopus;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\ProgramData\chocolatey\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\i686-pc-windows-msvc\\test\\run-pass\\backtrace-debuginfo.stage2-i686-pc-windows-msvc.exe"
stdout:
------------------------------------------
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:116"]
--- stdout
backtrace-debuginfo.rs:116
backtrace-debuginfo.rs:180
--- stderr
test case 0
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:77:5
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d0f6a - backtrace_debuginfo::inner
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:77
   6:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   7: 0x77640fa8 - RtlAllocateHeap
   8: 0x7427b31b - __rust_maybe_catch_panic
   9: 0x74247929 - <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt::h604edc6961ea296f
  10: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  11:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  12:  0x13d2420 - main
  13: 0x75227c03 - BaseThreadInitThunk
  14: 0x7765ad8e - RtlInitializeExceptionChain
  15: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:116"]
--- stdout
backtrace-debuginfo.rs:116
backtrace-debuginfo.rs:180
--- stderr
test case 1
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:78:5
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d1069 - backtrace_debuginfo::inner
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:78
   6:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   7:  0x13c8db9 - std::rt::lang_start::{{closure}}<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
   8: 0x74247320 - std::panicking::try::do_call::h3dcb477987bacec8
   9: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  10:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  11:  0x13d2420 - main
  12: 0x75227c03 - BaseThreadInitThunk
  13: 0x7765ad8e - RtlInitializeExceptionChain
  14: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:116", "backtrace-debuginfo.rs:79", "backtrace-debuginfo-aux.rs:15"]
--- stdout
backtrace-debuginfo-aux.rs:15
backtrace-debuginfo.rs:79
backtrace-debuginfo.rs:116
backtrace-debuginfo.rs:180
--- stderr
test case 2
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:80:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13c97a8 - backtrace_debuginfo::inner::{{closure}}
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:80
   6:  0x13d104b - backtrace_debuginfo::inner
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:79
   7:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   8: 0x77640fa8 - RtlAllocateHeap
   9: 0x7427b31b - __rust_maybe_catch_panic
  10: 0x74247929 - <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt::h604edc6961ea296f
  11: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  12:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  13:  0x13d2420 - main
  14: 0x75227c03 - BaseThreadInitThunk
  15: 0x7765ad8e - RtlInitializeExceptionChain
  16: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:116", "backtrace-debuginfo.rs:82", "backtrace-debuginfo-aux.rs:22"]
--- stdout
backtrace-debuginfo-aux.rs:22
backtrace-debuginfo.rs:82
backtrace-debuginfo.rs:116
backtrace-debuginfo.rs:180
--- stderr
test case 3
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:83:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13c98c8 - backtrace_debuginfo::inner::{{closure}}
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:83
   6:  0x13d10ac - backtrace_debuginfo::inner
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:82
   7:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   8: 0xfffffffe - <unknown>
   9:  0x13bffff - <unknown>
  10: 0x777001df - NlsAnsiCodePage
  11:   0xf953e7 - <unknown>
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117"]
--- stdout
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 4
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:91:5
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d11b6 - backtrace_debuginfo::inner_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:91
   6:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   7: 0xfffffffe - <unknown>
   8:  0x13bffff - <unknown>
   9: 0x777001df - NlsAnsiCodePage
  10:  0x11453e7 - <unknown>
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117"]
--- stdout
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 5
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:92:5
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d12e0 - backtrace_debuginfo::inner_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:92
   6:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   7:  0x13c8db9 - std::rt::lang_start::{{closure}}<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
   8: 0x74247320 - std::panicking::try::do_call::h3dcb477987bacec8
   9: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  10:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  11:  0x13d2420 - main
  12: 0x75227c03 - BaseThreadInitThunk
  13: 0x7765ad8e - RtlInitializeExceptionChain
  14: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117", "backtrace-debuginfo.rs:100"]
--- stdout
backtrace-debuginfo.rs:100
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 6
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:98:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d150b - backtrace_debuginfo::inner_inlined::inner_further_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:98
   6:  0x13d15b1 - backtrace_debuginfo::outer
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:117
   7:  0x13c8db9 - std::rt::lang_start::{{closure}}<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
   8: 0x7427b31b - __rust_maybe_catch_panic
   9: 0x74247929 - <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt::h604edc6961ea296f
  10: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  11:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  12:  0x13d2420 - main
  13: 0x75227c03 - BaseThreadInitThunk
  14: 0x7765ad8e - RtlInitializeExceptionChain
  15: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117", "backtrace-debuginfo.rs:102", "backtrace-debuginfo-aux.rs:15"]
--- stdout
backtrace-debuginfo-aux.rs:15
backtrace-debuginfo.rs:102
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 7
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:103:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13c99e8 - backtrace_debuginfo::inner_inlined::{{closure}}
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:103
   6:  0x13d1332 - backtrace_debuginfo::inner_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:102
   7:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   8:  0x13c8db9 - std::rt::lang_start::{{closure}}<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
   9: 0x74247320 - std::panicking::try::do_call::h3dcb477987bacec8
  10: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  11:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  12:  0x13d2420 - main
  13: 0x75227c03 - BaseThreadInitThunk
  14: 0x7765ad8e - RtlInitializeExceptionChain
  15: 0x7765ad59 - RtlInitializeExceptionChain
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117", "backtrace-debuginfo.rs:105", "backtrace-debuginfo-aux.rs:22"]
--- stdout
backtrace-debuginfo-aux.rs:22
backtrace-debuginfo.rs:105
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 8
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:106:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13c9b08 - backtrace_debuginfo::inner_inlined::{{closure}}
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:106
   6:  0x13d1382 - backtrace_debuginfo::inner_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:105
   7:  0x13d2352 - backtrace_debuginfo::main
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:180
   8: 0xfffffffe - <unknown>
   9:  0x13bffff - <unknown>
  10: 0x777001df - NlsAnsiCodePage
  11:  0x15a53e7 - <unknown>
---------------------------------------
trace does not match position list
still need to find ["backtrace-debuginfo.rs:180", "backtrace-debuginfo.rs:117", "backtrace-debuginfo.rs:111"]
--- stdout
backtrace-debuginfo.rs:111
backtrace-debuginfo.rs:117
backtrace-debuginfo.rs:180
--- stderr
test case 9
thread 'main' panicked at 'explicit panic', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:98:9
stack backtrace:
   0: 0x7424baf7 - std::sys_common::backtrace::print::h254b01aa8fead75d
   1: 0x742471d8 - std::panicking::take_hook::he8cbd3b7e615f54f
   2: 0x74246e73 - std::panicking::take_hook::he8cbd3b7e615f54f
   3: 0x742476a6 - std::panicking::rust_panic_with_hook::hb82d652fc6225b11
   4:  0x13c9e82 - std::panicking::begin_panic<str*>
                       at C:\projects\rust\src\libstd\panicking.rs:365
   5:  0x13d150b - backtrace_debuginfo::inner_inlined::inner_further_inlined
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:98
   6:  0x13d15b1 - backtrace_debuginfo::outer
                       at C:\projects\rust\src\test\run-pass\backtrace-debuginfo.rs:117
   7:  0x13c8db9 - std::rt::lang_start::{{closure}}<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
   8: 0x7427b31b - __rust_maybe_catch_panic
   9: 0x74247929 - <std::collections::hash::map::DefaultHasher as core::fmt::Debug>::fmt::h604edc6961ea296f
  10: 0x74245f26 - std::panic::catch_unwind::h3c4ca22136dca0f0
  11:  0x13c8d9d - std::rt::lang_start<()>
                       at C:\projects\rust\src\libstd\rt.rs:74
  12:  0x13d2420 - main
  13: 0x75227c03 - BaseThreadInitThunk
  14: 0x7765ad8e - RtlInitializeExceptionChain
  15: 0x7765ad59 - RtlInitializeExceptionChain
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'found some errors', C:\projects\rust\src/test\run-pass\backtrace-debuginfo.rs:170:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
------------------------------------------
thread '[run-pass] run-pass\backtrace-debuginfo.rs' panicked at 'explicit panic', tools\compiletest\src\runtest.rs:2901:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-pass] run-pass\backtrace-debuginfo.rs
test result: FAILED. 2932 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
thread 'main' panicked at 'Some tests failed', tools\compiletest\src\main.rs:478:22

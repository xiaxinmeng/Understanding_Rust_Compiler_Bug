plain
test [ui] src/test\ui\wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test\ui\cmse-nonsecure\cmse-nonsecure-entry\params-on-stack.rs stdout ----
\rustc_codegen_ssa\src\back\write.rs


1 error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1495:21
+ stack backtrace:
+    0:     0x7ffe889e009a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0356d9f2f1f4e2cb
+    1:     0x7ffe88a292f8 - core::fmt::write::haf464e9d18b89c06
+    2:     0x7ffe889d2de9 - <std::io::IoSlice as core::fmt::Debug>::fmt::h69fc3b9d4bbee946
+    3:     0x7ffe889e41a6 - std::panicking::default_hook::he97d090422dbc6ec
+    4:     0x7ffe889e3d9f - std::panicking::default_hook::he97d090422dbc6ec
+    5:     0x7ffe7f44ee1a - alloc[f817adc32f681e8e]::alloc::box_free::<chalk_ir[a04c97ddccbaeb16]::ConstData<rustc_middle[81940edb23a0c8f6]::traits::chalk::RustInterner>, alloc[f817adc32f681e8e]::alloc::Global>
+    6:     0x7ffe889e4a2b - std::panicking::rust_panic_with_hook::he5a60fda5678fdcc
+    7:     0x7ffe889e47bd - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::heba5f04221b7361d
+    8:     0x7ffe889e0d17 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0356d9f2f1f4e2cb
+    9:     0x7ffe889e4450 - rust_begin_unwind
+   10:     0x7ffe88a5fb75 - core::panicking::panic_fmt::h45d1b4ea4ced0df9
+   11:     0x7ffe88a258a1 - core::panicking::assert_failed_inner::h23364ba3e517b807
+   12:     0x7ffe85771dbe - core[82d4fd0155d04d33]::panicking::assert_failed::<rustc_middle[81940edb23a0c8f6]::ty::adt::AdtDef, rustc_middle[81940edb23a0c8f6]::ty::adt::AdtDef>
+   13:     0x7ffe7f71381b - RINvNtNtCskIhAAVJFOjc_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvXs0_CsjkTM8WGRKmD_18rustc_codegen_llvmNtB1o_18LlvmCodegenBackendNtNtNtCsepgPhyfs7NM_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods12spawn_threadNCINvNtNtB2s_4back
+   14:     0x7ffe7f890e67 - <rustc_middle[81940edb23a0c8f6]::ty::Ty as rustc_target[597fdf07fa72dfc2]::abi::TyAbiInterface<_>>::ty_and_layout_field::field_ty_or_layout::<rustc_codegen_llvm[e13b266d80266ded]::context::CodegenCx>
+   15:     0x7ffe889fa22c - std::sys::windows::thread::Thread::new::h1236dd314bc377ce
+   16:     0x7ffebb187974 - BaseThreadInitThunk
+   17:     0x7ffebb2ba2f1 - RtlUserThreadStart
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.64.0-nightly (cd1087c86 2022-07-28) running on x86_64-pc-windows-msvc
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args cmse-nonsecure\cmse-nonsecure-entry\params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.12\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui\\cmse-nonsecure\\cmse-nonsecure-entry\\params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\cmse-nonsecure\\cmse-nonsecure-entry\\params-on-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\cmse-nonsecure\\cmse-nonsecure-entry\\params-on-stack\\auxiliary"
stdout: none
--- stderr -------------------------------
error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', D:\a\rust\rust\compiler\rustc_codegen_ssa\src\back\write.rs:1495:21
stack backtrace:
   0:     0x7ffe889e009a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0356d9f2f1f4e2cb
   1:     0x7ffe88a292f8 - core::fmt::write::haf464e9d18b89c06
   2:     0x7ffe889d2de9 - <std::io::IoSlice as core::fmt::Debug>::fmt::h69fc3b9d4bbee946
   3:     0x7ffe889e41a6 - std::panicking::default_hook::he97d090422dbc6ec
   4:     0x7ffe889e3d9f - std::panicking::default_hook::he97d090422dbc6ec
   5:     0x7ffe7f44ee1a - alloc[f817adc32f681e8e]::alloc::box_free::<chalk_ir[a04c97ddccbaeb16]::ConstData<rustc_middle[81940edb23a0c8f6]::traits::chalk::RustInterner>, alloc[f817adc32f681e8e]::alloc::Global>
   6:     0x7ffe889e4a2b - std::panicking::rust_panic_with_hook::he5a60fda5678fdcc
   7:     0x7ffe889e47bd - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::heba5f04221b7361d
   8:     0x7ffe889e0d17 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0356d9f2f1f4e2cb
   9:     0x7ffe889e4450 - rust_begin_unwind
  10:     0x7ffe88a5fb75 - core::panicking::panic_fmt::h45d1b4ea4ced0df9
  11:     0x7ffe88a258a1 - core::panicking::assert_failed_inner::h23364ba3e517b807
  12:     0x7ffe85771dbe - core[82d4fd0155d04d33]::panicking::assert_failed::<rustc_middle[81940edb23a0c8f6]::ty::adt::AdtDef, rustc_middle[81940edb23a0c8f6]::ty::adt::AdtDef>
  13:     0x7ffe7f71381b - RINvNtNtCskIhAAVJFOjc_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvXs0_CsjkTM8WGRKmD_18rustc_codegen_llvmNtB1o_18LlvmCodegenBackendNtNtNtCsepgPhyfs7NM_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods12spawn_threadNCINvNtNtB2s_4back
  14:     0x7ffe7f890e67 - <rustc_middle[81940edb23a0c8f6]::ty::Ty as rustc_target[597fdf07fa72dfc2]::abi::TyAbiInterface<_>>::ty_and_layout_field::field_ty_or_layout::<rustc_codegen_llvm[e13b266d80266ded]::context::CodegenCx>
  15:     0x7ffe889fa22c - std::sys::windows::thread::Thread::new::h1236dd314bc377ce
  16:     0x7ffebb187974 - BaseThreadInitThunk
  17:     0x7ffebb2ba2f1 - RtlUserThreadStart
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (cd1087c86 2022-07-28) running on x86_64-pc-windows-msvc

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
 finished in 474.902 seconds
Build completed unsuccessfully in 0:41:31
make: *** [Makefile:72: ci-subset-2] Error 1

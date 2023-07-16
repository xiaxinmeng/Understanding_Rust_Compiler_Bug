
[01:51:45] ---- [run-make] run-make\cdylib-fewer-symbols stdout ----
[01:51:45] 	
[01:51:45] error: make failed
[01:51:45] status: exit code: 2
[01:51:45] command: "make"
[01:51:45] stdout:
[01:51:45] ------------------------------------------
[01:51:45] PATH="/c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/cdylib-fewer-symbols.stage2-x86_64-pc-windows-gnu:C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin:/c/projects/rust/build/x86_64-pc-windows-gnu/stage0-tools/x86_64-pc-windows-gnu/release/deps:/c/projects/rust/build/x86_64-pc-windows-gnu/stage0-sysroot/lib/rustlib/x86_64-pc-windows-gnu/lib:/c/Program Files (x86)/Inno Setup 5:/c/Python27:/c/projects/rust/mingw64/bin:/usr/bin:/c/Perl/site/bin:/c/Perl/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Program Files/7-Zip:/c/Program Files/Microsoft/Web Platform Installer:/c/Tools/GitVersion:/c/Tools/PsTools:/c/Program Files/Git LFS:/c/Program Files (x86)/Subversion/bin:/c/Program Files/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/110/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn:/c/Program Files/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/Tools/Binn/ManagementStudio:/c/Tools/WebDriver:/c/Program Files (x86)/Microsoft SDKs/TypeScript/1.4:/c/Program Files (x86)/Microsoft Visual Studio 12.0/Common7/IDE/PrivateAssemblies:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI/wbin:/c/Ruby193/bin:/c/Tools/NUnit/bin:/c/Tools/xUnit:/c/Tools/MSpec:/c/Tools/Coverity/bin:/c/Program Files (x86)/CMake/bin:/c/go/bin:/c/Program Files/Java/jdk1.8.0/bin:/c/Python27:/c/Program Files/nodejs:/c/Program Files (x86)/iojs:/c/Program Files/iojs:/c/Users/appveyor/AppData/Roaming/npm:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/MSBuild/14.0/Bin:/c/Tools/NuGet:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/CommonExtensions/Microsoft/TestWindow:/c/Program Files/Microsoft DNX/Dnvm:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Apache/Maven/bin:/c/Python27/Scripts:/c/Tools/NUnit3:/c/Program Files/Mercurial:/c/Program Files/LLVM/bin:/c/Program Files/dotnet:/c/Program Files/erl8.3/bin:/c/Tools/curl/bin:/c/Program Files/Amazon/AWSCLI:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft Visual Studio 14.0/Common7/IDE/Extensions/Microsoft/SQLDB/DAC/140:/c/Program Files/Git/cmd:/c/Program Files/Git/usr/bin:/c/Tools/vcpkg:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files (x86)/Yarn/bin:/c/ProgramData/chocolatey/bin:/c/Program Files (x86)/Microsoft SQL Server/140/Tools/Binn:/c/Program Files/Microsoft SQL Server/140/Tools/Binn:/c/Program Files/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/nodejs:/c/Users/appveyor/AppData/Local/Yarn/bin:/c/Users/appveyor/AppData/Roaming/npm:/c/Program Files/AppVeyor/BuildAgent:/c/projects/rust:/c/projects/rust/handle" 'C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/cdylib-fewer-symbols.stage2-x86_64-pc-windows-gnu -L /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/cdylib-fewer-symbols.stage2-x86_64-pc-windows-gnu  foo.rs
[01:51:45] nm -g "/c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/cdylib-fewer-symbols.stage2-x86_64-pc-windows-gnu/foo.dll" | "C:\projects\rust/src/etc/cat-and-grep.sh" -v __rdl_ __rde_ __rg_ __rust_
[01:51:45] [[[ begin stdout ]]]
[01:51:45]                  U .bss
...
<snip>
...
[01:51:45] 0000000066e38a38 B __onexitend
[01:51:45] 0000000066d83d30 T __rdl_alloc
[01:51:45] 0000000066d83f90 T __rdl_alloc_excess
[01:51:45] 0000000066d83f00 T __rdl_alloc_zeroed
[01:51:45] 0000000066d83df0 T __rdl_dealloc
[01:51:45] 0000000066d840a0 T __rdl_grow_in_place
[01:51:45] 0000000066d83dc0 T __rdl_oom
[01:51:45] 0000000066d83e10 T __rdl_realloc
[01:51:45] 0000000066d84000 T __rdl_realloc_excess
[01:51:45] 0000000066d84120 T __rdl_shrink_in_place
[01:51:45] 0000000066d83e00 T __rdl_usable_size
[01:51:45] 0000000066deee00 T __report_gsfailure
[01:51:45] 0000000066e21140 R __rt_psrelocs_end
[01:51:45] 0000000000000000 A __rt_psrelocs_size
[01:51:45] 0000000066e21140 R __rt_psrelocs_start
[01:51:45] 0000000066e21140 R __RUNTIME_PSEUDO_RELOC_LIST__
[01:51:45] 0000000066e21140 R __RUNTIME_PSEUDO_RELOC_LIST_END__
[01:51:45] 0000000066d81430 T __rust_alloc
[01:51:45] 0000000066d814a0 T __rust_alloc_excess
[01:51:45] 0000000066d81490 T __rust_alloc_zeroed
[01:51:45] 0000000066d81450 T __rust_dealloc
[01:51:45] 0000000066d814e0 T __rust_grow_in_place
[01:51:45] 0000000066dc9fc0 T __rust_maybe_catch_panic
[01:51:45] 0000000066d81440 T __rust_oom
[01:51:45] 0000000066d81470 T __rust_realloc
[01:51:45] 0000000066d814b0 T __rust_realloc_excess
[01:51:45] 0000000066d814f0 T __rust_shrink_in_place
[01:51:45] 0000000066dca010 T __rust_start_panic
[01:51:45] 0000000066d81460 T __rust_usable_size
[01:51:45] 0000000000001000 A __section_alignment__
...
<snip>
...
[01:51:45] 0000000066deeae4 T WSAStartup
[01:51:45] 
[01:51:45] [[[ end stdout ]]]
[01:51:45] Error: should not match: __rdl_
[01:51:45] Error: should not match: __rust_
[01:51:45] 
[01:51:45] ------------------------------------------
[01:51:45] stderr:
[01:51:45] ------------------------------------------
[01:51:45] make: *** [Makefile:12: all] Error 1
[01:51:45] 
[01:51:45] ------------------------------------------
[01:51:45] 
[01:51:45] thread '[run-make] run-make\cdylib-fewer-symbols' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2570:8
[01:51:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:51:45] 
[01:51:45] 
[01:51:45] failures:
[01:51:45]     [run-make] run-make\cdylib-fewer-symbols
[01:51:45] 
[01:51:45] test result: FAILED. 167 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:51:45] 
[01:51:45] thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:331:21

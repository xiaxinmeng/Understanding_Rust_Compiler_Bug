
...
[01:13:11] error in revision `rpass2`: compilation failed!
[01:13:11] status: exit code: 101
[01:13:11] command: PATH="C:\projects\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-tools\x86_64-pc-windows-msvc\release\deps;C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-sysroot\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\msys64\mingw64\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\ProgramData\chocolatey\bin;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files (x86)\nodejs;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "C:\\projects\\rust\\src/test\\incremental\\string_constant.rs" "-L" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental" "--target=x86_64-pc-windows-msvc" "--cfg" "rpass2" "-Z" "incremental=C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\string_constant.inc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\string_constant.stage2-x86_64-pc-windows-msvc.exe" "-Crpath" "-O" "-Lnative=C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-Z" "query-dep-graph" "-Z" "query-dep-graph" "-Zincremental-info" "-L" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\string_constant.stage2-x86_64-pc-windows-msvc.incremental.libaux"
[01:13:11] stdout:
[01:13:11] ------------------------------------------
[01:13:11] 
[01:13:11] ------------------------------------------
[01:13:11] stderr:
[01:13:11] ------------------------------------------
[01:13:11] {"message":"src\\librustc_incremental\\persist\\load.rs:131: unexpected Input DepNode: Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":null}
[01:13:11] note: the compiler unexpectedly panicked. this is a bug.
[01:13:11] 
[01:13:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:11] 
[01:13:11] note: rustc 1.22.0-dev running on x86_64-pc-windows-msvc
[01:13:11] 
[01:13:11] incremental: session directory: 8 files hard-linked
[01:13:11] incremental: session directory: 0 files copied
[01:13:11] thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:492:8
[01:13:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:11] 
[01:13:11] 
[01:13:11] ------------------------------------------
[01:13:11] 
[01:13:11] thread '[incremental] incremental\string_constant.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2435:8
[01:13:11] 
[01:13:11] 
[01:13:11] failures:
[01:13:11]     [incremental] incremental\change_add_field\struct_point.rs
[01:13:11]     [incremental] incremental\change_private_fn\struct_point.rs
[01:13:11]     [incremental] incremental\change_private_impl_method\struct_point.rs
[01:13:11]     [incremental] incremental\change_pub_inherent_method_body\struct_point.rs
[01:13:11]     [incremental] incremental\change_pub_inherent_method_sig\struct_point.rs
[01:13:11]     [incremental] incremental\change_symbol_export_status.rs
[01:13:11]     [incremental] incremental\dirty_clean.rs
[01:13:11]     [incremental] incremental\hashes\call_expressions.rs
[01:13:11]     [incremental] incremental\hello_world.rs
[01:13:11]     [incremental] incremental\ich_method_call_trait_scope.rs
[01:13:11]     [incremental] incremental\ich_resolve_results.rs
[01:13:11]     [incremental] incremental\issue-38222.rs
[01:13:11]     [incremental] incremental\remove_source_file\main.rs
[01:13:11]     [incremental] incremental\spans_in_type_debuginfo.rs
[01:13:11]     [incremental] incremental\spike.rs
[01:13:11]     [incremental] incremental\string_constant.rs
[01:13:11] 
[01:13:11] test result: FAILED. 60 passed; 16 failed; 2 ignored; 0 measured; 0 filtered out


---- [compile-fail] compile-fail\type_length_limit.rs stdout ----
	
error: failure produced the wrong error: exit code: 3780268884
status: exit code: 3780268884
command: PATH="C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-tools\x86_64-pc-windows-gnu\release\deps;C:\projects\rust\build\x86_64-pc-windows-gnu\stage0-sysroot\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\Program Files (x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw64\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Installer;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Common7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\nodejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apache\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\Program Files\Mercurial;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Amazon\AWSCLI;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\SQLDB\DAC\140;C:\Program Files (x86)\Yarn\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\ProgramData\chocolatey\bin;C:\Tools\vcpkg;C:\Program Files (x86)\nodejs;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Users\appveyor\AppData\Local\Yarn\bin;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "C:\\projects\\rust\\src/test\\compile-fail\\type_length_limit.rs" "-L" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\compile-fail" "--target=x86_64-pc-windows-gnu" "-C" "prefer-dynamic" "-o" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\compile-fail\\type_length_limit.stage2-x86_64-pc-windows-gnu.exe" "-Crpath" "-O" "-Lnative=C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\test\\compile-fail\\type_length_limit.stage2-x86_64-pc-windows-gnu.compile-fail.libaux" "-A" "unused"
stdout:
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Option<((((((G, G, G), (G, G, G), ...`
  |
  = note: consider adding a `#![type_length_limit="512"]` attribute to your crate
error: aborting due to previous error
------------------------------------------
thread '[compile-fail] compile-fail\type_length_limit.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2478:8
failures:
    [compile-fail] compile-fail\E0275.rs
    [compile-fail] compile-fail\E0404.rs
    [compile-fail] compile-fail\E0405.rs
    [compile-fail] compile-fail\bad-lint-cap.rs
    [compile-fail] compile-fail\cdylib-deps-must-be-static.rs
    [compile-fail] compile-fail\cfg-arg-invalid.rs
    [compile-fail] compile-fail\cfg-empty-codemap.rs
    [compile-fail] compile-fail\coherence-inherited-assoc-ty-cycle-err.rs
    [compile-fail] compile-fail\const-size_of-cycle.rs
    [compile-fail] compile-fail\dupe-symbols-1.rs
    [compile-fail] compile-fail\dupe-symbols-2.rs
    [compile-fail] compile-fail\dupe-symbols-3.rs
    [compile-fail] compile-fail\dupe-symbols-4.rs
    [compile-fail] compile-fail\dupe-symbols-5.rs
    [compile-fail] compile-fail\dupe-symbols-6.rs
    [compile-fail] compile-fail\dupe-symbols-7.rs
    [compile-fail] compile-fail\extern-with-type-bounds.rs
    [compile-fail] compile-fail\huge-array.rs
    [compile-fail] compile-fail\huge-enum.rs
    [compile-fail] compile-fail\huge-struct.rs
    [compile-fail] compile-fail\impl-trait\auto-trait-leak.rs
    [compile-fail] compile-fail\import-loop-2.rs
    [compile-fail] compile-fail\infinite-instantiation.rs
    [compile-fail] compile-fail\inhabitedness-infinite-loop.rs
    [compile-fail] compile-fail\issue-15919.rs
    [compile-fail] compile-fail\issue-17913.rs
    [compile-fail] compile-fail\issue-17999.rs
    [compile-fail] compile-fail\issue-18400.rs
    [compile-fail] compile-fail\issue-19660.rs
    [compile-fail] compile-fail\issue-20413.rs
    [compile-fail] compile-fail\issue-21946.rs
    [compile-fail] compile-fail\issue-22599.rs
    [compile-fail] compile-fail\issue-22638.rs
    [compile-fail] compile-fail\issue-23122-1.rs
    [compile-fail] compile-fail\issue-23122-2.rs
    [compile-fail] compile-fail\issue-2330.rs
    [compile-fail] compile-fail\issue-31109.rs
    [compile-fail] compile-fail\issue-31495.rs
    [compile-fail] compile-fail\issue-8727.rs
    [compile-fail] compile-fail\lang-item-missing.rs
    [compile-fail] compile-fail\linkage2.rs
    [compile-fail] compile-fail\linkage3.rs
    [compile-fail] compile-fail\lint-removed-allow.rs
    [compile-fail] compile-fail\lint-removed-cmdline.rs
    [compile-fail] compile-fail\lint-removed.rs
    [compile-fail] compile-fail\lint-renamed-allow.rs
    [compile-fail] compile-fail\lint-renamed-cmdline.rs
    [compile-fail] compile-fail\lint-renamed.rs
    [compile-fail] compile-fail\lint-unknown-lint.rs
    [compile-fail] compile-fail\liveness-dead.rs
    [compile-fail] compile-fail\liveness-unused.rs
    [compile-fail] compile-fail\main-wrong-location.rs
    [compile-fail] compile-fail\manual-link-bad-kind.rs
    [compile-fail] compile-fail\manual-link-bad-search-path.rs
    [compile-fail] compile-fail\mir-dataflow\def-inits-1.rs
    [compile-fail] compile-fail\mir-dataflow\inits-1.rs
    [compile-fail] compile-fail\mir-dataflow\uninits-1.rs
    [compile-fail] compile-fail\mir-dataflow\uninits-2.rs
    [compile-fail] compile-fail\no-implicit-prelude-nested.rs
    [compile-fail] compile-fail\no-implicit-prelude.rs
    [compile-fail] compile-fail\no_owned_box_lang_item.rs
    [compile-fail] compile-fail\panic-runtime\abort-link-to-unwind-dylib.rs
    [compile-fail] compile-fail\panic-runtime\bad-panic-flag1.rs
    [compile-fail] compile-fail\panic-runtime\bad-panic-flag2.rs
    [compile-fail] compile-fail\panic-runtime\libtest-unwinds.rs
    [compile-fail] compile-fail\panic-runtime\transitive-link-a-bunch.rs
    [compile-fail] compile-fail\panic-runtime\two-panic-runtimes.rs
    [compile-fail] compile-fail\panic-runtime\want-abort-got-unwind.rs
    [compile-fail] compile-fail\panic-runtime\want-abort-got-unwind2.rs
    [compile-fail] compile-fail\panic-runtime\want-unwind-got-abort.rs
    [compile-fail] compile-fail\panic-runtime\want-unwind-got-abort2.rs
    [compile-fail] compile-fail\privacy2.rs
    [compile-fail] compile-fail\privacy3.rs
    [compile-fail] compile-fail\recursion.rs
    [compile-fail] compile-fail\required-lang-item.rs
    [compile-fail] compile-fail\resolve-self-in-impl-2.rs
    [compile-fail] compile-fail\resolve-unknown-trait.rs
    [compile-fail] compile-fail\rmeta_lib.rs
    [compile-fail] compile-fail\simd-type-generic-monomorphisation.rs
    [compile-fail] compile-fail\trait-bounds-not-on-struct.rs
    [compile-fail] compile-fail\trait-test.rs
    [compile-fail] compile-fail\traits-inductive-overflow-simultaneous.rs
    [compile-fail] compile-fail\traits-inductive-overflow-supertrait.rs
    [compile-fail] compile-fail\traits-inductive-overflow-two-traits.rs
    [compile-fail] compile-fail\type_length_limit.rs
test result: FAILED. 2679 passed; 85 failed; 14 ignored; 0 measured; 0 filtered out

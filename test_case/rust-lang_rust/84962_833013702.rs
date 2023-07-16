plain
$DIR\not_a_real_file\mod.rs
$DIR\mod_file_not_exist_windows.rs
diff of stderr:

4 LL | mod not_a_real_file;
6    |
6    |
-    = help: to create the module `not_a_real_file`, create file "$DIR/not_a_real_file.rs"
+    = help: to create the module `not_a_real_file`, create file "$DIR/not_a_real_file.rs" or "$DIR/not_a_real_file/mod.rs"
9 error[E0433]: failed to resolve: use of undeclared crate or module `mod_file_aux`
10   --> $DIR/mod_file_not_exist_windows.rs:7:16



The actual stderr differed from the expected stderr.
Actual stderr saved to D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\ui\parser\mod_file_not_exist_windows\mod_file_not_exist_windows.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser\mod_file_not_exist_windows.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\mingw64\bin;C:\hostedtoolcache\windows\Python\3.9.4\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.4\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\4.4\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.5.1\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.0.5\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.292-10\x64\bin;C:\npm\prefix;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files\Microsoft SDKs\Azure\Azure Dev Spaces CLI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files\TortoiseSVN\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\ui\\parser\\mod_file_not_exist_windows.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\parser\\mod_file_not_exist_windows" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui\\parser\\mod_file_not_exist_windows\\auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0583]: file not found for module `not_a_real_file`
  --> D:\a\rust\rust\src/test\ui\parser\mod_file_not_exist_windows.rs:3:1
   |
LL | mod not_a_real_file; //~ ERROR file not found for module `not_a_real_file`
   |
   |
   = help: to create the module `not_a_real_file`, create file "D:\a\rust\rust\src/test\ui\parser\not_a_real_file.rs" or "D:\a\rust\rust\src/test\ui\parser\not_a_real_file\mod.rs"
error[E0433]: failed to resolve: use of undeclared crate or module `mod_file_aux`
  --> D:\a\rust\rust\src/test\ui\parser\mod_file_not_exist_windows.rs:7:16
   |
   |
LL |     assert_eq!(mod_file_aux::bar(), 10);
   |                ^^^^^^^^^^^^ use of undeclared crate or module `mod_file_aux`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0583.
For more information about an error, try `rustc --explain E0433`.
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-gnu target=x86_64-pc-windows-gnu


command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\ui" "--build-base" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.4\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.4\\x64\\python3.exe" "--gdb" "D:\\a\\rust\\rust\\mingw64\\bin\\gdb" "--llvm-version" "12.0.0-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 src/test/ui
Build completed unsuccessfully in 0:32:10
Build completed unsuccessfully in 0:32:10
make: *** [Makefile:86: ci-mingw-subset-2] Error 1

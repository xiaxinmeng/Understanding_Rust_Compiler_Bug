plain
2019-11-17T21:49:00.7016401Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-17T21:49:00.7016448Z 
2019-11-17T21:49:00.7016576Z   git checkout -b <new-branch-name>
2019-11-17T21:49:00.7016637Z 
2019-11-17T21:49:00.7016728Z HEAD is now at 3d5c5035e Auto merge of #66499 - GuillaumeGomez:rollup-c1ayizc, r=GuillaumeGomez
2019-11-17T21:49:00.7068936Z ##[section]Starting: Decide whether to run this job
2019-11-17T21:49:00.7166636Z ==============================================================================
2019-11-17T21:49:00.7166753Z Task         : Bash
2019-11-17T21:49:00.7166849Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-17T21:49:02.2821776Z BUILD_SOURCEBRANCHNAME=auto
2019-11-17T21:49:02.2821966Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-17T21:49:02.2822162Z BUILD_SOURCEVERSION=3d5c5035ea14259b339bd2bead4c03a10ed66746
2019-11-17T21:49:02.2822361Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-17T21:49:02.2822565Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66499 - GuillaumeGomez:rollup-c1ayizc, r=GuillaumeGomez
2019-11-17T21:49:02.2822980Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-17T21:49:02.2823171Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-17T21:49:02.2823395Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-17T21:49:02.2823805Z COMPUTERNAME=fv-az665
---
2019-11-17T23:59:56.2277367Z status: exit code: 2
2019-11-17T23:59:56.2277433Z command: "make"
2019-11-17T23:59:56.2277508Z stdout:
2019-11-17T23:59:56.2277569Z ------------------------------------------
2019-11-17T23:59:56.2277669Z make[1]: Entering directory '/d/a/1/s/src/test/run-make-fulldeps/rustdoc-themes'
2019-11-17T23:59:56.2277971Z cp D:\a\1\s/src/librustdoc/html/static/themes/light.css /d/a/1/s/build/i686-pc-windows-msvc/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/test.css
2019-11-17T23:59:56.2279696Z PATH="/d/a/1/s/build/i686-pc-windows-msvc/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes:D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x86:/c/Program Files (x86)/Windows Kits/10/bin/10.0.17763.0/x86:/c/Program Files (x86)/Microsoft Visual Studio/2017/Enterprise/VC/Tools/MSVC/14.16.27023/bin/HostX64/x64:/d/a/1/s/build/i686-pc-windows-msvc/stage0-bootstrap-tools/i686-pc-windows-msvc/release/deps:/d/a/1/s/build/i686-pc-windows-msvc/stage0/bin:/mingw32/bin:/d/a/1/s/ninja:/mingw32/bin:/c/Python27amd64:/usr/bin:/c/Program Files (x86)/Inno Setup 5:/d/a/1/s/sccache:/c/agents/2.160.1/externals/git/cmd:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/Program Files/Mercurial:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/Program Files/Boost/1.69.0:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/Java/zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64/bin:/c/npm/prefix:/c/Rust/.cargo/bin:/c/hostedtoolcache/windows/Ruby/2.5.5/x64/bin:/c/Go1.12.7/bin:/c/Program Files/Git/bin:/c/Program Files/Git/usr/bin:/c/Program Files/Git/mingw64/bin:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/Program Files/Microsoft MPI/Bin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/6:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/tools/php:/c/Program Files (x86)/Subversion/bin:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.2/bin:/c/Program Files/CMake/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/OpenSSL/bin:/c/Users/VssAdministrator/.dotnet/tools:/c/Program Files (x86)/Microsoft SQL Server/120" 'D:\a\1\s\build\i686-pc-windows-msvc\stage2\bin\rustdoc.exe' -L D:\a\1\s\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib -o "/d/a/1/s/build/i686-pc-windows-msvc/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/rustdoc-themes" foo.rs --theme /d/a/1/s/build/i686-pc-windows-msvc/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/test.css
2019-11-17T23:59:56.2281429Z D:\a\msys2\mingw32\bin\python2.7 D:\a\1\s/src/etc/htmldocck.py "/d/a/1/s/build/i686-pc-windows-msvc/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/rustdoc-themes" foo.rs
2019-11-17T23:59:56.2281640Z make[1]: Leaving directory '/d/a/1/s/src/test/run-make-fulldeps/rustdoc-themes'
2019-11-17T23:59:56.2281838Z ------------------------------------------
2019-11-17T23:59:56.2281921Z stderr:
2019-11-17T23:59:56.2281981Z ------------------------------------------
2019-11-17T23:59:56.2281981Z ------------------------------------------
2019-11-17T23:59:56.2282127Z /bin/sh: D:amsys2mingw32binpython2.7: command not found
2019-11-17T23:59:56.2282241Z make[1]: *** [Makefile:10: all] Error 127
2019-11-17T23:59:56.2282453Z ------------------------------------------
2019-11-17T23:59:56.2282542Z 
2019-11-17T23:59:56.2282598Z 
2019-11-17T23:59:56.2282630Z 
---
2019-11-17T23:59:56.2283416Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:537:22
2019-11-17T23:59:56.2283520Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-17T23:59:56.2291805Z 
2019-11-17T23:59:56.2291904Z 
2019-11-17T23:59:56.2293820Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\run-make-fulldeps" "--build-base" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\test\\run-make-fulldeps" "--stage-id" "stage2-i686-pc-windows-msvc" "--mode" "run-make" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\msys2\\mingw32\\bin\\python2.7" "--lldb-python" "D:\\a\\msys2\\mingw32\\bin\\python2.7" "--gdb" "D:\\a\\msys2\\mingw32\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x86\\cl.exe" "--cxx" "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Enterprise\\VC\\Tools\\MSVC\\14.16.27023\\bin\\HostX64\\x86\\cl.exe" "--cflags" "-nologo -MT -Brepro" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-ID:\\a\\1\\s\\src\\llvm-project\\llvm\\include -ID:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\include    /EHs-c- /GR- -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D_DEBUG_POINTER_IMPL= -D_CRT_SECURE_NO_DEPRECATE -D_CRT_SECURE_NO_WARNINGS -D_CRT_NONSTDC_NO_DEPRECATE -D_CRT_NONSTDC_NO_WARNINGS -D_SCL_SECURE_NO_DEPRECATE -D_SCL_SECURE_NO_WARNINGS -DUNICODE -D_UNICODE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--llvm-bin-dir" "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\llvm\\build\\bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-17T23:59:56.2295746Z 
2019-11-17T23:59:56.2295802Z 
2019-11-17T23:59:56.2295802Z 
2019-11-17T23:59:56.2988005Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2019-11-17T23:59:56.2988290Z Build completed unsuccessfully in 2:00:35
2019-11-17T23:59:56.3245186Z make: *** [Makefile:80: ci-subset-1] Error 1
2019-11-17T23:59:56.3826173Z   local time: Sun Nov 17 23:59:56 CUT 2019
2019-11-17T23:59:56.8195734Z   network time: Sun, 17 Nov 2019 23:59:56 GMT
2019-11-17T23:59:56.8217721Z == end clock drift check ==
2019-11-17T23:59:56.8882018Z 
2019-11-17T23:59:56.8882018Z 
2019-11-17T23:59:57.2034767Z ##[error]Bash exited with code '2'.
2019-11-17T23:59:57.2656367Z ##[section]Starting: Checkout
2019-11-17T23:59:57.3436821Z ==============================================================================
2019-11-17T23:59:57.3436965Z Task         : Get sources
2019-11-17T23:59:57.3437051Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

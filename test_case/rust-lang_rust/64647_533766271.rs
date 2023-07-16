plain
2019-09-21T01:23:14.9539302Z do so (now or later) by using -b with the checkout command again. Example:
2019-09-21T01:23:14.9539621Z 
2019-09-21T01:23:14.9539735Z   git checkout -b <new-branch-name>
2019-09-21T01:23:14.9539958Z 
2019-09-21T01:23:14.9540096Z HEAD is now at d7db0041e Auto merge of #64647 - tmandry:rollup-gnkrsne, r=tmandry
2019-09-21T01:23:14.9937219Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-09-21T01:23:15.0055864Z ==============================================================================
2019-09-21T01:23:15.0055998Z Task         : Bash
2019-09-21T01:23:15.0056091Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-09-21T03:44:50.3639365Z test [ui] ui\coherence\coherence_local_err_tuple.rs#old ... ok
2019-09-21T03:44:50.7072831Z test [ui] ui\coherence\coherence_local_ref.rs#old ... ok
2019-09-21T03:44:50.7125264Z test [ui] ui\coherence\coherence_local_ref.rs#re ... ok
2019-09-21T03:44:50.8148918Z test [ui] ui\coherence\conflicting-impl-with-err.rs ... ok
2019-09-21T03:44:51.1197879Z test [ui] ui\coherence\impl-foreign[foreign]-for-foreign.rs ... ok
2019-09-21T03:44:51.2407974Z test [ui] ui\coherence\impl-foreign[foreign]-for-local.rs ... ok
2019-09-21T03:44:51.3584075Z test [ui] ui\coherence\impl[t]-foreign[foreign[t],local]-for-foreign.rs ... FAILED
2019-09-21T03:44:51.6684902Z test [ui] ui\coherence\impl[t]-foreign[foreign]-for-fundamental[t].rs ... ok
2019-09-21T03:44:51.8168778Z test [ui] ui\coherence\impl[t]-foreign[foreign]-for-t.rs ... ok
2019-09-21T03:44:51.9073831Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t],local]-for-foreign.rs ... FAILED
2019-09-21T03:44:52.2335868Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-foreign.rs ... ok
2019-09-21T03:44:52.3245354Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs ... ok
2019-09-21T03:44:52.7131458Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-local.rs ... ok
2019-09-21T03:44:52.7237959Z test [ui] ui\coherence\impl[t]-foreign[fundamental[t]]-for-t.rs ... ok
2019-09-21T03:44:53.1780446Z test [ui] ui\coherence\impl[t]-foreign[local, fundamental[t]]-for-foreign.rs ... FAILED
2019-09-21T03:44:53.4536641Z test [ui] ui\coherence\impl[t]-foreign[local]-for-foreign.rs ... ok
2019-09-21T03:44:53.7754085Z test [ui] ui\coherence\impl[t]-foreign[local]-for-fundamental[t].rs ... ok
2019-09-21T03:44:53.8749190Z test [ui] ui\coherence\impl[t]-foreign[local]-for-local.rs ... ok
2019-09-21T03:44:54.2275443Z test [ui] ui\coherence\impl[t]-foreign[local]-for-t.rs ... ok
2019-09-21T03:44:54.2373836Z test [ui] ui\coherence\impl[t]-foreign[t]-for-foreign.rs ... ok
2019-09-21T03:44:54.6633952Z test [ui] ui\coherence\impl[t]-foreign[t]-for-fundamental.rs ... ok
2019-09-21T03:44:54.6817054Z test [ui] ui\coherence\impl[t]-foreign[t]-for-local.rs ... ok
2019-09-21T03:44:55.0852110Z test [ui] ui\coherence\impl[t]-foreign[t]-for-t.rs ... ok
2019-09-21T03:44:55.6550449Z test [ui] ui\collections-const-new.rs ... ok
2019-09-21T03:44:55.6551295Z test [ui] ui\command-exec.rs ... ignored
2019-09-21T03:44:55.7477079Z test [ui] ui\command-line-diagnostics.rs ... ok
2019-09-21T03:44:55.7478144Z test [ui] ui\command-pre-exec.rs ... ignored
---
2019-09-21T04:19:05.3151739Z test [ui] ui\zero-sized\zero-sized-vec-deque-push.rs ... ok
2019-09-21T04:19:05.3154531Z 
2019-09-21T04:19:05.3155108Z failures:
2019-09-21T04:19:05.3362174Z 
2019-09-21T04:19:05.3362409Z ---- [ui] ui\coherence\impl[t]-foreign[foreign[t],local]-for-foreign.rs stdout ----
2019-09-21T04:19:05.3362483Z 
2019-09-21T04:19:05.3362645Z error: auxiliary build of "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" failed to compile: 
2019-09-21T04:19:05.3362760Z status: exit code: 1
2019-09-21T04:19:05.3364894Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw64\bin;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.155.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--crate-type" "dylib" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary"
2019-09-21T04:19:05.3366601Z ------------------------------------------
2019-09-21T04:19:05.3366662Z 
2019-09-21T04:19:05.3366764Z ------------------------------------------
2019-09-21T04:19:05.3366849Z stderr:
2019-09-21T04:19:05.3366849Z stderr:
2019-09-21T04:19:05.3366946Z ------------------------------------------
2019-09-21T04:19:05.3367033Z warning: unused variable: `t`
2019-09-21T04:19:05.3367462Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:8:19
2019-09-21T04:19:05.3367586Z    |
2019-09-21T04:19:05.3367678Z LL |     fn foo(&self, t: T) { }
2019-09-21T04:19:05.3367803Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3367999Z    = note: `#[warn(unused_variables)]` on by default
2019-09-21T04:19:05.3368060Z 
2019-09-21T04:19:05.3368151Z warning: unused variable: `t`
2019-09-21T04:19:05.3368151Z warning: unused variable: `t`
2019-09-21T04:19:05.3368252Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:19
2019-09-21T04:19:05.3368371Z    |
2019-09-21T04:19:05.3368442Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3368567Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3368721Z warning: unused variable: `u`
2019-09-21T04:19:05.3368721Z warning: unused variable: `u`
2019-09-21T04:19:05.3368821Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:25
2019-09-21T04:19:05.3368930Z    |
2019-09-21T04:19:05.3369009Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3369136Z    |                         ^ help: consider prefixing with an underscore: `_u`
2019-09-21T04:19:05.3369202Z 
2019-09-21T04:19:05.3369304Z error: linking with `gcc` failed: exit code: 1
2019-09-21T04:19:05.3369386Z    |
2019-09-21T04:19:05.3370386Z    = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\dllcrt2.o" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary\\coherence_lib.coherence_lib.3a1fbbbh-cgu.0.rcgu.o" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary\\coherence_lib.dll" "-Wl,--version-script=C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcsB50Zc\\list" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary\\coherence_lib.2gio4epld6q3b7cu.rcgu.o" "-nodefaultlibs" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-a63ba71ebebc3c13" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcsB50Zc\\libcompiler_builtins-8825e03b8a6ff9f8.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-shared" "-Wl,--out-implib,D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[foreign[t],local]-for-foreign\\auxiliary\\coherence_lib.dll.lib" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
2019-09-21T04:19:05.3371423Z    = note: D:/a/1/s/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find local]-for-foreign\auxiliary\coherence_lib.dll.lib: No such file or directory
2019-09-21T04:19:05.3371592Z            collect2.exe: error: ld returned 1 exit status
2019-09-21T04:19:05.3371760Z 
2019-09-21T04:19:05.3371848Z error: aborting due to previous error
2019-09-21T04:19:05.3371904Z 
2019-09-21T04:19:05.3371944Z 
2019-09-21T04:19:05.3371944Z 
2019-09-21T04:19:05.3372042Z ------------------------------------------
2019-09-21T04:19:05.3372099Z 
2019-09-21T04:19:05.3372139Z 
2019-09-21T04:19:05.3372251Z ---- [ui] ui\coherence\impl[t]-foreign[fundamental[t],local]-for-foreign.rs stdout ----
2019-09-21T04:19:05.3372381Z 
2019-09-21T04:19:05.3372517Z error: auxiliary build of "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" failed to compile: 
2019-09-21T04:19:05.3372623Z status: exit code: 1
2019-09-21T04:19:05.3374465Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw64\bin;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.155.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--crate-type" "dylib" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary"
2019-09-21T04:19:05.3376061Z ------------------------------------------
2019-09-21T04:19:05.3376120Z 
2019-09-21T04:19:05.3376215Z ------------------------------------------
2019-09-21T04:19:05.3376298Z stderr:
2019-09-21T04:19:05.3376298Z stderr:
2019-09-21T04:19:05.3376393Z ------------------------------------------
2019-09-21T04:19:05.3376478Z warning: unused variable: `t`
2019-09-21T04:19:05.3376602Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:8:19
2019-09-21T04:19:05.3376722Z    |
2019-09-21T04:19:05.3376794Z LL |     fn foo(&self, t: T) { }
2019-09-21T04:19:05.3376916Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3377109Z    = note: `#[warn(unused_variables)]` on by default
2019-09-21T04:19:05.3377167Z 
2019-09-21T04:19:05.3377256Z warning: unused variable: `t`
2019-09-21T04:19:05.3377256Z warning: unused variable: `t`
2019-09-21T04:19:05.3377423Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:19
2019-09-21T04:19:05.3377544Z    |
2019-09-21T04:19:05.3377614Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3377739Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3377894Z warning: unused variable: `u`
2019-09-21T04:19:05.3377894Z warning: unused variable: `u`
2019-09-21T04:19:05.3377996Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:25
2019-09-21T04:19:05.3378104Z    |
2019-09-21T04:19:05.3378184Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3378308Z    |                         ^ help: consider prefixing with an underscore: `_u`
2019-09-21T04:19:05.3378418Z 
2019-09-21T04:19:05.3378520Z error: linking with `gcc` failed: exit code: 1
2019-09-21T04:19:05.3378603Z    |
2019-09-21T04:19:05.3379625Z    = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\dllcrt2.o" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary\\coherence_lib.coherence_lib.3a1fbbbh-cgu.0.rcgu.o" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary\\coherence_lib.dll" "-Wl,--version-script=C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcHgXOfH\\list" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary\\coherence_lib.2gio4epld6q3b7cu.rcgu.o" "-nodefaultlibs" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-a63ba71ebebc3c13" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcHgXOfH\\libcompiler_builtins-8825e03b8a6ff9f8.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-shared" "-Wl,--out-implib,D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[fundamental[t],local]-for-foreign\\auxiliary\\coherence_lib.dll.lib" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
2019-09-21T04:19:05.3380674Z    = note: D:/a/1/s/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find local]-for-foreign\auxiliary\coherence_lib.dll.lib: No such file or directory
2019-09-21T04:19:05.3380851Z            collect2.exe: error: ld returned 1 exit status
2019-09-21T04:19:05.3381009Z 
2019-09-21T04:19:05.3381099Z error: aborting due to previous error
2019-09-21T04:19:05.3381155Z 
2019-09-21T04:19:05.3381195Z 
2019-09-21T04:19:05.3381195Z 
2019-09-21T04:19:05.3381290Z ------------------------------------------
2019-09-21T04:19:05.3381346Z 
2019-09-21T04:19:05.3381386Z 
2019-09-21T04:19:05.3381501Z ---- [ui] ui\coherence\impl[t]-foreign[local, fundamental[t]]-for-foreign.rs stdout ----
2019-09-21T04:19:05.3381578Z 
2019-09-21T04:19:05.3381704Z error: auxiliary build of "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" failed to compile: 
2019-09-21T04:19:05.3381811Z status: exit code: 1
2019-09-21T04:19:05.3383765Z command: PATH="D:\a\1\s\build\x86_64-pc-windows-gnu\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;D:\a\1\s\build\x86_64-pc-windows-gnu\stage0\bin;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\mingw64\bin;D:\a\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.155.1\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\npm\prefix;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.5\x64\bin;C:\Go1.12.7\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.1\bin;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Program" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\coherence\\auxiliary\\coherence_lib.rs" "-Zthreads=1" "--target=x86_64-pc-windows-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--crate-type" "dylib" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary"
2019-09-21T04:19:05.3385627Z ------------------------------------------
2019-09-21T04:19:05.3385687Z 
2019-09-21T04:19:05.3385785Z ------------------------------------------
2019-09-21T04:19:05.3385866Z stderr:
2019-09-21T04:19:05.3385866Z stderr:
2019-09-21T04:19:05.3385961Z ------------------------------------------
2019-09-21T04:19:05.3386046Z warning: unused variable: `t`
2019-09-21T04:19:05.3386180Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:8:19
2019-09-21T04:19:05.3386292Z    |
2019-09-21T04:19:05.3386364Z LL |     fn foo(&self, t: T) { }
2019-09-21T04:19:05.3386484Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3386677Z    = note: `#[warn(unused_variables)]` on by default
2019-09-21T04:19:05.3386736Z 
2019-09-21T04:19:05.3386825Z warning: unused variable: `t`
2019-09-21T04:19:05.3386825Z warning: unused variable: `t`
2019-09-21T04:19:05.3386935Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:19
2019-09-21T04:19:05.3387044Z    |
2019-09-21T04:19:05.3387113Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3387235Z    |                   ^ help: consider prefixing with an underscore: `_t`
2019-09-21T04:19:05.3387386Z warning: unused variable: `u`
2019-09-21T04:19:05.3387386Z warning: unused variable: `u`
2019-09-21T04:19:05.3387485Z   --> D:\a\1\s\src/test\ui\coherence\auxiliary\coherence_lib.rs:12:25
2019-09-21T04:19:05.3387696Z    |
2019-09-21T04:19:05.3387781Z LL |     fn foo(&self, t: T, u: U) { }
2019-09-21T04:19:05.3387904Z    |                         ^ help: consider prefixing with an underscore: `_u`
2019-09-21T04:19:05.3387971Z 
2019-09-21T04:19:05.3388075Z error: linking with `gcc` failed: exit code: 1
2019-09-21T04:19:05.3388157Z    |
2019-09-21T04:19:05.3389172Z    = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\dllcrt2.o" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary\\coherence_lib.coherence_lib.3a1fbbbh-cgu.0.rcgu.o" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary\\coherence_lib.dll" "-Wl,--version-script=C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcmVWP8j\\list" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary\\coherence_lib.2gio4epld6q3b7cu.rcgu.o" "-nodefaultlibs" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-a63ba71ebebc3c13" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\rustcmVWP8j\\libcompiler_builtins-8825e03b8a6ff9f8.rlib" "-Wl,-Bdynamic" "-ladvapi32" "-lws2_32" "-luserenv" "-shared" "-Wl,--out-implib,D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui\\coherence\\impl[t]-foreign[local, fundamental[t]]-for-foreign\\auxiliary\\coherence_lib.dll.lib" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
2019-09-21T04:19:05.3390170Z    = note: D:/a/1/s/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find  fundamental[t]]-for-foreign\auxiliary\coherence_lib.dll.lib: No such file or directory
2019-09-21T04:19:05.3390397Z            collect2.exe: error: ld returned 1 exit status
2019-09-21T04:19:05.3390552Z 
2019-09-21T04:19:05.3390642Z error: aborting due to previous error
2019-09-21T04:19:05.3390698Z 
2019-09-21T04:19:05.3390737Z 
2019-09-21T04:19:05.3390737Z 
2019-09-21T04:19:05.3390886Z ------------------------------------------
2019-09-21T04:19:05.3390943Z 
2019-09-21T04:19:05.3391001Z 
2019-09-21T04:19:05.3391040Z 
2019-09-21T04:19:05.3391115Z failures:
2019-09-21T04:19:05.3391226Z     [ui] ui\coherence\impl[t]-foreign[foreign[t],local]-for-foreign.rs
2019-09-21T04:19:05.3391365Z     [ui] ui\coherence\impl[t]-foreign[fundamental[t],local]-for-foreign.rs
2019-09-21T04:19:05.3391482Z     [ui] ui\coherence\impl[t]-foreign[local, fundamental[t]]-for-foreign.rs
2019-09-21T04:19:05.3391659Z test result: FAILED. 8988 passed; 3 failed; 57 ignored; 0 measured; 0 filtered out
2019-09-21T04:19:05.3391735Z 
2019-09-21T04:19:05.3691168Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:536:22
2019-09-21T04:19:05.3691855Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T04:19:05.3691855Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-21T04:19:05.3692006Z 
2019-09-21T04:19:05.3692102Z 
2019-09-21T04:19:05.3693064Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-gnu" "--mode" "ui" "--target" "x86_64-pc-windows-gnu" "--host" "x86_64-pc-windows-gnu" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\native\\rust-test-helpers" "--docck-python" "C:\\Python27amd64\\python2.7" "--lldb-python" "C:\\Python27amd64\\python2.7" "--gdb" "D:\\a\\1\\s\\mingw64\\bin\\gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T04:19:05.3693823Z 
2019-09-21T04:19:05.3693867Z 
2019-09-21T04:19:05.4748062Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-09-21T04:19:05.4748499Z Build completed unsuccessfully in 2:44:43
2019-09-21T04:19:05.4748499Z Build completed unsuccessfully in 2:44:43
2019-09-21T04:19:05.5315948Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-09-21T04:19:05.5955147Z   local time: Sat Sep 21 04:19:05 CUT 2019
2019-09-21T04:19:06.0845580Z   network time: Sat, 21 Sep 2019 04:19:06 GMT
2019-09-21T04:19:06.0845852Z == end clock drift check ==
2019-09-21T04:19:06.0845852Z == end clock drift check ==
2019-09-21T04:19:06.4866317Z ##[error]Bash exited with code '2'.
2019-09-21T04:19:06.5822601Z ##[section]Starting: Upload CPU usage statistics
2019-09-21T04:19:06.6592245Z ==============================================================================
2019-09-21T04:19:06.6592375Z Task         : Bash
2019-09-21T04:19:06.6592453Z Description  : Run a Bash script on macOS, Linux, or Windows

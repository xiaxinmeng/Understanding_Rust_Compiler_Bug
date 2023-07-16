plain
Updating files:  98% (32544/33208)
Updating files:  99% (32876/33208)
Updating files: 100% (33208/33208)
Updating files: 100% (33208/33208), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'8426199ffec7cb66e9c25620ea4a7088c6227df3'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/edition-guide.active=true
---

---- [incremental] src/test\incremental\lto.rs stdout ----
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc

error in revision `rpass1`: compilation failed!
status: exit code: 1
command: PATH="D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.10.5\x64\Scripts;C:\hostedtoolcache\windows\Python\3.10.5\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.5\x64;C:\cabal\bin;C:\ghcup\bin;C:\tools\ghc-9.2.3\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.0\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.11\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.332-9\x64\bin;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.6\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\rust\\rust\\src/test\\incremental\\lto.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--cfg" "rpass1" "-C" "incremental=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\lto.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\a.exe" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-C" "lto" "-L" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `link.exe` failed: exit code: 1107
   |
   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\symbols.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\a.std-7161d0fe3f4e2d57.std.1454d1f4-cgu.0.rcgu.o.rcgu.o" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\auxiliary" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libstd-7161d0fe3f4e2d57.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libpanic_unwind-55e94581fe5d7254.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\librustc_demangle-785338920c30e5e9.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libstd_detect-59cc3f8530abacec.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libhashbrown-b2a3041d5f3102ec.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libminiz_oxide-47cf2fd1707f9fb6.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libadler-d0af301dfa81113f.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\librustc_std_workspace_alloc-ea86ac3db9a9abf9.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libunwind-45a6b23ecd955fbf.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libcfg_if-d05d8bd3e54f481c.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\liblibc-322b39f1863f3f59.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\liballoc-f9e34fc37f824e2a.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\librustc_std_workspace_core-4f3b79267cc34798.rlib" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustclEx4Rl\\libcore-de7c8d46fbd92bb9.rlib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-6d73ca9f8fd8b08a.rlib" "advapi32.lib" "userenv.lib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "msvcrt.lib" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\incremental\\lto\\a.exe" "/OPT:REF,ICF" "/DEBUG"
   = note: C:\Users\RUNNER~1\AppData\Local\Temp\rustclEx4Rl\libstd-7161d0fe3f4e2d57.rlib : fatal error LNK1107: invalid or corrupt file: cannot read at 0x8

error: aborting due to previous error
------------------------------------------

---

test result: FAILED. 152 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 24.31s

Build completed unsuccessfully in 0:26:35
make: *** [Makefile:70: ci-subset-1] Error 1

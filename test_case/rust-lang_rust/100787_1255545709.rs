
D:\src\rust> ./x.py test src/test/ui --test-args avoid-crash
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.12s
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.43s
Copying stage0 std from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 compiler artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.76s
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Assembling stage1 compiler (x86_64-pc-windows-msvc)
Building stage1 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.43s
Copying stage1 std from stage1 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Building stage0 tool compiletest (x86_64-pc-windows-msvc)
    Finished release [optimized] target(s) in 0.44s
Check compiletest suite=ui mode=ui (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)

running 1 test
F
failures:

---- [ui] src/test\ui\unpretty\avoid-crash.rs stdout ----

error: ui test compiled successfully!
status: exit code: 0
command: PATH="D:\src\rust\build\x86_64-pc-windows-msvc\stage1\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\MSVC\14.31.31103\bin\HostX64\x64;C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Tools\MSVC\14.31.31103\bin\HostX64\x64;D:\src\rust\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\src\rust\build\x86_64-pc-windows-msvc\stage0\bin;C:\Program Files\PowerShell\7;C:\Python310\Scripts\;C:\Python310\;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files\Oculus\Support\oculus-runtime;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\ProgramData\chocolatey\bin;C:\Program Files\dotnet\;C:\Program Files\NVIDIA Corporation\NVIDIA NvDLISR;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Microsoft SQL Server\150\Tools\Binn\;C:\Program Files\OpenJDK\jdk-18.0.2\bin;C:\Program Files\CMake\bin;C:\Program Files\Git\cmd;C:\Program Files\PowerShell\7\;C:\Program Files\Perforce\;C:\Users\khype\.cargo\bin;C:\Users\khype\AppData\Local\Microsoft\WindowsApps;C:\Users\khype\Documents\unison\bin;C:\tools\neovim\Neovim\bin;C:\Users\khype\.dotnet\tools;C:\tools\neovim\nvim-win64\bin;;C:\Users\khype\AppData\Local\Programs\Microsoft VS Code\bin" "D:\\src\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\bin\\rustc.exe" "D:\\src\\rust\\src/test\\ui\\unpretty\\avoid-crash.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "D:\\src\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\unpretty\\avoid-crash" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=D:\\src\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-Clink-arg=-Wl,/threads:1" "-Clinker=D:\\src\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld" "-o/tmp/" "-Zunpretty=ast-tree" "-L" "D:\\src\\rust\\build\\x86_64-pc-windows-msvc\\test\\ui\\unpretty\\avoid-crash\\auxiliary"
stdout: none
stderr: none



failures:
    [ui] src/test\ui\unpretty\avoid-crash.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13561 filtered out; finished in 0.05s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
Build completed unsuccessfully in 0:00:19

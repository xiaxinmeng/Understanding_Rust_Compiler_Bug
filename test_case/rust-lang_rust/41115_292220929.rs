
[01:45:20] failures:
[01:45:20] 
[01:45:20] ---- [run-pass] run-pass\simple_global_asm.rs stdout ----
[01:45:20]      
[01:45:20] error: compilation failed!
[01:45:20] status: exit code: 101
[01:45:20] command: PATH="C:\projects\rust\build\i686-pc-windows-gnu\stage2\bin;C:\projects\rust\build\i686-pc-windows-gnu\stage2-tools\i686-pc-windows-gnu\release\deps;C:\projects\rust\build\i686-pc-windows-gnu\stage2\lib\rustlib\i686-pc-windows-gnu\lib;C:\Program Files 
(x86)\Inno Setup 5;C:\Python27;C:\projects\rust\mingw32\bin;C:\msys64\usr\bin;C:\Perl\site\bin;C:\Perl\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Program Files\7-Zip;C:\Program Files\Microsoft\Web Platform Ins
taller;C:\Tools\GitVersion;C:\Tools\PsTools;C:\Program Files\Git LFS;C:\Program Files (x86)\Subversion\bin;C:\Program Files\Microsoft SQL Server\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\110\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server
\120\Tools\Binn;C:\Program Files\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\Tools\Binn\ManagementStudio;C:\Tools\WebDriver;C:\Program Files (x86)\Microsoft SDKs\TypeScript\1.4;C:\Program Files (x86)\Microsoft Visual Studio 12.0\Commo
n7\IDE\PrivateAssemblies;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI\wbin;C:\Ruby193\bin;C:\Tools\NUnit\bin;C:\Tools\xUnit;C:\Tools\MSpec;C:\Tools\Coverity\bin;C:\Program Files (x86)\CMake\bin;C:\go\bin;C:\Program Files\Java\jdk1.8.0\bin;C:\Python27;C:\Program Files\n
odejs;C:\Program Files (x86)\iojs;C:\Program Files\iojs;C:\Users\appveyor\AppData\Roaming\npm;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\MSBuild\14.0\Bin;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\Extensions\Microsoft\
SQLDB\DAC\120;C:\Tools\NuGet;C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files\Microsoft DNX\Dnvm;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL
 Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Apac
he\Maven\bin;C:\Python27\Scripts;C:\Tools\NUnit3;C:\ProgramData\chocolatey\bin;C:\Program Files\Mercurial;C:\Program Files (x86)\Yarn\bin;C:\Program Files\LLVM\bin;C:\Program Files\dotnet;C:\Program Files\erl8.3\bin;C:\Tools\curl\bin;C:\Program Files\Microsoft Service Fab
ric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files (x86)\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\usr\bin;C:\Program Files\Amazon\AWSCLI;C:\Users\appveyor\AppData\Local\Yarn\.bin;C:
\Users\appveyor\AppData\Roaming\npm;C:\Program Files\AppVeyor\BuildAgent;C:\projects\rust;C:\projects\rust\handle" C:\projects\rust\build\i686-pc-windows-gnu\stage2\bin\rustc.exe C:\projects\rust\src/test\run-pass\simple_global_asm.rs -L C:\projects\rust\build\i686-pc-win
dows-gnu\test\run-pass --target=i686-pc-windows-gnu --error-format json -L C:\projects\rust\build\i686-pc-windows-gnu\test\run-pass\simple_global_asm.stage2-i686-pc-windows-gnu.run-pass.libaux -C prefer-dynamic -o C:\projects\rust\build\i686-pc-windows-gnu\test\run-pass\simple_global_asm.stage2-i686-pc-windows-gnu.exe -Crpath -O -Lnative=C:\projects\rust\build\i686-pc-windows-gnu\native\rust-test-helpers
[01:45:20] stdout:
[01:45:20] ------------------------------------------
[01:45:20] 
[01:45:20] ------------------------------------------
[01:45:20] stderr:
[01:45:20] ------------------------------------------
[01:45:20] {"message":"foreign function is never used: `foo`","code":null,"level":"warning","spans":[{"file_name":"C:\\projects\\rust\\src/test\\run-pass\\simple_global_asm.rs","byte_start":617,"byte_end":626,"line_start":21,"line_end":21,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    fn foo();","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":null}
[01:45:20] {"message":"linking with `gcc` failed: exit code: 1","code":null,"level":"error","spans":[],"children":[{"message":"\"gcc\" \"-Wl,--enable-long-section-names\" \"-fno-use-linker-plugin\" \"-Wl,--nxcompat\" \"-nostdlib\" \"-Wl,--large-address-aware\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\\\\crt2.o\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\\\\rsbegin.o\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\test\\\\run-pass\\\\simple_global_asm.0.o\" \"-o\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\test\\\\run-pass\\\\simple_global_asm.stage2-i686-pc-windows-gnu.exe\" \"-Wl,--gc-sections\" \"-nodefaultlibs\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\test\\\\run-pass\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\test\\\\run-pass\\\\simple_global_asm.stage2-i686-pc-windows-gnu.run-pass.libaux\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\native\\\\rust-test-helpers\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\" \"-L\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\" \"-l\" \"std-4d6881ec6132b951\" \"-Wl,-Bstatic\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\\\\libcompiler_builtins-7ac5a34e9b48514f.rlib\" \"-Wl,-Bdynamic\" \"-l\" \"advapi32\" \"-l\" \"ws2_32\" \"-l\" \"userenv\" \"-l\" \"shell32\" \"-lmingwex\" \"-lmingw32\" \"-lgcc\" \"-lmsvcrt\" \"-luser32\" \"-lkernel32\" \"C:\\\\projects\\\\rust\\\\build\\\\i686-pc-windows-gnu\\\\stage2\\\\lib\\\\rustlib\\\\i686-pc-windows-gnu\\\\lib\\\\rsend.o\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"C:\\projects\\rust\\build\\i686-pc-windows-gnu\\test\\run-pass\\simple_global_asm.0.o:(.text+0x1): undefined reference to `baz'\r\ncollect2.exe: error: ld returned 1 exit status\r\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":null}
[01:45:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":null}
[01:45:20] 
[01:45:20] ------------------------------------------
[01:45:20] 
[01:45:20] thread '[run-pass] run-pass\simple_global_asm.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2621
[01:45:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:45:20] 
[01:45:20] 
[01:45:20] failures:
[01:45:20]     [run-pass] run-pass\simple_global_asm.rs

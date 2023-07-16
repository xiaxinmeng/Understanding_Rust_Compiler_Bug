
normalized stdout:
args


expected stdout:
args


diff of stdout:

 args
 

The actual stdout differed from the expected stdout.
Actual stdout saved to C:\Users\VSSADM~1\AppData\Local\Temp\compiletest9YVbIE\args.stdout
To update references, run this command from build directory:
tests/run-pass/update-references.sh 'C:\Users\VSSADM~1\AppData\Local\Temp\compiletest9YVbIE' 'args.rs'

error: 1 errors occurred comparing output.
status: exit code: 0
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\bin;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\curl-sys-602f6dde8610a19b\out\build;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libnghttp2-sys-7776a7dcba25a9ae\out\i\lib;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\build\libz-sys-b75492441773d9c1\out\build;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release;D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\a\1\s\ninja;C:\Python27amd64;D:\a\1\s\citools\msys64\mingw64\bin;D:\a\1\s\citools\msys64\usr\bin;D:\a\1\s\sccache;C:\agents\2.153.2\externals\git\cmd;C:\hostedtoolcache\windows\Python\3.6.8\x64;C:\hostedtoolcache\windows\Python\3.6.8\x64\Scripts;C:\Program Files\Mercurial;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\Program Files\Boost\1.69.0;C:\Users\VssAdministrator\.dotnet\tools;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.38.0.13-8.0.212-win_x64\bin;C:\npm\prefix;C:\Program Files\Rust\.cargo\bin;C:\hostedtoolcache\windows\Ruby\2.5.3\x64\bin;C:\Go1.12.4\bin;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;C:\Program Files\Git\mingw64\bin;C:\hostedtoolcache\windows\Python\3.6.8\Scripts;C:\hostedtoolcache\windows\Python\3.6.8;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\6;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\Subversion\bin;C:\Program Files\nodejs;C:\Program Files\CMake\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\OpenSSL\bin;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2-tools-bin\\miri.exe" "tests/run-pass\\args.rs" "-L" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\compiletest9YVbIE" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\compiletest9YVbIE\\args.stage-id.exe" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2" "-L" "C:\\Users\\VSSADM~1\\AppData\\Local\\Temp\\compiletest9YVbIE\\args.stage-id.aux" "-A" "unused"
stdout:
------------------------------------------
args

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

test [ui] run-pass\args.rs ... FAILED

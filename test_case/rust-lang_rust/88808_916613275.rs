plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v2' (SHA:5a4ac9002d0be2fb38bd78e4b4dbde5606d7042f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:558ff1b23ec813d7c88deb6075ac0ca398e66d09)
Download action repository 'ilammy/msvc-dev-cmd@v1' (SHA:7defe9254715b088f12dddd9942945a3d75cdad5)
git config --global core.autocrlf false
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-aarch64-msvc
---
Updating files:  98% (29723/30246)
Updating files:  99% (29944/30246)
Updating files: 100% (30246/30246)
Updating files: 100% (30246/30246), done.
Note: switching to 'refs/remotes/pull/88808/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at a3e69ac5 Merge 3da9835e94ceca13442d3b3780e9b5cd012b1d99 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'a3e69ac5bf253c3c2c80574ab1baf97aba641d26'
'a3e69ac5bf253c3c2c80574ab1baf97aba641d26'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
Skipping. This is only run when merging to the beta or stable branches.
##[group]Run ilammy/msvc-dev-cmd@v1
  arch: amd64
  sdk: 10.0.19041.0
env:
  CI_JOB_NAME: dist-aarch64-msvc
---
  SCRIPT: python x.py build library/std --stage=2
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
Found with vswhere: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Auxiliary\Build\vcvarsall.bat
##[group]Environment variables
Setting CommandPromptType
Setting DevEnvDir
Setting ExtensionSdkDir
Setting Framework40Version
Setting FrameworkDir
Setting FrameworkDIR64
Setting FrameworkVersion
Setting FrameworkVersion64
Setting FSHARPINSTALLDIR
Setting HTMLHelpDir
Setting LIB
Setting LIBPATH
Setting LIBPATH
Setting NETFXSDKDir
Setting Platform
Setting Platform
Setting UCRTVersion
Setting UniversalCRTSdkDir
Setting VCIDEInstallDir
Setting VCINSTALLDIR
Setting VCToolsInstallDir
Setting VCToolsRedistDir
Setting VCToolsVersion
Setting VisualStudioVersion
Setting VS160COMNTOOLS
Setting VSCMD_ARG_app_plat
Setting VSCMD_ARG_HOST_ARCH
Setting VSCMD_ARG_TGT_ARCH
Setting VSCMD_ARG_winsdk
Setting VSCMD_VER
Setting VSINSTALLDIR
Setting VSSDK150INSTALL
Setting VSSDKINSTALL
Setting WindowsLibPath
Setting WindowsSdkBinPath
Setting WindowsSdkDir
Setting WindowsSDKLibVersion
Setting WindowsSdkVerBinPath
Setting WindowsSDKVersion
Setting WindowsSDK_ExecutablePath_x64
Setting WindowsSDK_ExecutablePath_x86
Setting __devinit_path
Setting __DOTNET_ADD_64BIT
Setting __DOTNET_PREFERRED_BITNESS
Setting __VSCMD_PREINIT_PATH
Setting __VSCMD_script_err_count
Configured Developer Command Prompt
##[group]Run src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py build library/std --stage=2
  DEPLOY: 1
  WIX: /d/a/rust/rust/wix
  CommandPromptType: Native
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  Framework40Version: v4.0
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  FrameworkVersion: v4.0.30319
  FrameworkVersion64: v4.0.30319
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x64;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\lib\um\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\x64
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  Path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\\Extensions\Microsoft\IntelliCode\CLI;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\VCPackages;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\TestWindow;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\TeamFoundation\Team Explorer;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\MSBuild\Current\bin\Roslyn;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Team Tools\Performance Tools\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Team Tools\Performance Tools;C:\Program Files (x86)\Microsoft Visual Studio\Shared\Common\VSPerfCollectionTools\vs2019\\x64;C:\Program Files (x86)\Microsoft Visual Studio\Shared\Common\VSPerfCollectionTools\vs2019\;C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\;C:\Program Files (x86)\HTML Help Workshop;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit;C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\\MSBuild\Current\Bin;C:\Windows\Microsoft.NET\Framework64\v4.0.30319;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\;/d/a/rust/rust/ninja;D:\a\rust\rust/msys2/mingw64/bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;c:/msys64/usr/bin;/d/a/rust/rust/sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS\;C:\tools\zstd;C:\Program Files\Mercurial\;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7\;C:\Program Files\Microsoft\Web Platform Installer\;C:\Program Files\dotnet\;C:\Program Files\Microsoft SQL Server\130\Tools\Binn\;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn\;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn\;C:\Program Files\nodejs\;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;c:\tools\php;C:\SeleniumWebDrivers\ChromeDriver\;C:\SeleniumWebDrivers\EdgeDriver\;C:\Program Files\Amazon\AWSCLIV2\;C:\Program Files\Amazon\SessionManagerPlugin\bin\;C:\Program Files\Amazon\AWSSAMCLI\bin\;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server\;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps;C:\Program Files (x86)\Microsoft Visual Studio\Installer;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\Llvm\x64\bin;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\Linux\bin\ConnectionManagerExe
  Platform: x64
  UCRTVersion: 10.0.19041.0
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  VCToolsVersion: 14.29.30133
  VisualStudioVersion: 16.0
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  VSCMD_ARG_app_plat: Desktop
  VSCMD_ARG_HOST_ARCH: x64
  VSCMD_ARG_TGT_ARCH: x64
  VSCMD_ARG_winsdk: 10.0.19041.0
  VSCMD_VER: 16.11.2
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  WindowsSDKLibVersion: 10.0.19041.0\
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  WindowsSDKVersion: 10.0.19041.0\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  __DOTNET_ADD_64BIT: 1
  __DOTNET_PREFERRED_BITNESS: 64
  __VSCMD_PREINIT_PATH: /d/a/rust/rust/ninja;D:\a\rust\rust/msys2/mingw64/bin;C:\hostedtoolcache\windows\Python\3.9.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.9.7\x64;c:/msys64/usr/bin;/d/a/rust/rust/sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS\;C:\tools\zstd;C:\Program Files\Mercurial\;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7\;C:\Program Files\Microsoft\Web Platform Installer\;C:\Program Files\dotnet\;C:\Program Files\Microsoft SQL Server\130\Tools\Binn\;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn\;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn\;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn\;C:\Program Files\nodejs\;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;c:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver\;C:\SeleniumWebDrivers\EdgeDriver\;C:\Program Files\Amazon\AWSCLIV2\;C:\Program Files\Amazon\SessionManagerPlugin\bin\;C:\Program Files\Amazon\AWSSAMCLI\bin\;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server\;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps;C:\Program Files (x86)\Microsoft Visual Studio\Installer
  __VSCMD_script_err_count: 0
  AWS_SECRET_ACCESS_KEY: 
  TOOLSTATE_REPO_ACCESS_TOKEN: 
##[endgroup]
info: removing rustup home
---
[1313/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolUnknown.cpp.obj
[1314/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolUsingNamespace.cpp.obj
[1315/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolTypeVTable.cpp.obj
[1316/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\UDTLayout.cpp.obj
[1317/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumInjectedSources.cpp.obj
[1318/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIADataStream.cpp.obj
[1319/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumLineNumbers.cpp.obj
[1320/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymDumper.cpp.obj
[1321/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumFrameData.cpp.obj
[1322/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumDebugStreams.cpp.obj
[1323/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIALineNumber.cpp.obj
[1324/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSectionContribs.cpp.obj
[1325/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAFrameData.cpp.obj
[1326/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAError.cpp.obj
[1327/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIARawSymbol.cpp.obj
[1328/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASession.cpp.obj
[1329/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASourceFile.cpp.obj
[1330/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSymbols.cpp.obj
[1331/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIATable.cpp.obj
[1332/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSourceFiles.cpp.obj
[1333/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleDescriptor.cpp.obj
[1334/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumTables.cpp.obj
[1335/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAInjectedSource.cpp.obj
[1336/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASectionContrib.cpp.obj
[1338/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiStreamBuilder.cpp.obj
[1339/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\HashTable.cpp.obj
[1340/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\EnumTables.cpp.obj
[1341/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleList.cpp.obj
---
warning: 26 warnings generated.
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingRuntime.cpp:11:
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfiling.h:12:
warning: In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h:65:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\windows.h:171:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\windef.h:24:
warning: In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\minwindef.h:182:
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6370,20): error: use of undeclared identifier '__umulh'
warning:     *HighProduct = UnsignedMultiplyHigh(Multiplier, Multiplicand);
warning:                    ^
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6236,30): note: expanded from macro 'UnsignedMultiplyHigh'
warning: #define UnsignedMultiplyHigh __umulh
warning: 1 error generated.

error: failed to run custom build command for `profiler_builtins v0.0.0 (D:\a\rust\rust\library\profiler_builtins)`


Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\build\profiler_builtins-5d2a280ac529e14b\build-script-build` (exit code: 1)
  TARGET = Some("aarch64-pc-windows-msvc")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-pc-windows-msvc")
  CC_aarch64-pc-windows-msvc = None
---
  CFLAGS_aarch64_pc_windows_msvc = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fp,neon")
  DEBUG = Some("true")
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\GCDAProfiling.c"
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(310,8): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  fd = open(filename, O_RDWR | O_BINARY);
  cargo:warning=       ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(314,10): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    fd = open(filename, O_RDWR | O_CREAT | O_EXCL | O_BINARY, 0644);
  cargo:warning=         ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(320,12): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=      fd = open(filename, O_RDWR | O_CREAT | O_EXCL | O_BINARY, 0644);
  cargo:warning=           ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(326,14): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        fd = open(filename, O_RDWR | O_BINARY);
  cargo:warning=             ^
  cargo:warning=<command line>(4,14): note: expanded from here
  cargo:warning=#define open _open
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt_io.h(517,24): note: '_open' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_ _CRT_NONSTDC_DEPRECATE(_open) _CRT_INSECURE_DEPRECATE(_sopen_s)
  cargo:warning=                       ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(331,19): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=                  strerror(errnum));
  cargo:warning=                  ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(343,17): warning: '_fdopen' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fdopen. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  output_file = fdopen(fd, mode);
  cargo:warning=                ^
  cargo:warning=<command line>(5,16): note: expanded from here
  cargo:warning=#define fdopen _fdopen
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2431,28): note: '_fdopen' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fdopen)    _ACRTIMP FILE* __cdecl fdopen(_In_ int _FileHandle, _In_z_ char const* _Format);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=6 warnings generated.
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfiling.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfiling.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingBuffer.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingBuffer.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingFile.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingFile.c"
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(109,15): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=              fileno(File));
  cargo:warning=              ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(181,17): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  BufferSzStr = getenv("LLVM_VP_BUFFER_SIZE");
  cargo:warning=                ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(196,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(204,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(225,25): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=                        fileno(ProfileFile), 0);
  cargo:warning=                        ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(228,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(272,9): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  (void)COMPILER_RT_FTRUNCATE(ProfileFile,
  cargo:warning=        ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h(23,44): note: expanded from macro 'COMPILER_RT_FTRUNCATE'
  cargo:warning=#define COMPILER_RT_FTRUNCATE(f,l) _chsize(_fileno(f),l)
  cargo:warning=                                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(286,5): warning: 'strncpy' is deprecated: This function or variable may be unsafe. Consider using strncpy_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    strncpy(Copy, Filename, Length + 1);
  cargo:warning=    ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(334,1): note: 'strncpy' has been explicitly marked deprecated here
  cargo:warning=__DEFINE_CPP_OVERLOAD_STANDARD_NFUNC_0_2_EX(
  cargo:warning=^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(1935,17): note: expanded from macro '__DEFINE_CPP_OVERLOAD_STANDARD_NFUNC_0_2_EX'
  cargo:warning=                _CRT_INSECURE_DEPRECATE(_SecureFuncName) _DeclSpec _ReturnType __cdecl _FuncName(_SalAttributeDst _DstType *_Dst, _TType1 _TArg1, _TType2 _TArg2);
  cargo:warning=                ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(314,29): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  if (rc || (!*MergeDone && COMPILER_RT_FTRUNCATE(ProfileFile, 0L)) ||
  cargo:warning=                            ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h(23,44): note: expanded from macro 'COMPILER_RT_FTRUNCATE'
  cargo:warning=#define COMPILER_RT_FTRUNCATE(f,l) _chsize(_fileno(f),l)
  cargo:warning=                                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(317,14): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=             strerror(errno));
  cargo:warning=             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(331,10): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  return fopen(OutputName, "ab");
  cargo:warning=         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(372,16): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  OutputFile = fopen(OutputName, "w");
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(405,21): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  int initialized = getenv(LPROF_INIT_ONCE_ENV) != NULL;
  cargo:warning=                    ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(423,10): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  File = fopen(Filename, "w");
  cargo:warning=         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(621,47): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write profile: %s\n", strerror(errno));
  cargo:warning=                                              ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(631,59): warning: '_fileno' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fileno. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=      NULL, FileSize, PROT_READ | PROT_WRITE, MAP_SHARED, fileno(OutputFile), 0);
  cargo:warning=                                                          ^
  cargo:warning=<command line>(7,16): note: expanded from here
  cargo:warning=#define fileno _fileno
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(2433,28): note: '_fileno' has been explicitly marked deprecated here
  cargo:warning=        _Check_return_     _CRT_NONSTDC_DEPRECATE(_fileno)    _ACRTIMP int   __cdecl fileno(_In_ FILE* _Stream);
  cargo:warning=                           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(633,46): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Unable to mmap profile: %s\n", strerror(errno));
  cargo:warning=                                             ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(673,12): warning: 'fopen' is deprecated: This function or variable may be unsafe. Consider using fopen_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    File = fopen(Filename, "w+b");
  cargo:warning=           ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdio.h(212,20): note: 'fopen' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(fopen_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(793,61): warning: '_getpid' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _getpid. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=          if (snprintf(PidChars, MAX_PID_SIZE, "%ld", (long)getpid()) <= 0) {
  cargo:warning=                                                            ^
  cargo:warning=<command line>(6,16): note: expanded from here
  cargo:warning=#define getpid _getpid
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\process.h(363,9): note: '_getpid' has been explicitly marked deprecated here
  cargo:warning=        _CRT_NONSTDC_DEPRECATE(_getpid)
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(809,35): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        lprofCurFilename.TmpDir = getenv("TMPDIR");
  cargo:warning=                                  ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(875,9): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    if (getenv("LLVM_PROFILE_VERBOSE"))
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(879,9): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    if (getenv("LLVM_PROFILE_VERBOSE"))
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(957,29): warning: '_getpid' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _getpid. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=        int ProfilePoolId = getpid() % lprofCurFilename.MergePoolSize;
  cargo:warning=                            ^
  cargo:warning=<command line>(6,16): note: expanded from here
  cargo:warning=#define getpid _getpid
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\process.h(363,9): note: '_getpid' has been explicitly marked deprecated here
  cargo:warning=        _CRT_NONSTDC_DEPRECATE(_getpid)
  cargo:warning=        ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\corecrt.h(428,50): note: expanded from macro '_CRT_NONSTDC_DEPRECATE'
  cargo:warning=        #define _CRT_NONSTDC_DEPRECATE(_NewName) _CRT_DEPRECATE_TEXT(             \
  cargo:warning=                                                 ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(976,26): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=  const char *Filename = getenv("LLVM_PROFILE_FILE");
  cargo:warning=                         ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\stdlib.h(1183,20): note: 'getenv' has been explicitly marked deprecated here
  cargo:warning=    _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(1122,61): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write file \"%s\": %s\n", Filename, strerror(errno));
  cargo:warning=                                                            ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(1184,61): warning: 'strerror' is deprecated: This function or variable may be unsafe. Consider using strerror_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
  cargo:warning=    PROF_ERR("Failed to write file \"%s\": %s\n", Filename, strerror(errno));
  cargo:warning=                                                            ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\ucrt\string.h(177,16): note: 'strerror' has been explicitly marked deprecated here
  cargo:warning=_Check_return_ _CRT_INSECURE_DEPRECATE(strerror_s)
  cargo:warning=               ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(320,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
  cargo:warning=        #define _CRT_INSECURE_DEPRECATE(_Replacement) _CRT_DEPRECATE_TEXT(    \
  cargo:warning=                                                      ^
  cargo:warning=C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include\vcruntime.h(310,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
  cargo:warning=#define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
  cargo:warning=                                              ^
  cargo:warning=26 warnings generated.
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMerge.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMerge.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMergeFile.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingMergeFile.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingNameVar.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingNameVar.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformDarwin.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformDarwin.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformFuchsia.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformFuchsia.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformLinux.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformLinux.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformOther.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformOther.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformWindows.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingPlatformWindows.c"
  exit code: 0
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.cpp"
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile\InstrProfilingRuntime.cpp:11:
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfiling.h:12:
  cargo:warning=In file included from ../../src/llvm-project/compiler-rt\lib\profile/InstrProfilingPort.h:65:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\windows.h:171:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\windef.h:24:
  cargo:warning=In file included from C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\shared\minwindef.h:182:
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6370,20): error: use of undeclared identifier '__umulh'
  cargo:warning=    *HighProduct = UnsignedMultiplyHigh(Multiplier, Multiplicand);
  cargo:warning=                   ^
  cargo:warning=C:\Program Files (x86)\Windows Kits\10\include\10.0.20348.0\um\winnt.h(6236,30): note: expanded from macro 'UnsignedMultiplyHigh'
  cargo:warning=#define UnsignedMultiplyHigh __umulh
  cargo:warning=                             ^
  cargo:warning=1 error generated.

  --- stderr



  error occurred: Command "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "../../src/llvm-project/compiler-rt\\include" "/Zl" "-Dstrdup=_strdup" "-Dopen=_open" "-Dfdopen=_fdopen" "-Dgetpid=_getpid" "-Dfileno=_fileno" "-DCOMPILER_RT_HAS_ATOMICS=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\profiler_builtins-56a8736260b44f1c\\out\\../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.o" "-c" "../../src/llvm-project/compiler-rt\\lib\\profile\\InstrProfilingRuntime.cpp" with args "clang-cl.exe" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:24:44

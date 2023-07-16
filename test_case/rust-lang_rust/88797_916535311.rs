plain
Updating files:  98% (29643/30247)
Updating files:  99% (29945/30247)
Updating files: 100% (30247/30247)
Updating files: 100% (30247/30247), done.
Note: switching to 'refs/remotes/pull/88797/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 2871f48c Merge 403efb7b288e97ac7b743e88b2a04a98f8503e51 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'2871f48cfe8613726cd8f7cdfd01074d6de73de8'
'2871f48cfe8613726cd8f7cdfd01074d6de73de8'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python x.py dist",
 "DIST_REQUIRE_ALL_TOOLS": 0,
 "SELECT_WINDOWS_SDK": 1
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
adding extra environment variable SCRIPT
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: C:\Program Files\Git\usr\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
##[group]Run src/ci/scripts/install-msys2.sh
src/ci/scripts/install-msys2.sh
shell: C:\Program Files\Git\usr\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
##[group]Run src/ci/scripts/install-mingw.sh
src/ci/scripts/install-mingw.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
warning: mingw-w64-x86_64-binutils-2.37-4 is up to date -- skipping
warning: mingw-w64-x86_64-crt-git-9.0.0.6294.f5ac9206e-1 is up to date -- skipping
warning: mingw-w64-x86_64-gcc-10.3.0-5 is up to date -- skipping
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
src/ci/scripts/disable-git-crlf-conversion.sh
shell: c:/msys64/usr/bin\bash.EXE --noprofile --norc -e -o pipefail {0}
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
file:C:/Program Files/Git/etc/gitconfig diff.astextplain.textconv=astextplain
file:C:/Program Files/Git/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
file:C:/Program Files/Git/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
##[endgroup]
Skipping. This is only run when merging to the beta or stable branches.
##[group]Run src/ci/scripts/run-build-from-ci.sh
src/ci/scripts/run-build-from-ci.sh
---
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: python x.py dist
  SELECT_WINDOWS_SDK: 1
  DEPLOY: 1
  VSSDKINSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  WindowsSdkVerBinPath: C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\
  NETFXSDKDir: C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\
  VSSDK150INSTALL: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VSSDK
  VCToolsInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\
  WindowsSdkDir: C:\Program Files (x86)\Windows Kits\10\
  Platform: arm64
  VS160COMNTOOLS: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\
  WindowsSDK_ExecutablePath_x64: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\x64\
  __DOTNET_PREFERRED_BITNESS: 64
  PreferredToolArchitecture: x64
  VSCMD_VER: 16.11.2
  WindowsSDKVersion: 10.0.19041.0\
  VSCMD_ARG_winsdk: 10.0.19041.0
  LIB: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\ucrt\arm64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.19041.0\um\arm64
  WindowsLibPath: C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0
  VisualStudioVersion: 16.0
  CommandPromptType: Cross
  UniversalCRTSdkDir: C:\Program Files (x86)\Windows Kits\10\
  HTMLHelpDir: C:\Program Files (x86)\HTML Help Workshop
  VCIDEInstallDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\VC\
  INCLUDE: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
  __DOTNET_ADD_64BIT: 1
  __devinit_path: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\Tools\devinit\devinit.exe
  VSCMD_ARG_app_plat: Desktop
  VSINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\
  VCINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\
  WindowsSDKLibVersion: 10.0.19041.0\
  __VSCMD_PREINIT_PATH: C:\Program Files\PowerShell\7;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Users\runneradmin\bin;D:\a\rust\rust\sccache;C:\Users\runneradmin\.dotnet\tools;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.7.3\x64;C:\tools\ghc-9.0.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\R\R-4.1.1\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\usr\bin;C:\Program Files (x86)\pipx_bin;C:\hostedtoolcache\windows\go\1.15.15\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\hostedtoolcache\windows\Java_Adopt_jdk\8.0.302-8\x64\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files\nodejs;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\OpenJDK\jdk-16.0.2\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.8.2\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GVFS;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
  FSHARPINSTALLDIR: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\FSharp\Tools
  FrameworkDir: C:\Windows\Microsoft.NET\Framework64\
  WindowsSDK_ExecutablePath_x86: C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.8 Tools\
  FrameworkVersion: v4.0.30319
  UCRTVersion: 10.0.19041.0
  Framework40Version: v4.0
  VCToolsRedistDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Redist\MSVC\14.29.30133\
  __VSCMD_script_err_count: 0
  ExtensionSdkDir: C:\Program Files (x86)\Microsoft SDKs\Windows Kits\10\ExtensionSDKs
  WindowsSdkBinPath: C:\Program Files (x86)\Windows Kits\10\bin\
  VSCMD_ARG_TGT_ARCH: arm64
  FrameworkVersion64: v4.0.30319
  DevEnvDir: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\
  LIBPATH: C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\ATLMFC\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\ARM64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.19041.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.19041.0;C:\Windows\Microsoft.NET\Framework64\v4.0.30319
  VSCMD_ARG_HOST_ARCH: x64
  FrameworkDIR64: C:\Windows\Microsoft.NET\Framework64
  VCToolsVersion: 14.29.30133
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
  TOOLSTATE_REPO_ACCESS_TOKEN: 
##[endgroup]
---
[1311/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolTypeCustom.cpp.obj
[1312/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolTypeFunctionArg.cpp.obj
[1313/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolTypeDimension.cpp.obj
[1314/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymDumper.cpp.obj
[1315/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIADataStream.cpp.obj
[1316/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSectionContribs.cpp.obj
[1317/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumFrameData.cpp.obj
[1318/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumDebugStreams.cpp.obj
[1319/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumLineNumbers.cpp.obj
[1320/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumTables.cpp.obj
[1321/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolUsingNamespace.cpp.obj
[1322/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSymbols.cpp.obj
[1323/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\PDBSymbolUnknown.cpp.obj
[1324/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumSourceFiles.cpp.obj
[1325/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\UDTLayout.cpp.obj
[1326/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIALineNumber.cpp.obj
[1327/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleDescriptor.cpp.obj
[1328/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAEnumInjectedSources.cpp.obj
[1329/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAError.cpp.obj
[1330/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIATable.cpp.obj
[1331/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAFrameData.cpp.obj
[1332/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASectionContrib.cpp.obj
[1333/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASession.cpp.obj
[1335/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\EnumTables.cpp.obj
[1335/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\EnumTables.cpp.obj
[1336/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIAInjectedSource.cpp.obj
[1338/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleDescriptorBuilder.cpp.obj
[1338/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleDescriptorBuilder.cpp.obj
[1339/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIASourceFile.cpp.obj
[1341/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\InfoStream.cpp.obj
[1342/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\InfoStreamBuilder.cpp.obj
[1342/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\InfoStreamBuilder.cpp.obj
[1343/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\DIA\DIARawSymbol.cpp.obj
[1345/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\ModuleDebugStream.cpp.obj
[1346/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiStreamBuilder.cpp.obj
[1347/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\GlobalsStream.cpp.obj
[1348/2905] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\NativeEnumSymbols.cpp.obj
---
[2509/2905] Building CXX object tools\llvm-cfi-verify\lib\CMakeFiles\LLVMCFIVerify.dir\GraphBuilder.cpp.obj
[2510/2905] Linking CXX static library lib\LLVMCFIVerify.lib
[2511/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\gcov.cpp.obj
[2512/2905] Linking CXX executable bin\llvm-cfi-verify.exe
FAILED: bin/llvm-cfi-verify.exe 
cmd.exe /C "cd . && "C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe" -E vs_link_exe --intdir=tools\llvm-cfi-verify\CMakeFiles\llvm-cfi-verify.dir --rc=C:\PROGRA~2\WI3CF2~1\10\bin\100203~1.0\x64\rc.exe --mt=C:\PROGRA~2\WI3CF2~1\10\bin\100203~1.0\x64\mt.exe --manifests  -- C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\Llvm\x64\bin\lld-link.exe /nologo @CMakeFiles\llvm-cfi-verify.rsp  /out:bin\llvm-cfi-verify.exe /implib:lib\llvm-cfi-verify.lib /pdb:bin\llvm-cfi-verify.pdb /version:0.0 /machine:x64 /STACK:10000000 /INCREMENTAL:NO /subsystem:console  && cd ."
LINK: command "C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\Llvm\x64\bin\lld-link.exe /nologo @CMakeFiles\llvm-cfi-verify.rsp /out:bin\llvm-cfi-verify.exe /implib:lib\llvm-cfi-verify.lib /pdb:bin\llvm-cfi-verify.pdb /version:0.0 /machine:x64 /STACK:10000000 /INCREMENTAL:NO /subsystem:console /MANIFEST /MANIFESTFILE:bin\llvm-cfi-verify.exe.manifest" failed (exit code 1) with the following output:
lld-link: error: diaguids.lib(dia2_i.obj): machine type arm64 conflicts with x64

lld-link: error: diaguids.lib(guidstr.obj): machine type arm64 conflicts with x64

lld-link: error: atls.lib(atlbase.obj): machine type arm64 conflicts with x64
[2513/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\SourceCoverageViewText.cpp.obj
[2514/2905] Building CXX object tools\lli\CMakeFiles\lli.dir\lli.cpp.obj
[2515/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\CoverageExporterJson.cpp.obj
[2516/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\CoverageReport.cpp.obj
[2516/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\CoverageReport.cpp.obj
[2517/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\SourceCoverageViewHTML.cpp.obj
[2518/2905] Building CXX object tools\llvm-cov\CMakeFiles\llvm-cov.dir\CodeCoverage.cpp.obj
[2519/2905] Building CXX object lib\Passes\CMakeFiles\LLVMPasses.dir\PassBuilder.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
 finished in 1304.661 seconds
Build completed unsuccessfully in 0:24:37

plain
2019-11-22T03:17:34.0240490Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-22T03:17:34.0240854Z 
2019-11-22T03:17:34.0240938Z   git checkout -b <new-branch-name>
2019-11-22T03:17:34.0241004Z 
2019-11-22T03:17:34.0241088Z HEAD is now at 2363a5241 Auto merge of #66503 - thomasetter:panic-error-msg, r=joshtriplett
2019-11-22T03:17:34.0676400Z ##[section]Starting: Decide whether to run this job
2019-11-22T03:17:34.0784223Z ==============================================================================
2019-11-22T03:17:34.0784477Z Task         : Bash
2019-11-22T03:17:34.0784568Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-22T03:17:35.3167405Z BUILD_SOURCEBRANCHNAME=auto
2019-11-22T03:17:35.3167502Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-22T03:17:35.3167600Z BUILD_SOURCEVERSION=2363a5241004ed3ae5f4ef0cf3d16713f0c38392
2019-11-22T03:17:35.3167706Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-22T03:17:35.3167835Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66503 - thomasetter:panic-error-msg, r=joshtriplett
2019-11-22T03:17:35.3168096Z CI_JOB_NAME=x86_64-msvc-aux
2019-11-22T03:17:35.3168216Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-22T03:17:35.3168323Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-22T03:17:35.3168648Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-22T03:17:35.3170915Z GeckoWebDriver=C:\SeleniumWebDrivers\GeckoDriver
2019-11-22T03:17:35.3171000Z HOME=/c/Users/VssAdministrator
2019-11-22T03:17:35.3171145Z HOMEDRIVE=C:
2019-11-22T03:17:35.3171219Z HOMEPATH=\Users\VssAdministrator
2019-11-22T03:17:35.3171404Z I am not sure whether printing the `Any::type_id()` is useful, is there something better that we could print for non-string panic values?
2019-11-22T03:17:35.3171698Z INPUT_ARGUMENTS=
2019-11-22T03:17:35.3171942Z ImageVersion=20191028.1
2019-11-22T03:17:35.3172095Z JAVA_HOME=C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64
2019-11-22T03:17:35.3172513Z JAVA_HOME_11_X64=C:\Program Files\Java\zulu-11-azure-jdk_11.33.15-11.0.4-win_x64
---
2019-11-22T03:17:35.3174227Z MSDEPLOY_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-22T03:17:35.3174380Z MSMPI_BIN=C:\Program Files\Microsoft MPI\Bin\
2019-11-22T03:17:35.3174463Z MSYSTEM=MINGW64
2019-11-22T03:17:35.3174749Z MonAgentClientLocation=C:\Packages\Plugins\Microsoft.Azure.Geneva.GenevaMonitoring\2.12.0.1\Monitoring\Agent
2019-11-22T03:17:35.3174890Z More useful test error messages on should_panic(expected=...) mismatch
2019-11-22T03:17:35.3175173Z NPM_CONFIG_PREFIX=C:\npm\prefix
2019-11-22T03:17:35.3175255Z NUMBER_OF_PROCESSORS=2
2019-11-22T03:17:35.3175378Z OS=Windows_NT
2019-11-22T03:17:35.3176335Z PATH=/mingw64/bin:/usr/bin:/c/Users/VssAdministrator/bin:/c/agents/2.160.1/externals/git/cmd:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/Program Files/Mercurial:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/Program Files/Boost/1.69.0:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/Java/zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64/bin:/c/npm/prefix:/c/Rust/.cargo/bin:/c/hostedtoolcache/windows/Ruby/2.5.5/x64/bin:/c/Go1.12.7/bin:/bin:/usr/bin:/mingw64/bin:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/Program Files/Microsoft MPI/Bin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/6:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/cmd:/mingw64/bin:/usr/bin:/c/tools/php:/c/Program Files (x86)/Subversion/bin:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.2/bin:/c/Program Files/CMake/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/OpenSSL/bin:/c/Users/VssAdministrator/.dotnet/tools:/c/Program Files (x86)/Microsoft SQL Server/120
---
2019-11-22T03:17:35.3187069Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-11-22T03:17:35.3187226Z SYSTEM_TIMELINEID=a49d49e0-95c0-4776-94b8-be1a6502d92c
2019-11-22T03:17:35.3187317Z SYSTEM_TOTALJOBSINPHASE=16
2019-11-22T03:17:35.3187441Z SYSTEM_WORKFOLDER=D:\a
2019-11-22T03:17:35.3187558Z Shows both the actual as well as the expected panic value when a test with `should_panic(expected=...)` fails.
2019-11-22T03:17:35.3187811Z TEMP=/tmp
2019-11-22T03:17:35.3209688Z TERM=cygwin
2019-11-22T03:17:35.3209793Z TF_BUILD=True
2019-11-22T03:17:35.3209871Z TMP=/tmp
2019-11-22T03:17:35.3209871Z TMP=/tmp
2019-11-22T03:17:35.3209980Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-22T03:17:35.3210078Z TOOLSTATE_PUBLISH=1
2019-11-22T03:17:35.3210192Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-22T03:17:35.3210300Z This makes `should_panic` more consistent with `assert_eq`.
2019-11-22T03:17:35.3210500Z USERDOMAIN=fv-az379
2019-11-22T03:17:35.3210790Z USERDOMAIN_ROAMINGPROFILE=fv-az379
2019-11-22T03:17:35.3210950Z USERNAME=VssAdministrator
2019-11-22T03:17:35.3211077Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-22T03:21:19.4117618Z 
2019-11-22T03:21:19.4117756Z 
2019-11-22T03:21:19.4637527Z kill: 1005: No such process
2019-11-22T03:21:19.4648224Z kill: 1004: No such process
2019-11-22T04:05:21.1168680Z Chocolatey timed out waiting for the command to finish. The timeout 
2019-11-22T04:05:21.1169361Z  specified (or the default value) was '2700' seconds. Perhaps try a 
2019-11-22T04:05:21.1169475Z  higher `--execution-timeout`? See `choco -h` for details.
2019-11-22T04:05:22.0015163Z The install of msys2 was NOT successful.
2019-11-22T04:05:22.0016664Z Error while running 'C:\ProgramData\chocolatey\lib\msys2\tools\chocolateyinstall.ps1'.
2019-11-22T04:05:22.0017272Z  See log for details.
2019-11-22T04:05:24.0736917Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-22T04:05:24.0737085Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-22T04:05:24.0741033Z 
2019-11-22T04:05:24.0746390Z Failures
2019-11-22T04:05:24.0746390Z Failures
2019-11-22T04:05:24.0753709Z  - msys2 (exited -1) - Error while running 'C:\ProgramData\chocolatey\lib\msys2\tools\chocolateyinstall.ps1'.
2019-11-22T04:05:24.0753882Z  See log for details.
2019-11-22T04:05:24.0769604Z Did you know the proceeds of Pro (and some proceeds from other
2019-11-22T04:05:24.0769745Z  licensed editions) go into bettering the community infrastructure?
2019-11-22T04:05:24.0769894Z  Your support ensures an active community, keeps Chocolatey tip top,
2019-11-22T04:05:24.0769983Z  plus it nets you some awesome features!
2019-11-22T04:05:24.0769983Z  plus it nets you some awesome features!
2019-11-22T04:05:24.0770113Z  https://chocolatey.org/compare
2019-11-22T04:05:34.5314479Z The STDIO streams did not close within 10 seconds of the exit event from process 'C:\Program Files\Git\bin\bash.exe'. This may indicate a child process inherited the STDIO streams and has not yet exited.
2019-11-22T04:05:34.5387119Z ##[error]Bash exited with code '127'.
2019-11-22T04:05:39.5553869Z ##[section]Starting: Checkout
2019-11-22T04:05:39.5681100Z ==============================================================================
2019-11-22T04:05:39.5681259Z Task         : Get sources
2019-11-22T04:05:39.5681374Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

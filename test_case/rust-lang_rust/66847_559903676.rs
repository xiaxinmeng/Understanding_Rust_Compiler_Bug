plain
2019-11-30T01:48:08.3803989Z   |
2019-11-30T01:48:08.3804139Z   |
2019-11-30T01:48:08.3804318Z   |                ^^ invalid argument name in format string
2019-11-30T01:48:08.3804628Z  --> src/main.rs:2:16
2019-11-30T01:48:08.3805167Z 2 |     println!("{_x}", _x=0);
2019-11-30T01:48:08.3820110Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T01:48:08.3820321Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T01:48:08.3820418Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T01:48:08.3820491Z AGENT_ID=510
---
2019-11-30T01:48:08.3823055Z ANT_HOME=C:\ProgramData\chocolatey\lib\ant\apache-ant-1.10.5
2019-11-30T01:48:08.3823158Z APPDATA=C:\Users\VssAdministrator\AppData\Roaming
2019-11-30T01:48:08.3823258Z AZURE_EXTENSION_DIR=C:\Program Files\Common Files\AzureCliExtensionDirectory
2019-11-30T01:48:08.3823377Z AZURE_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-30T01:48:08.3823461Z Allow any identifier as format arg name
2019-11-30T01:48:08.3823769Z BOOST_ROOT_1_69_0=C:\Program Files\Boost\1.69.0
2019-11-30T01:48:08.3823841Z BUILD_ARTIFACTSTAGINGDIRECTORY=D:\a\1\a
2019-11-30T01:48:08.3823930Z BUILD_BINARIESDIRECTORY=D:\a\1\b
2019-11-30T01:48:08.3825354Z BUILD_BUILDID=14782
---
2019-11-30T01:48:08.3832960Z MonAgentClientLocation=C:\Packages\Plugins\Microsoft.Azure.Geneva.GenevaMonitoring\2.15.0.1\Monitoring\Agent
2019-11-30T01:48:08.3833054Z NPM_CONFIG_CACHE=C:\npm\cache
2019-11-30T01:48:08.3833146Z NPM_CONFIG_PREFIX=C:\npm\prefix
2019-11-30T01:48:08.3833232Z NUMBER_OF_PROCESSORS=2
2019-11-30T01:48:08.3833389Z Not supporting identifiers starting with underscore appears to have been an arbitrary limitation from 2013 in code that was most likely never reviewed: https://github.com/rust-lang/rust/pull/8245/files#diff-0347868ef389c805e97636623e4a4ea6R277
2019-11-30T01:48:08.3834356Z PATH=/mingw64/bin:/usr/bin:/c/Users/VssAdministrator/bin:/c/agents/2.160.1/externals/git/cmd:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/Program Files/Mercurial:/c/ProgramData/kind:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/Program Files/Mercurial:/c/Program Files/Boost/1.69.0:/c/Program Files/dotnet:/c/mysql-5.7.21-winx64/bin:/c/Program Files/Java/zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64/bin:/c/npm/prefix:/c/Rust/.cargo/bin:/c/hostedtoolcache/windows/Ruby/2.5.5/x64/bin:/c/Go1.12.7/bin:/bin:/usr/bin:/mingw64/bin:/c/hostedtoolcache/windows/Python/3.6.8/x64/Scripts:/c/hostedtoolcache/windows/Python/3.6.8/x64:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/Program Files/Microsoft MPI/Bin:/c/windows/system32:/c/windows:/c/windows/System32/Wbem:/c/windows/System32/WindowsPowerShell/v1.0:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/6:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/cmd:/mingw64/bin:/usr/bin:/c/tools/php:/c/Program Files (x86)/Subversion/bin:/c/Program Files/nodejs:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.6.2/bin:/c/Program Files/CMake/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/Program Files/OpenSSL/bin:/c/Users/VssAdministrator/.dotnet/tools:/c/Program Files (x86)/Microsoft SQL Server/120
2019-11-30T01:48:08.3835207Z PATHEXT=.COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.PY
2019-11-30T01:48:08.3835765Z PHPROOT=c:\tools\php
2019-11-30T01:48:08.3835859Z PIPELINE_WORKSPACE=D:\a\1
---
2019-11-30T01:48:08.3842229Z TMP=/tmp
2019-11-30T01:48:08.3842316Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:48:08.3842411Z TOOLSTATE_PUBLISH=1
2019-11-30T01:48:08.3842490Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:48:08.3842617Z The error message was dutifully improved in #50610 but is there any reason that leading underscore would be a special case?
2019-11-30T01:48:08.3842799Z This commit updates the format_args parser to accept identifiers with leading underscores.
2019-11-30T01:48:08.3842971Z USERDOMAIN=fv-az433
2019-11-30T01:48:08.3843037Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T01:48:08.3843120Z USERNAME=VssAdministrator
2019-11-30T01:48:08.3843188Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:49:57.1880634Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:49:57.1880897Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:49:57.1884664Z 
2019-11-30T01:49:57.1890154Z Failures
2019-11-30T01:49:57.1897266Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T01:49:57.1897623Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T01:49:57.7413035Z 
2019-11-30T01:49:57.7518496Z ##[error]Bash exited with code '1'.
2019-11-30T01:49:57.7660559Z ##[section]Starting: Checkout
2019-11-30T01:49:57.7780112Z ==============================================================================
2019-11-30T01:49:57.7780243Z Task         : Get sources
2019-11-30T01:49:57.7780358Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

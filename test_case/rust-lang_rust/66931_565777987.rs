plain
2019-12-15T04:47:35.8059987Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-15T04:47:35.8060047Z 
2019-12-15T04:47:35.8060132Z   git checkout -b <new-branch-name>
2019-12-15T04:47:35.8060183Z 
2019-12-15T04:47:35.8060288Z HEAD is now at 9e4d3cf93 Auto merge of #66931 - cjgillot:hirene-preamble, r=eddyb
2019-12-15T04:47:35.8387466Z ##[section]Starting: Setup environment
2019-12-15T04:47:35.8487098Z ==============================================================================
2019-12-15T04:47:35.8487183Z Task         : Bash
2019-12-15T04:47:35.8487270Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-15T04:47:37.4681656Z 
2019-12-15T04:47:37.4681838Z 
2019-12-15T04:47:37.4682000Z 
2019-12-15T04:47:37.4682230Z 
2019-12-15T04:47:37.4682478Z 1. This first PR lays the groundwork and migrates some low-hanging fruits.
2019-12-15T04:47:37.4683472Z 2. The second PR will migrate `hir::Expr`, `hir::Pat` and related.
2019-12-15T04:47:37.4685433Z 3. The third PR will migrate `hir::Ty` and related.
2019-12-15T04:47:37.4685724Z 4. The final PR will be dedicated to eventual cleanups.
2019-12-15T04:47:37.4691793Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-15T04:47:37.4691897Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-15T04:47:37.4691989Z AGENT_HOMEDIRECTORY=C:\agents\2.163.1
2019-12-15T04:47:37.4692061Z AGENT_ID=514
---
2019-12-15T04:47:37.4698374Z BUILD_SOURCEBRANCHNAME=auto
2019-12-15T04:47:37.4698453Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-15T04:47:37.4698566Z BUILD_SOURCEVERSION=9e4d3cf93102e00b9cdc17a1d08b943baa6426f5
2019-12-15T04:47:37.4698663Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-15T04:47:37.4698785Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66931 - cjgillot:hirene-preamble, r=eddyb
2019-12-15T04:47:37.4698978Z CI_JOB_NAME=dist-x86_64-msvc
2019-12-15T04:47:37.4699073Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-15T04:47:37.4699164Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-15T04:47:37.4699274Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-15T04:47:37.4701629Z HOMEPATH=\Users\VssAdministrator
2019-12-15T04:47:37.4701714Z IEWebDriver=C:\SeleniumWebDrivers\IEDriver
2019-12-15T04:47:37.4701811Z INPUT_ARGUMENTS=
2019-12-15T04:47:37.4701885Z ImageVersion=20191202.1
2019-12-15T04:47:37.4702045Z In order to make the transition as gradual as possible, some lowering routines receive `Box`-allocated data and move it into the arena. This is a bit wasteful, but hopefully temporary.
2019-12-15T04:47:37.4702683Z JAVA_HOME_11_X64=C:\Program Files\Java\zulu-11-azure-jdk_11.33.15-11.0.4-win_x64
2019-12-15T04:47:37.4702850Z JAVA_HOME_7_X64=C:\Program Files\Java\zulu-7-azure-jdk_7.31.0.5-7.0.232-win_x64
2019-12-15T04:47:37.4702967Z JAVA_HOME_8_X64=C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64
2019-12-15T04:47:37.4703093Z LOCALAPPDATA=C:\Users\VssAdministrator\AppData\Local
---
2019-12-15T04:47:37.4714926Z TMP=/tmp
2019-12-15T04:47:37.4715032Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-12-15T04:47:37.4715126Z TOOLSTATE_PUBLISH=1
2019-12-15T04:47:37.4715237Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-12-15T04:47:37.4715361Z This PR is the first in a series of 4, aiming at allocating the HIR on an arena, as a memory optimisation.
2019-12-15T04:47:37.4715629Z USERDOMAIN=fv-az425
2019-12-15T04:47:37.4715724Z USERDOMAIN_ROAMINGPROFILE=fv-az425
2019-12-15T04:47:37.4715822Z USERNAME=VssAdministrator
2019-12-15T04:47:37.4715901Z USERPROFILE=C:\Users\VssAdministrator
2019-12-15T04:47:37.4715901Z USERPROFILE=C:\Users\VssAdministrator
2019-12-15T04:47:37.4715999Z VCPKG_INSTALLATION_ROOT=C:\vcpkg
2019-12-15T04:47:37.4716101Z VS140COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\Tools\
2019-12-15T04:47:37.4716223Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-12-15T04:47:37.4716320Z VSTS_PROCESS_LOOKUP_ID=vsts_6e3207eb-ed6e-4edc-abe8-b8f6228b582f
2019-12-15T04:47:37.4716428Z WINDIR=C:\windows
2019-12-15T04:47:37.4716511Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-12-15T04:47:37.4717593Z Work mentored by @Zoxc.
2019-12-15T04:47:37.4717806Z agent.jobstatus=Succeeded
2019-12-15T04:47:37.4718979Z 
2019-12-15T04:47:37.4719650Z disk usage:
2019-12-15T04:47:37.5240031Z Filesystem            Size  Used Avail Use% Mounted on
---
2019-12-15T04:49:20.0111925Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T04:49:20.0112180Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-15T04:49:20.0117385Z 
2019-12-15T04:49:20.0122906Z Failures
2019-12-15T04:49:20.0130634Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-15T04:49:20.0131019Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-15T04:49:20.0147422Z Did you know the proceeds of Pro (and some proceeds from other
2019-12-15T04:49:20.0147661Z  licensed editions) go into bettering the community infrastructure?
2019-12-15T04:49:20.0147911Z  Your support ensures an active community, keeps Chocolatey tip top,
2019-12-15T04:49:20.0148132Z  plus it nets you some awesome features!
2019-12-15T04:49:20.0148132Z  plus it nets you some awesome features!
2019-12-15T04:49:20.0148324Z  https://chocolatey.org/compare
2019-12-15T04:49:20.5016741Z 
2019-12-15T04:49:20.5115296Z ##[error]Bash exited with code '1'.
2019-12-15T04:49:20.5294773Z ##[section]Starting: Checkout
2019-12-15T04:49:20.5453098Z ==============================================================================
2019-12-15T04:49:20.5453226Z Task         : Get sources
2019-12-15T04:49:20.5453323Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

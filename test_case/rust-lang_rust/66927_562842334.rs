plain
2019-12-07T09:30:37.4834926Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-07T09:30:37.4834984Z 
2019-12-07T09:30:37.4835696Z   git checkout -b <new-branch-name>
2019-12-07T09:30:37.4835927Z 
2019-12-07T09:30:37.4836002Z HEAD is now at 517286a22 Auto merge of #66927 - RalfJung:engines-dont-panic, r=oli-obk
2019-12-07T09:30:37.5214624Z ##[section]Starting: Setup environment
2019-12-07T09:30:37.5320147Z ==============================================================================
2019-12-07T09:30:37.5320241Z Task         : Bash
2019-12-07T09:30:37.5320299Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-07T09:30:39.2429868Z BUILD_SOURCEBRANCHNAME=auto
2019-12-07T09:30:39.2429923Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-07T09:30:39.2429997Z BUILD_SOURCEVERSION=517286a2264b49996c5c79e6a224250e0dd6ffc5
2019-12-07T09:30:39.2430077Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-07T09:30:39.2430158Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66927 - RalfJung:engines-dont-panic, r=oli-obk
2019-12-07T09:30:39.2430286Z CI_JOB_NAME=i686-mingw-1
2019-12-07T09:30:39.2430339Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-07T09:30:39.2430419Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-07T09:30:39.2430493Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-07T09:30:39.2430781Z ChocolateyInstall=C:\ProgramData\chocolatey
2019-12-07T09:30:39.2430853Z ChromeWebDriver=C:\SeleniumWebDrivers\ChromeDriver
2019-12-07T09:30:39.2430918Z CommonProgramFiles(x86)=C:\Program Files (x86)\Common Files
2019-12-07T09:30:39.2431004Z CommonProgramW6432=C:\Program Files\Common Files
2019-12-07T09:30:39.2431089Z ConstProp and ConstEval still use it, though. This will be addressed in future PRs.
2019-12-07T09:30:39.2431228Z ENDPOINT_URL_SYSTEMVSSCONNECTION=https://dev.azure.com/rust-lang/
2019-12-07T09:30:39.2431288Z EXEPATH=C:\Program Files\Git\bin
2019-12-07T09:30:39.2431372Z From what I can tell, all the error messages this removes are actually duplicates.
2019-12-07T09:30:39.2431434Z GCM_INTERACTIVE=Never
---
2019-12-07T09:30:39.2441312Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-12-07T09:30:39.2441774Z SYSTEM_TIMELINEID=3f3e4b70-f97a-4e68-afaa-e8b2a8366681
2019-12-07T09:30:39.2441858Z SYSTEM_TOTALJOBSINPHASE=16
2019-12-07T09:30:39.2441944Z SYSTEM_WORKFOLDER=D:\a
2019-12-07T09:30:39.2442118Z See https://github.com/rust-lang/rust/issues/66902 for context: panicking is not really an "interpreter error", but just part of a normal Rust execution. This is a first step towards removing the `InterpError::Panic` variant: the core Miri engine does not use it any more.
2019-12-07T09:30:39.2442420Z TEMP=/tmp
2019-12-07T09:30:39.2442499Z TERM=cygwin
2019-12-07T09:30:39.2442579Z TF_BUILD=True
2019-12-07T09:30:39.2442644Z TMP=/tmp
---
2019-12-07T11:22:21.8187937Z 
2019-12-07T11:22:21.8188064Z 
2019-12-07T11:22:21.8906322Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2019-12-07T11:22:21.8907085Z Build completed unsuccessfully in 1:42:05
2019-12-07T11:22:21.9561732Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2019-12-07T11:22:22.0182798Z   local time: Sat Dec  7 11:22:22 CUT 2019
2019-12-07T11:22:22.5221818Z   network time: Sat, 07 Dec 2019 11:22:22 GMT
2019-12-07T11:22:22.5232211Z == end clock drift check ==
2019-12-07T11:22:22.6105885Z 
2019-12-07T11:22:22.6105885Z 
2019-12-07T11:22:23.0183985Z ##[error]Bash exited with code '2'.
2019-12-07T11:22:23.0802814Z ##[section]Starting: Checkout
2019-12-07T11:22:23.1583095Z ==============================================================================
2019-12-07T11:22:23.1583396Z Task         : Get sources
2019-12-07T11:22:23.1583465Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

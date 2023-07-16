plain
2019-12-01T09:56:02.6295715Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T09:56:02.6295775Z 
2019-12-01T09:56:02.6295827Z   git checkout -b <new-branch-name>
2019-12-01T09:56:02.6295876Z 
2019-12-01T09:56:02.6295958Z HEAD is now at 1de4fab9b Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:56:02.6653783Z ##[section]Starting: Setup environment
2019-12-01T09:56:02.6754659Z ==============================================================================
2019-12-01T09:56:02.6754799Z Task         : Bash
2019-12-01T09:56:02.6754878Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T09:56:04.4038988Z 
2019-12-01T09:56:04.4039118Z 
2019-12-01T09:56:04.4039239Z 
2019-12-01T09:56:04.4039376Z 
2019-12-01T09:56:04.4039583Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T09:56:04.4039799Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T09:56:04.4041361Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T09:56:04.4041740Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T09:56:04.4042309Z  - #66828 (Less minification)
2019-12-01T09:56:04.4042707Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T09:56:04.4043904Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T09:56:04.4044354Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T09:56:04.4045800Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T09:56:04.4045987Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T09:56:04.4046145Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T09:56:04.4046308Z AGENT_ID=513
---
2019-12-01T09:56:04.4055976Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T09:56:04.4056063Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T09:56:04.4056143Z BUILD_SOURCEVERSION=1de4fab9b1be94e829069c48b02a84c9fdf9181e
2019-12-01T09:56:04.4056358Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T09:56:04.4056447Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:56:04.4057018Z CI_JOB_NAME=dist-x86_64-mingw
2019-12-01T09:56:04.4057089Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T09:56:04.4057180Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T09:56:04.4057260Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T09:56:16.9938496Z Chocolatey v0.10.15
2019-12-01T09:57:01.9994465Z Installing the following packages:
2019-12-01T09:57:01.9999642Z msys2
2019-12-01T09:57:02.0004390Z By installing you accept licenses for the packages.
2019-12-01T09:58:42.8224639Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-01T09:58:42.8225909Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-01T09:58:42.8232818Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T09:58:42.8233345Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T09:58:42.8233861Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T09:58:42.8234555Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T09:58:42.8235360Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T09:58:42.8235717Z  assistance.
2019-12-01T09:58:42.8339978Z 
2019-12-01T09:58:42.8340204Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T09:58:42.8340204Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T09:58:42.8340758Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T09:58:42.8345022Z 
2019-12-01T09:58:42.8350503Z Failures
2019-12-01T09:58:42.8357087Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T09:58:42.8357710Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T09:58:42.8358067Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T09:58:42.8358409Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T09:58:42.8359085Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T09:58:42.8359387Z  assistance.
2019-12-01T09:58:43.1890793Z 
2019-12-01T09:58:43.1890793Z 
2019-12-01T09:58:43.1968895Z ##[error]Bash exited with code '1'.
2019-12-01T09:58:43.2089841Z ##[section]Starting: Checkout
2019-12-01T09:58:43.2192952Z ==============================================================================
2019-12-01T09:58:43.2193072Z Task         : Get sources
2019-12-01T09:58:43.2193151Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

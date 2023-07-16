plain
2019-11-30T07:20:37.7502116Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T07:20:37.7503077Z 
2019-11-30T07:20:37.7503582Z   git checkout -b <new-branch-name>
2019-11-30T07:20:37.7504015Z 
2019-11-30T07:20:37.7504388Z HEAD is now at dc9a63fe5 Auto merge of #66759 - CAD97:patch-3, r=KodrAus
2019-11-30T07:20:37.7899904Z ##[section]Starting: Setup environment
2019-11-30T07:20:37.8013148Z ==============================================================================
2019-11-30T07:20:37.8013459Z Task         : Bash
2019-11-30T07:20:37.8013567Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T07:20:39.5325243Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T07:20:39.5325319Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T07:20:39.5325429Z BUILD_SOURCEVERSION=dc9a63fe5970e8ebb830bd4d283f8a508263f8c7
2019-11-30T07:20:39.5325530Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T07:20:39.5325647Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66759 - CAD97:patch-3, r=KodrAus
2019-11-30T07:20:39.5325831Z CI_JOB_NAME=dist-x86_64-msvc
2019-11-30T07:20:39.5326094Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T07:20:39.5326165Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T07:20:39.5326421Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T07:20:39.5338657Z TMP=/tmp
2019-11-30T07:20:39.5338901Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T07:20:39.5338989Z TOOLSTATE_PUBLISH=1
2019-11-30T07:20:39.5339062Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T07:20:39.5339163Z The iterator methods just forward to `slice::Iter`, which is `TrustedLen`.
2019-11-30T07:20:39.5339247Z This can probably be applied to other `Drain` structs as well.
2019-11-30T07:20:39.5339418Z USERDOMAIN=fv-az433
2019-11-30T07:20:39.5339479Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T07:20:39.5339558Z USERNAME=VssAdministrator
2019-11-30T07:20:39.5339621Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T07:22:10.8294810Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T07:22:10.8295171Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T07:22:10.8298927Z 
2019-11-30T07:22:10.8304161Z Failures
2019-11-30T07:22:10.8310582Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T07:22:10.8311217Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T07:22:11.3432238Z 
2019-11-30T07:22:11.3517881Z ##[error]Bash exited with code '1'.
2019-11-30T07:22:11.3685255Z ##[section]Starting: Checkout
2019-11-30T07:22:11.3798276Z ==============================================================================
2019-11-30T07:22:11.3798395Z Task         : Get sources
2019-11-30T07:22:11.3798493Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

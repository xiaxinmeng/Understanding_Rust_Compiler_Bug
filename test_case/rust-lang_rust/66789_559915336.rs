plain
2019-11-30T05:52:07.4774736Z BUILD_SOURCEVERSION=8d0f4d517534f9c3fe1b224519b8a1104bebdcdd
2019-11-30T05:52:07.4774828Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T05:52:07.4774938Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66789 - eddyb:mir-source-scope-local-data, r=oli-obk
2019-11-30T05:52:07.4775023Z BUILD_STAGINGDIRECTORY=D:\a\1\a
2019-11-30T05:52:07.4775286Z By having one `ClearCrossCrate<SourceScopeLocalData>` for each scope, as opposed to a single `ClearCrossCrate` for all the `SourceScopeLocalData`s, we can represent the fact that some scopes have `SourceScopeLocalData` associated with them, and some don't.
2019-11-30T05:52:07.4775518Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T05:52:07.4775611Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T05:52:07.4775873Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-30T05:52:07.4775961Z COMPUTERNAME=fv-az433
---
2019-11-30T05:52:07.4832261Z TMP=/tmp
2019-11-30T05:52:07.4832338Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T05:52:07.4832430Z TOOLSTATE_PUBLISH=1
2019-11-30T05:52:07.4832510Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T05:52:07.4832785Z This is useful when doing MIR inlining across crates, because the `ClearCrossCrate` will be `Clear` for the cross-crate MIR scopes and `Set` for the local ones.
2019-11-30T05:52:07.4832993Z USERDOMAIN=fv-az433
2019-11-30T05:52:07.4833077Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T05:52:07.4833147Z USERNAME=VssAdministrator
2019-11-30T05:52:07.4833226Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T05:52:07.4833734Z WINDIR=C:\windows
2019-11-30T05:52:07.4833805Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-30T05:52:07.4833885Z _=/usr/bin/printenv
2019-11-30T05:52:07.4833958Z agent.jobstatus=Succeeded
2019-11-30T05:52:07.4834050Z rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.
2019-11-30T05:52:07.4834213Z disk usage:
2019-11-30T05:52:07.5447973Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-30T05:52:07.5453938Z C:/Program Files/Git  256G  141G  116G  55% /
2019-11-30T05:52:07.5454208Z D:                     14G  2.0G   13G  15% /d
---
2019-11-30T05:53:16.6112744Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T05:53:16.6113125Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T05:53:16.6113430Z 
2019-11-30T05:53:16.6113766Z Failures
2019-11-30T05:53:16.6114126Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T05:53:16.6114492Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T05:53:20.9206173Z 
2019-11-30T05:53:20.9276442Z ##[error]Bash exited with code '1'.
2019-11-30T05:53:20.9417464Z ##[section]Starting: Checkout
2019-11-30T05:53:20.9514961Z ==============================================================================
2019-11-30T05:53:20.9515047Z Task         : Get sources
2019-11-30T05:53:20.9515116Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-11-30T01:14:05.8831108Z BUILD_SOURCEVERSION=afdfb609bb65cd078432d8bd60265755b5f0e7ce
2019-11-30T01:14:05.8831183Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T01:14:05.8831273Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66789 - eddyb:mir-source-scope-local-data, r=oli-obk
2019-11-30T01:14:05.8831343Z BUILD_STAGINGDIRECTORY=D:\a\1\a
2019-11-30T01:14:05.8831474Z By having one `ClearCrossCrate<SourceScopeLocalData>` for each scope, as opposed to a single `ClearCrossCrate` for all the `SourceScopeLocalData`s, we can represent the fact that some scopes have `SourceScopeLocalData` associated with them, and some don't.
2019-11-30T01:14:05.8831651Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T01:14:05.8831729Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T01:14:05.8831791Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-30T01:14:05.8831863Z COMPUTERNAME=fv-az433
---
2019-11-30T01:14:05.8844099Z TMP=/tmp
2019-11-30T01:14:05.8844157Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:14:05.8844235Z TOOLSTATE_PUBLISH=1
2019-11-30T01:14:05.8844297Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:14:05.8844461Z This is useful when doing MIR inlining across crates, because the `ClearCrossCrate` will be `Clear` for the cross-crate MIR scopes and `Set` for the local ones.
2019-11-30T01:14:05.8845148Z USERDOMAIN=fv-az433
2019-11-30T01:14:05.8845328Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T01:14:05.8845402Z USERNAME=VssAdministrator
2019-11-30T01:14:05.8845492Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:14:05.8845970Z WINDIR=C:\windows
2019-11-30T01:14:05.8846046Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-30T01:14:05.8846137Z _=/usr/bin/printenv
2019-11-30T01:14:05.8846206Z agent.jobstatus=Succeeded
2019-11-30T01:14:05.8846319Z rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.
2019-11-30T01:14:05.8859273Z disk usage:
2019-11-30T01:14:05.8859446Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-30T01:14:05.8859594Z C:/Program Files/Git  256G  141G  116G  55% /
2019-11-30T01:14:05.8859779Z D:                     14G  2.0G   13G  15% /d
---
2019-11-30T01:14:17.7788514Z Chocolatey v0.10.15
2019-11-30T01:14:46.6098590Z Installing the following packages:
2019-11-30T01:14:46.6099330Z msys2
2019-11-30T01:14:46.6099522Z By installing you accept licenses for the packages.
2019-11-30T01:16:27.2609228Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-11-30T01:16:27.2609542Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-11-30T01:16:27.2613071Z msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T01:16:27.2613482Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T01:16:27.2613608Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T01:16:27.2613722Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T01:16:27.2613897Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T01:16:27.2613988Z  assistance.
2019-11-30T01:16:27.2752702Z 
2019-11-30T01:16:27.2752906Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:16:27.2752906Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:16:27.2753197Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:16:27.2757721Z 
2019-11-30T01:16:27.2763148Z Failures
2019-11-30T01:16:27.2770657Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T01:16:27.2770788Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T01:16:27.2770913Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T01:16:27.2771034Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T01:16:27.2771266Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T01:16:27.2771348Z  assistance.
2019-11-30T01:16:27.6365076Z 
2019-11-30T01:16:27.6365076Z 
2019-11-30T01:16:27.6424624Z ##[error]Bash exited with code '1'.
2019-11-30T01:16:27.6607560Z ##[section]Starting: Checkout
2019-11-30T01:16:27.6722231Z ==============================================================================
2019-11-30T01:16:27.6722329Z Task         : Get sources
2019-11-30T01:16:27.6722566Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

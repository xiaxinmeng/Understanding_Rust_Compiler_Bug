plain
2019-11-30T01:34:15.0576988Z CommonProgramW6432=C:\Program Files\Common Files
2019-11-30T01:34:15.0577071Z DEPLOY_BUCKET=rust-lang-ci2
2019-11-30T01:34:15.0577190Z ENDPOINT_URL_SYSTEMVSSCONNECTION=https://dev.azure.com/rust-lang/
2019-11-30T01:34:15.0577288Z EXEPATH=C:\Program Files\Git\bin
2019-11-30T01:34:15.0577447Z Fixes https://github.com/rust-lang/miri/issues/1075: the shim around diverging closures turned into function pointers actually "obtains" a return place inside a diverging function, but just uses it as the return place for a diverging callee. Handle this by using NULL places.
2019-11-30T01:34:15.0577665Z GIT_TERMINAL_PROMPT=0
2019-11-30T01:34:15.0577725Z GOROOT=C:\Go1.12.7
2019-11-30T01:34:15.0577804Z GOROOT_1_10_X64=C:\Go1.10.8
2019-11-30T01:34:15.0577884Z GOROOT_1_11_X64=C:\Go1.11.12
---
2019-11-30T01:34:15.0591330Z TMP=/tmp
2019-11-30T01:34:15.0591419Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:34:15.0591520Z TOOLSTATE_PUBLISH=1
2019-11-30T01:34:15.0591605Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:34:15.0591914Z This is kind of a hack as it breaks our invariant that all places are dereferencable, but we'd eventually let raw pointers break that anyway I assume so that seems fine.
2019-11-30T01:34:15.0592093Z USERDOMAIN=fv-az433
2019-11-30T01:34:15.0592168Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T01:34:15.0592665Z USERNAME=VssAdministrator
2019-11-30T01:34:15.0592765Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:35:54.3969710Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:35:54.3970153Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:35:54.3973473Z 
2019-11-30T01:35:54.3977648Z Failures
2019-11-30T01:35:54.3986072Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T01:35:54.3986608Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T01:35:54.9088106Z 
2019-11-30T01:35:54.9176059Z ##[error]Bash exited with code '1'.
2019-11-30T01:35:54.9343444Z ##[section]Starting: Checkout
2019-11-30T01:35:54.9459800Z ==============================================================================
2019-11-30T01:35:54.9460058Z Task         : Get sources
2019-11-30T01:35:54.9460159Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

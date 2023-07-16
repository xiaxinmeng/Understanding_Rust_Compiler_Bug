plain
2019-11-30T07:44:33.3612930Z CommonProgramW6432=C:\Program Files\Common Files
2019-11-30T07:44:33.3612986Z DEPLOY_BUCKET=rust-lang-ci2
2019-11-30T07:44:33.3613071Z ENDPOINT_URL_SYSTEMVSSCONNECTION=https://dev.azure.com/rust-lang/
2019-11-30T07:44:33.3613132Z EXEPATH=C:\Program Files\Git\bin
2019-11-30T07:44:33.3613267Z Fixes https://github.com/rust-lang/miri/issues/1075: the shim around diverging closures turned into function pointers actually "obtains" a return place inside a diverging function, but just uses it as the return place for a diverging callee. Handle this by using NULL places.
2019-11-30T07:44:33.3613452Z GIT_TERMINAL_PROMPT=0
2019-11-30T07:44:33.3613519Z GOROOT=C:\Go1.12.7
2019-11-30T07:44:33.3613569Z GOROOT_1_10_X64=C:\Go1.10.8
2019-11-30T07:44:33.3613639Z GOROOT_1_11_X64=C:\Go1.11.12
---
2019-11-30T07:44:33.3622949Z TMP=/tmp
2019-11-30T07:44:33.3623029Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T07:44:33.3623091Z TOOLSTATE_PUBLISH=1
2019-11-30T07:44:33.3623170Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T07:44:33.3623270Z This is kind of a hack as it breaks our invariant that all places are dereferencable, but we'd eventually let raw pointers break that anyway I assume so that seems fine.
2019-11-30T07:44:33.3623482Z USERDOMAIN=fv-az379
2019-11-30T07:44:33.3623562Z USERDOMAIN_ROAMINGPROFILE=fv-az379
2019-11-30T07:44:33.3623632Z USERNAME=VssAdministrator
2019-11-30T07:44:33.3623685Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T07:46:03.4942196Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T07:46:03.4942316Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T07:46:03.4947236Z 
2019-11-30T07:46:03.4952654Z Failures
2019-11-30T07:46:03.4959678Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T07:46:03.4959800Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T07:46:03.4976941Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-11-30T07:46:03.4977050Z experience to the next level at
2019-11-30T07:46:03.4977125Z  https://chocolatey.org/compare
2019-11-30T07:46:04.0036096Z 
2019-11-30T07:46:04.0036096Z 
2019-11-30T07:46:04.0125540Z ##[error]Bash exited with code '1'.
2019-11-30T07:46:04.0245951Z ##[section]Starting: Checkout
2019-11-30T07:46:04.0339308Z ==============================================================================
2019-11-30T07:46:04.0339390Z Task         : Get sources
2019-11-30T07:46:04.0339460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

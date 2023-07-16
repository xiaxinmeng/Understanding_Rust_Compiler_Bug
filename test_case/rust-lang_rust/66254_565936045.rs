plain
2019-12-16T07:16:18.9965567Z TMP=/tmp
2019-12-16T07:16:18.9965656Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-12-16T07:16:18.9965731Z TOOLSTATE_PUBLISH=1
2019-12-16T07:16:18.9965822Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-12-16T07:16:18.9965971Z This seems like a reasonable change to make. If we don't provide `Layout::new::<T>` as `const`, then users can just instead do the more error prone `Layout::from_size_align_unchecked(mem::size_of::<T>(), mem::align_of::<T>())` for the same effect and an extra `unsafe { }` incantation.
2019-12-16T07:16:18.9966203Z USERDOMAIN=fv-az433
2019-12-16T07:16:18.9966770Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-12-16T07:16:18.9966903Z USERNAME=VssAdministrator
2019-12-16T07:16:18.9966977Z USERPROFILE=C:\Users\VssAdministrator
---
2019-12-16T07:16:31.6801285Z Chocolatey v0.10.15
2019-12-16T07:17:23.0516529Z Installing the following packages:
2019-12-16T07:17:23.0520022Z msys2
2019-12-16T07:17:23.0526557Z By installing you accept licenses for the packages.
2019-12-16T07:19:03.8815759Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-16T07:19:03.8816645Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-16T07:19:03.8824465Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-16T07:19:03.8824971Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-16T07:19:03.8825314Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-16T07:19:03.8825563Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-16T07:19:03.8826087Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-16T07:19:03.8826287Z  assistance.
2019-12-16T07:19:03.8941795Z 
2019-12-16T07:19:03.8941928Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-16T07:19:03.8941928Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-16T07:19:03.8942033Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-16T07:19:03.8946317Z 
2019-12-16T07:19:03.8950367Z Failures
2019-12-16T07:19:03.8956556Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-16T07:19:03.8956724Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-16T07:19:03.8956824Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-16T07:19:03.8957135Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-16T07:19:03.8957290Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-16T07:19:03.8957373Z  assistance.
2019-12-16T07:19:04.2795267Z 
2019-12-16T07:19:04.2795267Z 
2019-12-16T07:19:04.2877650Z ##[error]Bash exited with code '1'.
2019-12-16T07:19:04.3001392Z ##[section]Starting: Checkout
2019-12-16T07:19:04.3106352Z ==============================================================================
2019-12-16T07:19:04.3106450Z Task         : Get sources
2019-12-16T07:19:04.3106568Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

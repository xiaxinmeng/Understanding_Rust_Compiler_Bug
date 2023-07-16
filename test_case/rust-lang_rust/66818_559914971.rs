plain
2019-11-30T05:44:01.5247747Z 
2019-11-30T05:44:01.5247885Z 
2019-11-30T05:44:01.5248020Z 
2019-11-30T05:44:01.5248172Z 
2019-11-30T05:44:01.5248418Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T05:44:01.5248695Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T05:44:01.5248907Z     $ find src/libstd/os -name '*.rs' \
2019-11-30T05:44:01.5249112Z     $ git checkout $THIS_COMMIT^
2019-11-30T05:44:01.5249329Z     $ git diff $THIS_COMMIT  # there should be no difference
2019-11-30T05:44:01.5249551Z     $ git show --pretty= --name-only $THIS_COMMIT \
2019-11-30T05:44:01.5249771Z     $ rg libstd/os outstanding_files | xargs git checkout --
2019-11-30T05:44:01.5249976Z (Same strategy as #66691.)
2019-11-30T05:44:01.5250365Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T05:44:01.5250564Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T05:44:01.5250769Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T05:44:01.5250964Z AGENT_ID=510
---
2019-11-30T05:44:01.5302303Z TMP=/tmp
2019-11-30T05:44:01.5302495Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T05:44:01.5302710Z TOOLSTATE_PUBLISH=1
2019-11-30T05:44:01.5302902Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T05:44:01.5303236Z This commit applies rustfmt with rust-lang/rust's default settings to files in src/libstd/os *that are not involved in any currently open PR* to minimize merge conflicts. The list of files involved in open PRs was determined by querying GitHub's GraphQL API [with this script](https://gist.github.com/dtolnay/aa9c34993dc051a4f344d1b10e4487e8).
2019-11-30T05:44:01.5303643Z To confirm no funny business:
2019-11-30T05:44:01.5304041Z USERDOMAIN=fv-az433
2019-11-30T05:44:01.5304249Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T05:44:01.5304748Z USERNAME=VssAdministrator
2019-11-30T05:44:01.5304954Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T05:45:41.7913624Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T05:45:41.7914093Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T05:45:41.7917558Z 
2019-11-30T05:45:41.7921696Z Failures
2019-11-30T05:45:41.7927984Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T05:45:41.7928555Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T05:45:42.2446519Z 
2019-11-30T05:45:42.2541297Z ##[error]Bash exited with code '1'.
2019-11-30T05:45:42.2693798Z ##[section]Starting: Checkout
2019-11-30T05:45:42.2818412Z ==============================================================================
2019-11-30T05:45:42.2818538Z Task         : Get sources
2019-11-30T05:45:42.2818636Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

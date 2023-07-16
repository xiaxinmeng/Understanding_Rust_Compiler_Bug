plain
2019-11-30T01:20:06.2566108Z 
2019-11-30T01:20:06.2566223Z 
2019-11-30T01:20:06.2566337Z 
2019-11-30T01:20:06.2566466Z 
2019-11-30T01:20:06.2566657Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:20:06.2566891Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:20:06.2567292Z     $ find src/libstd/os -name '*.rs' \
2019-11-30T01:20:06.2567455Z     $ git checkout $THIS_COMMIT^
2019-11-30T01:20:06.2567627Z     $ git diff $THIS_COMMIT  # there should be no difference
2019-11-30T01:20:06.2567802Z     $ git show --pretty= --name-only $THIS_COMMIT \
2019-11-30T01:20:06.2568934Z     $ rg libstd/os outstanding_files | xargs git checkout --
2019-11-30T01:20:06.2570017Z (Same strategy as #66691.)
2019-11-30T01:20:06.2578795Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T01:20:06.2579051Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T01:20:06.2579238Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T01:20:06.2579414Z AGENT_ID=516
---
2019-11-30T01:20:06.2605458Z TMP=/tmp
2019-11-30T01:20:06.2605526Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:20:06.2605611Z TOOLSTATE_PUBLISH=1
2019-11-30T01:20:06.2605683Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:20:06.2605866Z This commit applies rustfmt with rust-lang/rust's default settings to files in src/libstd/os *that are not involved in any currently open PR* to minimize merge conflicts. The list of files involved in open PRs was determined by querying GitHub's GraphQL API [with this script](https://gist.github.com/dtolnay/aa9c34993dc051a4f344d1b10e4487e8).
2019-11-30T01:20:06.2606248Z To confirm no funny business:
2019-11-30T01:20:06.2606415Z USERDOMAIN=fv-az433
2019-11-30T01:20:06.2606493Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T01:20:06.2606708Z USERNAME=VssAdministrator
2019-11-30T01:20:06.2606838Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:21:46.0136392Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:21:46.0137059Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:21:46.0138290Z 
2019-11-30T01:21:46.0139112Z Failures
2019-11-30T01:21:46.0139555Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T01:21:46.0139854Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T01:21:46.0140126Z 
2019-11-30T01:21:46.0204747Z ##[error]Bash exited with code '1'.
2019-11-30T01:21:46.0356703Z ##[section]Starting: Checkout
2019-11-30T01:21:46.0475220Z ==============================================================================
2019-11-30T01:21:46.0475360Z Task         : Get sources
2019-11-30T01:21:46.0475906Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

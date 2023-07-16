plain
2019-11-30T01:24:07.4178514Z 
2019-11-30T01:24:07.4179277Z 
2019-11-30T01:24:07.4179566Z 
2019-11-30T01:24:07.4180050Z 
2019-11-30T01:24:07.4181462Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:24:07.4181989Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:24:07.4182236Z     $ find src/libstd/sys -name '*.rs' \
2019-11-30T01:24:07.4182404Z     $ git checkout $THIS_COMMIT^
2019-11-30T01:24:07.4182608Z     $ git diff $THIS_COMMIT  # there should be no difference
2019-11-30T01:24:07.4182822Z     $ git show --pretty= --name-only $THIS_COMMIT \
2019-11-30T01:24:07.4183008Z     $ rg libstd/sys outstanding_files | xargs git checkout --
2019-11-30T01:24:07.4183195Z (Same strategy as #66691.)
2019-11-30T01:24:07.4183548Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T01:24:07.4183739Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T01:24:07.4183910Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T01:24:07.4184087Z AGENT_ID=522
---
2019-11-30T01:24:07.4209777Z TMP=/tmp
2019-11-30T01:24:07.4209872Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:24:07.4209953Z TOOLSTATE_PUBLISH=1
2019-11-30T01:24:07.4210052Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:24:07.4210245Z This commit applies rustfmt with rust-lang/rust's default settings to files in src/libstd/sys *that are not involved in any currently open PR* to minimize merge conflicts. The list of files involved in open PRs was determined by querying GitHub's GraphQL API [with this script](https://gist.github.com/dtolnay/aa9c34993dc051a4f344d1b10e4487e8).
2019-11-30T01:24:07.4210482Z To confirm no funny business:
2019-11-30T01:24:07.4210650Z USERDOMAIN=fv-az379
2019-11-30T01:24:07.4210744Z USERDOMAIN_ROAMINGPROFILE=fv-az379
2019-11-30T01:24:07.4210829Z USERNAME=VssAdministrator
2019-11-30T01:24:07.4210898Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:25:36.1263949Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:25:36.1264290Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:25:36.1268354Z 
2019-11-30T01:25:36.1272829Z Failures
2019-11-30T01:25:36.1279539Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T01:25:36.1279873Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T01:25:36.7074855Z 
2019-11-30T01:25:36.7141833Z ##[error]Bash exited with code '1'.
2019-11-30T01:25:36.7289846Z ##[section]Starting: Checkout
2019-11-30T01:25:36.7409870Z ==============================================================================
2019-11-30T01:25:36.7409990Z Task         : Get sources
2019-11-30T01:25:36.7410081Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

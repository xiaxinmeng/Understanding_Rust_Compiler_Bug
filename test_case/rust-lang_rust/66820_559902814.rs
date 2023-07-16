plain
2019-11-30T01:28:09.3420781Z 
2019-11-30T01:28:09.3420920Z 
2019-11-30T01:28:09.3421036Z 
2019-11-30T01:28:09.3421152Z 
2019-11-30T01:28:09.3421372Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:28:09.3421598Z         | xargs rustfmt --edition=2018 --unstable-features --skip-children
2019-11-30T01:28:09.3421784Z     $ find src/libstd -name '*.rs' \
2019-11-30T01:28:09.3421972Z     $ git checkout $THIS_COMMIT^
2019-11-30T01:28:09.3422648Z     $ git diff $THIS_COMMIT  # there should be no difference
2019-11-30T01:28:09.3423525Z     $ git show --pretty= --name-only $THIS_COMMIT \
2019-11-30T01:28:09.3424218Z     $ rg libstd outstanding_files | xargs git checkout --
2019-11-30T01:28:09.3424344Z (Same strategy as #66691.)
2019-11-30T01:28:09.3425188Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T01:28:09.3425641Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T01:28:09.3425768Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T01:28:09.3430913Z AGENT_ID=514
---
2019-11-30T01:28:09.3455686Z TMP=/tmp
2019-11-30T01:28:09.3455781Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-30T01:28:09.3455881Z TOOLSTATE_PUBLISH=1
2019-11-30T01:28:09.3455967Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-30T01:28:09.3456262Z This commit applies rustfmt with rust-lang/rust's default settings to files in src/libstd *that are not involved in any currently open PR* to minimize merge conflicts, and are not part of libstd/os (#66818) or libstd/sys (#66819). The list of files involved in open PRs was determined by querying GitHub's GraphQL API [with this script](https://gist.github.com/dtolnay/aa9c34993dc051a4f344d1b10e4487e8).
2019-11-30T01:28:09.3456463Z To confirm no funny business:
2019-11-30T01:28:09.3456640Z USERDOMAIN=fv-az433
2019-11-30T01:28:09.3456711Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-30T01:28:09.3456801Z USERNAME=VssAdministrator
2019-11-30T01:28:09.3456898Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-30T01:28:24.0980620Z Chocolatey v0.10.15
2019-11-30T01:29:13.9302502Z Installing the following packages:
2019-11-30T01:29:13.9307120Z msys2
2019-11-30T01:29:13.9312550Z By installing you accept licenses for the packages.
2019-11-30T01:30:54.7842306Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-11-30T01:30:54.7842622Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-11-30T01:30:54.7849941Z msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T01:30:54.7850146Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T01:30:54.7850276Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T01:30:54.7850405Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T01:30:54.7850609Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T01:30:54.7850691Z  assistance.
2019-11-30T01:30:54.7955529Z 
2019-11-30T01:30:54.7955706Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:30:54.7955706Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T01:30:54.7955795Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T01:30:54.7961834Z 
2019-11-30T01:30:54.7965936Z Failures
2019-11-30T01:30:54.7974256Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-11-30T01:30:54.7974384Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-11-30T01:30:54.7974480Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-11-30T01:30:54.7974572Z If the package version is a prerelease and you didn't specify `--pre`,
2019-11-30T01:30:54.7974729Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-11-30T01:30:54.7974814Z  assistance.
2019-11-30T01:30:55.1537762Z 
2019-11-30T01:30:55.1537762Z 
2019-11-30T01:30:55.1689375Z ##[error]Bash exited with code '1'.
2019-11-30T01:30:55.1830455Z ##[section]Starting: Checkout
2019-11-30T01:30:55.1956655Z ==============================================================================
2019-11-30T01:30:55.1956768Z Task         : Get sources
2019-11-30T01:30:55.1956850Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/843afe9b-3872-4695-9ac5-7a9b89325d83.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71851/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71851/merge:refs/remotes/pull/71851/merge
fatal: couldn't find remote ref refs/pull/71851/merge
##[warning]Git fetch failed with exit code 128, back off 1.767 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71851/merge:refs/remotes/pull/71851/merge
fatal: couldn't find remote ref refs/pull/71851/merge
##[warning]Git fetch failed with exit code 128, back off 4.735 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71851/merge:refs/remotes/pull/71851/merge
fatal: couldn't find remote ref refs/pull/71851/merge
##[error]Git fetch failed with exit code: 128
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71851/merge to s
##[section]Starting: Checkout rust-lang/rust@refs/pull/71851/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71851/merge to s
Cleaning up task key
Start cleaning up orphan processes.
##[section]Finishing: Finalize Job
##[section]Finishing: Linux mingw-check

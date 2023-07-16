plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/de151594-ca62-43f8-a555-e75b7c08d6b2.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71965/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71965/merge:refs/remotes/pull/71965/merge
fatal: couldn't find remote ref refs/pull/71965/merge
##[warning]Git fetch failed with exit code 128, back off 1.975 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71965/merge:refs/remotes/pull/71965/merge
fatal: couldn't find remote ref refs/pull/71965/merge
##[warning]Git fetch failed with exit code 128, back off 3.703 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71965/merge:refs/remotes/pull/71965/merge
fatal: couldn't find remote ref refs/pull/71965/merge
##[error]Git fetch failed with exit code: 128
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71965/merge to s
##[section]Starting: Checkout rust-lang/rust@refs/pull/71965/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71965/merge to s
Cleaning up task key
Start cleaning up orphan processes.
##[section]Finishing: Finalize Job
##[section]Finishing: Linux x86_64-gnu-tools

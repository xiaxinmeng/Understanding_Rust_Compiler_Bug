plain
2020-03-17T02:10:37.3471018Z ========================== Starting Command Output ===========================
2020-03-17T02:10:37.3474625Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/605a3c4b-5d7f-4750-bb11-8bcff7590fcf.sh
2020-03-17T02:10:37.3474917Z 
2020-03-17T02:10:37.3479755Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T02:10:37.3500494Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70063/merge to s
2020-03-17T02:10:37.3503703Z Task         : Get sources
2020-03-17T02:10:37.3504012Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T02:10:37.3504316Z Version      : 1.0.0
2020-03-17T02:10:37.3504516Z Author       : Microsoft
---
2020-03-17T02:10:38.3428527Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T02:10:38.3433958Z ##[command]git config gc.auto 0
2020-03-17T02:10:38.3437668Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T02:10:38.3441002Z ##[command]git config --get-all http.proxy
2020-03-17T02:10:38.3446899Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70063/merge:refs/remotes/pull/70063/merge
2020-03-17T02:10:38.6163180Z fatal: couldn't find remote ref refs/pull/70063/merge
2020-03-17T02:10:38.7017797Z ##[warning]Git fetch failed with exit code 128, back off 6.319 seconds before retry.
2020-03-17T02:10:44.9595787Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70063/merge:refs/remotes/pull/70063/merge
2020-03-17T02:10:45.6435429Z fatal: couldn't find remote ref refs/pull/70063/merge
2020-03-17T02:10:45.7155817Z ##[warning]Git fetch failed with exit code 128, back off 3.264 seconds before retry.
2020-03-17T02:10:48.9116324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70063/merge:refs/remotes/pull/70063/merge
2020-03-17T02:10:49.6073310Z fatal: couldn't find remote ref refs/pull/70063/merge
2020-03-17T02:10:49.6665235Z ##[error]Git fetch failed with exit code: 128
2020-03-17T02:10:49.6680236Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70063/merge to s
2020-03-17T02:10:49.7103491Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70063/merge to s
2020-03-17T02:10:49.7108902Z Task         : Get sources
2020-03-17T02:10:49.7109245Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T02:10:49.7109589Z Version      : 1.0.0
2020-03-17T02:10:49.7109832Z Author       : Microsoft
2020-03-17T02:10:49.7109832Z Author       : Microsoft
2020-03-17T02:10:49.7110190Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T02:10:49.7110599Z ==============================================================================
2020-03-17T02:10:50.0248970Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70063/merge to s
2020-03-17T02:10:50.0336842Z Cleaning up task key
2020-03-17T02:10:50.0338150Z Start cleaning up orphan processes.
2020-03-17T02:10:50.0476741Z ##[section]Finishing: Finalize Job
2020-03-17T02:10:50.0518076Z ##[section]Finishing: Linux x86_64-gnu-tools

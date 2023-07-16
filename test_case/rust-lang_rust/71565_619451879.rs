plain
2020-04-25T23:00:44.3608695Z ========================== Starting Command Output ===========================
2020-04-25T23:00:44.3611377Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/be3f2fba-e57f-4229-9de8-2d3b042480db.sh
2020-04-25T23:00:44.3611634Z 
2020-04-25T23:00:44.3614667Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-25T23:00:44.3632341Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71565/merge to s
2020-04-25T23:00:44.3635204Z Task         : Get sources
2020-04-25T23:00:44.3635464Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T23:00:44.3635731Z Version      : 1.0.0
2020-04-25T23:00:44.3635907Z Author       : Microsoft
---
2020-04-25T23:00:45.3534270Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-25T23:00:45.3539694Z ##[command]git config gc.auto 0
2020-04-25T23:00:45.3543214Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-25T23:00:45.3546398Z ##[command]git config --get-all http.proxy
2020-04-25T23:00:45.3552302Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71565/merge:refs/remotes/pull/71565/merge
2020-04-25T23:00:45.8340524Z fatal: couldn't find remote ref refs/pull/71565/merge
2020-04-25T23:00:45.9394485Z ##[warning]Git fetch failed with exit code 128, back off 1.612 seconds before retry.
2020-04-25T23:00:47.4800188Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71565/merge:refs/remotes/pull/71565/merge
2020-04-25T23:00:47.9085666Z fatal: couldn't find remote ref refs/pull/71565/merge
2020-04-25T23:00:48.9076711Z ##[warning]Git fetch failed with exit code 128, back off 3.443 seconds before retry.
2020-04-25T23:00:51.3777106Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71565/merge:refs/remotes/pull/71565/merge
2020-04-25T23:00:51.9426743Z fatal: couldn't find remote ref refs/pull/71565/merge
2020-04-25T23:00:52.0230675Z ##[error]Git fetch failed with exit code: 128
2020-04-25T23:00:52.0254887Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71565/merge to s
2020-04-25T23:00:52.0637378Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71565/merge to s
2020-04-25T23:00:52.0643624Z Task         : Get sources
2020-04-25T23:00:52.0643947Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-25T23:00:52.0644250Z Version      : 1.0.0
2020-04-25T23:00:52.0644460Z Author       : Microsoft
2020-04-25T23:00:52.0644460Z Author       : Microsoft
2020-04-25T23:00:52.0644773Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-25T23:00:52.0645146Z ==============================================================================
2020-04-25T23:00:52.3777132Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71565/merge to s
2020-04-25T23:00:52.3874321Z Cleaning up task key
2020-04-25T23:00:52.3875811Z Start cleaning up orphan processes.
2020-04-25T23:00:52.4021045Z ##[section]Finishing: Finalize Job
2020-04-25T23:00:52.4063276Z ##[section]Finishing: Linux x86_64-gnu-llvm-8

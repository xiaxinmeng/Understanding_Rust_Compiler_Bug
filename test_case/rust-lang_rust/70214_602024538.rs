plain
2020-03-21T09:12:06.9712399Z ========================== Starting Command Output ===========================
2020-03-21T09:12:06.9714662Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1bb361b9-d376-44c6-91fd-bc4b6cda9e4a.sh
2020-03-21T09:12:06.9714950Z 
2020-03-21T09:12:06.9719395Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T09:12:06.9737507Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70214/merge to s
2020-03-21T09:12:06.9741560Z Task         : Get sources
2020-03-21T09:12:06.9741870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T09:12:06.9742188Z Version      : 1.0.0
2020-03-21T09:12:06.9742393Z Author       : Microsoft
---
2020-03-21T09:12:07.9653481Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T09:12:07.9658776Z ##[command]git config gc.auto 0
2020-03-21T09:12:07.9662392Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T09:12:07.9665751Z ##[command]git config --get-all http.proxy
2020-03-21T09:12:07.9671599Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70214/merge:refs/remotes/pull/70214/merge
2020-03-21T09:12:08.3009802Z fatal: couldn't find remote ref refs/pull/70214/merge
2020-03-21T09:12:08.3823826Z ##[warning]Git fetch failed with exit code 128, back off 8.51 seconds before retry.
2020-03-21T09:12:16.8307529Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70214/merge:refs/remotes/pull/70214/merge
2020-03-21T09:12:17.4201322Z fatal: couldn't find remote ref refs/pull/70214/merge
2020-03-21T09:12:17.4861704Z ##[warning]Git fetch failed with exit code 128, back off 3.454 seconds before retry.
2020-03-21T09:12:20.8845014Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70214/merge:refs/remotes/pull/70214/merge
2020-03-21T09:12:21.5030405Z fatal: couldn't find remote ref refs/pull/70214/merge
2020-03-21T09:12:21.5655055Z ##[error]Git fetch failed with exit code: 128
2020-03-21T09:12:21.5671340Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70214/merge to s
2020-03-21T09:12:21.6088827Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70214/merge to s
2020-03-21T09:12:21.6094586Z Task         : Get sources
2020-03-21T09:12:21.6094944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T09:12:21.6095290Z Version      : 1.0.0
2020-03-21T09:12:21.6095513Z Author       : Microsoft
2020-03-21T09:12:21.6095513Z Author       : Microsoft
2020-03-21T09:12:21.6095879Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T09:12:21.6096280Z ==============================================================================
2020-03-21T09:12:21.9089982Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70214/merge to s
2020-03-21T09:12:21.9176665Z Cleaning up task key
2020-03-21T09:12:21.9177997Z Start cleaning up orphan processes.
2020-03-21T09:12:21.9314750Z ##[section]Finishing: Finalize Job
2020-03-21T09:12:21.9354949Z ##[section]Finishing: Linux x86_64-gnu-tools

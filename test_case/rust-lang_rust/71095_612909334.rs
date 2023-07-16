plain
2020-04-13T13:54:41.8997285Z ========================== Starting Command Output ===========================
2020-04-13T13:54:41.8999937Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/535deaea-3ee1-44d9-aef2-9173afb8bda2.sh
2020-04-13T13:54:41.9000294Z 
2020-04-13T13:54:41.9004164Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T13:54:41.9017900Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-13T13:54:41.9020196Z Task         : Get sources
2020-04-13T13:54:41.9020423Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T13:54:41.9020632Z Version      : 1.0.0
2020-04-13T13:54:41.9020772Z Author       : Microsoft
---
2020-04-13T13:54:42.8931128Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T13:54:42.8940694Z ##[command]git config gc.auto 0
2020-04-13T13:54:42.8946874Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T13:54:42.8949495Z ##[command]git config --get-all http.proxy
2020-04-13T13:54:42.8954381Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71095/merge:refs/remotes/pull/71095/merge
---
2020-04-13T13:56:55.2274844Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-13T13:56:55.2307058Z 
2020-04-13T13:56:55.2364247Z ##[error]Bash exited with code '100'.
2020-04-13T13:56:55.2375509Z ##[section]Finishing: Install awscli
2020-04-13T13:56:55.2421844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-13T13:56:55.2426499Z Task         : Get sources
2020-04-13T13:56:55.2426839Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T13:56:55.2427172Z Version      : 1.0.0
2020-04-13T13:56:55.2427396Z Author       : Microsoft
2020-04-13T13:56:55.2427396Z Author       : Microsoft
2020-04-13T13:56:55.2427741Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T13:56:55.2428275Z ==============================================================================
2020-04-13T13:56:55.4683852Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T13:56:55.4728911Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-13T13:56:55.4808998Z Cleaning up task key
2020-04-13T13:56:55.4810164Z Start cleaning up orphan processes.
2020-04-13T13:56:55.4945475Z Terminate orphan process: pid (3660) (python)
2020-04-13T13:56:55.5025959Z ##[section]Finishing: Finalize Job

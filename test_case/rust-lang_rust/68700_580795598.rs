plain
2020-01-31T16:02:41.8900972Z ========================== Starting Command Output ===========================
2020-01-31T16:02:41.8903816Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/45971c8c-4b52-4995-b981-095186d5626f.sh
2020-01-31T16:02:41.8904013Z 
2020-01-31T16:02:41.8907594Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T16:02:41.8914200Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:02:41.8915954Z Task         : Get sources
2020-01-31T16:02:41.8915989Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T16:02:41.8916021Z Version      : 1.0.0
2020-01-31T16:02:41.8916054Z Author       : Microsoft
---
2020-01-31T16:02:42.9375241Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T16:02:42.9386411Z ##[command]git config gc.auto 0
2020-01-31T16:02:42.9389141Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T16:02:42.9391118Z ##[command]git config --get-all http.proxy
2020-01-31T16:02:42.9397374Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68700/merge:refs/remotes/pull/68700/merge
---
2020-01-31T16:07:37.0944373Z 
2020-01-31T16:07:37.0979986Z error: incorrect close delimiter: `)`
2020-01-31T16:07:37.0980299Z   --> src/liballoc/task.rs:86:6
2020-01-31T16:07:37.0980566Z    |
2020-01-31T16:07:37.0980896Z 58 | fn raw_waker<W: Wake + Send + Sync + 'static>(waker: Arc<W>) -> RawWaker {
2020-01-31T16:07:37.0981288Z    |                                                                          - un-closed delimiter
2020-01-31T16:07:37.0981799Z 86 |     ))
2020-01-31T16:07:37.0982136Z    |      ^ incorrect close delimiter
2020-01-31T16:07:37.0986490Z 
2020-01-31T16:07:37.1049187Z error: aborting due to 2 previous errors
---
2020-01-31T16:07:38.4393165Z   local time: Fri Jan 31 16:07:38 UTC 2020
2020-01-31T16:07:38.7283554Z   network time: Fri, 31 Jan 2020 16:07:38 GMT
2020-01-31T16:07:38.7284506Z == end clock drift check ==
2020-01-31T16:07:39.7393634Z 
2020-01-31T16:07:39.7483734Z ##[error]Bash exited with code '1'.
2020-01-31T16:07:39.7498098Z ##[section]Finishing: Run build
2020-01-31T16:07:39.7516226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:07:39.7518505Z Task         : Get sources
2020-01-31T16:07:39.7518553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T16:07:39.7518617Z Version      : 1.0.0
2020-01-31T16:07:39.7518676Z Author       : Microsoft
2020-01-31T16:07:39.7518676Z Author       : Microsoft
2020-01-31T16:07:39.7518723Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-31T16:07:39.7518772Z ==============================================================================
2020-01-31T16:07:40.1682207Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-31T16:07:40.1723981Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:07:40.1836052Z Cleaning up task key
2020-01-31T16:07:40.1836988Z Start cleaning up orphan processes.
2020-01-31T16:07:40.1951712Z Terminate orphan process: pid (3967) (python)
2020-01-31T16:07:40.2150862Z ##[section]Finishing: Finalize Job

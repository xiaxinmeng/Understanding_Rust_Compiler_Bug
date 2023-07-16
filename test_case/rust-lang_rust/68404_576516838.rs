plain
2020-01-21T04:47:02.7620594Z ========================== Starting Command Output ===========================
2020-01-21T04:47:02.7622336Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e4ebb276-cf78-4f38-8657-2d818dd1abc0.sh
2020-01-21T04:47:02.7622380Z 
2020-01-21T04:47:02.7627735Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T04:47:02.7635815Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T04:47:02.7637968Z Task         : Get sources
2020-01-21T04:47:02.7638007Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T04:47:02.7638042Z Version      : 1.0.0
2020-01-21T04:47:02.7638140Z Author       : Microsoft
---
2020-01-21T04:47:03.7351117Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T04:47:03.7362676Z ##[command]git config gc.auto 0
2020-01-21T04:47:03.7365307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T04:47:03.7367518Z ##[command]git config --get-all http.proxy
2020-01-21T04:47:03.7374394Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
---
2020-01-21T04:50:01.8552767Z   local time: Tue Jan 21 04:50:01 UTC 2020
2020-01-21T04:50:01.9918106Z   network time: Tue, 21 Jan 2020 04:50:01 GMT
2020-01-21T04:50:01.9922643Z == end clock drift check ==
2020-01-21T04:50:10.0223003Z 
2020-01-21T04:50:10.0339216Z ##[error]Bash exited with code '1'.
2020-01-21T04:50:10.0354284Z ##[section]Finishing: Run build
2020-01-21T04:50:10.0373012Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T04:50:10.0375708Z Task         : Get sources
2020-01-21T04:50:10.0375762Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T04:50:10.0375838Z Version      : 1.0.0
2020-01-21T04:50:10.0375885Z Author       : Microsoft
2020-01-21T04:50:10.0375885Z Author       : Microsoft
2020-01-21T04:50:10.0375939Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T04:50:10.0376421Z ==============================================================================
2020-01-21T04:50:11.2869189Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T04:50:11.2888778Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T04:50:11.3053643Z Cleaning up task key
2020-01-21T04:50:11.3054885Z Start cleaning up orphan processes.
2020-01-21T04:50:11.3215588Z Terminate orphan process: pid (4760) (python)
2020-01-21T04:50:11.3461366Z ##[section]Finishing: Finalize Job

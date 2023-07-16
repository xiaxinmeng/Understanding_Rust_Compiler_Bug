plain
2020-02-12T16:25:16.9216445Z ========================== Starting Command Output ===========================
2020-02-12T16:25:16.9218698Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/03518e6f-d810-47ec-b3f8-9e9e16a0fbaf.sh
2020-02-12T16:25:16.9218871Z 
2020-02-12T16:25:16.9221968Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T16:25:16.9227699Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T16:25:16.9229670Z Task         : Get sources
2020-02-12T16:25:16.9229700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T16:25:16.9229741Z Version      : 1.0.0
2020-02-12T16:25:16.9229770Z Author       : Microsoft
---
2020-02-12T16:25:17.9164376Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T16:25:17.9174308Z ##[command]git config gc.auto 0
2020-02-12T16:25:17.9177090Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T16:25:17.9179704Z ##[command]git config --get-all http.proxy
2020-02-12T16:25:17.9186657Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69079/merge:refs/remotes/pull/69079/merge
---
2020-02-12T16:52:55.0887595Z   local time: Wed Feb 12 16:52:55 UTC 2020
2020-02-12T16:52:55.6422287Z   network time: Wed, 12 Feb 2020 16:52:55 GMT
2020-02-12T16:52:55.6425607Z == end clock drift check ==
2020-02-12T16:52:57.1865731Z 
2020-02-12T16:52:57.1962446Z ##[error]Bash exited with code '1'.
2020-02-12T16:52:57.1973106Z ##[section]Finishing: Run build
2020-02-12T16:52:57.1992105Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T16:52:57.1994242Z Task         : Get sources
2020-02-12T16:52:57.1994298Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T16:52:57.1994350Z Version      : 1.0.0
2020-02-12T16:52:57.1994384Z Author       : Microsoft
2020-02-12T16:52:57.1994384Z Author       : Microsoft
2020-02-12T16:52:57.1994440Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-12T16:52:57.1994480Z ==============================================================================
2020-02-12T16:52:57.5885695Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-12T16:52:57.5926750Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T16:52:57.6034298Z Cleaning up task key
2020-02-12T16:52:57.6035117Z Start cleaning up orphan processes.
2020-02-12T16:52:57.6134839Z Terminate orphan process: pid (3475) (python)
2020-02-12T16:52:57.6316521Z ##[section]Finishing: Finalize Job

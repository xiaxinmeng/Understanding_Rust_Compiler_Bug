plain
2020-03-19T20:25:51.3552705Z ========================== Starting Command Output ===========================
2020-03-19T20:25:51.3555962Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c6f292c2-bc30-4d40-be11-08d975edf61b.sh
2020-03-19T20:25:51.3556252Z 
2020-03-19T20:25:51.3560449Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T20:25:51.3579475Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70166/merge to s
2020-03-19T20:25:51.3582680Z Task         : Get sources
2020-03-19T20:25:51.3582987Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T20:25:51.3583287Z Version      : 1.0.0
2020-03-19T20:25:51.3583508Z Author       : Microsoft
---
2020-03-19T20:25:52.3528398Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T20:25:52.3533741Z ##[command]git config gc.auto 0
2020-03-19T20:25:52.3537370Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T20:25:52.3540470Z ##[command]git config --get-all http.proxy
2020-03-19T20:25:52.3547331Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70166/merge:refs/remotes/pull/70166/merge
---
2020-03-19T20:30:34.6756229Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-19T20:30:37.4606346Z error: unused import: `Hasher`
2020-03-19T20:30:37.4607326Z  --> src/libcore/ops/range.rs:2:25
2020-03-19T20:30:37.4608153Z   |
2020-03-19T20:30:37.4608938Z 2 | use crate::hash::{Hash, Hasher};
2020-03-19T20:30:37.4610403Z   |
2020-03-19T20:30:37.4611158Z   = note: `-D unused-imports` implied by `-D warnings`
2020-03-19T20:30:37.4611578Z 
2020-03-19T20:30:41.8147018Z    Compiling libc v0.2.66
---
2020-03-19T20:30:43.9431889Z   local time: Thu Mar 19 20:30:43 UTC 2020
2020-03-19T20:30:44.2355751Z   network time: Thu, 19 Mar 2020 20:30:44 GMT
2020-03-19T20:30:44.2358579Z == end clock drift check ==
2020-03-19T20:30:50.7882457Z 
2020-03-19T20:30:50.7954970Z ##[error]Bash exited with code '1'.
2020-03-19T20:30:50.7969298Z ##[section]Finishing: Run build
2020-03-19T20:30:50.8014055Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70166/merge to s
2020-03-19T20:30:50.8019368Z Task         : Get sources
2020-03-19T20:30:50.8019692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T20:30:50.8020017Z Version      : 1.0.0
2020-03-19T20:30:50.8020232Z Author       : Microsoft
2020-03-19T20:30:50.8020232Z Author       : Microsoft
2020-03-19T20:30:50.8020571Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T20:30:50.8021154Z ==============================================================================
2020-03-19T20:30:51.1241002Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T20:30:51.1285480Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70166/merge to s
2020-03-19T20:30:51.1368332Z Cleaning up task key
2020-03-19T20:30:51.1369688Z Start cleaning up orphan processes.
2020-03-19T20:30:51.1545289Z Terminate orphan process: pid (3406) (python)
2020-03-19T20:30:51.1725360Z ##[section]Finishing: Finalize Job

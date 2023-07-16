plain
2020-02-22T18:58:08.0800073Z ========================== Starting Command Output ===========================
2020-02-22T18:58:08.0802714Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8415089e-1344-4fdb-bc36-7e9f49265364.sh
2020-02-22T18:58:08.0802930Z 
2020-02-22T18:58:08.0806237Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T18:58:08.0825676Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69380/merge to s
2020-02-22T18:58:08.0829177Z Task         : Get sources
2020-02-22T18:58:08.0829510Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T18:58:08.0829758Z Version      : 1.0.0
2020-02-22T18:58:08.0829926Z Author       : Microsoft
---
2020-02-22T18:58:09.1061735Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T18:58:09.1065845Z ##[command]git config gc.auto 0
2020-02-22T18:58:09.1068627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T18:58:09.1071292Z ##[command]git config --get-all http.proxy
2020-02-22T18:58:09.1077370Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69380/merge:refs/remotes/pull/69380/merge
---
2020-02-22T19:07:17.2043551Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-02-22T19:07:19.8878647Z error[E0308]: mismatched types
2020-02-22T19:07:19.8879694Z    --> src/librustdoc/passes/collect_intra_doc_links.rs:352:29
2020-02-22T19:07:19.8880621Z     |
2020-02-22T19:07:19.8881456Z 352 |                 id if id != hir_id => Some(self.cx.as_local_hir_id(id).unwrap()),
2020-02-22T19:07:19.8882614Z     |                             ^^^^^^ expected struct `rustc_hir::def_id::DefId`, found struct `rustc_hir::HirId`
2020-02-22T19:07:20.0773844Z error: aborting due to previous error
2020-02-22T19:07:20.0774319Z 
2020-02-22T19:07:20.0774801Z For more information about this error, try `rustc --explain E0308`.
2020-02-22T19:07:20.0892182Z error: could not compile `rustdoc`.
---
2020-02-22T19:07:20.1001906Z   local time: Sat Feb 22 19:07:20 UTC 2020
2020-02-22T19:07:20.2524880Z   network time: Sat, 22 Feb 2020 19:07:20 GMT
2020-02-22T19:07:20.2528477Z == end clock drift check ==
2020-02-22T19:07:20.7676029Z 
2020-02-22T19:07:20.7739554Z ##[error]Bash exited with code '1'.
2020-02-22T19:07:20.7764253Z ##[section]Finishing: Run build
2020-02-22T19:07:20.7803097Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69380/merge to s
2020-02-22T19:07:20.7807719Z Task         : Get sources
2020-02-22T19:07:20.7808006Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T19:07:20.7808289Z Version      : 1.0.0
2020-02-22T19:07:20.7808472Z Author       : Microsoft
2020-02-22T19:07:20.7808472Z Author       : Microsoft
2020-02-22T19:07:20.7808768Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T19:07:20.7809127Z ==============================================================================
2020-02-22T19:07:21.0780290Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T19:07:21.0820140Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69380/merge to s
2020-02-22T19:07:21.0906678Z Cleaning up task key
2020-02-22T19:07:21.0907937Z Start cleaning up orphan processes.
2020-02-22T19:07:21.1060736Z Terminate orphan process: pid (6059) (python)
2020-02-22T19:07:21.1198583Z ##[section]Finishing: Finalize Job

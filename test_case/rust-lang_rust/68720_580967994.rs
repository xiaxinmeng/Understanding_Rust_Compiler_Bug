plain
2020-02-01T00:06:18.4515563Z ========================== Starting Command Output ===========================
2020-02-01T00:06:18.4516863Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/660ea01b-4eb0-4fa0-9509-37204b325c11.sh
2020-02-01T00:06:18.4516895Z 
2020-02-01T00:06:18.4520061Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T00:06:18.4525531Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68720/merge to s
2020-02-01T00:06:18.4527045Z Task         : Get sources
2020-02-01T00:06:18.4527092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T00:06:18.4527123Z Version      : 1.0.0
2020-02-01T00:06:18.4527153Z Author       : Microsoft
---
2020-02-01T00:06:19.3486710Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T00:06:19.3588768Z ##[command]git config gc.auto 0
2020-02-01T00:06:19.3666659Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T00:06:19.3715994Z ##[command]git config --get-all http.proxy
2020-02-01T00:06:19.3870153Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68720/merge:refs/remotes/pull/68720/merge
---
2020-02-01T00:09:00.9002111Z Looks like docker image is the same as before, not uploading
2020-02-01T00:09:01.9594394Z [CI_JOB_NAME=mingw-check]
2020-02-01T00:09:02.0009558Z [CI_JOB_NAME=mingw-check]
2020-02-01T00:09:02.0009775Z == clock drift check ==
2020-02-01T00:09:02.0009824Z   local time: Sat Feb  1 00:09:01 UTC 2020
2020-02-01T00:09:02.2874374Z   network time: Sat, 01 Feb 2020 00:09:02 GMT
2020-02-01T00:09:02.2904361Z Starting sccache server...
2020-02-01T00:09:02.3856222Z configure: processing command line
2020-02-01T00:09:02.3856814Z configure: 
2020-02-01T00:09:02.3857728Z configure: rust.parallel-compiler := True
---
2020-02-01T00:15:23.0652362Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-02-01T00:15:43.9665427Z error[E0609]: no field `no_parallel_llvm` on type `rustc_session::options::Options`
2020-02-01T00:15:43.9666965Z    --> src/librustc_codegen_llvm/llvm_util.rs:118:23
2020-02-01T00:15:43.9667750Z     |
2020-02-01T00:15:43.9668970Z 118 |         if !sess.opts.no_parallel_llvm {
2020-02-01T00:15:43.9670137Z     |
2020-02-01T00:15:43.9670137Z     |
2020-02-01T00:15:43.9670767Z     = note: available fields are: `crate_types`, `optimize`, `debug_assertions`, `debuginfo`, `lint_opts` ... and 25 others
2020-02-01T00:15:44.1358529Z error: aborting due to previous error
2020-02-01T00:15:44.1359854Z 
2020-02-01T00:15:44.1360954Z For more information about this error, try `rustc --explain E0609`.
2020-02-01T00:15:44.1526183Z error: could not compile `rustc_codegen_llvm`.
2020-02-01T00:15:44.1526183Z error: could not compile `rustc_codegen_llvm`.
2020-02-01T00:15:44.1526794Z 
2020-02-01T00:15:44.1527466Z To learn more, run the command again with --verbose.
2020-02-01T00:15:44.1560118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-01T00:15:44.1576717Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-01T00:15:44.1577472Z Build completed unsuccessfully in 0:06:41
2020-02-01T00:15:44.1637017Z == clock drift check ==
2020-02-01T00:15:44.1652510Z   local time: Sat Feb  1 00:15:44 UTC 2020
2020-02-01T00:15:44.1652510Z   local time: Sat Feb  1 00:15:44 UTC 2020
2020-02-01T00:15:44.3276873Z   network time: Sat, 01 Feb 2020 00:15:44 GMT
2020-02-01T00:15:44.7077964Z 
2020-02-01T00:15:44.7077964Z 
2020-02-01T00:15:44.7233472Z ##[error]Bash exited with code '1'.
2020-02-01T00:15:44.7245880Z ##[section]Finishing: Run build
2020-02-01T00:15:44.7268476Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68720/merge to s
2020-02-01T00:15:44.7270096Z Task         : Get sources
2020-02-01T00:15:44.7270148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T00:15:44.7270190Z Version      : 1.0.0
2020-02-01T00:15:44.7270226Z Author       : Microsoft
2020-02-01T00:15:44.7270226Z Author       : Microsoft
2020-02-01T00:15:44.7270279Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T00:15:44.7270325Z ==============================================================================
2020-02-01T00:15:45.1751400Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T00:15:45.1804186Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68720/merge to s
2020-02-01T00:15:45.1926046Z Cleaning up task key
2020-02-01T00:15:45.1926887Z Start cleaning up orphan processes.
2020-02-01T00:15:45.2047615Z Terminate orphan process: pid (4755) (python)
2020-02-01T00:15:45.3012171Z ##[section]Finishing: Finalize Job

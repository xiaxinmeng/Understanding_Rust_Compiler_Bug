plain
2020-03-16T00:29:02.8721044Z ========================== Starting Command Output ===========================
2020-03-16T00:29:02.8725937Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e323d16a-a5d0-4a92-8c80-b8dd996730b2.sh
2020-03-16T00:29:02.8726412Z 
2020-03-16T00:29:02.8732670Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T00:29:02.8749969Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69838/merge to s
2020-03-16T00:29:02.8753004Z Task         : Get sources
2020-03-16T00:29:02.8753276Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T00:29:02.8753518Z Version      : 1.0.0
2020-03-16T00:29:02.8753683Z Author       : Microsoft
---
2020-03-16T00:29:04.5003099Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T00:29:04.5010285Z ##[command]git config gc.auto 0
2020-03-16T00:29:04.5014500Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T00:29:04.5018346Z ##[command]git config --get-all http.proxy
2020-03-16T00:29:04.5026740Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69838/merge:refs/remotes/pull/69838/merge
---
2020-03-16T01:00:20.5829212Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-16T01:00:20.7167735Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-03-16T01:00:20.7521673Z    Compiling backtrace v0.3.44
2020-03-16T01:00:21.0581118Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-16T01:00:21.0920892Z error: lint `private_no_mangle_fns` has been removed: `no longer a warning, `#[no_mangle]` functions always exported`
2020-03-16T01:00:21.0921971Z   --> src/libpanic_unwind/gcc.rs:39:10
2020-03-16T01:00:21.0922428Z    |
2020-03-16T01:00:21.0922937Z 39 | #![allow(private_no_mangle_fns)]
2020-03-16T01:00:21.0924029Z    |
2020-03-16T01:00:21.0924596Z    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
2020-03-16T01:00:21.0979435Z 
2020-03-16T01:00:21.1508025Z error: aborting due to previous error
---
2020-03-16T01:00:21.2387790Z   local time: Mon Mar 16 01:00:21 UTC 2020
2020-03-16T01:00:21.4040759Z   network time: Mon, 16 Mar 2020 01:00:21 GMT
2020-03-16T01:00:21.4042064Z == end clock drift check ==
2020-03-16T01:00:23.8685962Z 
2020-03-16T01:00:23.8787524Z ##[error]Bash exited with code '1'.
2020-03-16T01:00:23.8804548Z ##[section]Finishing: Run build
2020-03-16T01:00:23.8860568Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69838/merge to s
2020-03-16T01:00:23.8867539Z Task         : Get sources
2020-03-16T01:00:23.8867932Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T01:00:23.8868292Z Version      : 1.0.0
2020-03-16T01:00:23.8868578Z Author       : Microsoft
2020-03-16T01:00:23.8868578Z Author       : Microsoft
2020-03-16T01:00:23.8868983Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T01:00:23.8869447Z ==============================================================================
2020-03-16T01:00:24.2585188Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T01:00:24.2637597Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69838/merge to s
2020-03-16T01:00:24.2728843Z Cleaning up task key
2020-03-16T01:00:24.2730024Z Start cleaning up orphan processes.
2020-03-16T01:00:24.2940418Z Terminate orphan process: pid (3835) (python)
2020-03-16T01:00:24.3280421Z ##[section]Finishing: Finalize Job

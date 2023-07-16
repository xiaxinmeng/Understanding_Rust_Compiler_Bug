plain
2020-03-08T01:18:05.9491753Z ========================== Starting Command Output ===========================
2020-03-08T01:18:05.9496561Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b6ca4dbe-8d45-48bd-82f7-5d0a613c55d5.sh
2020-03-08T01:18:05.9497073Z 
2020-03-08T01:18:05.9501785Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T01:18:05.9522283Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T01:18:05.9525846Z Task         : Get sources
2020-03-08T01:18:05.9526190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T01:18:05.9526506Z Version      : 1.0.0
2020-03-08T01:18:05.9526721Z Author       : Microsoft
---
2020-03-08T01:18:07.6133544Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T01:18:07.6144415Z ##[command]git config gc.auto 0
2020-03-08T01:18:07.6151440Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T01:18:07.6158522Z ##[command]git config --get-all http.proxy
2020-03-08T01:18:07.6169898Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69749/merge:refs/remotes/pull/69749/merge
---
2020-03-08T01:53:09.7572709Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-08T01:53:19.2236191Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-08T01:53:34.3797415Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-08T01:53:40.3343939Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-08T01:54:10.2336224Z error: internal compiler error: src/librustc/ty/subst.rs:565: type parameter `impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/#4` (impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/4) out of range when substituting (root type=Some(for<'r> fn(&'r mut rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>) {<rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]> as std::ops::Drop>::drop})) substs=[rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>]
2020-03-08T01:54:10.2355643Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-03-08T01:54:10.2361372Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-08T01:54:10.2361636Z 
2020-03-08T01:54:10.2367118Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-08T01:54:10.2367118Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-08T01:54:10.2367337Z 
2020-03-08T01:54:10.2368194Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-08T01:54:10.2369105Z note: rustc 1.43.0-nightly (7273cc48c 2020-03-08) running on x86_64-unknown-linux-gnu
2020-03-08T01:54:10.2369351Z 
2020-03-08T01:54:10.2369351Z 
2020-03-08T01:54:10.2370341Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-08T01:54:10.2372522Z note: some of the compiler flags provided by cargo are hidden
2020-03-08T01:54:10.2372727Z 
2020-03-08T01:54:10.4035863Z error: aborting due to previous error
2020-03-08T01:54:10.4036130Z 
---
2020-03-08T01:54:21.5524409Z   local time: Sun Mar  8 01:54:21 UTC 2020
2020-03-08T01:54:21.6107845Z   network time: Sun, 08 Mar 2020 01:54:21 GMT
2020-03-08T01:54:21.6111648Z == end clock drift check ==
2020-03-08T01:54:22.3211545Z 
2020-03-08T01:54:22.3285847Z ##[error]Bash exited with code '1'.
2020-03-08T01:54:22.3310410Z ##[section]Finishing: Run build
2020-03-08T01:54:22.3379230Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T01:54:22.3384284Z Task         : Get sources
2020-03-08T01:54:22.3384646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T01:54:22.3384982Z Version      : 1.0.0
2020-03-08T01:54:22.3385204Z Author       : Microsoft
2020-03-08T01:54:22.3385204Z Author       : Microsoft
2020-03-08T01:54:22.3385559Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T01:54:22.3386078Z ==============================================================================
2020-03-08T01:54:22.6836816Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T01:54:22.6881491Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T01:54:22.6978775Z Cleaning up task key
2020-03-08T01:54:22.6979915Z Start cleaning up orphan processes.
2020-03-08T01:54:22.7160107Z Terminate orphan process: pid (3634) (python)
2020-03-08T01:54:22.7374446Z ##[section]Finishing: Finalize Job

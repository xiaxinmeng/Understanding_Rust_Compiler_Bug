plain
2020-03-05T22:34:13.0317320Z ========================== Starting Command Output ===========================
2020-03-05T22:34:13.0321583Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/31357785-b4bc-47a2-847e-2fd2be15262f.sh
2020-03-05T22:34:13.0321809Z 
2020-03-05T22:34:13.0326209Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-05T22:34:13.0345976Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-05T22:34:13.0349278Z Task         : Get sources
2020-03-05T22:34:13.0349507Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T22:34:13.0349733Z Version      : 1.0.0
2020-03-05T22:34:13.0349928Z Author       : Microsoft
---
2020-03-05T22:34:14.3138820Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-05T22:34:14.3151502Z ##[command]git config gc.auto 0
2020-03-05T22:34:14.3160300Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-05T22:34:14.3167224Z ##[command]git config --get-all http.proxy
2020-03-05T22:34:14.3175227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69749/merge:refs/remotes/pull/69749/merge
---
2020-03-05T23:05:05.7454395Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-05T23:05:12.0380261Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-05T23:05:27.5010842Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-05T23:05:32.7331014Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-05T23:05:59.0153739Z error: internal compiler error: src/librustc/ty/subst.rs:565: type parameter `impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/#4` (impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/4) out of range when substituting (root type=Some(for<'r> fn(&'r mut rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>) {<rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]> as std::ops::Drop>::drop})) substs=[rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>]
2020-03-05T23:05:59.0160997Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-03-05T23:05:59.0161603Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-05T23:05:59.0164484Z 
2020-03-05T23:05:59.0169131Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-05T23:05:59.0169131Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-05T23:05:59.0171715Z 
2020-03-05T23:05:59.0174964Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-05T23:05:59.0181273Z note: rustc 1.43.0-nightly (22e8e7cc1 2020-03-05) running on x86_64-unknown-linux-gnu
2020-03-05T23:05:59.0185624Z 
2020-03-05T23:05:59.0185624Z 
2020-03-05T23:05:59.0189288Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-05T23:05:59.0195668Z note: some of the compiler flags provided by cargo are hidden
2020-03-05T23:05:59.0198792Z 
2020-03-05T23:05:59.1958176Z error: aborting due to previous error
2020-03-05T23:05:59.1959053Z 
---
2020-03-05T23:06:10.3653948Z   local time: Thu Mar  5 23:06:10 UTC 2020
2020-03-05T23:06:10.6380735Z   network time: Thu, 05 Mar 2020 23:06:10 GMT
2020-03-05T23:06:10.6386600Z == end clock drift check ==
2020-03-05T23:06:11.3375101Z 
2020-03-05T23:06:11.3452876Z ##[error]Bash exited with code '1'.
2020-03-05T23:06:11.3467014Z ##[section]Finishing: Run build
2020-03-05T23:06:11.3524793Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-05T23:06:11.3531238Z Task         : Get sources
2020-03-05T23:06:11.3531766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T23:06:11.3532233Z Version      : 1.0.0
2020-03-05T23:06:11.3532425Z Author       : Microsoft
2020-03-05T23:06:11.3532425Z Author       : Microsoft
2020-03-05T23:06:11.3532739Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-05T23:06:11.3533124Z ==============================================================================
2020-03-05T23:06:11.6908912Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-05T23:06:11.6966824Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-05T23:06:11.7054442Z Cleaning up task key
2020-03-05T23:06:11.7055450Z Start cleaning up orphan processes.
2020-03-05T23:06:11.7461779Z Terminate orphan process: pid (20029) (python)
2020-03-05T23:06:11.7487268Z ##[section]Finishing: Finalize Job

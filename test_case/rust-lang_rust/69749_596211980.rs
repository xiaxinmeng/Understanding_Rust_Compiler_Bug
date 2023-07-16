plain
2020-03-08T13:56:32.1616367Z ========================== Starting Command Output ===========================
2020-03-08T13:56:32.1634304Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/984a6282-90e8-408a-b8af-c1dca1e1d7d9.sh
2020-03-08T13:56:32.4024256Z 
2020-03-08T13:56:32.4112789Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T13:56:32.4150874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T13:56:32.4168071Z Task         : Get sources
2020-03-08T13:56:32.4168622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T13:56:32.4169181Z Version      : 1.0.0
2020-03-08T13:56:32.4169595Z Author       : Microsoft
---
2020-03-08T13:56:35.1231383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T13:56:35.1484328Z ##[command]git config gc.auto 0
2020-03-08T13:56:35.1544432Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T13:56:35.1560030Z ##[command]git config --get-all http.proxy
2020-03-08T13:56:35.1662659Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69749/merge:refs/remotes/pull/69749/merge
---
2020-03-08T14:31:49.1084382Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-08T14:31:58.5970642Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-08T14:32:12.9898221Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-08T14:32:18.8713276Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-08T14:32:48.3841371Z error: internal compiler error: src/librustc/ty/subst.rs:565: type parameter `impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/#4` (impl FnOnce(&mut StableHashingContext<'_>, &R) -> Option<Fingerprint>/4) out of range when substituting (root type=Some(for<'r> fn(&'r mut rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>) {<rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]> as std::ops::Drop>::drop})) substs=[rustc_data_structures::OnDrop<[closure@src/librustc/ty/context.rs:1702:29: 1702:65 old:usize]>]
2020-03-08T14:32:48.3895501Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-03-08T14:32:48.3896191Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-08T14:32:48.3896756Z 
2020-03-08T14:32:48.3897036Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-08T14:32:48.3897036Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-08T14:32:48.3898071Z 
2020-03-08T14:32:48.3899104Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-08T14:32:48.3900322Z note: rustc 1.43.0-nightly (ff8cd0073 2020-03-08) running on x86_64-unknown-linux-gnu
2020-03-08T14:32:48.3900852Z 
2020-03-08T14:32:48.3900852Z 
2020-03-08T14:32:48.3901967Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-08T14:32:48.3902794Z note: some of the compiler flags provided by cargo are hidden
2020-03-08T14:32:48.3903095Z 
2020-03-08T14:32:49.2182511Z error: aborting due to previous error
2020-03-08T14:32:49.2182759Z 
---
2020-03-08T14:32:59.4436036Z   local time: Sun Mar  8 14:32:59 UTC 2020
2020-03-08T14:32:59.7296691Z   network time: Sun, 08 Mar 2020 14:32:59 GMT
2020-03-08T14:32:59.7300838Z == end clock drift check ==
2020-03-08T14:33:00.4737888Z 
2020-03-08T14:33:00.4835157Z ##[error]Bash exited with code '1'.
2020-03-08T14:33:00.4857977Z ##[section]Finishing: Run build
2020-03-08T14:33:00.4916980Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T14:33:00.4922885Z Task         : Get sources
2020-03-08T14:33:00.4923250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T14:33:00.4923564Z Version      : 1.0.0
2020-03-08T14:33:00.4923785Z Author       : Microsoft
2020-03-08T14:33:00.4923785Z Author       : Microsoft
2020-03-08T14:33:00.4924154Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T14:33:00.4924559Z ==============================================================================
2020-03-08T14:33:00.8550270Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T14:33:00.8598415Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69749/merge to s
2020-03-08T14:33:00.8703523Z Cleaning up task key
2020-03-08T14:33:00.8704830Z Start cleaning up orphan processes.
2020-03-08T14:33:00.8899135Z Terminate orphan process: pid (4112) (python)
2020-03-08T14:33:00.9280048Z ##[section]Finishing: Finalize Job

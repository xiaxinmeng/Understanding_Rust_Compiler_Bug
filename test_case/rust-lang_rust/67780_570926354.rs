plain
2020-01-05T15:57:40.8847512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T15:57:40.9074951Z ##[command]git config gc.auto 0
2020-01-05T15:57:40.9140058Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T15:57:40.9203369Z ##[command]git config --get-all http.proxy
2020-01-05T15:57:40.9351315Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67780/merge:refs/remotes/pull/67780/merge
---
2020-01-05T16:24:01.2867426Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-01-05T16:24:01.2886861Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-05T16:24:01.6879880Z    Compiling cc v1.0.47
2020-01-05T16:24:01.6880214Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-01-05T16:24:06.4571088Z error: internal compiler error: src/librustc/ty/query/mod.rs:96: `tcx.associated_item_def_ids(DefId(0:5925 ~ core[a1e0]::fmt[0]::Debug[0]))` unsupported by its crate
2020-01-05T16:24:06.4572342Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
2020-01-05T16:24:06.4572439Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T16:24:06.4572626Z 
2020-01-05T16:24:06.4577822Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-05T16:24:06.4577822Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-05T16:24:06.4577956Z 
2020-01-05T16:24:06.4579060Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-05T16:24:06.4579329Z 
2020-01-05T16:24:06.4580848Z note: rustc 1.42.0-nightly (179694ba8 2020-01-05) running on x86_64-unknown-linux-gnu
2020-01-05T16:24:06.4580895Z 
2020-01-05T16:24:06.4581420Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-05T16:24:06.4581549Z note: some of the compiler flags provided by cargo are hidden
2020-01-05T16:24:06.4581581Z 
2020-01-05T16:24:06.4851204Z error: aborting due to previous error
2020-01-05T16:24:06.4851436Z 
---
2020-01-05T16:24:07.7672224Z   local time: Sun Jan  5 16:24:07 UTC 2020
2020-01-05T16:24:08.1820397Z   network time: Sun, 05 Jan 2020 16:24:08 GMT
2020-01-05T16:24:08.1831333Z == end clock drift check ==
2020-01-05T16:24:10.4063622Z 
2020-01-05T16:24:10.4161739Z ##[error]Bash exited with code '1'.
2020-01-05T16:24:10.4203578Z ##[section]Starting: Checkout
2020-01-05T16:24:10.4205759Z ==============================================================================
2020-01-05T16:24:10.4205820Z Task         : Get sources
2020-01-05T16:24:10.4205868Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

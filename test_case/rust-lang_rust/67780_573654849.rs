plain
2020-01-13T12:42:20.1002348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T12:42:20.1108938Z ##[command]git config gc.auto 0
2020-01-13T12:42:20.1164327Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T12:42:20.1236961Z ##[command]git config --get-all http.proxy
2020-01-13T12:42:20.1401045Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67780/merge:refs/remotes/pull/67780/merge
---
2020-01-13T13:08:55.3577288Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-01-13T13:08:55.3612431Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-13T13:08:55.6482075Z    Compiling cc v1.0.49
2020-01-13T13:08:55.6521563Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-01-13T13:09:00.3138950Z error: internal compiler error: src/librustc/ty/query/mod.rs:99: `tcx.associated_item_def_ids(DefId(0:5917 ~ core[6099]::fmt[0]::Debug[0]))` unsupported by its crate
2020-01-13T13:09:00.3140422Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:872:9
2020-01-13T13:09:00.3140854Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-13T13:09:00.3141088Z 
2020-01-13T13:09:00.3144603Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-13T13:09:00.3144603Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-13T13:09:00.3146771Z 
2020-01-13T13:09:00.3147956Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-13T13:09:00.3148232Z 
2020-01-13T13:09:00.3148892Z note: rustc 1.42.0-nightly (d4d5b3ad7 2020-01-13) running on x86_64-unknown-linux-gnu
2020-01-13T13:09:00.3149092Z 
2020-01-13T13:09:00.3149750Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-13T13:09:00.3153404Z note: some of the compiler flags provided by cargo are hidden
2020-01-13T13:09:00.3153732Z 
2020-01-13T13:09:00.3403081Z error: aborting due to previous error
2020-01-13T13:09:00.3403761Z 
---
2020-01-13T13:09:01.8252248Z   local time: Mon Jan 13 13:09:01 UTC 2020
2020-01-13T13:09:02.1205336Z   network time: Mon, 13 Jan 2020 13:09:02 GMT
2020-01-13T13:09:02.1205453Z == end clock drift check ==
2020-01-13T13:09:03.6562252Z 
2020-01-13T13:09:03.6670261Z ##[error]Bash exited with code '1'.
2020-01-13T13:09:03.6925005Z ##[section]Starting: Checkout
2020-01-13T13:09:03.6927267Z ==============================================================================
2020-01-13T13:09:03.6927331Z Task         : Get sources
2020-01-13T13:09:03.6927384Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

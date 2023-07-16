plain
2019-09-07T19:03:13.3436563Z [RUSTC-TIMING] cmake test:false 3.998
2019-09-07T19:03:13.3476284Z    Compiling compiler_builtins v0.1.18
2019-09-07T19:03:16.4180781Z [RUSTC-TIMING] build_script_build test:false 3.064
2019-09-07T19:03:16.4214505Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-09-07T19:03:16.6846620Z error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4038), local_id: 61 }) with HirId::owner DefId(0:4038 ~ core[dca0]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4034 ~ core[dca0]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])
2019-09-07T19:03:16.6847313Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-09-07T19:03:16.6847434Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-07T19:03:16.8476406Z error: aborting due to previous error
2019-09-07T19:03:16.8476624Z 
2019-09-07T19:03:16.8476624Z 
2019-09-07T19:03:16.9414430Z 
2019-09-07T19:03:16.9414900Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-07T19:03:16.9419911Z 
2019-09-07T19:03:16.9421742Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-09-07T19:03:16.9431241Z 
2019-09-07T19:03:16.9432000Z note: rustc 1.39.0-nightly (1c025bbf2 2019-09-07) running on x86_64-unknown-linux-gnu
2019-09-07T19:03:16.9432070Z 
2019-09-07T19:03:16.9432630Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C linker=clang -C debuginfo=1 -C prefer-dynamic -C linker=clang -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-09-07T19:03:16.9432885Z note: some of the compiler flags provided by cargo are hidden
2019-09-07T19:03:16.9432954Z 
2019-09-07T19:03:16.9845731Z [RUSTC-TIMING] core test:false 21.083
2019-09-07T19:03:16.9846186Z error: Could not compile `core`.
---
2019-09-07T19:03:17.5871203Z == clock drift check ==
2019-09-07T19:03:17.5892387Z   local time: Sat Sep  7 19:03:17 UTC 2019
2019-09-07T19:03:17.8673895Z   network time: Sat, 07 Sep 2019 19:03:17 GMT
2019-09-07T19:03:17.8675888Z == end clock drift check ==
2019-09-07T19:03:21.9005854Z ##[error]Bash exited with code '1'.
2019-09-07T19:03:21.9047804Z ##[section]Starting: Upload CPU usage statistics
2019-09-07T19:03:21.9053339Z ==============================================================================
2019-09-07T19:03:21.9053462Z Task         : Bash
2019-09-07T19:03:21.9053542Z Description  : Run a Bash script on macOS, Linux, or Windows

plain
2020-01-11T22:23:04.9229154Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T22:23:04.9241348Z ##[command]git config gc.auto 0
2020-01-11T22:23:04.9243900Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T22:23:04.9246017Z ##[command]git config --get-all http.proxy
2020-01-11T22:23:04.9248670Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67681/merge:refs/remotes/pull/67681/merge
---
2020-01-11T23:06:01.3849661Z 594 | |         })
2020-01-11T23:06:01.3849945Z 595 | |     }
2020-01-11T23:06:01.3850189Z     | |_____^
2020-01-11T23:06:01.3854401Z 
2020-01-11T23:06:01.3862386Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:346:17
2020-01-11T23:06:01.3872324Z 
2020-01-11T23:06:01.3878189Z error: internal compiler error: unexpected panic
2020-01-11T23:06:01.3882048Z 
2020-01-11T23:06:01.3931512Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-11T23:06:01.3931512Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-11T23:06:01.3931568Z 
2020-01-11T23:06:01.3932251Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-11T23:06:01.3932316Z 
2020-01-11T23:06:01.3932637Z note: rustc 1.42.0-nightly (1bdc742a3 2020-01-11) running on x86_64-unknown-linux-gnu
2020-01-11T23:06:01.3932673Z 
2020-01-11T23:06:01.3933133Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-11T23:06:01.3933262Z note: some of the compiler flags provided by cargo are hidden
2020-01-11T23:06:01.3933295Z 
2020-01-11T23:06:01.4467995Z error: could not compile `rustc`.
2020-01-11T23:06:01.4471299Z warning: build failed, waiting for other jobs to finish...
---
2020-01-11T23:08:46.0325432Z   local time: Sat Jan 11 23:08:46 UTC 2020
2020-01-11T23:08:46.3268921Z   network time: Sat, 11 Jan 2020 23:08:46 GMT
2020-01-11T23:08:46.3270528Z == end clock drift check ==
2020-01-11T23:08:47.2925811Z 
2020-01-11T23:08:47.3042653Z ##[error]Bash exited with code '1'.
2020-01-11T23:08:47.3080049Z ##[section]Starting: Checkout
2020-01-11T23:08:47.3081984Z ==============================================================================
2020-01-11T23:08:47.3082047Z Task         : Get sources
2020-01-11T23:08:47.3082117Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

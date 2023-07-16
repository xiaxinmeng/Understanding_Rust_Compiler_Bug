plain
2019-12-23T04:35:26.3625144Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T04:35:26.3778549Z ##[command]git config gc.auto 0
2019-12-23T04:35:26.3865312Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T04:35:26.3925268Z ##[command]git config --get-all http.proxy
2019-12-23T04:35:26.4077803Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65989/merge:refs/remotes/pull/65989/merge
---
2019-12-23T05:03:59.1955567Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-12-23T05:03:59.1956091Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-12-23T05:03:59.5006915Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-12-23T05:04:00.2169666Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-12-23T05:04:00.2169913Z   left: `UserFacing`,
2019-12-23T05:04:00.2170247Z  right: `All`', src/librustc_mir/util/elaborate_drops.rs:205:13
2019-12-23T05:04:00.2170344Z 
2019-12-23T05:04:00.2170390Z error: internal compiler error: unexpected panic
2019-12-23T05:04:00.2170440Z 
2019-12-23T05:04:00.2170487Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-23T05:04:00.2170487Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-23T05:04:00.2170518Z 
2019-12-23T05:04:00.2171099Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-23T05:04:00.2171145Z 
2019-12-23T05:04:00.2171376Z note: rustc 1.42.0-nightly (28b34ef01 2019-12-22) running on x86_64-unknown-linux-gnu
2019-12-23T05:04:00.2171404Z 
2019-12-23T05:04:00.2171972Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-12-23T05:04:00.2172057Z note: some of the compiler flags provided by cargo are hidden
2019-12-23T05:04:00.2172100Z 
2019-12-23T05:04:01.1965293Z error: could not compile `core`.
2019-12-23T05:04:01.1966373Z warning: build failed, waiting for other jobs to finish...
---
2019-12-23T05:04:04.9772320Z   local time: Mon Dec 23 05:04:04 UTC 2019
2019-12-23T05:04:05.1351661Z   network time: Mon, 23 Dec 2019 05:04:05 GMT
2019-12-23T05:04:05.1355756Z == end clock drift check ==
2019-12-23T05:04:08.1424932Z 
2019-12-23T05:04:08.1525776Z ##[error]Bash exited with code '1'.
2019-12-23T05:04:08.1561036Z ##[section]Starting: Checkout
2019-12-23T05:04:08.1562735Z ==============================================================================
2019-12-23T05:04:08.1562805Z Task         : Get sources
2019-12-23T05:04:08.1562849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-10-26T02:48:59.5443478Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T02:48:59.5663439Z ##[command]git config gc.auto 0
2019-10-26T02:48:59.5761243Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T02:48:59.5843606Z ##[command]git config --get-all http.proxy
2019-10-26T02:48:59.5997043Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65083/merge:refs/remotes/pull/65083/merge
---
2019-10-26T03:20:49.0774287Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-26T03:20:49.2335575Z    Compiling backtrace v0.3.37
2019-10-26T03:20:49.8368338Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-10-26T03:20:50.0332944Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-26T03:21:11.8940876Z error: internal compiler error: encountered unmarked API: DefId(11:1405 ~ hashbrown[a991]::map[0]::HashMap[0]::S[0])
2019-10-26T03:21:11.8942833Z    --> src/libstd/collections/hash/map.rs:203:31
2019-10-26T03:21:11.8944437Z 203 |     base: base::HashMap<K, V, S>,
2019-10-26T03:21:11.8945133Z     |                               ^
2019-10-26T03:21:11.8951860Z 
2019-10-26T03:21:11.8951860Z 
2019-10-26T03:21:11.8978563Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:391:17
2019-10-26T03:21:11.8978762Z 
2019-10-26T03:21:11.8978814Z error: internal compiler error: unexpected panic
2019-10-26T03:21:11.8978849Z 
2019-10-26T03:21:11.8978915Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-26T03:21:11.8978915Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-26T03:21:11.8978951Z 
2019-10-26T03:21:11.8979461Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-26T03:21:11.8979821Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-26T03:21:11.8979859Z 
2019-10-26T03:21:11.8979859Z 
2019-10-26T03:21:11.8980370Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type dylib --crate-type rlib
2019-10-26T03:21:11.8980506Z note: some of the compiler flags provided by cargo are hidden
2019-10-26T03:21:11.8980540Z 
2019-10-26T03:21:11.9230044Z error: could not compile `std`.
2019-10-26T03:21:11.9230205Z 
---
2019-10-26T03:21:11.9352911Z   local time: Sat Oct 26 03:21:11 UTC 2019
2019-10-26T03:21:12.1000381Z   network time: Sat, 26 Oct 2019 03:21:12 GMT
2019-10-26T03:21:12.1001928Z == end clock drift check ==
2019-10-26T03:21:14.6664326Z 
2019-10-26T03:21:14.6775526Z ##[error]Bash exited with code '1'.
2019-10-26T03:21:14.6817847Z ##[section]Starting: Checkout
2019-10-26T03:21:14.6819943Z ==============================================================================
2019-10-26T03:21:14.6820001Z Task         : Get sources
2019-10-26T03:21:14.6820050Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-10-09T22:36:08.1008699Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T22:36:08.1164586Z ##[command]git config gc.auto 0
2019-10-09T22:36:08.1290322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T22:36:08.1405392Z ##[command]git config --get-all http.proxy
2019-10-09T22:36:08.1638606Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65083/merge:refs/remotes/pull/65083/merge
---
2019-10-09T23:06:45.2404702Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-09T23:06:45.3945961Z    Compiling backtrace v0.3.37
2019-10-09T23:06:45.9325189Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-10-09T23:06:46.0185225Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-09T23:07:07.2498113Z error: internal compiler error: encountered unmarked API: DefId(11:1404 ~ hashbrown[d725]::map[0]::HashMap[0]::S[0])
2019-10-09T23:07:07.2507295Z    --> src/libstd/collections/hash/map.rs:203:31
2019-10-09T23:07:07.2509523Z 203 |     base: base::HashMap<K, V, S>,
2019-10-09T23:07:07.2510524Z     |                               ^
2019-10-09T23:07:07.2511192Z 
2019-10-09T23:07:07.2511192Z 
2019-10-09T23:07:07.2512000Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:383:17
2019-10-09T23:07:07.2512828Z 
2019-10-09T23:07:07.2513326Z error: internal compiler error: unexpected panic
2019-10-09T23:07:07.2513575Z 
2019-10-09T23:07:07.2514419Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-09T23:07:07.2514419Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-09T23:07:07.2514725Z 
2019-10-09T23:07:07.2515900Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-09T23:07:07.2517181Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-09T23:07:07.2517660Z 
2019-10-09T23:07:07.2517660Z 
2019-10-09T23:07:07.2518628Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type dylib --crate-type rlib
2019-10-09T23:07:07.2519434Z note: some of the compiler flags provided by cargo are hidden
2019-10-09T23:07:07.2519912Z 
2019-10-09T23:07:07.2677379Z error: could not compile `std`.
2019-10-09T23:07:07.2678146Z 
---
2019-10-09T23:07:07.2753273Z == clock drift check ==
2019-10-09T23:07:07.2771791Z   local time: Wed Oct  9 23:07:07 UTC 2019
2019-10-09T23:07:07.4276879Z   network time: Wed, 09 Oct 2019 23:07:07 GMT
2019-10-09T23:07:07.4280039Z == end clock drift check ==
2019-10-09T23:07:09.1637999Z ##[error]Bash exited with code '1'.
2019-10-09T23:07:09.1689817Z ##[section]Starting: Checkout
2019-10-09T23:07:09.1691716Z ==============================================================================
2019-10-09T23:07:09.1691772Z Task         : Get sources
2019-10-09T23:07:09.1691837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

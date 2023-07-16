plain
2019-11-11T10:51:08.6865004Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T10:51:08.7056706Z ##[command]git config gc.auto 0
2019-11-11T10:51:08.7138200Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T10:51:08.7210035Z ##[command]git config --get-all http.proxy
2019-11-11T10:51:08.7368315Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66288/merge:refs/remotes/pull/66288/merge
---
2019-11-11T11:01:02.6107524Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-11-11T11:01:11.0259079Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-11T11:01:12.6238290Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-11T11:01:31.3828063Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-11T11:02:10.9104829Z error[E0599]: no method named `record_instant_event` found for type `measureme::profiler::Profiler<measureme::mmap_serialization_sink::MmapSerializationSink>` in the current scope
2019-11-11T11:02:10.9106437Z    --> src/librustc/util/profiling.rs:248:31
2019-11-11T11:02:10.9107133Z     |
2019-11-11T11:02:10.9108472Z 248 |             profiler.profiler.record_instant_event(
2019-11-11T11:02:10.9109242Z     |                               ^^^^^^^^^^^^^^^^^^^^ method not found in `measureme::profiler::Profiler<measureme::mmap_serialization_sink::MmapSerializationSink>`
2019-11-11T11:02:11.2477002Z error: aborting due to previous error
2019-11-11T11:02:11.2477116Z 
2019-11-11T11:02:11.2477859Z For more information about this error, try `rustc --explain E0599`.
2019-11-11T11:02:11.4140611Z error: could not compile `rustc`.
---
2019-11-11T11:02:17.3269584Z   local time: Mon Nov 11 11:02:17 UTC 2019
2019-11-11T11:02:17.6045578Z   network time: Mon, 11 Nov 2019 11:02:17 GMT
2019-11-11T11:02:17.6045685Z == end clock drift check ==
2019-11-11T11:02:18.5504844Z 
2019-11-11T11:02:18.5618061Z ##[error]Bash exited with code '1'.
2019-11-11T11:02:18.5646864Z ##[section]Starting: Checkout
2019-11-11T11:02:18.5648458Z ==============================================================================
2019-11-11T11:02:18.5648644Z Task         : Get sources
2019-11-11T11:02:18.5648707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

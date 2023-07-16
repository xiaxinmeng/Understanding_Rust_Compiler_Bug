plain
2019-11-24T00:17:09.8970849Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T00:17:10.6131138Z ##[command]git config gc.auto 0
2019-11-24T00:17:10.6137374Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T00:17:10.6140066Z ##[command]git config --get-all http.proxy
2019-11-24T00:17:10.6142363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66683/merge:refs/remotes/pull/66683/merge
---
2019-11-24T00:23:03.2629521Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-11-24T00:23:03.6537236Z    Compiling cmake v0.1.38
2019-11-24T00:23:05.5571117Z    Compiling compiler_builtins v0.1.18
2019-11-24T00:23:07.2104324Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-24T00:23:07.4217751Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-11-24T00:23:07.4218110Z    --> src/libcore/time.rs:516:30
2019-11-24T00:23:07.4218492Z     |
2019-11-24T00:23:07.4218805Z 516 |         (self.secs as f64) + (self.nanos as f64) / (NANOS_PER_SEC as f64)
2019-11-24T00:23:07.4219271Z     |
2019-11-24T00:23:07.4219271Z     |
2019-11-24T00:23:07.4220001Z     = note: for more information, see issue ***/issues/57563
2019-11-24T00:23:07.4220250Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-24T00:23:07.4220284Z 
2019-11-24T00:23:07.4256815Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-11-24T00:23:07.4257092Z    --> src/libcore/time.rs:533:30
2019-11-24T00:23:07.4257303Z     |
2019-11-24T00:23:07.4257569Z 533 |         (self.secs as f32) + (self.nanos as f32) / (NANOS_PER_SEC as f32)
2019-11-24T00:23:07.4258031Z     |
2019-11-24T00:23:07.4258031Z     |
2019-11-24T00:23:07.4258880Z     = note: for more information, see issue ***/issues/57563
2019-11-24T00:23:07.4259162Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-24T00:23:08.0461635Z    Compiling backtrace-sys v0.1.32
2019-11-24T00:23:08.6252797Z error: aborting due to 2 previous errors
2019-11-24T00:23:08.6254220Z 
2019-11-24T00:23:08.6263350Z For more information about this error, try `rustc --explain E0723`.
---
2019-11-24T00:23:08.9050928Z   local time: Sun Nov 24 00:23:08 UTC 2019
2019-11-24T00:23:09.4367181Z   network time: Sun, 24 Nov 2019 00:23:09 GMT
2019-11-24T00:23:09.4368978Z == end clock drift check ==
2019-11-24T00:23:11.1147561Z 
2019-11-24T00:23:11.1239990Z ##[error]Bash exited with code '1'.
2019-11-24T00:23:11.1263232Z ##[section]Starting: Checkout
2019-11-24T00:23:11.1265638Z ==============================================================================
2019-11-24T00:23:11.1265691Z Task         : Get sources
2019-11-24T00:23:11.1265752Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

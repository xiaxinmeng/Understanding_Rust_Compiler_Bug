plain
2019-09-23T04:17:57.9332535Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-23T04:17:57.9508127Z ##[command]git config gc.auto 0
2019-09-23T04:17:57.9579611Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-23T04:17:58.6557923Z ##[command]git config --get-all http.proxy
2019-09-23T04:17:58.6564483Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64600/merge:refs/remotes/pull/64600/merge
---
2019-09-23T04:24:01.4772415Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-09-23T04:24:04.3260088Z error: unused import: `Try`
2019-09-23T04:24:04.3260685Z   --> src/libcore/slice/mod.rs:31:25
2019-09-23T04:24:04.3261338Z    |
2019-09-23T04:24:04.3261948Z 31 | use crate::ops::{FnMut, Try, self};
2019-09-23T04:24:04.3262524Z    |
2019-09-23T04:24:04.3262802Z    = note: `-D unused-imports` implied by `-D warnings`
2019-09-23T04:24:04.3262889Z 
2019-09-23T04:24:10.3420841Z    Compiling libc v0.2.62
---
2019-09-23T04:24:12.9117615Z == clock drift check ==
2019-09-23T04:24:12.9132669Z   local time: Mon Sep 23 04:24:12 UTC 2019
2019-09-23T04:24:12.9984868Z   network time: Mon, 23 Sep 2019 04:24:12 GMT
2019-09-23T04:24:12.9990300Z == end clock drift check ==
2019-09-23T04:24:26.2030889Z ##[error]Bash exited with code '1'.
2019-09-23T04:24:26.2065868Z ##[section]Starting: Checkout
2019-09-23T04:24:26.2067691Z ==============================================================================
2019-09-23T04:24:26.2067734Z Task         : Get sources
2019-09-23T04:24:26.2067789Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

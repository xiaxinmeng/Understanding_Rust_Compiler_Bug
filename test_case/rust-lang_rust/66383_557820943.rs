plain
2019-11-23T18:01:57.0453333Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T18:01:57.0646527Z ##[command]git config gc.auto 0
2019-11-23T18:01:57.0715217Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T18:01:57.0799968Z ##[command]git config --get-all http.proxy
2019-11-23T18:01:57.0957980Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66383/merge:refs/remotes/pull/66383/merge
---
2019-11-23T18:07:44.4064071Z   = note: `#[warn(unused_imports)]` on by default
2019-11-23T18:07:44.4069502Z 
2019-11-23T18:07:55.5105651Z     Finished release [optimized] target(s) in 1m 28s
2019-11-23T18:07:55.5214050Z tidy check
2019-11-23T18:07:55.6670989Z tidy error: /checkout/src/liballoc/benches/vec.rs:614: line longer than 100 chars
2019-11-23T18:07:56.4768237Z tidy error: /checkout/src/libcore/iter/adapters/mod.rs:1014: line longer than 100 chars
2019-11-23T18:07:56.4768472Z tidy error: /checkout/src/libcore/iter/adapters/mod.rs:1149: line longer than 100 chars
2019-11-23T18:07:56.4768581Z tidy error: /checkout/src/libcore/iter/adapters/mod.rs:1689: line longer than 100 chars
2019-11-23T18:07:56.4768635Z tidy error: /checkout/src/libcore/iter/adapters/mod.rs:1801: line longer than 100 chars
2019-11-23T18:07:58.3121989Z some tidy checks failed
2019-11-23T18:07:58.3125289Z Found 485 error codes
2019-11-23T18:07:58.3125654Z Found 0 error codes with no tests
2019-11-23T18:07:58.3125780Z Done!
2019-11-23T18:07:58.3125780Z Done!
2019-11-23T18:07:58.3126087Z 
2019-11-23T18:07:58.3126180Z 
2019-11-23T18:07:58.3127200Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-23T18:07:58.3127803Z 
2019-11-23T18:07:58.3127845Z 
2019-11-23T18:07:58.3134373Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-23T18:07:58.3134840Z Build completed unsuccessfully in 0:01:32
2019-11-23T18:07:58.3134840Z Build completed unsuccessfully in 0:01:32
2019-11-23T18:07:58.3189293Z == clock drift check ==
2019-11-23T18:07:58.3200149Z   local time: Sat Nov 23 18:07:58 UTC 2019
2019-11-23T18:07:58.4689104Z   network time: Sat, 23 Nov 2019 18:07:58 GMT
2019-11-23T18:07:58.4693205Z == end clock drift check ==
2019-11-23T18:07:59.8010741Z 
2019-11-23T18:07:59.8125808Z ##[error]Bash exited with code '1'.
2019-11-23T18:07:59.8155095Z ##[section]Starting: Checkout
2019-11-23T18:07:59.8156450Z ==============================================================================
2019-11-23T18:07:59.8156655Z Task         : Get sources
2019-11-23T18:07:59.8156710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

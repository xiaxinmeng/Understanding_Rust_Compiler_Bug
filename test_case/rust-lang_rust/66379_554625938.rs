plain
2019-11-16T10:30:18.1945459Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T10:30:18.2147862Z ##[command]git config gc.auto 0
2019-11-16T10:30:18.2205594Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T10:30:18.2262576Z ##[command]git config --get-all http.proxy
2019-11-16T10:30:18.2373409Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-16T10:35:42.2485841Z    Compiling serde_json v1.0.40
2019-11-16T10:35:43.6540573Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-16T10:35:52.5728511Z     Finished release [optimized] target(s) in 1m 11s
2019-11-16T10:35:52.5807813Z tidy check
2019-11-16T10:35:53.2798262Z tidy error: /checkout/src/libcore/ptr/mod.rs:1077: trailing whitespace
2019-11-16T10:35:53.2798368Z tidy error: /checkout/src/libcore/ptr/mod.rs:1080: trailing whitespace
2019-11-16T10:35:53.2798443Z tidy error: /checkout/src/libcore/ptr/mod.rs:1088: trailing whitespace
2019-11-16T10:35:53.2798501Z tidy error: /checkout/src/libcore/ptr/mod.rs:1089: trailing whitespace
2019-11-16T10:35:53.2798543Z tidy error: /checkout/src/libcore/ptr/mod.rs:1092: trailing whitespace
2019-11-16T10:35:53.2798584Z tidy error: /checkout/src/libcore/ptr/mod.rs:1093: trailing whitespace
2019-11-16T10:35:53.2803805Z tidy error: /checkout/src/libcore/ptr/mod.rs:1941: trailing whitespace
2019-11-16T10:35:53.2803883Z tidy error: /checkout/src/libcore/ptr/mod.rs:1949: trailing whitespace
2019-11-16T10:35:53.2803927Z tidy error: /checkout/src/libcore/ptr/mod.rs:1950: trailing whitespace
2019-11-16T10:35:53.2803999Z tidy error: /checkout/src/libcore/ptr/mod.rs:1951: line longer than 100 chars
2019-11-16T10:35:53.2804046Z tidy error: /checkout/src/libcore/ptr/mod.rs:1953: trailing whitespace
2019-11-16T10:35:53.2804091Z tidy error: /checkout/src/libcore/ptr/mod.rs:1954: trailing whitespace
2019-11-16T10:35:54.5806751Z Found 441 error codes
2019-11-16T10:35:54.5807316Z Found 0 error codes with no tests
2019-11-16T10:35:54.5807542Z Done!
2019-11-16T10:35:54.5807683Z some tidy checks failed
2019-11-16T10:35:54.5807683Z some tidy checks failed
2019-11-16T10:35:54.5807855Z 
2019-11-16T10:35:54.5807953Z 
2019-11-16T10:35:54.5808876Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-16T10:35:54.5809215Z 
2019-11-16T10:35:54.5809303Z 
2019-11-16T10:35:54.5811902Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-16T10:35:54.5812095Z Build completed unsuccessfully in 0:01:14
2019-11-16T10:35:54.5812095Z Build completed unsuccessfully in 0:01:14
2019-11-16T10:35:54.5857010Z == clock drift check ==
2019-11-16T10:35:54.5867380Z   local time: Sat Nov 16 10:35:54 UTC 2019
2019-11-16T10:35:54.8601603Z   network time: Sat, 16 Nov 2019 10:35:54 GMT
2019-11-16T10:35:54.8603561Z == end clock drift check ==
2019-11-16T10:35:56.2615024Z 
2019-11-16T10:35:56.2699083Z ##[error]Bash exited with code '1'.
2019-11-16T10:35:56.2725086Z ##[section]Starting: Checkout
2019-11-16T10:35:56.2726515Z ==============================================================================
2019-11-16T10:35:56.2726577Z Task         : Get sources
2019-11-16T10:35:56.2726616Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

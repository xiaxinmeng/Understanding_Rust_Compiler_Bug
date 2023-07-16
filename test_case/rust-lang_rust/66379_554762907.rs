plain
2019-11-17T16:14:27.3213565Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T16:14:27.3406469Z ##[command]git config gc.auto 0
2019-11-17T16:14:27.3492687Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T16:14:27.3564490Z ##[command]git config --get-all http.proxy
2019-11-17T16:14:27.8802357Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-17T16:20:22.5840921Z    Compiling serde_json v1.0.40
2019-11-17T16:20:24.2591288Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-17T16:20:34.8502101Z     Finished release [optimized] target(s) in 1m 25s
2019-11-17T16:20:34.8599799Z tidy check
2019-11-17T16:20:35.7173762Z tidy error: /checkout/src/libcore/ptr/mod.rs:1077: trailing whitespace
2019-11-17T16:20:35.7174567Z tidy error: /checkout/src/libcore/ptr/mod.rs:1080: trailing whitespace
2019-11-17T16:20:35.7174841Z tidy error: /checkout/src/libcore/ptr/mod.rs:1088: trailing whitespace
2019-11-17T16:20:35.7175061Z tidy error: /checkout/src/libcore/ptr/mod.rs:1089: trailing whitespace
2019-11-17T16:20:35.7175310Z tidy error: /checkout/src/libcore/ptr/mod.rs:1092: trailing whitespace
2019-11-17T16:20:35.7175528Z tidy error: /checkout/src/libcore/ptr/mod.rs:1093: trailing whitespace
2019-11-17T16:20:35.7178891Z tidy error: /checkout/src/libcore/ptr/mod.rs:1941: trailing whitespace
2019-11-17T16:20:35.7179210Z tidy error: /checkout/src/libcore/ptr/mod.rs:1949: trailing whitespace
2019-11-17T16:20:35.7179453Z tidy error: /checkout/src/libcore/ptr/mod.rs:1950: trailing whitespace
2019-11-17T16:20:35.7179703Z tidy error: /checkout/src/libcore/ptr/mod.rs:1951: line longer than 100 chars
2019-11-17T16:20:35.7179874Z tidy error: /checkout/src/libcore/ptr/mod.rs:1953: trailing whitespace
2019-11-17T16:20:35.7180040Z tidy error: /checkout/src/libcore/ptr/mod.rs:1954: trailing whitespace
2019-11-17T16:20:37.4149216Z Found 441 error codes
2019-11-17T16:20:37.4149318Z Found 0 error codes with no tests
2019-11-17T16:20:37.4150846Z Done!
2019-11-17T16:20:37.4150894Z some tidy checks failed
2019-11-17T16:20:37.4150894Z some tidy checks failed
2019-11-17T16:20:37.4150923Z 
2019-11-17T16:20:37.4150947Z 
2019-11-17T16:20:37.4151793Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-17T16:20:37.4151890Z 
2019-11-17T16:20:37.4151971Z 
2019-11-17T16:20:37.4164951Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-17T16:20:37.4165048Z Build completed unsuccessfully in 0:01:28
2019-11-17T16:20:37.4165048Z Build completed unsuccessfully in 0:01:28
2019-11-17T16:20:37.4217230Z == clock drift check ==
2019-11-17T16:20:37.4232218Z   local time: Sun Nov 17 16:20:37 UTC 2019
2019-11-17T16:20:37.7002269Z   network time: Sun, 17 Nov 2019 16:20:37 GMT
2019-11-17T16:20:37.7006646Z == end clock drift check ==
2019-11-17T16:20:39.0708015Z 
2019-11-17T16:20:39.0783948Z ##[error]Bash exited with code '1'.
2019-11-17T16:20:39.0809970Z ##[section]Starting: Checkout
2019-11-17T16:20:39.0812244Z ==============================================================================
2019-11-17T16:20:39.0812293Z Task         : Get sources
2019-11-17T16:20:39.0812334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

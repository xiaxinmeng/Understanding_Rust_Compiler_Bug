plain
2019-10-27T16:35:33.9913010Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T16:35:34.0138914Z ##[command]git config gc.auto 0
2019-10-27T16:35:34.0203162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T16:35:34.0257599Z ##[command]git config --get-all http.proxy
2019-10-27T16:35:34.0379301Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65794/merge:refs/remotes/pull/65794/merge
---
2019-10-27T16:41:55.0434551Z     Finished release [optimized] target(s) in 1m 30s
2019-10-27T16:41:55.0543704Z tidy check
2019-10-27T16:41:56.4107559Z * 586 error codes
2019-10-27T16:41:56.4107692Z * highest error code: E0741
2019-10-27T16:41:56.4556131Z tidy error: /checkout/src/libsyntax/feature_gate/removed.rs:104: feature on_unimplemented is not sorted by "since" (version number)
2019-10-27T16:41:57.0882793Z tidy error: The Unstable Book has a 'language feature' section 'on-unimplemented' which doesn't correspond to an unstable language feature
2019-10-27T16:41:57.7757313Z Found 484 error codes
2019-10-27T16:41:57.7758115Z Found 0 error codes with no tests
2019-10-27T16:41:57.7758425Z Done!
2019-10-27T16:41:57.7761496Z some tidy checks failed
2019-10-27T16:41:57.7761496Z some tidy checks failed
2019-10-27T16:41:57.7764466Z 
2019-10-27T16:41:57.7764751Z 
2019-10-27T16:41:57.7766705Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-27T16:41:57.7769857Z 
2019-10-27T16:41:57.7770093Z 
2019-10-27T16:41:57.7770375Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-27T16:41:57.7770597Z Build completed unsuccessfully in 0:01:34
2019-10-27T16:41:57.7770597Z Build completed unsuccessfully in 0:01:34
2019-10-27T16:41:57.7821585Z == clock drift check ==
2019-10-27T16:41:57.7832430Z   local time: Sun Oct 27 16:41:57 UTC 2019
2019-10-27T16:41:59.0802118Z   network time: Sun, 27 Oct 2019 16:41:59 GMT
2019-10-27T16:41:59.0802321Z == end clock drift check ==
2019-10-27T16:42:00.5298208Z 
2019-10-27T16:42:00.5413633Z ##[error]Bash exited with code '1'.
2019-10-27T16:42:00.5449136Z ##[section]Starting: Checkout
2019-10-27T16:42:00.5451092Z ==============================================================================
2019-10-27T16:42:00.5451299Z Task         : Get sources
2019-10-27T16:42:00.5451360Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

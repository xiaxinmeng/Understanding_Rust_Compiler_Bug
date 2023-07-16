plain
2019-10-20T21:48:51.0307812Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T21:48:51.0484772Z ##[command]git config gc.auto 0
2019-10-20T21:48:51.0585132Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T21:48:51.0639443Z ##[command]git config --get-all http.proxy
2019-10-20T21:48:51.0791253Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65640/merge:refs/remotes/pull/65640/merge
---
2019-10-20T21:55:32.2858595Z    Compiling serde_json v1.0.40
2019-10-20T21:55:34.0812840Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-20T21:55:45.9559988Z     Finished release [optimized] target(s) in 1m 33s
2019-10-20T21:55:45.9641303Z tidy check
2019-10-20T21:55:46.7989827Z tidy error: /checkout/src/libsyntax/parse/parser/diagnostics.rs:925: line longer than 100 chars
2019-10-20T21:55:48.2337312Z some tidy checks failed
2019-10-20T21:55:48.2342702Z Found 482 error codes
2019-10-20T21:55:48.2342815Z Found 0 error codes with no tests
2019-10-20T21:55:48.2342865Z Done!
2019-10-20T21:55:48.2342865Z Done!
2019-10-20T21:55:48.2349831Z 
2019-10-20T21:55:48.2349933Z 
2019-10-20T21:55:48.2350905Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-20T21:55:48.2361274Z 
2019-10-20T21:55:48.2361307Z 
2019-10-20T21:55:48.2361388Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-20T21:55:48.2361447Z Build completed unsuccessfully in 0:01:36
2019-10-20T21:55:48.2361447Z Build completed unsuccessfully in 0:01:36
2019-10-20T21:55:48.2402513Z == clock drift check ==
2019-10-20T21:55:48.2418923Z   local time: Sun Oct 20 21:55:48 UTC 2019
2019-10-20T21:55:48.4046359Z   network time: Sun, 20 Oct 2019 21:55:48 GMT
2019-10-20T21:55:48.4053868Z == end clock drift check ==
2019-10-20T21:55:49.7384120Z 
2019-10-20T21:55:49.7512639Z ##[error]Bash exited with code '1'.
2019-10-20T21:55:49.7566807Z ##[section]Starting: Checkout
2019-10-20T21:55:49.7569998Z ==============================================================================
2019-10-20T21:55:49.7570087Z Task         : Get sources
2019-10-20T21:55:49.7570136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

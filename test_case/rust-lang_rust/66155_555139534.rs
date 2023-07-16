plain
2019-11-18T18:04:38.6373339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T18:04:38.6578076Z ##[command]git config gc.auto 0
2019-11-18T18:04:38.6657108Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T18:04:38.6721902Z ##[command]git config --get-all http.proxy
2019-11-18T18:04:39.3179459Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66155/merge:refs/remotes/pull/66155/merge
---
2019-11-18T18:11:04.2161013Z    Compiling serde_json v1.0.40
2019-11-18T18:11:06.0701168Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-18T18:11:17.7628291Z     Finished release [optimized] target(s) in 1m 31s
2019-11-18T18:11:17.7721260Z tidy check
2019-11-18T18:11:19.2419733Z tidy error: duplicate error code: 593
2019-11-18T18:11:19.2420589Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs:320: E0593: include_str!("./error_codes/E0593.md"),
2019-11-18T18:11:19.2420977Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs:321: E0593: include_str!("./error_codes/E0594.md"),
2019-11-18T18:11:20.6674390Z some tidy checks failed
2019-11-18T18:11:20.6674468Z Found 441 error codes
2019-11-18T18:11:20.6674550Z Found 0 error codes with no tests
2019-11-18T18:11:20.6674621Z Done!
2019-11-18T18:11:20.6674621Z Done!
2019-11-18T18:11:20.6674653Z 
2019-11-18T18:11:20.6674680Z 
2019-11-18T18:11:20.6675543Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-18T18:11:20.6675646Z 
2019-11-18T18:11:20.6675692Z 
2019-11-18T18:11:20.6678693Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-18T18:11:20.6678998Z Build completed unsuccessfully in 0:01:35
2019-11-18T18:11:20.6678998Z Build completed unsuccessfully in 0:01:35
2019-11-18T18:11:20.6732323Z == clock drift check ==
2019-11-18T18:11:20.6744182Z   local time: Mon Nov 18 18:11:20 UTC 2019
2019-11-18T18:11:20.9508226Z   network time: Mon, 18 Nov 2019 18:11:20 GMT
2019-11-18T18:11:20.9508918Z == end clock drift check ==
2019-11-18T18:11:22.3187591Z 
2019-11-18T18:11:22.3304308Z ##[error]Bash exited with code '1'.
2019-11-18T18:11:22.3353973Z ##[section]Starting: Checkout
2019-11-18T18:11:22.3355644Z ==============================================================================
2019-11-18T18:11:22.3355702Z Task         : Get sources
2019-11-18T18:11:22.3355766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

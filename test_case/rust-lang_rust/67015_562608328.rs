plain
2019-12-06T14:48:55.0760520Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-06T14:48:55.0776789Z ##[command]git config gc.auto 0
2019-12-06T14:48:55.0780595Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-06T14:48:55.0785926Z ##[command]git config --get-all http.proxy
2019-12-06T14:48:55.0798799Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67015/merge:refs/remotes/pull/67015/merge
---
2019-12-06T14:54:24.5216199Z    Compiling serde_json v1.0.40
2019-12-06T14:54:26.0230944Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-06T14:54:35.7512859Z     Finished release [optimized] target(s) in 1m 16s
2019-12-06T14:54:35.7603144Z tidy check
2019-12-06T14:54:36.3534742Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:651: line longer than 100 chars
2019-12-06T14:54:38.2187671Z Found 486 error codes
2019-12-06T14:54:38.2192599Z Found 0 error codes with no tests
2019-12-06T14:54:38.2192906Z Done!
2019-12-06T14:54:38.2193124Z some tidy checks failed
2019-12-06T14:54:38.2193124Z some tidy checks failed
2019-12-06T14:54:38.2193267Z 
2019-12-06T14:54:38.2193381Z 
2019-12-06T14:54:38.2194470Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-06T14:54:38.2194994Z 
2019-12-06T14:54:38.2195151Z 
2019-12-06T14:54:38.2199717Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-06T14:54:38.2199984Z Build completed unsuccessfully in 0:01:20
2019-12-06T14:54:38.2199984Z Build completed unsuccessfully in 0:01:20
2019-12-06T14:54:38.2244595Z == clock drift check ==
2019-12-06T14:54:38.2274856Z   local time: Fri Dec  6 14:54:38 UTC 2019
2019-12-06T14:54:38.4899868Z   network time: Fri, 06 Dec 2019 14:54:38 GMT
2019-12-06T14:54:38.4901627Z == end clock drift check ==
2019-12-06T14:54:39.8182547Z 
2019-12-06T14:54:39.8268611Z ##[error]Bash exited with code '1'.
2019-12-06T14:54:39.8295209Z ##[section]Starting: Checkout
2019-12-06T14:54:39.8297121Z ==============================================================================
2019-12-06T14:54:39.8297188Z Task         : Get sources
2019-12-06T14:54:39.8297228Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

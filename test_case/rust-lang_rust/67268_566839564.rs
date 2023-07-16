plain
2019-12-18T02:16:03.1945755Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T02:16:03.2160983Z ##[command]git config gc.auto 0
2019-12-18T02:16:03.2212793Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T02:16:03.2266305Z ##[command]git config --get-all http.proxy
2019-12-18T02:16:03.2394048Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67268/merge:refs/remotes/pull/67268/merge
---
2019-12-18T02:21:43.8955227Z    Compiling serde_json v1.0.40
2019-12-18T02:21:45.1784024Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-18T02:21:54.6523072Z     Finished release [optimized] target(s) in 1m 14s
2019-12-18T02:21:54.6603987Z tidy check
2019-12-18T02:21:55.5154215Z tidy error: /checkout/src/librustc_typeck/astconv.rs:1585: trailing whitespace
2019-12-18T02:21:55.5154581Z tidy error: /checkout/src/librustc_typeck/astconv.rs:1588: trailing whitespace
2019-12-18T02:21:55.5154686Z tidy error: /checkout/src/librustc_typeck/astconv.rs:1654: line longer than 100 chars
2019-12-18T02:21:56.8688315Z some tidy checks failed
2019-12-18T02:21:56.8688408Z Found 486 error codes
2019-12-18T02:21:56.8688443Z Found 0 error codes with no tests
2019-12-18T02:21:56.8688476Z Done!
2019-12-18T02:21:56.8688476Z Done!
2019-12-18T02:21:56.8688518Z 
2019-12-18T02:21:56.8688540Z 
2019-12-18T02:21:56.8689606Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-18T02:21:56.8689723Z 
2019-12-18T02:21:56.8689744Z 
2019-12-18T02:21:56.8692781Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-18T02:21:56.8693052Z Build completed unsuccessfully in 0:01:18
2019-12-18T02:21:56.8693052Z Build completed unsuccessfully in 0:01:18
2019-12-18T02:21:56.8739603Z == clock drift check ==
2019-12-18T02:21:56.8747832Z   local time: Wed Dec 18 02:21:56 UTC 2019
2019-12-18T02:21:57.0349702Z   network time: Wed, 18 Dec 2019 02:21:57 GMT
2019-12-18T02:21:57.0351700Z == end clock drift check ==
2019-12-18T02:21:59.0407853Z 
2019-12-18T02:21:59.0505244Z ##[error]Bash exited with code '1'.
2019-12-18T02:21:59.0527873Z ##[section]Starting: Checkout
2019-12-18T02:21:59.0529273Z ==============================================================================
2019-12-18T02:21:59.0529334Z Task         : Get sources
2019-12-18T02:21:59.0529371Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

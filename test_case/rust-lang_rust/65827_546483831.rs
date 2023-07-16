plain
2019-10-25T19:16:08.1958895Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T19:16:08.2146464Z ##[command]git config gc.auto 0
2019-10-25T19:16:08.2222725Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T19:16:08.2298228Z ##[command]git config --get-all http.proxy
2019-10-25T19:16:08.2442445Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65827/merge:refs/remotes/pull/65827/merge
---
2019-10-25T19:22:25.8915899Z    Compiling serde_json v1.0.40
2019-10-25T19:22:27.6303156Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-25T19:22:38.9407364Z     Finished release [optimized] target(s) in 1m 27s
2019-10-25T19:22:38.9485451Z tidy check
2019-10-25T19:22:39.3909079Z tidy error: /checkout/src/librustc_errors/emitter.rs:1821: line longer than 100 chars
2019-10-25T19:22:41.1837240Z some tidy checks failed
2019-10-25T19:22:41.1837365Z Found 484 error codes
2019-10-25T19:22:41.1837444Z Found 0 error codes with no tests
2019-10-25T19:22:41.1838112Z Done!
2019-10-25T19:22:41.1838112Z Done!
2019-10-25T19:22:41.1838149Z 
2019-10-25T19:22:41.1838174Z 
2019-10-25T19:22:41.1839079Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-25T19:22:41.1839182Z 
2019-10-25T19:22:41.1839227Z 
2019-10-25T19:22:41.1842432Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-25T19:22:41.1842919Z Build completed unsuccessfully in 0:01:31
2019-10-25T19:22:41.1842919Z Build completed unsuccessfully in 0:01:31
2019-10-25T19:22:41.1899622Z == clock drift check ==
2019-10-25T19:22:41.1919365Z   local time: Fri Oct 25 19:22:41 UTC 2019
2019-10-25T19:22:41.3276932Z   network time: Fri, 25 Oct 2019 19:22:41 GMT
2019-10-25T19:22:41.3281346Z == end clock drift check ==
2019-10-25T19:22:42.6618293Z 
2019-10-25T19:22:42.6718767Z ##[error]Bash exited with code '1'.
2019-10-25T19:22:42.6753295Z ##[section]Starting: Checkout
2019-10-25T19:22:42.6754992Z ==============================================================================
2019-10-25T19:22:42.6755050Z Task         : Get sources
2019-10-25T19:22:42.6755118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

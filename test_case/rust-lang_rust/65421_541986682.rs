plain
2019-10-15T00:22:00.6246429Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T00:22:00.6326101Z ##[command]git config gc.auto 0
2019-10-15T00:22:00.6399836Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T00:22:00.6465710Z ##[command]git config --get-all http.proxy
2019-10-15T00:22:00.6604600Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65421/merge:refs/remotes/pull/65421/merge
---
2019-10-15T00:28:06.5193589Z    Compiling serde_json v1.0.40
2019-10-15T00:28:08.2973557Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-15T00:28:19.8778392Z     Finished release [optimized] target(s) in 1m 31s
2019-10-15T00:28:19.8878443Z tidy check
2019-10-15T00:28:20.1028604Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:9: line longer than 100 chars
2019-10-15T00:28:20.1030091Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:11: line longer than 100 chars
2019-10-15T00:28:20.1662858Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:34: line longer than 100 chars
2019-10-15T00:28:20.1664651Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:42: line longer than 100 chars
2019-10-15T00:28:22.2252907Z some tidy checks failed
2019-10-15T00:28:22.2256698Z Found 482 error codes
2019-10-15T00:28:22.2257230Z Found 0 error codes with no tests
2019-10-15T00:28:22.2257430Z Done!
2019-10-15T00:28:22.2257430Z Done!
2019-10-15T00:28:22.2258161Z 
2019-10-15T00:28:22.2258503Z 
2019-10-15T00:28:22.2267393Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-15T00:28:22.2304459Z 
2019-10-15T00:28:22.2304493Z 
2019-10-15T00:28:22.2304731Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-15T00:28:22.2305054Z Build completed unsuccessfully in 0:01:34
2019-10-15T00:28:22.2305054Z Build completed unsuccessfully in 0:01:34
2019-10-15T00:28:22.2309884Z == clock drift check ==
2019-10-15T00:28:22.2349600Z   local time: Tue Oct 15 00:28:22 UTC 2019
2019-10-15T00:28:22.3187104Z   network time: Tue, 15 Oct 2019 00:28:22 GMT
2019-10-15T00:28:22.3189572Z == end clock drift check ==
2019-10-15T00:28:23.1508772Z ##[error]Bash exited with code '1'.
2019-10-15T00:28:23.1559859Z ##[section]Starting: Checkout
2019-10-15T00:28:23.1562039Z ==============================================================================
2019-10-15T00:28:23.1562119Z Task         : Get sources
2019-10-15T00:28:23.1562166Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

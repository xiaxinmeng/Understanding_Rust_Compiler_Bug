plain
2019-09-24T03:23:30.6139977Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T03:23:30.6328875Z ##[command]git config gc.auto 0
2019-09-24T03:23:30.6414985Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T03:23:30.6472640Z ##[command]git config --get-all http.proxy
2019-09-24T03:23:30.6623863Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64726/merge:refs/remotes/pull/64726/merge
---
2019-09-24T03:30:45.8786990Z    Compiling serde_json v1.0.40
2019-09-24T03:30:47.8615655Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-24T03:30:59.6278557Z     Finished release [optimized] target(s) in 1m 36s
2019-09-24T03:30:59.6361664Z tidy check
2019-09-24T03:31:00.1507278Z tidy error: /checkout/src/libcore/macros.rs:572: line longer than 100 chars
2019-09-24T03:31:01.7306553Z some tidy checks failed
2019-09-24T03:31:01.7316934Z 
2019-09-24T03:31:01.7316934Z 
2019-09-24T03:31:01.7322818Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-24T03:31:01.7323329Z 
2019-09-24T03:31:01.7323358Z 
2019-09-24T03:31:01.7331242Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-24T03:31:01.7331326Z Build completed unsuccessfully in 0:01:39
2019-09-24T03:31:01.7331326Z Build completed unsuccessfully in 0:01:39
2019-09-24T03:31:01.7377616Z == clock drift check ==
2019-09-24T03:31:01.7392956Z   local time: Tue Sep 24 03:31:01 UTC 2019
2019-09-24T03:31:01.8244109Z   network time: Tue, 24 Sep 2019 03:31:01 GMT
2019-09-24T03:31:01.8251601Z == end clock drift check ==
2019-09-24T03:31:03.2351728Z ##[error]Bash exited with code '1'.
2019-09-24T03:31:03.2387449Z ##[section]Starting: Checkout
2019-09-24T03:31:03.2390193Z ==============================================================================
2019-09-24T03:31:03.2390251Z Task         : Get sources
2019-09-24T03:31:03.2390299Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-02T16:06:47.1576383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T16:06:47.1783018Z ##[command]git config gc.auto 0
2019-08-02T16:06:47.1883225Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T16:06:47.1936352Z ##[command]git config --get-all http.proxy
2019-08-02T16:06:47.2086815Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63218/merge:refs/remotes/pull/63218/merge
---
2019-08-02T16:07:21.5018050Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T16:07:21.5018086Z 
2019-08-02T16:07:21.5018341Z   git checkout -b <new-branch-name>
2019-08-02T16:07:21.5018376Z 
2019-08-02T16:07:21.5018447Z HEAD is now at ca9c177e7 Merge 9cb948feea23e18d26f2457e8f672c36e4f91e5d into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T16:07:21.5188243Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T16:07:21.5191412Z ==============================================================================
2019-08-02T16:07:21.5191496Z Task         : Bash
2019-08-02T16:07:21.5191548Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T16:13:37.0293952Z    Compiling serde_json v1.0.40
2019-08-02T16:13:41.6026146Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-02T16:13:50.7512535Z     Finished release [optimized] target(s) in 1m 33s
2019-08-02T16:13:50.7582351Z tidy check
2019-08-02T16:13:51.2858777Z tidy error: /checkout/src/bootstrap/native.rs:128: line longer than 100 chars
2019-08-02T16:13:52.8508267Z some tidy checks failed
2019-08-02T16:13:52.8514531Z 
2019-08-02T16:13:52.8514531Z 
2019-08-02T16:13:52.8515484Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T16:13:52.8515597Z 
2019-08-02T16:13:52.8515642Z 
2019-08-02T16:13:52.8524437Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T16:13:52.8524551Z Build completed unsuccessfully in 0:01:37
2019-08-02T16:13:52.8524551Z Build completed unsuccessfully in 0:01:37
2019-08-02T16:13:54.2181394Z ##[error]Bash exited with code '1'.
2019-08-02T16:13:54.2215416Z ##[section]Starting: Checkout
2019-08-02T16:13:54.2217222Z ==============================================================================
2019-08-02T16:13:54.2217284Z Task         : Get sources
2019-08-02T16:13:54.2217336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

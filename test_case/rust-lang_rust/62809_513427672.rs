plain
2019-07-19T22:53:07.9903859Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T22:53:08.0090103Z ##[command]git config gc.auto 0
2019-07-19T22:53:08.0160358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T22:53:08.0207424Z ##[command]git config --get-all http.proxy
2019-07-19T22:53:08.0337878Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62809/merge:refs/remotes/pull/62809/merge
---
2019-07-19T22:53:41.6658887Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T22:53:41.6658934Z 
2019-07-19T22:53:41.6659121Z   git checkout -b <new-branch-name>
2019-07-19T22:53:41.6659146Z 
2019-07-19T22:53:41.6659186Z HEAD is now at 8dba87ec3 Merge 4c61a6673ad884d734289e060ce4aba03f76e5bf into e3cebcb3bd4ffaf86bb0cdfd2af5b7e698717b01
2019-07-19T22:53:41.6797268Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T22:53:41.6799655Z ==============================================================================
2019-07-19T22:53:41.6799703Z Task         : Bash
2019-07-19T22:53:41.6799759Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T22:59:47.1878903Z    Compiling serde_json v1.0.40
2019-07-19T22:59:50.6761493Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-19T22:59:58.9380708Z     Finished release [optimized] target(s) in 1m 27s
2019-07-19T22:59:58.9448852Z tidy check
2019-07-19T22:59:59.8023614Z tidy error: /checkout/src/libstd/sys/wasm/fast_thread_local.rs:4: TODO is deprecated; use FIXME
2019-07-19T23:00:00.6984914Z some tidy checks failed
2019-07-19T23:00:00.6985758Z 
2019-07-19T23:00:00.6985758Z 
2019-07-19T23:00:00.6986718Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-19T23:00:00.6993005Z 
2019-07-19T23:00:00.6993032Z 
2019-07-19T23:00:00.7005982Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-19T23:00:00.7006351Z Build completed unsuccessfully in 0:01:30
2019-07-19T23:00:00.7006351Z Build completed unsuccessfully in 0:01:30
2019-07-19T23:00:02.0075862Z ##[error]Bash exited with code '1'.
2019-07-19T23:00:02.0125060Z ##[section]Starting: Checkout
2019-07-19T23:00:02.0126927Z ==============================================================================
2019-07-19T23:00:02.0127018Z Task         : Get sources
2019-07-19T23:00:02.0127065Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-10-21T13:27:56.6508188Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T13:27:56.6677626Z ##[command]git config gc.auto 0
2019-10-21T13:27:56.6758657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T13:27:56.6832822Z ##[command]git config --get-all http.proxy
2019-10-21T13:27:56.6979301Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-21T13:34:21.6623953Z    Compiling serde_json v1.0.40
2019-10-21T13:34:23.4895853Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-21T13:34:35.4973369Z     Finished release [optimized] target(s) in 1m 29s
2019-10-21T13:34:35.5048521Z tidy check
2019-10-21T13:34:36.1872067Z tidy error: /checkout/src/libcore/macros.rs:13: TODO is deprecated; use FIXME
2019-10-21T13:34:36.1873145Z tidy error: /checkout/src/libcore/macros.rs:20: TODO is deprecated; use FIXME
2019-10-21T13:34:37.9039435Z Found 482 error codes
2019-10-21T13:34:37.9039577Z Found 0 error codes with no tests
2019-10-21T13:34:37.9039656Z Done!
2019-10-21T13:34:37.9039694Z some tidy checks failed
2019-10-21T13:34:37.9039694Z some tidy checks failed
2019-10-21T13:34:37.9045236Z 
2019-10-21T13:34:37.9046293Z 
2019-10-21T13:34:37.9047407Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-21T13:34:37.9047544Z 
2019-10-21T13:34:37.9047569Z 
2019-10-21T13:34:37.9057494Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-21T13:34:37.9057795Z Build completed unsuccessfully in 0:01:33
2019-10-21T13:34:37.9057795Z Build completed unsuccessfully in 0:01:33
2019-10-21T13:34:37.9113779Z == clock drift check ==
2019-10-21T13:34:37.9137894Z   local time: Mon Oct 21 13:34:37 UTC 2019
2019-10-21T13:34:37.9983621Z   network time: Mon, 21 Oct 2019 13:34:38 GMT
2019-10-21T13:34:37.9990207Z == end clock drift check ==
2019-10-21T13:34:39.3129101Z 
2019-10-21T13:34:39.3246984Z ##[error]Bash exited with code '1'.
2019-10-21T13:34:39.3281228Z ##[section]Starting: Checkout
2019-10-21T13:34:39.3283737Z ==============================================================================
2019-10-21T13:34:39.3283800Z Task         : Get sources
2019-10-21T13:34:39.3283867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

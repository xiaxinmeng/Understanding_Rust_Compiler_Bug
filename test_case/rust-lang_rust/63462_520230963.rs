plain
2019-08-11T13:45:23.7527542Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T13:45:23.7713514Z ##[command]git config gc.auto 0
2019-08-11T13:45:23.7817010Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T13:45:23.7870410Z ##[command]git config --get-all http.proxy
2019-08-11T13:45:23.8007055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63462/merge:refs/remotes/pull/63462/merge
---
2019-08-11T13:45:58.3469733Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T13:45:58.3469760Z 
2019-08-11T13:45:58.3469934Z   git checkout -b <new-branch-name>
2019-08-11T13:45:58.3469974Z 
2019-08-11T13:45:58.3470029Z HEAD is now at 1337a0ee6 Merge dea327c9000850e28de94929219f8550f1bbf420 into 2b78e10ac1454d2d4190c575f6ece03f484ac398
2019-08-11T13:45:58.3655737Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T13:45:58.3659621Z ==============================================================================
2019-08-11T13:45:58.3659675Z Task         : Bash
2019-08-11T13:45:58.3659731Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T13:51:27.5685048Z    Compiling serde_json v1.0.40
2019-08-11T13:51:31.5802663Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-11T13:51:39.7298205Z     Finished release [optimized] target(s) in 1m 24s
2019-08-11T13:51:39.7365573Z tidy check
2019-08-11T13:51:39.8461190Z tidy error: /checkout/src/libcore/fmt/builders.rs:254: line longer than 100 chars
2019-08-11T13:51:39.9237112Z tidy error: /checkout/src/libsyntax_ext/deriving/generic/mod.rs:1390: line longer than 100 chars
2019-08-11T13:51:41.5283691Z some tidy checks failed
2019-08-11T13:51:41.5283833Z 
2019-08-11T13:51:41.5283833Z 
2019-08-11T13:51:41.5284816Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-11T13:51:41.5285128Z 
2019-08-11T13:51:41.5285151Z 
2019-08-11T13:51:41.5291292Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-11T13:51:41.5291629Z Build completed unsuccessfully in 0:01:27
2019-08-11T13:51:41.5291629Z Build completed unsuccessfully in 0:01:27
2019-08-11T13:51:43.8161627Z ##[error]Bash exited with code '1'.
2019-08-11T13:51:43.8190213Z ##[section]Starting: Checkout
2019-08-11T13:51:43.8191614Z ==============================================================================
2019-08-11T13:51:43.8191656Z Task         : Get sources
2019-08-11T13:51:43.8191692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

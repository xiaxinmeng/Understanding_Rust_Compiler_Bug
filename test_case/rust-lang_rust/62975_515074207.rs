plain
2019-07-25T14:27:18.6721709Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T14:27:18.6935507Z ##[command]git config gc.auto 0
2019-07-25T14:27:18.7000849Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T14:27:18.7058701Z ##[command]git config --get-all http.proxy
2019-07-25T14:27:18.7205447Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62975/merge:refs/remotes/pull/62975/merge
---
2019-07-25T14:27:53.3248713Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T14:27:53.3248846Z 
2019-07-25T14:27:53.3249123Z   git checkout -b <new-branch-name>
2019-07-25T14:27:53.3249286Z 
2019-07-25T14:27:53.3249407Z HEAD is now at 3856514b7 Merge 0baccc7e7593dac83b126675a561380231a74950 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T14:27:53.3390766Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T14:27:53.3395372Z ==============================================================================
2019-07-25T14:27:53.3395434Z Task         : Bash
2019-07-25T14:27:53.3395501Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T14:34:24.5598132Z    Compiling serde_json v1.0.40
2019-07-25T14:34:29.1645400Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-25T14:34:38.0934284Z     Finished release [optimized] target(s) in 1m 33s
2019-07-25T14:34:38.1005974Z tidy check
2019-07-25T14:34:38.4701473Z tidy error: /checkout/src/librustc/hir/lowering.rs:446: line longer than 100 chars
2019-07-25T14:34:40.1821045Z some tidy checks failed
2019-07-25T14:34:40.1823846Z 
2019-07-25T14:34:40.1823846Z 
2019-07-25T14:34:40.1825556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-25T14:34:40.1826388Z 
2019-07-25T14:34:40.1826411Z 
2019-07-25T14:34:40.1840117Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T14:34:40.1840208Z Build completed unsuccessfully in 0:01:36
2019-07-25T14:34:40.1840208Z Build completed unsuccessfully in 0:01:36
2019-07-25T14:34:41.5915909Z ##[error]Bash exited with code '1'.
2019-07-25T14:34:41.5947113Z ##[section]Starting: Checkout
2019-07-25T14:34:41.5948783Z ==============================================================================
2019-07-25T14:34:41.5948830Z Task         : Get sources
2019-07-25T14:34:41.5948869Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

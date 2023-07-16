plain
2019-08-08T19:46:37.0284911Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T19:46:37.0503906Z ##[command]git config gc.auto 0
2019-08-08T19:46:37.0594277Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T19:46:37.0657429Z ##[command]git config --get-all http.proxy
2019-08-08T19:46:37.0810957Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61969/merge:refs/remotes/pull/61969/merge
---
2019-08-08T19:47:13.7929108Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T19:47:13.7929178Z 
2019-08-08T19:47:13.7929390Z   git checkout -b <new-branch-name>
2019-08-08T19:47:13.7929420Z 
2019-08-08T19:47:13.7929469Z HEAD is now at 8e5eab5b4 Merge c284340450580470fe98ca3718642aa19b3fc035 into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T19:47:13.8101331Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T19:47:13.8104070Z ==============================================================================
2019-08-08T19:47:13.8104784Z Task         : Bash
2019-08-08T19:47:13.8104834Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T19:53:21.4534714Z    Compiling serde_json v1.0.40
2019-08-08T19:53:26.0958426Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-08T19:53:35.4579587Z     Finished release [optimized] target(s) in 1m 37s
2019-08-08T19:53:35.4651980Z tidy check
2019-08-08T19:53:36.0525140Z tidy error: /checkout/src/libstd/path.rs:1127: line longer than 100 chars
2019-08-08T19:53:36.0525370Z tidy error: /checkout/src/libstd/path.rs:1129: line longer than 100 chars
2019-08-08T19:53:36.0531324Z tidy error: /checkout/src/libstd/path.rs:1755: line longer than 100 chars
2019-08-08T19:53:36.0584488Z tidy error: /checkout/src/libstd/ffi/os_str.rs:103: line longer than 100 chars
2019-08-08T19:53:36.0591445Z tidy error: /checkout/src/libstd/ffi/c_str.rs:199: line longer than 100 chars
2019-08-08T19:53:36.0591526Z tidy error: /checkout/src/libstd/ffi/c_str.rs:201: line longer than 100 chars
2019-08-08T19:53:36.0889025Z tidy error: /checkout/src/libstd/sys_common/os_str_bytes.rs:24: line longer than 100 chars
2019-08-08T19:53:37.5557130Z some tidy checks failed
2019-08-08T19:53:37.5560023Z 
2019-08-08T19:53:37.5560023Z 
2019-08-08T19:53:37.5561494Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-08T19:53:37.5561693Z 
2019-08-08T19:53:37.5561725Z 
2019-08-08T19:53:37.5572257Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-08T19:53:37.5572579Z Build completed unsuccessfully in 0:01:40
2019-08-08T19:53:37.5572579Z Build completed unsuccessfully in 0:01:40
2019-08-08T19:53:38.9417095Z ##[error]Bash exited with code '1'.
2019-08-08T19:53:38.9449216Z ##[section]Starting: Checkout
2019-08-08T19:53:38.9451514Z ==============================================================================
2019-08-08T19:53:38.9451574Z Task         : Get sources
2019-08-08T19:53:38.9451622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

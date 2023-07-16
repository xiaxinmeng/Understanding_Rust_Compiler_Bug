plain
2019-07-26T16:39:44.9776872Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T16:39:45.0040904Z ##[command]git config gc.auto 0
2019-07-26T16:39:45.0065109Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T16:39:45.0125551Z ##[command]git config --get-all http.proxy
2019-07-26T16:39:45.0259149Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62948/merge:refs/remotes/pull/62948/merge
---
2019-07-26T16:40:20.3331406Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T16:40:20.3332519Z 
2019-07-26T16:40:20.3333924Z   git checkout -b <new-branch-name>
2019-07-26T16:40:20.3334975Z 
2019-07-26T16:40:20.3336569Z HEAD is now at a62d482c5 Merge 36935a9ceea2570ec9025296c1b0257c8b692198 into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T16:40:20.3477059Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T16:40:20.3479785Z ==============================================================================
2019-07-26T16:40:20.3479838Z Task         : Bash
2019-07-26T16:40:20.3480403Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T16:46:19.0271961Z    Compiling serde_json v1.0.40
2019-07-26T16:46:23.4741104Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T16:46:31.1611984Z     Finished release [optimized] target(s) in 1m 24s
2019-07-26T16:46:31.1678858Z tidy check
2019-07-26T16:46:31.3339314Z tidy error: /checkout/src/libsyntax/test_snippet.rs:46: line longer than 100 chars
2019-07-26T16:46:32.8679625Z some tidy checks failed
2019-07-26T16:46:32.8681117Z 
2019-07-26T16:46:32.8681117Z 
2019-07-26T16:46:32.8699963Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T16:46:32.8700451Z 
2019-07-26T16:46:32.8700556Z 
2019-07-26T16:46:32.8710330Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T16:46:32.8710671Z Build completed unsuccessfully in 0:01:27
2019-07-26T16:46:32.8710671Z Build completed unsuccessfully in 0:01:27
2019-07-26T16:46:34.2387548Z ##[error]Bash exited with code '1'.
2019-07-26T16:46:34.2439807Z ##[section]Starting: Checkout
2019-07-26T16:46:34.2441199Z ==============================================================================
2019-07-26T16:46:34.2441242Z Task         : Get sources
2019-07-26T16:46:34.2441296Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

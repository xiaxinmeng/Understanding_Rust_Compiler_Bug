plain
2019-08-05T16:24:29.0644832Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T16:24:29.0831785Z ##[command]git config gc.auto 0
2019-08-05T16:24:29.0900769Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T16:24:29.0961266Z ##[command]git config --get-all http.proxy
2019-08-05T16:24:29.1099564Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63294/merge:refs/remotes/pull/63294/merge
---
2019-08-05T16:25:04.3108729Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T16:25:04.3108988Z 
2019-08-05T16:25:04.3109465Z   git checkout -b <new-branch-name>
2019-08-05T16:25:04.3109739Z 
2019-08-05T16:25:04.3109953Z HEAD is now at 6e1e72d49 Merge f40190a6a541479e504d783cd984670c6fa6cb39 into 4be067558962c004b638e4c6f162d50f7c0c98b6
2019-08-05T16:25:04.3256136Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T16:25:04.3258445Z ==============================================================================
2019-08-05T16:25:04.3258895Z Task         : Bash
2019-08-05T16:25:04.3258946Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T16:30:50.1786949Z    Compiling serde_json v1.0.40
2019-08-05T16:30:54.3462222Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-05T16:31:02.8425266Z     Finished release [optimized] target(s) in 1m 27s
2019-08-05T16:31:03.4407251Z tidy check
2019-08-05T16:31:03.4408392Z tidy error: /checkout/src/test/ui/async-await/drop-order/drop-order-when-cancelled.rs:202: trailing whitespace
2019-08-05T16:31:04.7512593Z some tidy checks failed
2019-08-05T16:31:04.7513400Z 
2019-08-05T16:31:04.7513400Z 
2019-08-05T16:31:04.7514539Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-05T16:31:04.7514888Z 
2019-08-05T16:31:04.7514974Z 
2019-08-05T16:31:04.7518122Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-05T16:31:04.7518357Z Build completed unsuccessfully in 0:01:30
2019-08-05T16:31:04.7518357Z Build completed unsuccessfully in 0:01:30
2019-08-05T16:31:06.2473775Z ##[error]Bash exited with code '1'.
2019-08-05T16:31:06.2523112Z ##[section]Starting: Checkout
2019-08-05T16:31:06.2524581Z ==============================================================================
2019-08-05T16:31:06.2524635Z Task         : Get sources
2019-08-05T16:31:06.2524676Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

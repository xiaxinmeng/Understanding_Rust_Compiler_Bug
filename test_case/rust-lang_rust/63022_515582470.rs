plain
2019-07-26T19:46:20.0780415Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T19:46:20.1021076Z ##[command]git config gc.auto 0
2019-07-26T19:46:20.1095592Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T19:46:20.1150525Z ##[command]git config --get-all http.proxy
2019-07-26T19:46:20.1292200Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63022/merge:refs/remotes/pull/63022/merge
---
2019-07-26T19:46:54.0587008Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T19:46:54.0587042Z 
2019-07-26T19:46:54.0587269Z   git checkout -b <new-branch-name>
2019-07-26T19:46:54.0587300Z 
2019-07-26T19:46:54.0587369Z HEAD is now at 638dbe97f Merge 08b8b7d4924d0902eadc5a03f289c922157fb0bc into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T19:46:54.0773908Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T19:46:54.0777763Z ==============================================================================
2019-07-26T19:46:54.0777824Z Task         : Bash
2019-07-26T19:46:54.0777888Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T19:53:10.0004936Z    Compiling serde_json v1.0.40
2019-07-26T19:53:14.3531960Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T19:53:23.2054365Z     Finished release [optimized] target(s) in 1m 28s
2019-07-26T19:53:23.2124767Z tidy check
2019-07-26T19:53:23.3655914Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:1592: trailing whitespace
2019-07-26T19:53:23.3656849Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:1596: trailing whitespace
2019-07-26T19:53:25.1944725Z some tidy checks failed
2019-07-26T19:53:25.1945202Z 
2019-07-26T19:53:25.1945202Z 
2019-07-26T19:53:25.1947123Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T19:53:25.1947352Z 
2019-07-26T19:53:25.1947376Z 
2019-07-26T19:53:25.1960429Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T19:53:25.1960714Z Build completed unsuccessfully in 0:01:31
2019-07-26T19:53:25.1960714Z Build completed unsuccessfully in 0:01:31
2019-07-26T19:53:26.5772658Z ##[error]Bash exited with code '1'.
2019-07-26T19:53:26.5826138Z ##[section]Starting: Checkout
2019-07-26T19:53:26.5828819Z ==============================================================================
2019-07-26T19:53:26.5828867Z Task         : Get sources
2019-07-26T19:53:26.5828924Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

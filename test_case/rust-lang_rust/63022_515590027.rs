plain
2019-07-26T20:11:30.7850846Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T20:11:30.8034310Z ##[command]git config gc.auto 0
2019-07-26T20:11:30.8107924Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T20:11:30.8165596Z ##[command]git config --get-all http.proxy
2019-07-26T20:11:30.8296477Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63022/merge:refs/remotes/pull/63022/merge
---
2019-07-26T20:12:06.1148506Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T20:12:06.1148538Z 
2019-07-26T20:12:06.1148756Z   git checkout -b <new-branch-name>
2019-07-26T20:12:06.1148785Z 
2019-07-26T20:12:06.1148835Z HEAD is now at f7ecaf5c2 Merge 3615cc28d0664191d680a963267515f7abb83c20 into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T20:12:06.1335613Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T20:12:06.1339151Z ==============================================================================
2019-07-26T20:12:06.1339211Z Task         : Bash
2019-07-26T20:12:06.1339258Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T20:18:24.7679924Z    Compiling serde_json v1.0.40
2019-07-26T20:18:29.1322223Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T20:18:37.8355605Z     Finished release [optimized] target(s) in 1m 30s
2019-07-26T20:18:37.8422147Z tidy check
2019-07-26T20:18:38.0009820Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:1592: trailing whitespace
2019-07-26T20:18:38.0010627Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:1596: trailing whitespace
2019-07-26T20:18:39.7303145Z some tidy checks failed
2019-07-26T20:18:39.7305147Z 
2019-07-26T20:18:39.7305147Z 
2019-07-26T20:18:39.7306695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T20:18:39.7307670Z 
2019-07-26T20:18:39.7307768Z 
2019-07-26T20:18:39.7318078Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T20:18:39.7318210Z Build completed unsuccessfully in 0:01:33
2019-07-26T20:18:39.7318210Z Build completed unsuccessfully in 0:01:33
2019-07-26T20:18:41.0684123Z ##[error]Bash exited with code '1'.
2019-07-26T20:18:41.0717667Z ##[section]Starting: Checkout
2019-07-26T20:18:41.0719166Z ==============================================================================
2019-07-26T20:18:41.0719232Z Task         : Get sources
2019-07-26T20:18:41.0719271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

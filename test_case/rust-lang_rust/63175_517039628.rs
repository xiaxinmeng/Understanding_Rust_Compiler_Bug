plain
2019-07-31T21:44:39.1595484Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T21:44:39.1777303Z ##[command]git config gc.auto 0
2019-07-31T21:44:39.1851197Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T21:44:39.1899141Z ##[command]git config --get-all http.proxy
2019-07-31T21:44:39.2050902Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63175/merge:refs/remotes/pull/63175/merge
---
2019-07-31T21:45:16.9271457Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T21:45:16.9271488Z 
2019-07-31T21:45:16.9271720Z   git checkout -b <new-branch-name>
2019-07-31T21:45:16.9271751Z 
2019-07-31T21:45:16.9271799Z HEAD is now at a33b6907c Merge d52a989c71a8e3dc644a873f3835144bd6040349 into e3976fff44e6ce14c2f92252e6a807800b9aa7c0
2019-07-31T21:45:16.9426443Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T21:45:16.9429887Z ==============================================================================
2019-07-31T21:45:16.9429951Z Task         : Bash
2019-07-31T21:45:16.9430015Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T21:51:28.6266415Z    Compiling serde_json v1.0.40
2019-07-31T21:51:33.0119672Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-31T21:51:41.8241199Z     Finished release [optimized] target(s) in 1m 32s
2019-07-31T21:51:41.8306234Z tidy check
2019-07-31T21:51:42.1205930Z tidy error: /checkout/src/librustc_driver/args.rs:8: line longer than 100 chars
2019-07-31T21:51:42.1206032Z tidy error: /checkout/src/librustc_driver/args.rs:9: line longer than 100 chars
2019-07-31T21:51:42.1206078Z tidy error: /checkout/src/librustc_driver/args.rs:11: line longer than 100 chars
2019-07-31T21:51:42.1206141Z tidy error: /checkout/src/librustc_driver/args.rs:12: line longer than 100 chars
2019-07-31T21:51:42.1206188Z tidy error: /checkout/src/librustc_driver/args.rs:13: line longer than 100 chars
2019-07-31T21:51:42.1206261Z tidy error: /checkout/src/librustc_driver/args.rs:120: line longer than 100 chars
2019-07-31T21:51:42.1206324Z tidy error: /checkout/src/librustc_driver/args.rs:121: line longer than 100 chars
2019-07-31T21:51:43.3678344Z tidy error: `/checkout/src/librustc_driver/args.rs:166` contains `#[test]`; unit tests and benchmarks must be placed into separate files or directories named `tests.rs`, `benches.rs`, `tests` or `benches`
2019-07-31T21:51:43.6953609Z some tidy checks failed
2019-07-31T21:51:43.6956172Z 
2019-07-31T21:51:43.6956172Z 
2019-07-31T21:51:43.6962388Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-31T21:51:43.6962597Z 
2019-07-31T21:51:43.6962625Z 
2019-07-31T21:51:43.6973516Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-31T21:51:43.6978358Z Build completed unsuccessfully in 0:01:35
2019-07-31T21:51:43.6978358Z Build completed unsuccessfully in 0:01:35
2019-07-31T21:51:45.0073125Z ##[error]Bash exited with code '1'.
2019-07-31T21:51:45.0109403Z ##[section]Starting: Checkout
2019-07-31T21:51:45.0111254Z ==============================================================================
2019-07-31T21:51:45.0111343Z Task         : Get sources
2019-07-31T21:51:45.0111389Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

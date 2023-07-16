plain
2019-08-03T15:00:44.5715589Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T15:00:44.5899379Z ##[command]git config gc.auto 0
2019-08-03T15:00:44.5981211Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T15:00:44.6048014Z ##[command]git config --get-all http.proxy
2019-08-03T15:00:44.6194905Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63240/merge:refs/remotes/pull/63240/merge
---
2019-08-03T15:01:20.1234686Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T15:01:20.1234720Z 
2019-08-03T15:01:20.1234952Z   git checkout -b <new-branch-name>
2019-08-03T15:01:20.1234998Z 
2019-08-03T15:01:20.1235098Z HEAD is now at cb58730d3 Merge 4745a76c7ebf061f549c6284f415147aa597266e into 8e917f48382c6afaf50568263b89d35fba5d98e4
2019-08-03T15:01:20.1391194Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-03T15:01:20.1394027Z ==============================================================================
2019-08-03T15:01:20.1394099Z Task         : Bash
2019-08-03T15:01:20.1394142Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-03T15:07:31.3047541Z    Compiling serde_json v1.0.40
2019-08-03T15:07:35.7273256Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-03T15:07:44.7256935Z     Finished release [optimized] target(s) in 1m 32s
2019-08-03T15:07:44.7338175Z tidy check
2019-08-03T15:07:44.9328058Z tidy error: /checkout/src/test/ui/privacy/legacy-ctor-visibility.rs: ignoring line length unnecessarily
2019-08-03T15:07:46.7507448Z some tidy checks failed
2019-08-03T15:07:46.7515022Z 
2019-08-03T15:07:46.7515022Z 
2019-08-03T15:07:46.7516400Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-03T15:07:46.7517060Z 
2019-08-03T15:07:46.7517251Z 
2019-08-03T15:07:46.7523277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-03T15:07:46.7523371Z Build completed unsuccessfully in 0:01:35
2019-08-03T15:07:46.7523371Z Build completed unsuccessfully in 0:01:35
2019-08-03T15:07:48.1127606Z ##[error]Bash exited with code '1'.
2019-08-03T15:07:48.1161535Z ##[section]Starting: Checkout
2019-08-03T15:07:48.1163488Z ==============================================================================
2019-08-03T15:07:48.1163560Z Task         : Get sources
2019-08-03T15:07:48.1163606Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

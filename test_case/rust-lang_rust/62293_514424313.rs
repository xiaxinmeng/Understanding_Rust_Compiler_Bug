plain
2019-07-23T23:29:57.9765597Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T23:29:58.0009568Z ##[command]git config gc.auto 0
2019-07-23T23:29:58.0081795Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T23:29:58.0160380Z ##[command]git config --get-all http.proxy
2019-07-23T23:29:58.0310174Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62293/merge:refs/remotes/pull/62293/merge
---
2019-07-23T23:30:33.7525807Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T23:30:33.7526075Z 
2019-07-23T23:30:33.7526519Z   git checkout -b <new-branch-name>
2019-07-23T23:30:33.7526810Z 
2019-07-23T23:30:33.7527047Z HEAD is now at c831287cb Merge 02bb3a790a5989867a96f656590fd7820da10375 into 299ef86e1f8b3e53154f834115752c719b611fa1
2019-07-23T23:30:33.7670810Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T23:30:33.7673669Z ==============================================================================
2019-07-23T23:30:33.7673727Z Task         : Bash
2019-07-23T23:30:33.7673772Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T23:37:08.2039846Z     Finished release [optimized] target(s) in 1m 32s
2019-07-23T23:37:08.2111115Z tidy check
2019-07-23T23:37:09.1525010Z * 577 error codes
2019-07-23T23:37:09.1525532Z * highest error code: E0732
2019-07-23T23:37:09.1525645Z tidy error: libsyntax/feature_gate.rs:642: feature await_macro is not sorted by since
2019-07-23T23:37:10.0731746Z some tidy checks failed
2019-07-23T23:37:10.0732135Z 
2019-07-23T23:37:10.0732135Z 
2019-07-23T23:37:10.0732984Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-23T23:37:10.0733172Z 
2019-07-23T23:37:10.0733196Z 
2019-07-23T23:37:10.0746547Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-23T23:37:10.0746630Z Build completed unsuccessfully in 0:01:35
2019-07-23T23:37:10.0746630Z Build completed unsuccessfully in 0:01:35
2019-07-23T23:37:11.5642645Z ##[error]Bash exited with code '1'.
2019-07-23T23:37:11.5694483Z ##[section]Starting: Checkout
2019-07-23T23:37:11.5696852Z ==============================================================================
2019-07-23T23:37:11.5696910Z Task         : Get sources
2019-07-23T23:37:11.5696958Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-09T18:58:56.1753702Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T18:58:57.0631482Z ##[command]git config gc.auto 0
2019-08-09T18:58:57.0638203Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T18:58:57.0644501Z ##[command]git config --get-all http.proxy
2019-08-09T18:58:57.0650685Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-09T18:59:33.8038702Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T18:59:33.8040515Z 
2019-08-09T18:59:33.8041821Z   git checkout -b <new-branch-name>
2019-08-09T18:59:33.8042711Z 
2019-08-09T18:59:33.8131939Z HEAD is now at ea669ca31 Merge 6167608f4df2aae92a3e8dcecb81507e63655107 into 534b42394d743511db1335d5ed08d507ab7c6e73
2019-08-09T18:59:33.8208172Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T18:59:33.8211330Z ==============================================================================
2019-08-09T18:59:33.8211390Z Task         : Bash
2019-08-09T18:59:33.8211435Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T19:05:34.7726278Z    Compiling serde_json v1.0.40
2019-08-09T19:05:39.1898687Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-09T19:05:47.9965855Z     Finished release [optimized] target(s) in 1m 31s
2019-08-09T19:05:48.0034284Z tidy check
2019-08-09T19:05:48.4119332Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-09T19:05:48.4140065Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-09T19:05:48.4140161Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-09T19:05:48.4140256Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-09T19:05:48.4145268Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-09T19:05:48.4145664Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-09T19:05:48.4145865Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-09T19:05:48.4146041Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-09T19:05:48.4146266Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-09T19:05:48.4146473Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-09T19:05:48.4146662Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-09T19:05:48.4146834Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-09T19:05:48.4147004Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-09T19:05:48.4147416Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-09T19:05:48.4147643Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-09T19:05:48.4147837Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-09T19:05:48.4170834Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-09T19:05:48.4275590Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-09T19:05:48.4275717Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-09T19:05:48.4275792Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-09T19:05:48.4275861Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-09T19:05:48.4276175Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-09T19:05:48.4310419Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-09T19:05:48.8498312Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-09T19:05:50.0183022Z some tidy checks failed
2019-08-09T19:05:50.0187925Z 
2019-08-09T19:05:50.0187925Z 
2019-08-09T19:05:50.0188780Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-09T19:05:50.0188925Z 
2019-08-09T19:05:50.0188951Z 
2019-08-09T19:05:50.0203994Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-09T19:05:50.0204086Z Build completed unsuccessfully in 0:01:34
2019-08-09T19:05:50.0204086Z Build completed unsuccessfully in 0:01:34
2019-08-09T19:05:51.3409270Z ##[error]Bash exited with code '1'.
2019-08-09T19:05:51.3444948Z ##[section]Starting: Checkout
2019-08-09T19:05:51.3446734Z ==============================================================================
2019-08-09T19:05:51.3446812Z Task         : Get sources
2019-08-09T19:05:51.3446865Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

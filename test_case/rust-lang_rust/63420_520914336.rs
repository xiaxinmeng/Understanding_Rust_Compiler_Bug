plain
2019-08-13T16:26:34.8951060Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T16:26:34.9144223Z ##[command]git config gc.auto 0
2019-08-13T16:26:34.9225964Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T16:26:34.9302426Z ##[command]git config --get-all http.proxy
2019-08-13T16:26:34.9429028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-13T16:27:09.6084022Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T16:27:09.6084054Z 
2019-08-13T16:27:09.6084274Z   git checkout -b <new-branch-name>
2019-08-13T16:27:09.6084304Z 
2019-08-13T16:27:09.6084352Z HEAD is now at ccedcbf60 Merge 60f80e7ea570cda669f2eb67bbbdc78397f81322 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T16:27:09.6232317Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T16:27:09.6235650Z ==============================================================================
2019-08-13T16:27:09.6235710Z Task         : Bash
2019-08-13T16:27:09.6235758Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T16:33:01.7818769Z    Compiling serde_json v1.0.40
2019-08-13T16:33:06.3466365Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T16:33:15.2745438Z     Finished release [optimized] target(s) in 1m 33s
2019-08-13T16:33:15.2822378Z tidy check
2019-08-13T16:33:15.7105603Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-13T16:33:15.7125831Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-13T16:33:15.7125939Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-13T16:33:15.7126030Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-13T16:33:15.7126082Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-13T16:33:15.7126134Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-13T16:33:15.7126201Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-13T16:33:15.7126254Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-13T16:33:15.7126305Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-13T16:33:15.7126375Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-13T16:33:15.7126432Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-13T16:33:15.7126666Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-13T16:33:15.7126747Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-13T16:33:15.7126799Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-13T16:33:15.7126850Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-13T16:33:15.7127100Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-13T16:33:15.7156750Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-13T16:33:15.7255097Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-13T16:33:15.7255210Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-13T16:33:15.7255284Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-13T16:33:15.7255340Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-13T16:33:15.7255410Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-13T16:33:15.7286159Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-13T16:33:16.1130225Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-13T16:33:17.1761840Z some tidy checks failed
2019-08-13T16:33:17.1763280Z 
2019-08-13T16:33:17.1763280Z 
2019-08-13T16:33:17.1764664Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T16:33:17.1765024Z 
2019-08-13T16:33:17.1765062Z 
2019-08-13T16:33:17.1774849Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T16:33:17.1775157Z Build completed unsuccessfully in 0:01:36
2019-08-13T16:33:17.1775157Z Build completed unsuccessfully in 0:01:36
2019-08-13T16:33:18.5310166Z ##[error]Bash exited with code '1'.
2019-08-13T16:33:18.5343306Z ##[section]Starting: Checkout
2019-08-13T16:33:18.5345232Z ==============================================================================
2019-08-13T16:33:18.5345463Z Task         : Get sources
2019-08-13T16:33:18.5345521Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-14T16:23:10.2199607Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T16:23:10.2383149Z ##[command]git config gc.auto 0
2019-08-14T16:23:11.0195915Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T16:23:11.0206458Z ##[command]git config --get-all http.proxy
2019-08-14T16:23:11.0212755Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-14T16:23:45.6182977Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T16:23:45.6183040Z 
2019-08-14T16:23:45.6183264Z   git checkout -b <new-branch-name>
2019-08-14T16:23:45.6183294Z 
2019-08-14T16:23:45.6183341Z HEAD is now at ad82b65fa Merge c6dead04371f736a1098ad395d637ec237eafa74 into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T16:23:45.6330948Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T16:23:45.6334348Z ==============================================================================
2019-08-14T16:23:45.6334405Z Task         : Bash
2019-08-14T16:23:45.6334450Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T16:29:31.5699346Z    Compiling serde_json v1.0.40
2019-08-14T16:29:35.7562094Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-14T16:29:44.2501859Z     Finished release [optimized] target(s) in 1m 27s
2019-08-14T16:29:44.2569136Z tidy check
2019-08-14T16:29:44.6608571Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-14T16:29:44.6630257Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-14T16:29:44.6630344Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-14T16:29:44.6630428Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-14T16:29:44.6630514Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-14T16:29:44.6630581Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-14T16:29:44.6630658Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-14T16:29:44.6630715Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-14T16:29:44.6630786Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-14T16:29:44.6630843Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-14T16:29:44.6630899Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-14T16:29:44.6630971Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-14T16:29:44.6631338Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-14T16:29:44.6631404Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-14T16:29:44.6631475Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-14T16:29:44.6631532Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-14T16:29:44.6661109Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-14T16:29:44.6764775Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-14T16:29:44.6764889Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-14T16:29:44.6803296Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-14T16:29:44.6825715Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-14T16:29:44.6826113Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-14T16:29:44.6826303Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-14T16:29:45.0700866Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-14T16:29:46.1795222Z some tidy checks failed
2019-08-14T16:29:46.1795394Z 
2019-08-14T16:29:46.1795394Z 
2019-08-14T16:29:46.1796291Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-14T16:29:46.1796420Z 
2019-08-14T16:29:46.1796474Z 
2019-08-14T16:29:46.1803751Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-14T16:29:46.1803843Z Build completed unsuccessfully in 0:01:30
2019-08-14T16:29:46.1803843Z Build completed unsuccessfully in 0:01:30
2019-08-14T16:29:46.1851384Z == clock drift check ==
2019-08-14T16:29:46.1862183Z   local time: Wed Aug 14 16:29:46 UTC 2019
2019-08-14T16:29:46.4634165Z   network time: Wed, 14 Aug 2019 16:29:46 GMT
2019-08-14T16:29:46.4634275Z == end clock drift check ==
2019-08-14T16:29:47.8125953Z ##[error]Bash exited with code '1'.
2019-08-14T16:29:47.8156777Z ##[section]Starting: Checkout
2019-08-14T16:29:47.8158339Z ==============================================================================
2019-08-14T16:29:47.8158388Z Task         : Get sources
2019-08-14T16:29:47.8158428Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

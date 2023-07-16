plain
2019-08-14T15:24:03.9809397Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T15:24:03.9985087Z ##[command]git config gc.auto 0
2019-08-14T15:24:04.0073741Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T15:24:04.0140884Z ##[command]git config --get-all http.proxy
2019-08-14T15:24:04.0296882Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-14T15:24:41.4227751Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T15:24:41.4227781Z 
2019-08-14T15:24:41.4227992Z   git checkout -b <new-branch-name>
2019-08-14T15:24:41.4228019Z 
2019-08-14T15:24:41.4228064Z HEAD is now at e51c2bcad Merge 33e24d8b515b80eef47d0aa7048a7a5d28f8aa26 into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T15:24:41.4386056Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T15:24:41.4389043Z ==============================================================================
2019-08-14T15:24:41.4389100Z Task         : Bash
2019-08-14T15:24:41.4389147Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T15:31:05.7850305Z    Compiling serde_json v1.0.40
2019-08-14T15:31:10.5104353Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-14T15:31:19.6355771Z     Finished release [optimized] target(s) in 1m 35s
2019-08-14T15:31:19.6429573Z tidy check
2019-08-14T15:31:20.0740212Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-14T15:31:20.0740375Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-14T15:31:20.0740511Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-14T15:31:20.0740569Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-14T15:31:20.0740625Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-14T15:31:20.0740700Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-14T15:31:20.0740755Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-14T15:31:20.0740810Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-14T15:31:20.0740884Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-14T15:31:20.0740946Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-14T15:31:20.0741008Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-14T15:31:20.0741081Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-14T15:31:20.0741136Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-14T15:31:20.0741190Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-14T15:31:20.0741263Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-14T15:31:20.0741481Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-14T15:31:20.0747397Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-14T15:31:20.0847327Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-14T15:31:20.0847450Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-14T15:31:20.0847537Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-14T15:31:20.0847597Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-14T15:31:20.0847655Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-14T15:31:20.0880680Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-14T15:31:20.5236130Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-14T15:31:21.7527244Z some tidy checks failed
2019-08-14T15:31:21.7528574Z 
2019-08-14T15:31:21.7528574Z 
2019-08-14T15:31:21.7530165Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-14T15:31:21.7531079Z 
2019-08-14T15:31:21.7531187Z 
2019-08-14T15:31:21.7541259Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-14T15:31:21.7541541Z Build completed unsuccessfully in 0:01:38
2019-08-14T15:31:21.7541541Z Build completed unsuccessfully in 0:01:38
2019-08-14T15:31:21.7598859Z == clock drift check ==
2019-08-14T15:31:21.7607773Z   local time: Wed Aug 14 15:31:21 UTC 2019
2019-08-14T15:31:21.9205496Z   network time: Wed, 14 Aug 2019 15:31:21 GMT
2019-08-14T15:31:21.9205667Z == end clock drift check ==
2019-08-14T15:31:24.0464697Z ##[error]Bash exited with code '1'.
2019-08-14T15:31:24.0497871Z ##[section]Starting: Checkout
2019-08-14T15:31:24.0499492Z ==============================================================================
2019-08-14T15:31:24.0499558Z Task         : Get sources
2019-08-14T15:31:24.0499619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

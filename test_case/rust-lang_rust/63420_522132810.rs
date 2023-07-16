plain
2019-08-16T19:38:33.6389952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T19:38:33.6616888Z ##[command]git config gc.auto 0
2019-08-16T19:38:33.6698091Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T19:38:33.6771385Z ##[command]git config --get-all http.proxy
2019-08-16T19:38:34.5944160Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-16T19:39:07.9561391Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T19:39:07.9561423Z 
2019-08-16T19:39:07.9561684Z   git checkout -b <new-branch-name>
2019-08-16T19:39:07.9561734Z 
2019-08-16T19:39:07.9561787Z HEAD is now at 9babba00b Merge e6c89d3965ed16433502fb4e1a23a68501d4b00e into 9a32ad0dd51f8451aa6e39d7e9ea89483cb8fcfa
2019-08-16T19:39:07.9721073Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T19:39:07.9723904Z ==============================================================================
2019-08-16T19:39:07.9723959Z Task         : Bash
2019-08-16T19:39:07.9724003Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T19:45:29.0731698Z    Compiling serde_json v1.0.40
2019-08-16T19:45:30.8764102Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-16T19:45:41.7777948Z     Finished release [optimized] target(s) in 1m 31s
2019-08-16T19:45:41.7909904Z tidy check
2019-08-16T19:45:42.2014869Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-16T19:45:42.2040313Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-16T19:45:42.2040413Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-16T19:45:42.2040478Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-16T19:45:42.2040561Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-16T19:45:42.2040626Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-16T19:45:42.2047068Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-16T19:45:42.2047560Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-16T19:45:42.2047813Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-16T19:45:42.2048097Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-16T19:45:42.2048281Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-16T19:45:42.2048478Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-16T19:45:42.2048649Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-16T19:45:42.2048837Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-16T19:45:42.2049010Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-16T19:45:42.2049190Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-16T19:45:42.2074815Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-16T19:45:42.2182960Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-16T19:45:42.2183089Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-16T19:45:42.2183162Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-16T19:45:42.2183230Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-16T19:45:42.2183316Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-16T19:45:42.2220940Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-16T19:45:42.6236653Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-16T19:45:43.7827791Z some tidy checks failed
2019-08-16T19:45:43.7829258Z 
2019-08-16T19:45:43.7829258Z 
2019-08-16T19:45:43.7830304Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-16T19:45:43.7830640Z 
2019-08-16T19:45:43.7830770Z 
2019-08-16T19:45:43.7850598Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-16T19:45:43.7850671Z Build completed unsuccessfully in 0:01:34
2019-08-16T19:45:43.7850671Z Build completed unsuccessfully in 0:01:34
2019-08-16T19:45:43.7902554Z == clock drift check ==
2019-08-16T19:45:43.7917370Z   local time: Fri Aug 16 19:45:43 UTC 2019
2019-08-16T19:45:43.9419691Z   network time: Fri, 16 Aug 2019 19:45:43 GMT
2019-08-16T19:45:43.9421564Z == end clock drift check ==
2019-08-16T19:45:45.3126075Z ##[error]Bash exited with code '1'.
2019-08-16T19:45:45.3160173Z ##[section]Starting: Checkout
2019-08-16T19:45:45.3162036Z ==============================================================================
2019-08-16T19:45:45.3162099Z Task         : Get sources
2019-08-16T19:45:45.3162168Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

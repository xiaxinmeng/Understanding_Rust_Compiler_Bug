plain
2019-08-15T22:04:49.3944107Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T22:04:49.4143255Z ##[command]git config gc.auto 0
2019-08-15T22:04:49.4231229Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T22:04:49.4284170Z ##[command]git config --get-all http.proxy
2019-08-15T22:04:49.4441642Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-15T22:05:23.8310107Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T22:05:23.8310143Z 
2019-08-15T22:05:23.8310507Z   git checkout -b <new-branch-name>
2019-08-15T22:05:23.8310544Z 
2019-08-15T22:05:23.8310597Z HEAD is now at c90ea6c31 Merge 944375d1070d32195d33de02e2896575bfb1cfab into f7af19c279b8b7ea3d2c21fcbd67164af8d5d968
2019-08-15T22:05:23.8485193Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T22:05:23.8488203Z ==============================================================================
2019-08-15T22:05:23.8488285Z Task         : Bash
2019-08-15T22:05:23.8488340Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T22:12:00.6038027Z    Compiling serde_json v1.0.40
2019-08-15T22:12:02.4797323Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-15T22:12:13.6704668Z     Finished release [optimized] target(s) in 1m 33s
2019-08-15T22:12:13.6771096Z tidy check
2019-08-15T22:12:14.0982359Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-15T22:12:14.1001956Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-15T22:12:14.1002054Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-15T22:12:14.1002218Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-15T22:12:14.1002301Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-15T22:12:14.1002380Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-15T22:12:14.1002444Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-15T22:12:14.1002524Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-15T22:12:14.1002587Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-15T22:12:14.1002651Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-15T22:12:14.1002732Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-15T22:12:14.1002805Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-15T22:12:14.1002869Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-15T22:12:14.1002957Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-15T22:12:14.1003022Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-15T22:12:14.1003101Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-15T22:12:14.1030257Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-15T22:12:14.1139256Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-15T22:12:14.1139380Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-15T22:12:14.1139662Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-15T22:12:14.1139746Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-15T22:12:14.1139843Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-15T22:12:14.1174008Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-15T22:12:14.5611338Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-15T22:12:15.7534436Z some tidy checks failed
2019-08-15T22:12:15.7535763Z 
2019-08-15T22:12:15.7535763Z 
2019-08-15T22:12:15.7536973Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-15T22:12:15.7537642Z 
2019-08-15T22:12:15.7537898Z 
2019-08-15T22:12:15.7541711Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-15T22:12:15.7542358Z Build completed unsuccessfully in 0:01:36
2019-08-15T22:12:15.7542358Z Build completed unsuccessfully in 0:01:36
2019-08-15T22:12:15.7598847Z == clock drift check ==
2019-08-15T22:12:15.7612490Z   local time: Thu Aug 15 22:12:15 UTC 2019
2019-08-15T22:12:15.9151139Z   network time: Thu, 15 Aug 2019 22:12:15 GMT
2019-08-15T22:12:15.9153280Z == end clock drift check ==
2019-08-15T22:12:17.5699040Z ##[error]Bash exited with code '1'.
2019-08-15T22:12:17.5747277Z ##[section]Starting: Checkout
2019-08-15T22:12:17.5749191Z ==============================================================================
2019-08-15T22:12:17.5749251Z Task         : Get sources
2019-08-15T22:12:17.5749319Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

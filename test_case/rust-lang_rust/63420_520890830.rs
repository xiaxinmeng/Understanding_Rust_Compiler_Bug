plain
2019-08-13T15:26:17.6897599Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T15:26:17.7058682Z ##[command]git config gc.auto 0
2019-08-13T15:26:17.7132420Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T15:26:17.7195444Z ##[command]git config --get-all http.proxy
2019-08-13T15:26:17.7345722Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-13T15:26:53.0664207Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T15:26:53.0664803Z 
2019-08-13T15:26:53.0666121Z   git checkout -b <new-branch-name>
2019-08-13T15:26:53.0666926Z 
2019-08-13T15:26:53.0668851Z HEAD is now at 976bbd9ee Merge d384b63ccce9203761004da18bd88f5a47190cd5 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T15:26:53.0818216Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T15:26:53.0821374Z ==============================================================================
2019-08-13T15:26:53.0821433Z Task         : Bash
2019-08-13T15:26:53.0821479Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T15:32:47.8135364Z    Compiling serde_json v1.0.40
2019-08-13T15:32:51.9386855Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T15:33:00.2549408Z     Finished release [optimized] target(s) in 1m 26s
2019-08-13T15:33:00.2618302Z tidy check
2019-08-13T15:33:00.7231889Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-13T15:33:00.7257419Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-13T15:33:00.7257501Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-13T15:33:00.7257583Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-13T15:33:00.7267408Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-13T15:33:00.7267487Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-13T15:33:00.7267613Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-13T15:33:00.7267684Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-13T15:33:00.7267739Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-13T15:33:00.7267813Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-13T15:33:00.7267867Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-13T15:33:00.7267936Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-13T15:33:00.7267996Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-13T15:33:00.7268247Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-13T15:33:00.7268322Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-13T15:33:00.7268387Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-13T15:33:00.7290793Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-13T15:33:00.7374499Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-13T15:33:00.7379211Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-13T15:33:00.7379311Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-13T15:33:00.7379372Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-13T15:33:00.7379441Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-13T15:33:00.7408302Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-13T15:33:01.0999687Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-13T15:33:02.1418038Z some tidy checks failed
2019-08-13T15:33:02.1418846Z 
2019-08-13T15:33:02.1418846Z 
2019-08-13T15:33:02.1420360Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T15:33:02.1420543Z 
2019-08-13T15:33:02.1420572Z 
2019-08-13T15:33:02.1427907Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T15:33:02.1428223Z Build completed unsuccessfully in 0:01:29
2019-08-13T15:33:02.1428223Z Build completed unsuccessfully in 0:01:29
2019-08-13T15:33:03.4262458Z ##[error]Bash exited with code '1'.
2019-08-13T15:33:03.4294423Z ##[section]Starting: Checkout
2019-08-13T15:33:03.4295853Z ==============================================================================
2019-08-13T15:33:03.4295897Z Task         : Get sources
2019-08-13T15:33:03.4295952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

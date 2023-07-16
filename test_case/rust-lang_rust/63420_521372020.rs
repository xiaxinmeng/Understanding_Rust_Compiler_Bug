plain
2019-08-14T18:34:13.4334937Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T18:34:13.4485273Z ##[command]git config gc.auto 0
2019-08-14T18:34:13.4560007Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T18:34:13.4612552Z ##[command]git config --get-all http.proxy
2019-08-14T18:34:13.4745391Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-14T18:34:49.6053873Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T18:34:49.6053951Z 
2019-08-14T18:34:49.6054534Z   git checkout -b <new-branch-name>
2019-08-14T18:34:49.6054615Z 
2019-08-14T18:34:49.6054700Z HEAD is now at f92dd7233 Merge 1ef4cfe61b0e465f12ecc87d2fdf3d4fabf76d3c into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T18:34:49.6213373Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T18:34:49.6216992Z ==============================================================================
2019-08-14T18:34:49.6217062Z Task         : Bash
2019-08-14T18:34:49.6217134Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T18:40:47.4864473Z    Compiling serde_json v1.0.40
2019-08-14T18:40:51.7450554Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-14T18:41:00.4586628Z     Finished release [optimized] target(s) in 1m 29s
2019-08-14T18:41:00.4656144Z tidy check
2019-08-14T18:41:00.8946868Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-14T18:41:00.8965832Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-14T18:41:00.8966846Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-14T18:41:00.8966951Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-14T18:41:00.8967022Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-14T18:41:00.8967092Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-14T18:41:00.8967179Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-14T18:41:00.8967266Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-14T18:41:00.8967339Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-14T18:41:00.8967439Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-14T18:41:00.8967509Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-14T18:41:00.8967595Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-14T18:41:00.8967666Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-14T18:41:00.8967735Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-14T18:41:00.8967819Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-14T18:41:00.8967897Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-14T18:41:00.9029056Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-14T18:41:00.9104256Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-14T18:41:00.9104640Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-14T18:41:00.9104724Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-14T18:41:00.9114283Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-14T18:41:00.9114402Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-14T18:41:00.9144451Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-14T18:41:01.2921141Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-14T18:41:02.3793361Z some tidy checks failed
2019-08-14T18:41:02.3795535Z 
2019-08-14T18:41:02.3795535Z 
2019-08-14T18:41:02.3800290Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-14T18:41:02.3801096Z 
2019-08-14T18:41:02.3801193Z 
2019-08-14T18:41:02.3813854Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-14T18:41:02.3813973Z Build completed unsuccessfully in 0:01:32
2019-08-14T18:41:02.3813973Z Build completed unsuccessfully in 0:01:32
2019-08-14T18:41:02.3868298Z == clock drift check ==
2019-08-14T18:41:02.3878053Z   local time: Wed Aug 14 18:41:02 UTC 2019
2019-08-14T18:41:02.4549668Z   network time: Wed, 14 Aug 2019 18:41:02 GMT
2019-08-14T18:41:02.4550455Z == end clock drift check ==
2019-08-14T18:41:04.4426484Z ##[error]Bash exited with code '1'.
2019-08-14T18:41:04.4484452Z ##[section]Starting: Checkout
2019-08-14T18:41:04.4486538Z ==============================================================================
2019-08-14T18:41:04.4486608Z Task         : Get sources
2019-08-14T18:41:04.4486683Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

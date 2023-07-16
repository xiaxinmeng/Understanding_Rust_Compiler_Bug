plain
2019-08-15T03:00:00.1946169Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T03:00:00.2149175Z ##[command]git config gc.auto 0
2019-08-15T03:00:00.2232092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T03:00:00.2275401Z ##[command]git config --get-all http.proxy
2019-08-15T03:00:00.2419690Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-15T03:00:34.1833981Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T03:00:34.1834029Z 
2019-08-15T03:00:34.1834258Z   git checkout -b <new-branch-name>
2019-08-15T03:00:34.1834291Z 
2019-08-15T03:00:34.1834344Z HEAD is now at 63f797e93 Merge 009442b3ff52a54f62aec9bae9d89bf0d6a1e8d9 into 082cf2f9d136166cd1d552d3fb5abb1c46c99a14
2019-08-15T03:00:34.1994156Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T03:00:34.1997263Z ==============================================================================
2019-08-15T03:00:34.1997323Z Task         : Bash
2019-08-15T03:00:34.1997383Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T03:03:01.8119139Z Looks like docker image is the same as before, not uploading
2019-08-15T03:03:11.2642948Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-15T03:03:11.4218296Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-15T03:03:11.4248396Z == clock drift check ==
2019-08-15T03:03:11.4257281Z   local time: Thu Aug 15 03:03:11 UTC 2019
2019-08-15T03:03:11.5113008Z   network time: Thu, 15 Aug 2019 03:03:11 GMT
2019-08-15T03:03:11.5140128Z Starting sccache server...
2019-08-15T03:03:11.6043573Z configure: processing command line
2019-08-15T03:03:11.6043734Z configure: 
2019-08-15T03:03:11.6044624Z configure: rust.dist-src        := False
---
2019-08-15T03:06:59.0009885Z    Compiling serde_json v1.0.40
2019-08-15T03:07:00.9265517Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-15T03:07:11.8856470Z     Finished release [optimized] target(s) in 1m 30s
2019-08-15T03:07:11.8930281Z tidy check
2019-08-15T03:07:12.3084406Z tidy error: /checkout/src/librustc_mir/build/expr/as_rvalue.rs:524: line longer than 100 chars
2019-08-15T03:07:12.3103182Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:122: line longer than 100 chars
2019-08-15T03:07:12.3103337Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:125: line longer than 100 chars
2019-08-15T03:07:12.3103433Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:126: line longer than 100 chars
2019-08-15T03:07:12.3103499Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:127: line longer than 100 chars
2019-08-15T03:07:12.3103581Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:129: line longer than 100 chars
2019-08-15T03:07:12.3103644Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:131: line longer than 100 chars
2019-08-15T03:07:12.3103719Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:132: line longer than 100 chars
2019-08-15T03:07:12.3103801Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:133: line longer than 100 chars
2019-08-15T03:07:12.3103864Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:134: line longer than 100 chars
2019-08-15T03:07:12.3103927Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:137: line longer than 100 chars
2019-08-15T03:07:12.3104210Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:138: line longer than 100 chars
2019-08-15T03:07:12.3104285Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:139: line longer than 100 chars
2019-08-15T03:07:12.3104349Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:141: line longer than 100 chars
2019-08-15T03:07:12.3104427Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:142: line longer than 100 chars
2019-08-15T03:07:12.3104500Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:145: line longer than 100 chars
2019-08-15T03:07:12.3129828Z tidy error: /checkout/src/librustc_mir/borrow_check/move_errors.rs:314: line longer than 100 chars
2019-08-15T03:07:12.3250548Z tidy error: /checkout/src/librustc_mir/borrow_check/prefixes.rs:156: line longer than 100 chars
2019-08-15T03:07:12.3250673Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:162: line longer than 100 chars
2019-08-15T03:07:12.3250755Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:180: line longer than 100 chars
2019-08-15T03:07:12.3290331Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:211: line longer than 100 chars
2019-08-15T03:07:12.3290438Z tidy error: /checkout/src/librustc_mir/borrow_check/places_conflict.rs:292: line longer than 100 chars
2019-08-15T03:07:12.3290498Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:826: line longer than 100 chars
2019-08-15T03:07:12.7435154Z tidy error: /checkout/src/librustc/mir/visit.rs:698: line longer than 100 chars
2019-08-15T03:07:13.9045919Z some tidy checks failed
2019-08-15T03:07:13.9046041Z 
2019-08-15T03:07:13.9046041Z 
2019-08-15T03:07:13.9046869Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-15T03:07:13.9047027Z 
2019-08-15T03:07:13.9047054Z 
2019-08-15T03:07:13.9052518Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-15T03:07:13.9052591Z Build completed unsuccessfully in 0:01:34
2019-08-15T03:07:13.9052591Z Build completed unsuccessfully in 0:01:34
2019-08-15T03:07:13.9101104Z == clock drift check ==
2019-08-15T03:07:13.9114698Z   local time: Thu Aug 15 03:07:13 UTC 2019
2019-08-15T03:07:14.2015469Z   network time: Thu, 15 Aug 2019 03:07:14 GMT
2019-08-15T03:07:14.2016873Z == end clock drift check ==
2019-08-15T03:07:15.6631932Z ##[error]Bash exited with code '1'.
2019-08-15T03:07:15.6677297Z ##[section]Starting: Checkout
2019-08-15T03:07:15.6679278Z ==============================================================================
2019-08-15T03:07:15.6679363Z Task         : Get sources
2019-08-15T03:07:15.6679420Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

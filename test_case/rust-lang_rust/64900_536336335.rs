plain
2019-09-29T17:10:16.5430625Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T17:10:16.5666382Z ##[command]git config gc.auto 0
2019-09-29T17:10:16.5743039Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T17:10:16.5794160Z ##[command]git config --get-all http.proxy
2019-09-29T17:10:16.5949006Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64900/merge:refs/remotes/pull/64900/merge
---
2019-09-29T19:02:13.7289338Z    Compiling mdbook-linkcheck v0.3.0
2019-09-29T19:02:27.8136668Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-29T19:02:32.5599246Z     Finished release [optimized] target(s) in 8m 38s
2019-09-29T19:04:30.4648811Z Error: there are broken links
2019-09-29T19:04:30.4650706Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-09-29T19:04:30.4652131Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-09-29T19:04:30.4652785Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-09-29T19:04:30.4653390Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-09-29T19:04:30.4653917Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-09-29T19:23:06.3610754Z    |
2019-09-29T19:23:06.3610797Z    = note: `#[deny(const_err)]` on by default
2019-09-29T19:23:06.3610828Z 
2019-09-29T19:23:06.3611213Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-29T19:23:06.3611261Z   left: `Size { raw: 0 }`,
2019-09-29T19:23:06.3611525Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-29T19:23:06.3611628Z 
2019-09-29T19:23:06.3611669Z error: internal compiler error: unexpected panic
2019-09-29T19:23:06.3611696Z 
2019-09-29T19:23:06.3611752Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-29T19:23:06.3611752Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-29T19:23:06.3611781Z 
2019-09-29T19:23:06.3612179Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-29T19:23:06.3612620Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-29T19:23:06.3612651Z 
2019-09-29T19:23:06.3613530Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-29T19:23:06.3613573Z 
---
2019-09-29T19:23:06.3622367Z -LL |     &empty[1..5];
2019-09-29T19:23:06.3622769Z -   |            ^
2019-09-29T19:23:06.3622953Z -   |
2019-09-29T19:23:06.3623210Z -   = note: `-D clippy::out-of-bounds-indexing` implied by `-D warnings`
2019-09-29T19:23:06.3623950Z +thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-29T19:23:06.3624039Z +  left: `Size { raw: 0 }`,
2019-09-29T19:23:06.3624978Z + right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-29T19:23:06.3625060Z +note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-29T19:23:06.3625661Z -error: range is out of bounds
2019-09-29T19:23:06.3625913Z -  --> $DIR/empty_array.rs:8:16
2019-09-29T19:23:06.3626096Z -   |
2019-09-29T19:23:06.3626293Z -LL |     &empty[0..=4];
---
2019-09-29T19:23:06.3642108Z -  --> $DIR/empty_array.rs:10:12
2019-09-29T19:23:06.3642415Z -   |
2019-09-29T19:23:06.3642628Z -LL |     &empty[1..];
2019-09-29T19:23:06.3643305Z -   |            ^
2019-09-29T19:23:06.3644369Z +note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-29T19:23:06.3645544Z -error: range is out of bounds
2019-09-29T19:23:06.3645773Z -  --> $DIR/empty_array.rs:11:14
2019-09-29T19:23:06.3645968Z -   |
2019-09-29T19:23:06.3646157Z -LL |     &empty[..4];
---
2019-09-29T19:23:06.3648103Z -  --> $DIR/empty_array.rs:12:16
2019-09-29T19:23:06.3648301Z -   |
2019-09-29T19:23:06.3648501Z -LL |     &empty[0..=0];
2019-09-29T19:23:06.3648696Z -   |                ^
2019-09-29T19:23:06.3648932Z +note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-29T19:23:06.3649184Z -error: range is out of bounds
2019-09-29T19:23:06.3649403Z -  --> $DIR/empty_array.rs:13:15
2019-09-29T19:23:06.3649588Z -   |
2019-09-29T19:23:06.3649784Z -LL |     &empty[..=0];
---
2019-09-29T19:23:06.3651525Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'out_of_bounds_indexing/empty_array.rs'
2019-09-29T19:23:06.3651689Z 
2019-09-29T19:23:06.3651731Z error: 1 errors occurred comparing output.
2019-09-29T19:23:06.3651773Z status: exit code: 101
2019-09-29T19:23:06.3653316Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/out_of_bounds_indexing/empty_array.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id.aux" "-A" "unused"
2019-09-29T19:23:06.3653834Z ------------------------------------------
2019-09-29T19:23:06.3653871Z 
2019-09-29T19:23:06.3654111Z ------------------------------------------
2019-09-29T19:23:06.3654157Z stderr:
2019-09-29T19:23:06.3654157Z stderr:
2019-09-29T19:23:06.3654395Z ------------------------------------------
2019-09-29T19:23:06.3655351Z {"message":"index out of bounds: the len is 0 but the index is 0","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/out_of_bounds_indexing/empty_array.rs","byte_start":147,"byte_end":155,"line_start":6,"line_end":6,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    empty[0];","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(const_err)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: index out of bounds: the len is 0 but the index is 0\n  --> tests/ui/out_of_bounds_indexing/empty_array.rs:6:5\n   |\nLL |     empty[0];\n   |     ^^^^^^^^\n   |\n   = note: `#[deny(const_err)]` on by default\n\n"}
2019-09-29T19:23:06.3655807Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-29T19:23:06.3655861Z   left: `Size { raw: 0 }`,
2019-09-29T19:23:06.3656165Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-29T19:23:06.3656263Z 
2019-09-29T19:23:06.3656316Z error: internal compiler error: unexpected panic
2019-09-29T19:23:06.3656346Z 
2019-09-29T19:23:06.3656391Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-29T19:23:06.3656391Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-29T19:23:06.3656422Z 
2019-09-29T19:23:06.3657110Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-29T19:23:06.3657497Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-29T19:23:06.3657552Z 
2019-09-29T19:23:06.3657786Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-29T19:23:06.3657819Z 
---
2019-09-29T20:06:49.3420781Z Verifying status of rustfmt...
2019-09-29T20:06:49.3436183Z Verifying status of clippy-driver...
2019-09-29T20:06:49.3451528Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-29T20:06:49.3462437Z 
2019-09-29T20:06:49.3463074Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-29T20:06:49.3463141Z 
2019-09-29T20:06:49.3463489Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-29T20:06:49.3463550Z commit another update.
2019-09-29T20:06:49.3463583Z 
2019-09-29T20:06:49.3463887Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-29T20:06:49.3464173Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-29T20:06:49.3464228Z proper steps.
2019-09-29T20:06:49.3511921Z   local time: Sun Sep 29 20:06:49 UTC 2019
2019-09-29T20:06:49.5100801Z   network time: Sun, 29 Sep 2019 20:06:49 GMT
2019-09-29T20:06:49.5105234Z == end clock drift check ==
2019-09-29T20:06:49.5105234Z == end clock drift check ==
2019-09-29T20:06:50.7201664Z ##[error]Bash exited with code '3'.
2019-09-29T20:06:50.7293879Z ##[section]Starting: Checkout
2019-09-29T20:06:50.7296130Z ==============================================================================
2019-09-29T20:06:50.7296192Z Task         : Get sources
2019-09-29T20:06:50.7296244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

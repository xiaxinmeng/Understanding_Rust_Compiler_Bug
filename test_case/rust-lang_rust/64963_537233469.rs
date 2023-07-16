plain
2019-10-01T18:31:36.2022885Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T18:31:36.2224947Z ##[command]git config gc.auto 0
2019-10-01T18:31:36.2291471Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T18:31:36.2344171Z ##[command]git config --get-all http.proxy
2019-10-01T18:31:36.2497363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64963/merge:refs/remotes/pull/64963/merge
---
2019-10-01T20:11:05.8155056Z    Compiling mdbook-linkcheck v0.3.0
2019-10-01T20:11:18.3100905Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-10-01T20:11:22.2836601Z     Finished release [optimized] target(s) in 7m 44s
2019-10-01T20:13:06.3552950Z Error: there are broken links
2019-10-01T20:13:06.3554712Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-01T20:13:06.3556166Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-01T20:13:06.3556703Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-01T20:13:06.3557953Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-01T20:13:06.3558630Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-10-01T20:29:31.4133613Z    |
2019-10-01T20:29:31.4133648Z    = note: `#[deny(const_err)]` on by default
2019-10-01T20:29:31.4133673Z 
2019-10-01T20:29:31.4133876Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-10-01T20:29:31.4133917Z   left: `Size { raw: 0 }`,
2019-10-01T20:29:31.4134129Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-10-01T20:29:31.4134228Z 
2019-10-01T20:29:31.4134275Z error: internal compiler error: unexpected panic
2019-10-01T20:29:31.4134299Z 
2019-10-01T20:29:31.4134349Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-01T20:29:31.4134349Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-01T20:29:31.4139034Z 
2019-10-01T20:29:31.4141783Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-01T20:29:31.4145472Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-01T20:29:31.4145503Z 
2019-10-01T20:29:31.4145683Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-10-01T20:29:31.4145903Z 
---
2019-10-01T20:29:31.4151850Z -LL |     &empty[1..5];
2019-10-01T20:29:31.4151996Z -   |            ^
2019-10-01T20:29:31.4152145Z -   |
2019-10-01T20:29:31.4152405Z -   = note: `-D clippy::out-of-bounds-indexing` implied by `-D warnings`
2019-10-01T20:29:31.4152620Z +thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-10-01T20:29:31.4152677Z +  left: `Size { raw: 0 }`,
2019-10-01T20:29:31.4152890Z + right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-10-01T20:29:31.4152936Z +note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-01T20:29:31.4153143Z -error: range is out of bounds
2019-10-01T20:29:31.4153300Z -  --> $DIR/empty_array.rs:8:16
2019-10-01T20:29:31.4153437Z -   |
2019-10-01T20:29:31.4153602Z -LL |     &empty[0..=4];
---
2019-10-01T20:29:31.4155015Z -  --> $DIR/empty_array.rs:10:12
2019-10-01T20:29:31.4155167Z -   |
2019-10-01T20:29:31.4155315Z -LL |     &empty[1..];
2019-10-01T20:29:31.4155459Z -   |            ^
2019-10-01T20:29:31.4155918Z +note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-01T20:29:31.4156626Z -error: range is out of bounds
2019-10-01T20:29:31.4156868Z -  --> $DIR/empty_array.rs:11:14
2019-10-01T20:29:31.4157048Z -   |
2019-10-01T20:29:31.4157257Z -LL |     &empty[..4];
---
2019-10-01T20:29:31.4158177Z -  --> $DIR/empty_array.rs:12:16
2019-10-01T20:29:31.4158356Z -   |
2019-10-01T20:29:31.4158548Z -LL |     &empty[0..=0];
2019-10-01T20:29:31.4158755Z -   |                ^
2019-10-01T20:29:31.4158980Z +note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-10-01T20:29:31.4159224Z -error: range is out of bounds
2019-10-01T20:29:31.4159445Z -  --> $DIR/empty_array.rs:13:15
2019-10-01T20:29:31.4159779Z -   |
2019-10-01T20:29:31.4159927Z -LL |     &empty[..=0];
---
2019-10-01T20:29:31.4161387Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'out_of_bounds_indexing/empty_array.rs'
2019-10-01T20:29:31.4161422Z 
2019-10-01T20:29:31.4161458Z error: 1 errors occurred comparing output.
2019-10-01T20:29:31.4161506Z status: exit code: 101
2019-10-01T20:29:31.4168032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/out_of_bounds_indexing/empty_array.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id.aux" "-A" "unused"
2019-10-01T20:29:31.4173371Z ------------------------------------------
2019-10-01T20:29:31.4173452Z 
2019-10-01T20:29:31.4175590Z ------------------------------------------
2019-10-01T20:29:31.4175984Z stderr:
2019-10-01T20:29:31.4175984Z stderr:
2019-10-01T20:29:31.4178729Z ------------------------------------------
2019-10-01T20:29:31.4180173Z {"message":"index out of bounds: the len is 0 but the index is 0","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/out_of_bounds_indexing/empty_array.rs","byte_start":147,"byte_end":155,"line_start":6,"line_end":6,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    empty[0];","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(const_err)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: index out of bounds: the len is 0 but the index is 0\n  --> tests/ui/out_of_bounds_indexing/empty_array.rs:6:5\n   |\nLL |     empty[0];\n   |     ^^^^^^^^\n   |\n   = note: `#[deny(const_err)]` on by default\n\n"}
2019-10-01T20:29:31.4180458Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-10-01T20:29:31.4180516Z   left: `Size { raw: 0 }`,
2019-10-01T20:29:31.4180738Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-10-01T20:29:31.4180831Z 
2019-10-01T20:29:31.4180867Z error: internal compiler error: unexpected panic
2019-10-01T20:29:31.4180891Z 
2019-10-01T20:29:31.4180927Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-01T20:29:31.4180927Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-01T20:29:31.4180967Z 
2019-10-01T20:29:31.4181426Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-01T20:29:31.4181663Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-01T20:29:31.4181698Z 
2019-10-01T20:29:31.4181872Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-10-01T20:29:31.4181899Z 
---
2019-10-01T21:14:15.3464042Z Verifying status of rustfmt...
2019-10-01T21:14:15.3480024Z Verifying status of clippy-driver...
2019-10-01T21:14:15.3497203Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-01T21:14:15.3506792Z 
2019-10-01T21:14:15.3510712Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-01T21:14:15.3510772Z 
2019-10-01T21:14:15.3511071Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-01T21:14:15.3511160Z commit another update.
2019-10-01T21:14:15.3511192Z 
2019-10-01T21:14:15.3511455Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-01T21:14:15.3511880Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-01T21:14:15.3511918Z proper steps.
2019-10-01T21:14:15.3526577Z   local time: Tue Oct  1 21:14:15 UTC 2019
2019-10-01T21:14:15.5107247Z   network time: Tue, 01 Oct 2019 21:14:15 GMT
2019-10-01T21:14:15.5107324Z == end clock drift check ==
2019-10-01T21:14:15.5107324Z == end clock drift check ==
2019-10-01T21:14:16.0822414Z ##[error]Bash exited with code '3'.
2019-10-01T21:14:16.0859189Z ##[section]Starting: Checkout
2019-10-01T21:14:16.0861127Z ==============================================================================
2019-10-01T21:14:16.0861204Z Task         : Get sources
2019-10-01T21:14:16.0861251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

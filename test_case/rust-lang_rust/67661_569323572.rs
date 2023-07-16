plain
2019-12-27T18:05:32.5977228Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/const_eval/fn.op_to_const.html"
2019-12-27T18:05:32.5977299Z 
2019-12-27T18:05:32.5977528Z     ┌── miri.md:83:19 ───
2019-12-27T18:05:32.5977890Z     │
2019-12-27T18:05:32.5978640Z  83 │ [`ConstValue`] by [`op_to_const`]: the former representation is geared towards
2019-12-27T18:05:32.5979460Z     │
2019-12-27T18:05:32.5979502Z 
2019-12-27T18:05:32.5979582Z Error: One or more incorrect links
2019-12-27T18:05:32.6030304Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
---
2019-12-27T18:16:17.9474681Z test [ui] ui/crashes/ice-2594.rs ... ok
2019-12-27T18:16:18.1756051Z test [ui] ui/crashes/ice-2727.rs ... ok
2019-12-27T18:16:18.4277829Z test [ui] ui/crashes/ice-2760.rs ... ok
2019-12-27T18:16:18.6233442Z normalized stderr:
2019-12-27T18:16:18.6240959Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T18:16:18.6241196Z 
2019-12-27T18:16:18.6241266Z error: internal compiler error: unexpected panic
2019-12-27T18:16:18.6241303Z 
2019-12-27T18:16:18.6241371Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6241371Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6241408Z 
2019-12-27T18:16:18.6241692Z note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
2019-12-27T18:16:18.6241762Z 
2019-12-27T18:16:18.6241990Z note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T18:16:18.6249182Z 
2019-12-27T18:16:18.6250575Z 
2019-12-27T18:16:18.6252565Z expected stderr:
2019-12-27T18:16:18.6254022Z 
2019-12-27T18:16:18.6254022Z 
2019-12-27T18:16:18.6258962Z 
2019-12-27T18:16:18.6261979Z diff of stderr:
2019-12-27T18:16:18.6263685Z 
2019-12-27T18:16:18.6267701Z +thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T18:16:18.6305095Z +
2019-12-27T18:16:18.6305448Z +error: internal compiler error: unexpected panic
2019-12-27T18:16:18.6305688Z +
2019-12-27T18:16:18.6305787Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6305787Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6305864Z +
2019-12-27T18:16:18.6306453Z +note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
2019-12-27T18:16:18.6306550Z +
2019-12-27T18:16:18.6306893Z +note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T18:16:18.6307050Z +
2019-12-27T18:16:18.6307088Z 
2019-12-27T18:16:18.6307177Z The actual stderr differed from the expected stderr.
2019-12-27T18:16:18.6307668Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base/crashes/ice-2774.stderr
2019-12-27T18:16:18.6307668Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base/crashes/ice-2774.stderr
2019-12-27T18:16:18.6307826Z To update references, run this command from build directory:
2019-12-27T18:16:18.6308574Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base' 'crashes/ice-2774.rs'
2019-12-27T18:16:18.6308852Z 
2019-12-27T18:16:18.6309112Z error: 1 errors occurred comparing output.
2019-12-27T18:16:18.6309166Z status: exit code: 101
2019-12-27T18:16:18.6310927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/crashes/ice-2774.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base/crashes/ice-2774.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-87059d2b2cc41bb9.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a05b3223ced77470.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-492e94bb4f14fcd4.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-2d9b2f14cb2e358b/out/test_build_base/crashes/ice-2774.stage-id.aux" "-A" "unused"
2019-12-27T18:16:18.6311809Z ------------------------------------------
2019-12-27T18:16:18.6311883Z 
2019-12-27T18:16:18.6312094Z ------------------------------------------
2019-12-27T18:16:18.6312167Z stderr:
2019-12-27T18:16:18.6312167Z stderr:
2019-12-27T18:16:18.6312558Z ------------------------------------------
2019-12-27T18:16:18.6312859Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T18:16:18.6313439Z 
2019-12-27T18:16:18.6313496Z error: internal compiler error: unexpected panic
2019-12-27T18:16:18.6313553Z 
2019-12-27T18:16:18.6313610Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6313610Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T18:16:18.6313650Z 
2019-12-27T18:16:18.6313947Z note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
2019-12-27T18:16:18.6314004Z 
2019-12-27T18:16:18.6314270Z note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T18:16:18.6314359Z 
2019-12-27T18:16:18.6314643Z ------------------------------------------
2019-12-27T18:16:18.6314689Z 
2019-12-27T18:16:18.6315391Z test [ui] ui/crashes/ice-2774.rs ... FAILED
---
2019-12-27T18:31:07.9362045Z [RUSTC-TIMING] git2 test:false 24.673
2019-12-27T18:31:14.1440926Z error[E0308]: mismatched types
2019-12-27T18:31:14.1442095Z    --> src/tools/rls/rls/src/build/cargo.rs:667:67
2019-12-27T18:31:14.1443160Z     |
2019-12-27T18:31:14.1443769Z 667 |             .or_insert_with(|| ConfigValue::Table(HashMap::new(), config_path.clone()));
2019-12-27T18:31:14.1445141Z     |                                                                   |
2019-12-27T18:31:14.1445141Z     |                                                                   |
2019-12-27T18:31:14.1446021Z     |                                                                   expected enum `cargo::util::config::value::Definition`, found struct `std::path::PathBuf`
2019-12-27T18:31:14.1447091Z     |                                                                   help: try using a variant of the expected enum: `cargo::util::config::value::Definition::Path(config_path.clone())`
2019-12-27T18:31:14.6164825Z error[E0308]: mismatched types
2019-12-27T18:31:14.6165243Z    --> src/tools/rls/rls/src/build/cargo.rs:689:56
2019-12-27T18:31:14.6165483Z     |
2019-12-27T18:31:14.6165483Z     |
2019-12-27T18:31:14.6165811Z 689 |         let td_value = ConfigValue::String(target_dir, config_path);
2019-12-27T18:31:14.6166468Z     |                                                        |
2019-12-27T18:31:14.6166468Z     |                                                        |
2019-12-27T18:31:14.6166900Z     |                                                        expected enum `cargo::util::config::value::Definition`, found struct `std::path::PathBuf`
2019-12-27T18:31:14.6167528Z     |                                                        help: try using a variant of the expected enum: `cargo::util::config::value::Definition::Path(config_path)`
2019-12-27T18:31:15.3389055Z error[E0061]: this function takes 9 parameters but 8 parameters were supplied
2019-12-27T18:31:15.3389509Z   --> src/tools/rls/rls/src/project_model.rs:51:16
2019-12-27T18:31:15.3389845Z    |
2019-12-27T18:31:15.3389845Z    |
2019-12-27T18:31:15.3390261Z 51 |         config.configure(0, Some(true), &None, false, false, false, &None, &[])?;
2019-12-27T18:31:15.3394908Z 
2019-12-27T18:31:15.7018704Z error: aborting due to 3 previous errors
2019-12-27T18:31:15.7022798Z 
2019-12-27T18:31:15.7032857Z Some errors have detailed explanations: E0061, E0308.
---
2019-12-27T18:39:33.7746536Z Verifying status of rustfmt...
2019-12-27T18:39:33.7746784Z Verifying status of clippy-driver...
2019-12-27T18:39:33.7747052Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-27T18:39:33.7747489Z 
2019-12-27T18:39:33.7747782Z We detected that this PR updated 'clippy-driver', but its tests failed.
2019-12-27T18:39:33.7748032Z 
2019-12-27T18:39:33.7748491Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-12-27T18:39:33.7748594Z commit another update.
2019-12-27T18:39:33.7748635Z 
2019-12-27T18:39:33.7748925Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-12-27T18:39:33.7749245Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-12-27T18:39:33.7749343Z proper steps.
2019-12-27T18:39:33.7753983Z Build completed unsuccessfully in 0:00:01
2019-12-27T18:39:33.7843829Z == clock drift check ==
2019-12-27T18:39:33.7856778Z   local time: Fri Dec 27 18:39:33 UTC 2019
2019-12-27T18:39:34.0864075Z   network time: Fri, 27 Dec 2019 18:39:34 GMT
2019-12-27T18:39:34.0864075Z   network time: Fri, 27 Dec 2019 18:39:34 GMT
2019-12-27T18:39:34.0866141Z == end clock drift check ==
2019-12-27T18:39:35.2713415Z 
2019-12-27T18:39:35.2806230Z ##[error]Bash exited with code '1'.
2019-12-27T18:39:35.2842181Z ##[section]Starting: Checkout
2019-12-27T18:39:35.2844833Z ==============================================================================
2019-12-27T18:39:35.2844977Z Task         : Get sources
2019-12-27T18:39:35.2845041Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

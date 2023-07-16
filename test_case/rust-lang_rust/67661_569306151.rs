plain
2019-12-27T14:52:13.1598141Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T14:52:13.1618077Z ##[command]git config gc.auto 0
2019-12-27T14:52:13.1621357Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T14:52:13.1624929Z ##[command]git config --get-all http.proxy
2019-12-27T14:52:13.1630226Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67661/merge:refs/remotes/pull/67661/merge
---
2019-12-27T16:21:08.6292456Z  53 │ variables. [`UniversalRegions`] contains indices for all the free regions in
2019-12-27T16:21:08.6292668Z     │            ^ Server responded with 404 Not Found
2019-12-27T16:21:08.6292996Z     │
2019-12-27T16:21:08.6293038Z 
2019-12-27T16:21:08.6293592Z error: The server responded with 404 Not Found for "***/tree/master/src/librustc_mir/borrow_check/nll/region_infer/"
2019-12-27T16:21:08.6293853Z     ┌── borrow_check/region_inference.md:81:34 ───
2019-12-27T16:21:08.6294004Z     │
2019-12-27T16:21:08.6294201Z  81 │ for all regions is maintained in [the
2019-12-27T16:21:08.6294592Z     │                                  ^ Server responded with 404 Not Found
---
2019-12-27T16:21:08.6375897Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/const_eval/fn.op_to_const.html"
2019-12-27T16:21:08.6375936Z 
2019-12-27T16:21:08.6376149Z     ┌── miri.md:83:19 ───
2019-12-27T16:21:08.6376348Z     │
2019-12-27T16:21:08.6376601Z  83 │ [`ConstValue`] by [`op_to_const`]: the former representation is geared towards
2019-12-27T16:21:08.6377013Z     │
2019-12-27T16:21:08.6377059Z 
2019-12-27T16:21:08.6377101Z Error: One or more incorrect links
2019-12-27T16:21:08.6377183Z 
---
2019-12-27T16:31:43.1004952Z test [ui] ui/crashes/ice-2594.rs ... ok
2019-12-27T16:31:43.1658163Z test [ui] ui/crashes/ice-2727.rs ... ok
2019-12-27T16:31:43.6281154Z test [ui] ui/crashes/ice-2760.rs ... ok
2019-12-27T16:31:43.6681507Z normalized stderr:
2019-12-27T16:31:43.6682479Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T16:31:43.6682648Z 
2019-12-27T16:31:43.6682704Z error: internal compiler error: unexpected panic
2019-12-27T16:31:43.6682730Z 
2019-12-27T16:31:43.6682795Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6682795Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6682819Z 
2019-12-27T16:31:43.6683249Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-27T16:31:43.6683279Z 
2019-12-27T16:31:43.6683651Z note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T16:31:43.6683729Z 
2019-12-27T16:31:43.6683749Z 
2019-12-27T16:31:43.6683783Z expected stderr:
2019-12-27T16:31:43.6683804Z 
2019-12-27T16:31:43.6683804Z 
2019-12-27T16:31:43.6683823Z 
2019-12-27T16:31:43.6683873Z diff of stderr:
2019-12-27T16:31:43.6683894Z 
2019-12-27T16:31:43.6684163Z +thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T16:31:43.6684267Z +
2019-12-27T16:31:43.6684303Z +error: internal compiler error: unexpected panic
2019-12-27T16:31:43.6684335Z +
2019-12-27T16:31:43.6684391Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6684391Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6684548Z +
2019-12-27T16:31:43.6684839Z +note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-27T16:31:43.6684878Z +
2019-12-27T16:31:43.6685071Z +note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T16:31:43.6685162Z +
2019-12-27T16:31:43.6685182Z 
2019-12-27T16:31:43.6685217Z The actual stderr differed from the expected stderr.
2019-12-27T16:31:43.6685534Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f1e112bfe6e65d12/out/test_build_base/crashes/ice-2774.stderr
---
2019-12-27T16:31:43.6687892Z 
2019-12-27T16:31:43.6688061Z ------------------------------------------
2019-12-27T16:31:43.6688097Z stderr:
2019-12-27T16:31:43.6688286Z ------------------------------------------
2019-12-27T16:31:43.6688510Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-27T16:31:43.6688609Z 
2019-12-27T16:31:43.6688720Z error: internal compiler error: unexpected panic
2019-12-27T16:31:43.6688749Z 
2019-12-27T16:31:43.6688784Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6688784Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-27T16:31:43.6688825Z 
2019-12-27T16:31:43.6689089Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-27T16:31:43.6689116Z 
2019-12-27T16:31:43.6689325Z note: Clippy version: clippy 0.0.212 (0fcb530 2019-12-27)
2019-12-27T16:31:43.6689371Z 
2019-12-27T16:31:43.6689539Z ------------------------------------------
2019-12-27T16:31:43.6689563Z 
2019-12-27T16:31:43.6689750Z test [ui] ui/crashes/ice-2774.rs ... FAILED
---
2019-12-27T16:46:19.3333581Z    Compiling cargo v0.43.0 (/checkout/src/tools/cargo)
2019-12-27T16:46:43.0778847Z error[E0308]: mismatched types
2019-12-27T16:46:43.0796685Z    --> src/tools/rls/rls/src/build/cargo.rs:667:67
2019-12-27T16:46:43.0798740Z     |
2019-12-27T16:46:43.0799065Z 667 |             .or_insert_with(|| ConfigValue::Table(HashMap::new(), config_path.clone()));
2019-12-27T16:46:43.0802737Z     |                                                                   |
2019-12-27T16:46:43.0802737Z     |                                                                   |
2019-12-27T16:46:43.0803162Z     |                                                                   expected enum `cargo::util::config::value::Definition`, found struct `std::path::PathBuf`
2019-12-27T16:46:43.0803575Z     |                                                                   help: try using a variant of the expected enum: `cargo::util::config::value::Definition::Path(config_path.clone())`
2019-12-27T16:46:43.5261389Z error[E0308]: mismatched types
2019-12-27T16:46:43.5261941Z    --> src/tools/rls/rls/src/build/cargo.rs:689:56
2019-12-27T16:46:43.5262146Z     |
2019-12-27T16:46:43.5262146Z     |
2019-12-27T16:46:43.5262422Z 689 |         let td_value = ConfigValue::String(target_dir, config_path);
2019-12-27T16:46:43.5263041Z     |                                                        |
2019-12-27T16:46:43.5263041Z     |                                                        |
2019-12-27T16:46:43.5263407Z     |                                                        expected enum `cargo::util::config::value::Definition`, found struct `std::path::PathBuf`
2019-12-27T16:46:43.5263768Z     |                                                        help: try using a variant of the expected enum: `cargo::util::config::value::Definition::Path(config_path)`
2019-12-27T16:46:44.1949511Z error[E0061]: this function takes 9 parameters but 8 parameters were supplied
2019-12-27T16:46:44.1949827Z   --> src/tools/rls/rls/src/project_model.rs:51:16
2019-12-27T16:46:44.1950112Z    |
2019-12-27T16:46:44.1950112Z    |
2019-12-27T16:46:44.1950537Z 51 |         config.configure(0, Some(true), &None, false, false, false, &None, &[])?;
2019-12-27T16:46:44.1950911Z 
2019-12-27T16:46:44.5482506Z error: aborting due to 3 previous errors
2019-12-27T16:46:44.5482594Z 
2019-12-27T16:46:44.5487737Z Some errors have detailed explanations: E0061, E0308.
---
2019-12-27T16:55:02.1459477Z Verifying status of rustfmt...
2019-12-27T16:55:02.1459695Z Verifying status of clippy-driver...
2019-12-27T16:55:02.1459961Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-27T16:55:02.1459994Z 
2019-12-27T16:55:02.1460411Z We detected that this PR updated 'clippy-driver', but its tests failed.
2019-12-27T16:55:02.1460441Z 
2019-12-27T16:55:02.1461025Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-12-27T16:55:02.1461120Z commit another update.
2019-12-27T16:55:02.1461166Z 
2019-12-27T16:55:02.1461480Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-12-27T16:55:02.1461710Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-12-27T16:55:02.1461771Z proper steps.
2019-12-27T16:55:02.1467625Z Build completed unsuccessfully in 0:00:01
2019-12-27T16:55:02.1519595Z == clock drift check ==
2019-12-27T16:55:02.1536350Z   local time: Fri Dec 27 16:55:02 UTC 2019
2019-12-27T16:55:02.6797295Z   network time: Fri, 27 Dec 2019 16:55:02 GMT
2019-12-27T16:55:02.6797295Z   network time: Fri, 27 Dec 2019 16:55:02 GMT
2019-12-27T16:55:02.6797915Z == end clock drift check ==
2019-12-27T16:55:03.9314538Z 
2019-12-27T16:55:03.9380597Z ##[error]Bash exited with code '1'.
2019-12-27T16:55:03.9416845Z ##[section]Starting: Checkout
2019-12-27T16:55:03.9418402Z ==============================================================================
2019-12-27T16:55:03.9418468Z Task         : Get sources
2019-12-27T16:55:03.9418504Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

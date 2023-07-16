plain
2020-04-12T20:34:28.6826145Z ========================== Starting Command Output ===========================
2020-04-12T20:34:28.6829602Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/69fb6ee0-eae0-4c36-b10e-673badad0acf.sh
2020-04-12T20:34:28.6829866Z 
2020-04-12T20:34:28.6833963Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T20:34:28.6853434Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-12T20:34:28.6857237Z Task         : Get sources
2020-04-12T20:34:28.6857700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:34:28.6857982Z Version      : 1.0.0
2020-04-12T20:34:28.6858236Z Author       : Microsoft
---
2020-04-12T20:34:29.9606481Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T20:34:29.9618002Z ##[command]git config gc.auto 0
2020-04-12T20:34:29.9624290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T20:34:29.9629258Z ##[command]git config --get-all http.proxy
2020-04-12T20:34:29.9640664Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-12T20:36:54.2135448Z Looks like docker image is the same as before, not uploading
2020-04-12T20:36:58.8656526Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:36:58.8919197Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:36:58.8947741Z == clock drift check ==
2020-04-12T20:36:58.8958934Z   local time: Sun Apr 12 20:36:58 UTC 2020
2020-04-12T20:36:58.9571274Z   network time: Sun, 12 Apr 2020 20:36:58 GMT
2020-04-12T20:36:58.9596756Z Starting sccache server...
2020-04-12T20:36:59.0432762Z configure: processing command line
2020-04-12T20:36:59.0433993Z configure: 
2020-04-12T20:36:59.0435021Z configure: rust.dist-src        := False
---
2020-04-12T20:42:23.8220777Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T20:42:25.3601058Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T20:42:26.9240313Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T20:42:28.5772120Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T20:42:37.0613939Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T20:42:40.1049909Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T20:42:44.6091852Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T20:42:48.8075280Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T20:42:57.7470372Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T21:05:34.4819437Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T21:05:36.1648376Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T21:05:38.0793798Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T21:05:40.4714010Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T21:05:50.0376702Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T21:05:53.7839277Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T21:05:58.7894021Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T21:06:03.9590777Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T21:06:13.9559995Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T21:06:13.9559995Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T21:06:51.7948248Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-12T21:06:55.1591688Z error[E0277]: `<<<T as ty::codec::EncodableWithShorthand>::Variant as std::marker::DiscriminantKind>::Discriminant as std::convert::TryFrom<usize>>::Error` doesn't implement `std::fmt::Debug`
2020-04-12T21:06:55.1592916Z   --> src/librustc_middle/ty/codec.rs:81:56
2020-04-12T21:06:55.1593449Z    |
2020-04-12T21:06:55.1597282Z 65 |     <T::Variant as DiscriminantKind>::Discriminant: Ord + TryFrom<usize>,
2020-04-12T21:06:55.1599133Z    |                                                                          - help: consider further restricting the associated type: `, <<<T as ty::codec::EncodableWithShorthand>::Variant as std::marker::DiscriminantKind>::Discriminant as std::convert::TryFrom<usize>>::Error: std::fmt::Debug`
2020-04-12T21:06:55.1600339Z ...
2020-04-12T21:06:55.1601036Z 81 |     assert!(discriminant < SHORTHAND_OFFSET.try_into().unwrap());
2020-04-12T21:06:55.1602743Z    |                                                        ^^^^^^ `<<<T as ty::codec::EncodableWithShorthand>::Variant as std::marker::DiscriminantKind>::Discriminant as std::convert::TryFrom<usize>>::Error` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-04-12T21:06:55.1603968Z    |
2020-04-12T21:06:55.1605093Z    = help: the trait `std::fmt::Debug` is not implemented for `<<<T as ty::codec::EncodableWithShorthand>::Variant as std::marker::DiscriminantKind>::Discriminant as std::convert::TryFrom<usize>>::Error`
2020-04-12T21:06:59.9424768Z error: aborting due to previous error
2020-04-12T21:06:59.9425969Z 
2020-04-12T21:06:59.9429056Z For more information about this error, try `rustc --explain E0277`.
2020-04-12T21:06:59.9622756Z error: could not compile `rustc_middle`.
---
2020-04-12T21:07:00.9472474Z expected success, got: exit code: 101
2020-04-12T21:07:00.9484698Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-12T21:07:00.9485522Z Build completed unsuccessfully in 0:28:15
2020-04-12T21:07:00.9537071Z == clock drift check ==
2020-04-12T21:07:00.9558523Z   local time: Sun Apr 12 21:07:00 UTC 2020
2020-04-12T21:07:01.0194201Z   network time: Sun, 12 Apr 2020 21:07:01 GMT
2020-04-12T21:07:01.4585959Z 
2020-04-12T21:07:01.4585959Z 
2020-04-12T21:07:01.4623176Z ##[error]Bash exited with code '1'.
2020-04-12T21:07:01.4636196Z ##[section]Finishing: Run build
2020-04-12T21:07:01.4994932Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-12T21:07:01.4999233Z Task         : Get sources
2020-04-12T21:07:01.4999494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T21:07:01.4999757Z Version      : 1.0.0
2020-04-12T21:07:01.4999926Z Author       : Microsoft
2020-04-12T21:07:01.4999926Z Author       : Microsoft
2020-04-12T21:07:01.5000216Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T21:07:01.5000548Z ==============================================================================
2020-04-12T21:07:01.8065391Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T21:07:01.8112173Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-12T21:07:01.8194231Z Cleaning up task key
2020-04-12T21:07:01.8195872Z Start cleaning up orphan processes.
2020-04-12T21:07:01.8377835Z Terminate orphan process: pid (3556) (python)
2020-04-12T21:07:01.8578429Z ##[section]Finishing: Finalize Job

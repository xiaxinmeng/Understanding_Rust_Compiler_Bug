plain
2019-09-06T15:24:28.8703713Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T15:24:28.8966805Z ##[command]git config gc.auto 0
2019-09-06T15:24:28.9025457Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T15:24:28.9083901Z ##[command]git config --get-all http.proxy
2019-09-06T15:24:28.9210554Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64227/merge:refs/remotes/pull/64227/merge
---
2019-09-06T15:33:03.8965099Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-06T15:33:17.1640032Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-06T15:34:09.4818291Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-09-06T15:34:13.5556462Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-09-06T15:34:16.9666612Z error[E0027]: pattern does not mention field `var_id`
2019-09-06T15:34:16.9666957Z    --> src/librustc_mir/borrow_check/move_errors.rs:110:71
2019-09-06T15:34:16.9667134Z     |
2019-09-06T15:34:16.9667471Z 110 |                       if let Some(ClearCrossCrate::Set(BindingForm::Var(VarBindingForm {
2019-09-06T15:34:16.9667737Z     |  _______________________________________________________________________^
2019-09-06T15:34:16.9668093Z 111 | |                         opt_match_place: Some((ref opt_match_place, match_span)),
2019-09-06T15:34:16.9668352Z 112 | |                         binding_mode: _,
2019-09-06T15:34:16.9668851Z 113 | |                         opt_ty_info: _,
2019-09-06T15:34:16.9669369Z 114 | |                         pat_span: _,
2019-09-06T15:34:16.9669644Z 115 | |                     }))) = local_decl.is_user_variable
2019-09-06T15:34:16.9670111Z     | |_____________________^ missing field `var_id`
2019-09-06T15:34:17.7347786Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-06T15:34:17.7347786Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-06T15:34:18.4042910Z error[E0063]: missing field `var_id` in initializer of `rustc::mir::VarBindingForm<'_>`
2019-09-06T15:34:18.4043324Z    --> src/librustc_mir/build/mod.rs:844:76
2019-09-06T15:34:18.4043629Z     |
2019-09-06T15:34:18.4044480Z 844 |                                 Some(ClearCrossCrate::Set(BindingForm::Var(VarBindingForm {
2019-09-06T15:34:18.4044843Z     |                                                                            ^^^^^^^^^^^^^^ missing `var_id`
2019-09-06T15:34:18.4044889Z 
2019-09-06T15:34:18.6136157Z error[E0063]: missing field `var_id` in initializer of `rustc::mir::VarBindingForm<'_>`
2019-09-06T15:34:18.6137228Z      |
2019-09-06T15:34:18.6137228Z      |
2019-09-06T15:34:18.6137537Z 1740 |             is_user_variable: Some(ClearCrossCrate::Set(BindingForm::Var(VarBindingForm {
2019-09-06T15:34:18.6138273Z      |                                                                          ^^^^^^^^^^^^^^ missing `var_id`
2019-09-06T15:34:19.3659320Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-09-06T15:34:20.5663210Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-09-06T15:34:21.8161055Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-09-06T15:34:22.5420850Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
---
2019-09-06T15:34:25.6594863Z == clock drift check ==
2019-09-06T15:34:25.6608895Z   local time: Fri Sep  6 15:34:25 UTC 2019
2019-09-06T15:34:25.8187297Z   network time: Fri, 06 Sep 2019 15:34:25 GMT
2019-09-06T15:34:25.8190673Z == end clock drift check ==
2019-09-06T15:34:26.9624580Z ##[error]Bash exited with code '1'.
2019-09-06T15:34:26.9661674Z ##[section]Starting: Checkout
2019-09-06T15:34:26.9664122Z ==============================================================================
2019-09-06T15:34:26.9664168Z Task         : Get sources
2019-09-06T15:34:26.9664221Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

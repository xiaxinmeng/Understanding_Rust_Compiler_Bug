plain
2019-10-07T04:32:38.0719397Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T04:32:38.0965398Z ##[command]git config gc.auto 0
2019-10-07T04:32:38.1025688Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T04:32:38.1103755Z ##[command]git config --get-all http.proxy
2019-10-07T04:32:38.9766392Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65040/merge:refs/remotes/pull/65040/merge
---
2019-10-07T04:40:54.1061229Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-07T04:41:00.3461361Z error[E0308]: mismatched types
2019-10-07T04:41:00.3462691Z     --> src/libsyntax/parse/parser/item.rs:1098:13
2019-10-07T04:41:00.3463256Z      |
2019-10-07T04:41:00.3464243Z 1096 |     fn parse_use_tree_glob_or_nested(&mut self) -> PResult<'a, UseTreeKind> {
2019-10-07T04:41:00.3465610Z      |                                                    ------------------------ expected `std::result::Result<ast::UseTreeKind, rustc_errors::diagnostic_builder::DiagnosticBuilder<'a>>` because of return type
2019-10-07T04:41:00.3466282Z 1097 |         if self.eat(&token::BinOp(token::Star)) {
2019-10-07T04:41:00.3466776Z 1098 |             UseTreeKind::Glob
2019-10-07T04:41:00.3467535Z      |             ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found enum `ast::UseTreeKind`
2019-10-07T04:41:00.3467994Z      |
2019-10-07T04:41:00.3468902Z      = note: expected type `std::result::Result<ast::UseTreeKind, rustc_errors::diagnostic_builder::DiagnosticBuilder<'a>>`
2019-10-07T04:41:00.3469697Z                 found type `ast::UseTreeKind`
2019-10-07T04:41:00.3918024Z error[E0308]: mismatched types
2019-10-07T04:41:00.3918298Z     --> src/libsyntax/parse/parser/item.rs:1100:13
2019-10-07T04:41:00.3919195Z      |
2019-10-07T04:41:00.3919195Z      |
2019-10-07T04:41:00.3919540Z 1096 |     fn parse_use_tree_glob_or_nested(&mut self) -> PResult<'a, UseTreeKind> {
2019-10-07T04:41:00.3920040Z      |                                                    ------------------------ expected `std::result::Result<ast::UseTreeKind, rustc_errors::diagnostic_builder::DiagnosticBuilder<'a>>` because of return type
2019-10-07T04:41:00.3920293Z ...
2019-10-07T04:41:00.3920590Z 1100 |             UseTreeKind::Nested(self.parse_use_tree_list()?)
2019-10-07T04:41:00.3921018Z      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found enum `ast::UseTreeKind`
2019-10-07T04:41:00.3921246Z      |
2019-10-07T04:41:00.3921641Z      = note: expected type `std::result::Result<ast::UseTreeKind, rustc_errors::diagnostic_builder::DiagnosticBuilder<'a>>`
2019-10-07T04:41:00.3921881Z                 found type `ast::UseTreeKind`
2019-10-07T04:41:02.1080317Z error: aborting due to 2 previous errors
2019-10-07T04:41:02.1080471Z 
2019-10-07T04:41:02.1080806Z For more information about this error, try `rustc --explain E0308`.
2019-10-07T04:41:02.1605970Z error: could not compile `syntax`.
---
2019-10-07T04:41:02.1706130Z == clock drift check ==
2019-10-07T04:41:02.1727463Z   local time: Mon Oct  7 04:41:02 UTC 2019
2019-10-07T04:41:02.3249927Z   network time: Mon, 07 Oct 2019 04:41:02 GMT
2019-10-07T04:41:02.3250185Z == end clock drift check ==
2019-10-07T04:41:03.5796762Z ##[error]Bash exited with code '1'.
2019-10-07T04:41:03.5843563Z ##[section]Starting: Checkout
2019-10-07T04:41:03.5845267Z ==============================================================================
2019-10-07T04:41:03.5845338Z Task         : Get sources
2019-10-07T04:41:03.5845384Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

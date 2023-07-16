plain
2019-09-16T23:05:06.9297783Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T23:05:06.9489989Z ##[command]git config gc.auto 0
2019-09-16T23:05:06.9562880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T23:05:06.9621225Z ##[command]git config --get-all http.proxy
2019-09-16T23:05:06.9766400Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64527/merge:refs/remotes/pull/64527/merge
---
2019-09-16T23:14:09.3306147Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-09-16T23:14:09.4901056Z error: this file contains an un-closed delimiter
2019-09-16T23:14:09.4902525Z    --> src/librustc_mir/transform/const_prop.rs:824:3
2019-09-16T23:14:09.4904053Z     |
2019-09-16T23:14:09.4904635Z 651 | impl<'mir, 'tcx> MutVisitor<'tcx> for ConstPropagator<'mir, 'tcx> {
2019-09-16T23:14:09.4905236Z     |                                                                   - un-closed delimiter
2019-09-16T23:14:09.4905641Z ...
2019-09-16T23:14:09.4906147Z 690 |                                 if self.should_const_prop() {
2019-09-16T23:14:09.4906879Z     |                                                             - this delimiter might not be properly closed...
2019-09-16T23:14:09.4907665Z 696 |                             }
2019-09-16T23:14:09.4908158Z     |                             - ...as it matches this but it has different indentation
2019-09-16T23:14:09.4908499Z ...
2019-09-16T23:14:09.4908879Z 824 | }
2019-09-16T23:14:09.4908879Z 824 | }
2019-09-16T23:14:09.4909249Z     |   ^
2019-09-16T23:14:09.4909389Z 
2019-09-16T23:14:09.4928369Z error: expected one of `.`, `;`, `?`, `}`, or an operator, found `=>`
2019-09-16T23:14:09.4929713Z     |
2019-09-16T23:14:09.4929713Z     |
2019-09-16T23:14:09.4930176Z 700 |                 Err(LayoutError::Unsized(_ty)) => {
2019-09-16T23:14:09.4930725Z     |                                                ^^ expected one of `.`, `;`, `?`, `}`, or an operator here
2019-09-16T23:14:09.4951336Z error: unexpected `self` parameter in function
2019-09-16T23:14:09.4951939Z    --> src/librustc_mir/transform/const_prop.rs:715:9
2019-09-16T23:14:09.4952355Z     |
2019-09-16T23:14:09.4952914Z 715 |         &mut self,
---
2019-09-16T23:14:11.5114772Z == clock drift check ==
2019-09-16T23:14:11.5134534Z   local time: Mon Sep 16 23:14:11 UTC 2019
2019-09-16T23:14:11.6657494Z   network time: Mon, 16 Sep 2019 23:14:11 GMT
2019-09-16T23:14:11.6658014Z == end clock drift check ==
2019-09-16T23:14:12.4254934Z ##[error]Bash exited with code '1'.
2019-09-16T23:14:12.4286848Z ##[section]Starting: Checkout
2019-09-16T23:14:12.4288502Z ==============================================================================
2019-09-16T23:14:12.4288568Z Task         : Get sources
2019-09-16T23:14:12.4288609Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-07-19T17:59:44.2316925Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T17:59:44.2534775Z ##[command]git config gc.auto 0
2019-07-19T17:59:44.2606092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T17:59:44.2661659Z ##[command]git config --get-all http.proxy
2019-07-19T17:59:44.2788772Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62791/merge:refs/remotes/pull/62791/merge
---
2019-07-19T18:00:20.2560392Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T18:00:20.2560425Z 
2019-07-19T18:00:20.2560662Z   git checkout -b <new-branch-name>
2019-07-19T18:00:20.2560692Z 
2019-07-19T18:00:20.2560741Z HEAD is now at 166683225 Merge 1f089f05f5681120df52120352c339067eb8c4da into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T18:00:20.2697985Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T18:00:20.2700601Z ==============================================================================
2019-07-19T18:00:20.2700675Z Task         : Bash
2019-07-19T18:00:20.2700721Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T18:07:40.0836295Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-07-19T18:07:48.7157300Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-19T18:07:50.1624499Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-19T18:07:51.3973471Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-19T18:07:53.8764429Z error[E0425]: cannot find value `cm` in this scope
2019-07-19T18:07:53.8776969Z    --> src/libsyntax/parse/diagnostics.rs:380:28
2019-07-19T18:07:53.8777243Z     |
2019-07-19T18:07:53.8777599Z 380 |             let next_pos = cm.lookup_char_pos(self.token.span.lo());
2019-07-19T18:07:53.8778044Z     |                            ^^ help: a local variable with a similar name exists: `sm`
2019-07-19T18:07:53.8778118Z 
2019-07-19T18:07:53.8793798Z error[E0425]: cannot find value `cm` in this scope
2019-07-19T18:07:53.8794140Z    --> src/libsyntax/parse/diagnostics.rs:381:26
2019-07-19T18:07:53.8794400Z     |
2019-07-19T18:07:53.8794709Z 381 |             let op_pos = cm.lookup_char_pos(sp.hi());
2019-07-19T18:07:53.8795068Z     |                          ^^ help: a local variable with a similar name exists: `sm`
2019-07-19T18:07:58.7909368Z error: aborting due to 2 previous errors
2019-07-19T18:07:58.7910166Z 
2019-07-19T18:07:58.7910759Z For more information about this error, try `rustc --explain E0425`.
2019-07-19T18:07:58.8469091Z error: Could not compile `syntax`.
2019-07-19T18:07:58.8469091Z error: Could not compile `syntax`.
2019-07-19T18:07:58.8469895Z 
2019-07-19T18:07:58.8470738Z To learn more, run the command again with --verbose.
2019-07-19T18:07:58.8503267Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-19T18:07:58.8514631Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-19T18:07:58.8514887Z Build completed unsuccessfully in 0:04:33
2019-07-19T18:07:58.8514887Z Build completed unsuccessfully in 0:04:33
2019-07-19T18:07:59.8342304Z ##[error]Bash exited with code '1'.
2019-07-19T18:07:59.8374814Z ##[section]Starting: Checkout
2019-07-19T18:07:59.8376577Z ==============================================================================
2019-07-19T18:07:59.8376631Z Task         : Get sources
2019-07-19T18:07:59.8376676Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

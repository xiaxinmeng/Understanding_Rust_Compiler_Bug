plain
2019-12-23T04:05:47.6958340Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T04:05:47.7204887Z ##[command]git config gc.auto 0
2019-12-23T04:05:47.7258112Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T04:05:47.7317754Z ##[command]git config --get-all http.proxy
2019-12-23T04:05:47.7453461Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65989/merge:refs/remotes/pull/65989/merge
---
2019-12-23T04:13:00.7147559Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-12-23T04:13:42.0743630Z error: variable does not need to be mutable
2019-12-23T04:13:42.0744196Z     --> src/librustc/ty/mod.rs:1705:39
2019-12-23T04:13:42.0744444Z      |
2019-12-23T04:13:42.0744781Z 1705 |     pub fn with_reveal_all_normalized(mut self, tcx: TyCtxt<'tcx>) -> Self {
2019-12-23T04:13:42.0745468Z      |                                       |
2019-12-23T04:13:42.0745963Z      |                                       help: remove this `mut`
2019-12-23T04:13:42.0746311Z      |
2019-12-23T04:13:42.0746519Z      = note: `-D unused-mut` implied by `-D warnings`
2019-12-23T04:13:42.0746519Z      = note: `-D unused-mut` implied by `-D warnings`
2019-12-23T04:13:42.0746564Z 
2019-12-23T04:13:53.9690559Z error: aborting due to previous error
2019-12-23T04:13:53.9690718Z 
2019-12-23T04:13:54.0146646Z error: could not compile `rustc`.
2019-12-23T04:13:54.0147272Z 
2019-12-23T04:13:54.0147748Z To learn more, run the command again with --verbose.
2019-12-23T04:13:54.0172857Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T04:13:54.0194156Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T04:13:54.0194560Z Build completed unsuccessfully in 0:04:45
2019-12-23T04:13:54.0253093Z == clock drift check ==
2019-12-23T04:13:54.0320956Z   local time: Mon Dec 23 04:13:54 UTC 2019
2019-12-23T04:13:54.0320956Z   local time: Mon Dec 23 04:13:54 UTC 2019
2019-12-23T04:13:54.3098876Z   network time: Mon, 23 Dec 2019 04:13:54 GMT
2019-12-23T04:13:54.3102297Z == end clock drift check ==
2019-12-23T04:13:54.9009604Z 
2019-12-23T04:13:54.9126130Z ##[error]Bash exited with code '1'.
2019-12-23T04:13:54.9175024Z ##[section]Starting: Checkout
2019-12-23T04:13:54.9176697Z ==============================================================================
2019-12-23T04:13:54.9176763Z Task         : Get sources
2019-12-23T04:13:54.9176804Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

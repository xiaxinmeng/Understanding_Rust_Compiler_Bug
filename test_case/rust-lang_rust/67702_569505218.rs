plain
2019-12-29T13:14:38.0825567Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T13:14:38.1018829Z ##[command]git config gc.auto 0
2019-12-29T13:14:38.1097481Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T13:14:38.1157431Z ##[command]git config --get-all http.proxy
2019-12-29T13:14:38.1298291Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67702/merge:refs/remotes/pull/67702/merge
---
2019-12-29T13:22:10.4666104Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-29T13:22:10.9636509Z error[E0308]: mismatched types
2019-12-29T13:22:10.9636880Z    --> src/libsyntax_expand/proc_macro_server.rs:331:33
2019-12-29T13:22:10.9637131Z     |
2019-12-29T13:22:10.9637424Z 331 |         let sym = nfc_normalize(sym.as_str());
2019-12-29T13:22:10.9638019Z     |                                 |
2019-12-29T13:22:10.9638403Z     |                                 expected `&str`, found struct `syntax_pos::symbol::SymbolStr`
2019-12-29T13:22:10.9638728Z     |                                 help: consider borrowing here: `&sym.as_str()`
2019-12-29T13:22:10.9671245Z 
2019-12-29T13:22:10.9671245Z 
2019-12-29T13:22:11.4837731Z error: aborting due to previous error
2019-12-29T13:22:11.4842813Z 
2019-12-29T13:22:11.4855278Z For more information about this error, try `rustc --explain E0308`.
2019-12-29T13:22:11.4922524Z error: could not compile `syntax_expand`.
2019-12-29T13:22:11.4935646Z warning: build failed, waiting for other jobs to finish...
2019-12-29T13:23:11.9115770Z error: build failed
2019-12-29T13:23:11.9117974Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-29T13:23:11.9180958Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-29T13:23:11.9182422Z Build completed unsuccessfully in 0:05:11
2019-12-29T13:23:11.9185840Z == clock drift check ==
2019-12-29T13:23:11.9212808Z   local time: Sun Dec 29 13:23:11 UTC 2019
2019-12-29T13:23:11.9212808Z   local time: Sun Dec 29 13:23:11 UTC 2019
2019-12-29T13:23:12.2000768Z   network time: Sun, 29 Dec 2019 13:23:12 GMT
2019-12-29T13:23:12.2004291Z == end clock drift check ==
2019-12-29T13:23:13.2976083Z 
2019-12-29T13:23:13.3044460Z ##[error]Bash exited with code '1'.
2019-12-29T13:23:13.3076138Z ##[section]Starting: Checkout
2019-12-29T13:23:13.3077557Z ==============================================================================
2019-12-29T13:23:13.3077624Z Task         : Get sources
2019-12-29T13:23:13.3077664Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2020-01-07T15:56:35.1578699Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T15:56:35.1595238Z ##[command]git config gc.auto 0
2020-01-07T15:56:35.1597773Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T15:56:35.1603079Z ##[command]git config --get-all http.proxy
2020-01-07T15:56:35.1607041Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67914/merge:refs/remotes/pull/67914/merge
---
2020-01-07T16:05:05.2870392Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-01-07T16:05:06.7015621Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2020-01-07T16:05:06.7016080Z   --> src/librustc_mir/transform/const_prop.rs:92:17
2020-01-07T16:05:06.7016330Z    |
2020-01-07T16:05:06.7016673Z 92 |         if !tcx.substitute_normalize_and_test_predicates(
2020-01-07T16:05:06.7017341Z 
2020-01-07T16:05:07.8292193Z error: aborting due to previous error
2020-01-07T16:05:07.8295526Z 
2020-01-07T16:05:07.8312520Z For more information about this error, try `rustc --explain E0061`.
2020-01-07T16:05:07.8312520Z For more information about this error, try `rustc --explain E0061`.
2020-01-07T16:05:07.8424145Z error: could not compile `rustc_mir`.
2020-01-07T16:05:07.8424564Z warning: build failed, waiting for other jobs to finish...
2020-01-07T16:05:11.3232048Z error: build failed
2020-01-07T16:05:11.3264043Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-07T16:05:11.3279390Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-07T16:05:11.3279688Z Build completed unsuccessfully in 0:06:14
2020-01-07T16:05:11.3343424Z == clock drift check ==
2020-01-07T16:05:11.3356013Z   local time: Tue Jan  7 16:05:11 UTC 2020
2020-01-07T16:05:11.3356013Z   local time: Tue Jan  7 16:05:11 UTC 2020
2020-01-07T16:05:11.4427822Z   network time: Tue, 07 Jan 2020 16:05:11 GMT
2020-01-07T16:05:11.4432225Z == end clock drift check ==
2020-01-07T16:05:11.7995635Z 
2020-01-07T16:05:11.8071029Z ##[error]Bash exited with code '1'.
2020-01-07T16:05:11.8106115Z ##[section]Starting: Checkout
2020-01-07T16:05:11.8109139Z ==============================================================================
2020-01-07T16:05:11.8109197Z Task         : Get sources
2020-01-07T16:05:11.8109247Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

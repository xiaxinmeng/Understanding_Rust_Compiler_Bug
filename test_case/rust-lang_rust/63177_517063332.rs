plain
2019-07-31T23:41:03.8982670Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T23:41:03.9150102Z ##[command]git config gc.auto 0
2019-07-31T23:41:03.9236904Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T23:41:03.9294060Z ##[command]git config --get-all http.proxy
2019-07-31T23:41:03.9434894Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63177/merge:refs/remotes/pull/63177/merge
---
2019-07-31T23:41:36.9830792Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T23:41:36.9830843Z 
2019-07-31T23:41:36.9831084Z   git checkout -b <new-branch-name>
2019-07-31T23:41:36.9831116Z 
2019-07-31T23:41:36.9831170Z HEAD is now at e9a63e959 Merge 2727a675ae54addc21be6411a3678f94a65e6936 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-07-31T23:41:36.9984317Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T23:41:36.9987579Z ==============================================================================
2019-07-31T23:41:36.9987648Z Task         : Bash
2019-07-31T23:41:36.9987701Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T23:46:47.6440612Z    |
2019-07-31T23:46:47.6440989Z 76 | #![feature(const_generics)]
2019-07-31T23:46:47.6441346Z    |            ^^^^^^^^^^^^^^
2019-07-31T23:46:47.6441429Z 
2019-07-31T23:46:50.6673294Z error[E0545]: incorrect 'issue'
2019-07-31T23:46:50.6674689Z     --> src/libcore/iter/traits/iterator.rs:1988:5
2019-07-31T23:46:50.6675668Z      |
2019-07-31T23:46:50.6676528Z 1988 |     #[unstable(feature = "find_result", reason = "new API", issue = "?")]
2019-07-31T23:46:50.6677666Z 
2019-07-31T23:46:54.4750823Z error[E0308]: match arms have incompatible types
2019-07-31T23:46:54.4751330Z     --> src/libcore/iter/traits/iterator.rs:1996:29
2019-07-31T23:46:54.4751766Z      |
2019-07-31T23:46:54.4751766Z      |
2019-07-31T23:46:54.4752102Z 1994 | /             match f(&x) {
2019-07-31T23:46:54.4752469Z 1995 | |                 Ok(false) => LoopState::Continue,
2019-07-31T23:46:54.4753030Z      | |                              ------------------- this is found to be of type `fn(_) -> iter::LoopState<_, _> {iter::LoopState::<_, _>::Continue}`
2019-07-31T23:46:54.4753411Z 1996 | |                 Ok(true) => LoopState::Break(Ok(x)),
2019-07-31T23:46:54.4753864Z      | |                             ^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found enum `iter::LoopState`
2019-07-31T23:46:54.4754232Z 1997 | |                 Err(x) => LoopState::Break(Err(x)),
2019-07-31T23:46:54.4754587Z 1998 | |             }
2019-07-31T23:46:54.4755162Z      | |_____________- `match` arms have incompatible types
2019-07-31T23:46:54.4755433Z      |
2019-07-31T23:46:54.4755795Z      = note: expected type `fn(_) -> iter::LoopState<_, _> {iter::LoopState::<_, _>::Continue}`
2019-07-31T23:46:54.4756139Z                 found type `iter::LoopState<_, result::Result<<Self as iter::traits::iterator::Iterator>::Item, _>>`
2019-07-31T23:46:55.9620479Z    Compiling libc v0.2.54
2019-07-31T23:46:56.7731307Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-31T23:46:57.3140985Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-31T23:46:58.0733258Z error: aborting due to 2 previous errors
2019-07-31T23:46:58.0733258Z error: aborting due to 2 previous errors
2019-07-31T23:46:58.0737909Z 
2019-07-31T23:46:58.0744544Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T23:46:58.1696024Z error: Could not compile `core`.
2019-07-31T23:46:58.1696376Z warning: build failed, waiting for other jobs to finish...
2019-07-31T23:46:58.5388409Z error: build failed
2019-07-31T23:46:58.5409624Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T23:46:58.5419268Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-31T23:46:58.5420229Z Build completed unsuccessfully in 0:02:21
2019-07-31T23:46:58.5420229Z Build completed unsuccessfully in 0:02:21
2019-07-31T23:47:11.8762510Z ##[error]Bash exited with code '1'.
2019-07-31T23:47:11.8807467Z ##[section]Starting: Checkout
2019-07-31T23:47:11.8809213Z ==============================================================================
2019-07-31T23:47:11.8809292Z Task         : Get sources
2019-07-31T23:47:11.8809346Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

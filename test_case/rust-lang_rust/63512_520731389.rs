plain
2019-08-13T07:51:58.8444493Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T07:51:59.7806996Z ##[command]git config gc.auto 0
2019-08-13T07:51:59.7813758Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T07:51:59.7815946Z ##[command]git config --get-all http.proxy
2019-08-13T07:51:59.7820758Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63512/merge:refs/remotes/pull/63512/merge
---
2019-08-13T07:52:36.3385597Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T07:52:36.3385698Z 
2019-08-13T07:52:36.3386055Z   git checkout -b <new-branch-name>
2019-08-13T07:52:36.3386086Z 
2019-08-13T07:52:36.3386154Z HEAD is now at f6523ad5f Merge 5be725f94ea68f3365d3c4d56b0bfb9af57a9bda into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T07:52:36.3547982Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T07:52:36.3551553Z ==============================================================================
2019-08-13T07:52:36.3551701Z Task         : Bash
2019-08-13T07:52:36.3551754Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T07:57:29.5289706Z 76 | #![feature(const_generics)]
2019-08-13T07:57:29.5301927Z    |            ^^^^^^^^^^^^^^
2019-08-13T07:57:29.5304390Z 
2019-08-13T07:57:37.7226616Z error[E0308]: mismatched types
2019-08-13T07:57:37.7232157Z   --> src/libcore/task/poll.rs:90:53
2019-08-13T07:57:37.7237530Z    |
2019-08-13T07:57:37.7243150Z 90 |             Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(f(t)))),
2019-08-13T07:57:37.7250184Z    |                                                     ^^^^^^^^^^^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7252475Z    |
2019-08-13T07:57:37.7305888Z    = note: expected type `result::Result<_, E>`
2019-08-13T07:57:37.7306945Z               found type `option::Option<result::Result<_, _>>`
2019-08-13T07:57:37.7331617Z error[E0308]: mismatched types
2019-08-13T07:57:37.7331617Z error[E0308]: mismatched types
2019-08-13T07:57:37.7336133Z   --> src/libcore/task/poll.rs:91:54
2019-08-13T07:57:37.7339513Z    |
2019-08-13T07:57:37.7343331Z 91 |             Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
2019-08-13T07:57:37.7348984Z    |                                                      ^^^^^^^^^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7349649Z    |
2019-08-13T07:57:37.7353738Z    = note: expected type `result::Result<U, _>`
2019-08-13T07:57:37.7377634Z               found type `option::Option<result::Result<_, _>>`
2019-08-13T07:57:37.7436859Z error[E0308]: mismatched types
2019-08-13T07:57:37.7436859Z error[E0308]: mismatched types
2019-08-13T07:57:37.7440475Z   --> src/libcore/task/poll.rs:92:46
2019-08-13T07:57:37.7444091Z    |
2019-08-13T07:57:37.7449163Z 92 |             Poll::Ready(None) => Poll::Ready(None),
2019-08-13T07:57:37.7452564Z    |                                              ^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7457245Z    |
2019-08-13T07:57:37.7460072Z    = note: expected type `result::Result<U, E>`
2019-08-13T07:57:37.7462824Z               found type `option::Option<_>`
2019-08-13T07:57:37.7567593Z error[E0308]: mismatched types
2019-08-13T07:57:37.7567593Z error[E0308]: mismatched types
2019-08-13T07:57:37.7571165Z    --> src/libcore/task/poll.rs:102:53
2019-08-13T07:57:37.7575450Z     |
2019-08-13T07:57:37.7579727Z 102 |             Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(t))),
2019-08-13T07:57:37.7624815Z     |                                                     ^^^^^^^^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7626884Z     |
2019-08-13T07:57:37.7627218Z     = note: expected type `result::Result<_, U>`
2019-08-13T07:57:37.7627550Z                found type `option::Option<result::Result<_, _>>`
2019-08-13T07:57:37.7675325Z error[E0308]: mismatched types
2019-08-13T07:57:37.7675325Z error[E0308]: mismatched types
2019-08-13T07:57:37.7679921Z    --> src/libcore/task/poll.rs:103:54
2019-08-13T07:57:37.7683593Z     |
2019-08-13T07:57:37.7688067Z 103 |             Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(f(e)))),
2019-08-13T07:57:37.7741750Z     |                                                      ^^^^^^^^^^^^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7742451Z     = note: expected type `result::Result<T, _>`
2019-08-13T07:57:37.7742451Z     = note: expected type `result::Result<T, _>`
2019-08-13T07:57:37.7742785Z                found type `option::Option<result::Result<_, _>>`
2019-08-13T07:57:37.7762804Z error[E0308]: mismatched types
2019-08-13T07:57:37.7762804Z error[E0308]: mismatched types
2019-08-13T07:57:37.7769859Z    --> src/libcore/task/poll.rs:104:46
2019-08-13T07:57:37.7823531Z     |
2019-08-13T07:57:37.7823866Z 104 |             Poll::Ready(None) => Poll::Ready(None),
2019-08-13T07:57:37.7824262Z     |                                              ^^^^ expected enum `result::Result`, found enum `option::Option`
2019-08-13T07:57:37.7825782Z     = note: expected type `result::Result<T, U>`
2019-08-13T07:57:37.7826064Z                found type `option::Option<_>`
2019-08-13T07:57:37.7826102Z 
2019-08-13T07:57:38.0085502Z    Compiling libc v0.2.60
---
2019-08-13T07:57:40.2907104Z For more information about this error, try `rustc --explain E0308`.
2019-08-13T07:57:40.3940121Z error: Could not compile `core`.
2019-08-13T07:57:40.3959247Z warning: build failed, waiting for other jobs to finish...
2019-08-13T07:57:40.7881263Z error: build failed
2019-08-13T07:57:40.7922125Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-13T07:57:40.7935057Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-13T07:57:40.7935321Z Build completed unsuccessfully in 0:02:16
2019-08-13T07:57:40.7935321Z Build completed unsuccessfully in 0:02:16
2019-08-13T07:57:54.0764032Z ##[error]Bash exited with code '1'.
2019-08-13T07:57:54.0806103Z ##[section]Starting: Checkout
2019-08-13T07:57:54.0813263Z ==============================================================================
2019-08-13T07:57:54.0813350Z Task         : Get sources
2019-08-13T07:57:54.0813403Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

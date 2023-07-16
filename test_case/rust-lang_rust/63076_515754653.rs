plain
2019-07-28T11:03:37.7233072Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T11:03:37.7420150Z ##[command]git config gc.auto 0
2019-07-28T11:03:37.7503494Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T11:03:38.6856136Z ##[command]git config --get-all http.proxy
2019-07-28T11:03:38.6863190Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63076/merge:refs/remotes/pull/63076/merge
---
2019-07-28T11:04:10.6590312Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T11:04:10.6590459Z 
2019-07-28T11:04:10.6590815Z   git checkout -b <new-branch-name>
2019-07-28T11:04:10.6590966Z 
2019-07-28T11:04:10.6591094Z HEAD is now at a139c0e9d Merge 73d11f592286bba39b0113a0fd9eabb874d60a90 into 9a239ef4ded03d155c72b68b5a2dd7aff013e141
2019-07-28T11:04:10.6759304Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T11:04:10.6762554Z ==============================================================================
2019-07-28T11:04:10.6762622Z Task         : Bash
2019-07-28T11:04:10.6762677Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-28T11:11:27.3246787Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-28T11:11:39.8692823Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-28T11:12:28.3315706Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-28T11:12:29.4440505Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-28T11:12:29.6625022Z error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
2019-07-28T11:12:29.6625426Z    --> src/librustc_mir/interpret/memory.rs:573:67
2019-07-28T11:12:29.6625649Z     |
2019-07-28T11:12:29.6625952Z 573 |                 bug!("We already checked function pointers above"),
2019-07-28T11:12:29.6626616Z     |                                                                   ^ expected one of `.`, `;`, `?`, `}`, or an operator here
2019-07-28T11:12:36.0173225Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-07-28T11:12:37.6010721Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-07-28T11:12:38.3470244Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-07-28T11:12:39.1113668Z error: aborting due to previous error
2019-07-28T11:12:39.1113668Z error: aborting due to previous error
2019-07-28T11:12:39.1119045Z 
2019-07-28T11:12:39.1810772Z error: Could not compile `rustc_mir`.
2019-07-28T11:12:39.1811130Z warning: build failed, waiting for other jobs to finish...
2019-07-28T11:12:39.5605427Z error: build failed
2019-07-28T11:12:39.5637702Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-28T11:12:39.5646548Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-28T11:12:39.5646648Z Build completed unsuccessfully in 0:05:32
2019-07-28T11:12:39.5646648Z Build completed unsuccessfully in 0:05:32
2019-07-28T11:12:40.7241265Z ##[error]Bash exited with code '1'.
2019-07-28T11:12:40.7282379Z ##[section]Starting: Checkout
2019-07-28T11:12:40.7284207Z ==============================================================================
2019-07-28T11:12:40.7284270Z Task         : Get sources
2019-07-28T11:12:40.7284325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-13T01:43:01.8221555Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T01:43:01.8406297Z ##[command]git config gc.auto 0
2019-08-13T01:43:01.8482912Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T01:43:01.8548186Z ##[command]git config --get-all http.proxy
2019-08-13T01:43:01.8693185Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63507/merge:refs/remotes/pull/63507/merge
---
2019-08-13T01:43:40.6829195Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T01:43:40.6829264Z 
2019-08-13T01:43:40.6829802Z   git checkout -b <new-branch-name>
2019-08-13T01:43:40.6829953Z 
2019-08-13T01:43:40.6830284Z HEAD is now at 404f22e9c Merge c96135e642349e5ece7a550e8b80de320585cda0 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T01:43:40.6996123Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T01:43:40.6999076Z ==============================================================================
2019-08-13T01:43:40.6999127Z Task         : Bash
2019-08-13T01:43:40.6999188Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T02:18:17.7698711Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-13T02:18:23.0062950Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-13T02:18:42.4437244Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-13T02:20:17.5064612Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-08-13T02:21:05.5748567Z error: usage of `ty::TyKind::<kind>`
2019-08-13T02:21:05.5749002Z    --> src/librustc/infer/error_reporting/need_type_info.rs:178:33
2019-08-13T02:21:05.5749402Z     |
2019-08-13T02:21:05.5778752Z 178 |             Some(ty::TyS { sty: ty::TyKind::Closure(def_id, substs), .. }) => {
2019-08-13T02:21:05.5779215Z     |                                 ^^^^^^^^^^ help: try using ty::<kind> directly: `ty`
2019-08-13T02:21:05.5779516Z     |
2019-08-13T02:21:05.5779851Z     = note: `-D rustc::usage-of-ty-tykind` implied by `-D rustc::internal`
2019-08-13T02:21:06.6584023Z error: aborting due to previous error
2019-08-13T02:21:06.6584134Z 
2019-08-13T02:21:07.0971692Z error: Could not compile `rustc`.
2019-08-13T02:21:07.0972044Z warning: build failed, waiting for other jobs to finish...
2019-08-13T02:21:07.0972044Z warning: build failed, waiting for other jobs to finish...
2019-08-13T02:21:10.6068212Z error: build failed
2019-08-13T02:21:10.6096564Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-13T02:21:10.6097006Z expected success, got: exit code: 101
2019-08-13T02:21:10.6109350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-13T02:21:10.6109425Z Build completed unsuccessfully in 0:30:47
2019-08-13T02:21:11.3141856Z ##[error]Bash exited with code '1'.
2019-08-13T02:21:11.3195400Z ##[section]Starting: Checkout
2019-08-13T02:21:11.3197813Z ==============================================================================
2019-08-13T02:21:11.3197875Z Task         : Get sources
2019-08-13T02:21:11.3197927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

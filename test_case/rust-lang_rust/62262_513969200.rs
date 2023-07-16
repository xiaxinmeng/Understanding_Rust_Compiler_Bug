plain
2019-07-22T21:10:41.2981243Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-22T21:10:41.3177615Z ##[command]git config gc.auto 0
2019-07-22T21:10:41.3244196Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-22T21:10:41.3292057Z ##[command]git config --get-all http.proxy
2019-07-22T21:10:41.3421359Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62262/merge:refs/remotes/pull/62262/merge
---
2019-07-22T21:11:16.4056600Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T21:11:16.4056835Z 
2019-07-22T21:11:16.4057622Z   git checkout -b <new-branch-name>
2019-07-22T21:11:16.4057794Z 
2019-07-22T21:11:16.4057946Z HEAD is now at f606ee43b Merge fe440ebf2099e08d8744532811936288ae863de9 into e649e903440bfd919bfc9db848c28df6d795a116
2019-07-22T21:11:16.4196318Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-22T21:11:16.4199772Z ==============================================================================
2019-07-22T21:11:16.4199826Z Task         : Bash
2019-07-22T21:11:16.4199866Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-22T21:42:54.1224568Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-22T21:43:11.7851956Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-22T21:44:29.5429451Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-22T21:50:54.4920566Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-22T21:51:11.7993994Z error: unused `std::iter::Map` in field `replace_with` that must be used
2019-07-22T21:51:11.8002260Z    --> src/librustc_mir/transform/add_retag.rs:100:13
2019-07-22T21:51:11.8003082Z     |
2019-07-22T21:51:11.8003681Z 100 | /             basic_blocks[START_BLOCK].statements.splice(0..0,
2019-07-22T21:51:11.8004454Z 101 | |                 places.into_iter().map(|place| Statement {
2019-07-22T21:51:11.8004912Z 102 | |                     source_info,
2019-07-22T21:51:11.8005895Z 103 | |                     kind: StatementKind::Retag(RetagKind::FnEntry, place),
2019-07-22T21:51:11.8006827Z 105 | |             );
2019-07-22T21:51:11.8007238Z     | |______________^
2019-07-22T21:51:11.8007627Z     |
2019-07-22T21:51:11.8007627Z     |
2019-07-22T21:51:11.8008226Z     = note: `#[deny(unused_must_use)]` on by default
2019-07-22T21:51:11.8009533Z     = note: iterators are lazy and do nothing unless consumed
2019-07-22T21:51:11.9801389Z error: aborting due to previous error
2019-07-22T21:51:11.9802807Z 
2019-07-22T21:51:12.1386600Z error: Could not compile `rustc_mir`.
2019-07-22T21:51:12.1393896Z warning: build failed, waiting for other jobs to finish...
2019-07-22T21:51:12.1393896Z warning: build failed, waiting for other jobs to finish...
2019-07-22T21:52:16.2301490Z error: build failed
2019-07-22T21:52:16.2331373Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-22T21:52:16.2331480Z expected success, got: exit code: 101
2019-07-22T21:52:16.2337040Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-22T21:52:16.2337166Z Build completed unsuccessfully in 0:34:27
2019-07-22T21:52:17.1239792Z ##[error]Bash exited with code '1'.
2019-07-22T21:52:17.1271604Z ##[section]Starting: Checkout
2019-07-22T21:52:17.1273052Z ==============================================================================
2019-07-22T21:52:17.1273093Z Task         : Get sources
2019-07-22T21:52:17.1273129Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

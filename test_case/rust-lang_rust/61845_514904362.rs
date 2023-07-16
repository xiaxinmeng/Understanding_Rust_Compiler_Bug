plain
2019-07-25T05:26:35.4271226Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T05:26:35.4489042Z ##[command]git config gc.auto 0
2019-07-25T05:26:35.4553778Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T05:26:36.2061320Z ##[command]git config --get-all http.proxy
2019-07-25T05:26:36.2064485Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61845/merge:refs/remotes/pull/61845/merge
---
2019-07-25T05:27:09.5690867Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T05:27:09.5690896Z 
2019-07-25T05:27:09.5691116Z   git checkout -b <new-branch-name>
2019-07-25T05:27:09.5691145Z 
2019-07-25T05:27:09.5691193Z HEAD is now at aab8eb087 Merge e0264ab811e4435a9b83db46886b6b8bc11c194b into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-25T05:27:09.5827478Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T05:27:09.5830468Z ==============================================================================
2019-07-25T05:27:09.5830546Z Task         : Bash
2019-07-25T05:27:09.5830594Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T05:34:13.5354437Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-25T05:34:14.6401157Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-25T05:34:26.0080128Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-25T05:34:33.9531396Z error[E0308]: mismatched types
2019-07-25T05:34:33.9532530Z     --> src/librustc/dep_graph/graph.rs:1031:40
2019-07-25T05:34:33.9533239Z      |
2019-07-25T05:34:33.9533741Z 1031 |             node_count: AtomicU64::new(prev_graph_node_count),
2019-07-25T05:34:33.9534317Z      |                                        ^^^^^^^^^^^^^^^^^^^^^ expected u64, found usize
2019-07-25T05:34:33.9534813Z help: you can convert an `usize` to `u64` and panic if the converted value wouldn't fit
2019-07-25T05:34:33.9535234Z      |
2019-07-25T05:34:33.9535744Z 1031 |             node_count: AtomicU64::new(prev_graph_node_count.try_into().unwrap()),
2019-07-25T05:34:33.9537032Z 
2019-07-25T05:34:47.8986309Z error: aborting due to previous error
2019-07-25T05:34:47.8986561Z 
2019-07-25T05:34:47.8987133Z For more information about this error, try `rustc --explain E0308`.
2019-07-25T05:34:47.8987133Z For more information about this error, try `rustc --explain E0308`.
2019-07-25T05:34:48.0701845Z error: Could not compile `rustc`.
2019-07-25T05:34:48.0701951Z 
2019-07-25T05:34:48.0702412Z To learn more, run the command again with --verbose.
2019-07-25T05:34:48.0725470Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-25T05:34:48.0741538Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-25T05:34:48.0741606Z Build completed unsuccessfully in 0:04:36
2019-07-25T05:34:48.0741606Z Build completed unsuccessfully in 0:04:36
2019-07-25T05:34:48.7510405Z ##[error]Bash exited with code '1'.
2019-07-25T05:34:48.7542328Z ##[section]Starting: Checkout
2019-07-25T05:34:48.7544535Z ==============================================================================
2019-07-25T05:34:48.7544614Z Task         : Get sources
2019-07-25T05:34:48.7544663Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

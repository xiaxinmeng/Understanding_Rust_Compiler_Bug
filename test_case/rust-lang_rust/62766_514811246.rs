plain
2019-07-24T21:12:41.2899954Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T21:12:41.3096181Z ##[command]git config gc.auto 0
2019-07-24T21:12:41.8327643Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T21:12:41.8333547Z ##[command]git config --get-all http.proxy
2019-07-24T21:12:41.8341011Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62766/merge:refs/remotes/pull/62766/merge
---
2019-07-24T21:13:14.9785727Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T21:13:14.9785981Z 
2019-07-24T21:13:14.9786267Z   git checkout -b <new-branch-name>
2019-07-24T21:13:14.9786296Z 
2019-07-24T21:13:14.9786338Z HEAD is now at 7c8dda4dd Merge a769554d4c123fac107a5ca4703f999e0951885b into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T21:13:14.9936050Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T21:13:14.9938380Z ==============================================================================
2019-07-24T21:13:14.9938427Z Task         : Bash
2019-07-24T21:13:14.9938483Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T21:22:06.1344510Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-07-24T21:22:09.4399341Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-07-24T21:22:10.6042427Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-07-24T21:22:11.3610468Z     Checking rustc_interface v0.0.0 (/checkout/src/librustc_interface)
2019-07-24T21:22:11.9200623Z error[E0609]: no field `emit_artifact_notifications` on type `rustc::session::config::DebuggingOptions`
2019-07-24T21:22:11.9201095Z    --> src/librustc_interface/passes.rs:693:41
2019-07-24T21:22:11.9201387Z     |
2019-07-24T21:22:11.9206675Z 693 |             if sess.opts.debugging_opts.emit_artifact_notifications {
2019-07-24T21:22:11.9207850Z     |
2019-07-24T21:22:11.9207850Z     |
2019-07-24T21:22:11.9208391Z     = note: available fields are: `codegen_backend`, `verbose`, `span_free_formats`, `identify_regions`, `borrowck` ... and 106 others
2019-07-24T21:22:12.2209273Z error: aborting due to previous error
2019-07-24T21:22:12.2209413Z 
2019-07-24T21:22:12.2209747Z For more information about this error, try `rustc --explain E0609`.
2019-07-24T21:22:12.2437109Z error: Could not compile `rustc_interface`.
2019-07-24T21:22:12.2437109Z error: Could not compile `rustc_interface`.
2019-07-24T21:22:12.2437693Z 
2019-07-24T21:22:12.2438030Z To learn more, run the command again with --verbose.
2019-07-24T21:22:12.2459464Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-24T21:22:12.2478667Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-24T21:22:12.2478981Z Build completed unsuccessfully in 0:05:43
2019-07-24T21:22:12.2478981Z Build completed unsuccessfully in 0:05:43
2019-07-24T21:22:13.5319738Z ##[error]Bash exited with code '1'.
2019-07-24T21:22:13.5352100Z ##[section]Starting: Checkout
2019-07-24T21:22:13.5353475Z ==============================================================================
2019-07-24T21:22:13.5353535Z Task         : Get sources
2019-07-24T21:22:13.5353572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

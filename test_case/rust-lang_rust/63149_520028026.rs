plain
2019-08-09T18:46:29.5461739Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T18:46:29.5461926Z 
2019-08-09T18:46:29.5462310Z   git checkout -b <new-branch-name>
2019-08-09T18:46:29.5462511Z 
2019-08-09T18:46:29.5462941Z HEAD is now at 047f9faa8 Auto merge of #63149 - petrochenkov:lazypop2, r=eddyb
2019-08-09T18:46:29.5629402Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T18:46:29.5632337Z ==============================================================================
2019-08-09T18:46:29.5632443Z Task         : Bash
2019-08-09T18:46:29.5632525Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T18:54:39.9359151Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-08-09T18:54:41.1093100Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-09T18:54:41.6917094Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
2019-08-09T18:54:41.9702335Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-09T18:54:43.2198530Z error[E0599]: no variant or associated item named `AssocExistential` found for type `rustc::hir::def::DefKind` in the current scope
2019-08-09T18:54:43.2204323Z    --> src/librustc_resolve/build_reduced_graph.rs:665:33
2019-08-09T18:54:43.2210247Z     |
2019-08-09T18:54:43.2211591Z 665 |             | Res::Def(DefKind::AssocExistential, _)
2019-08-09T18:54:43.2217825Z     |                                 ^^^^^^^^^^^^^^^^ variant or associated item not found in `rustc::hir::def::DefKind`
2019-08-09T18:54:43.4002363Z error: aborting due to previous error
2019-08-09T18:54:43.4002501Z 
2019-08-09T18:54:43.4003124Z For more information about this error, try `rustc --explain E0599`.
2019-08-09T18:54:43.4188001Z error: Could not compile `rustc_resolve`.
2019-08-09T18:54:43.4188001Z error: Could not compile `rustc_resolve`.
2019-08-09T18:54:43.4188400Z warning: build failed, waiting for other jobs to finish...
2019-08-09T18:54:53.7419786Z error: build failed
2019-08-09T18:54:53.7446416Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-09T18:54:53.7447144Z expected success, got: exit code: 101
2019-08-09T18:54:53.7456765Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-09T18:54:53.7457172Z Build completed unsuccessfully in 0:05:36
2019-08-09T18:54:55.0424325Z ##[error]Bash exited with code '1'.
2019-08-09T18:54:55.0455823Z ##[section]Starting: Upload CPU usage statistics
2019-08-09T18:54:55.0458957Z ==============================================================================
2019-08-09T18:54:55.0459055Z Task         : Bash
2019-08-09T18:54:55.0459430Z Description  : Run a Bash script on macOS, Linux, or Windows

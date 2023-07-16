plain
2020-03-23T00:10:19.1518625Z ========================== Starting Command Output ===========================
2020-03-23T00:10:19.1521367Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/90527b04-aee4-4ed3-9821-ae6b14500a5e.sh
2020-03-23T00:10:19.1521693Z 
2020-03-23T00:10:19.1525087Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T00:10:19.1544893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-23T00:10:19.1548476Z Task         : Get sources
2020-03-23T00:10:19.1548763Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T00:10:19.1549060Z Version      : 1.0.0
2020-03-23T00:10:19.1549247Z Author       : Microsoft
---
2020-03-23T00:10:20.1547294Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T00:10:20.1553203Z ##[command]git config gc.auto 0
2020-03-23T00:10:20.1557462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T00:10:20.1561403Z ##[command]git config --get-all http.proxy
2020-03-23T00:10:20.1568652Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69866/merge:refs/remotes/pull/69866/merge
---
2020-03-23T00:18:17.9649857Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-23T00:18:18.9394214Z error[E0599]: no method named `def_span` found for reference `&'a rustc_span::source_map::SourceMap` in the current scope
2020-03-23T00:18:18.9395871Z    --> src/librustc_builtin_macros/proc_macro_harness.rs:201:51
2020-03-23T00:18:18.9396843Z     |
2020-03-23T00:18:18.9397875Z 201 |             self.handler.span_err(self.source_map.def_span(item.span), msg);
2020-03-23T00:18:18.9399364Z     |                                                   ^^^^^^^^ method not found in `&'a rustc_span::source_map::SourceMap`
2020-03-23T00:18:18.9416697Z error[E0599]: no method named `def_span` found for reference `&'a rustc_span::source_map::SourceMap` in the current scope
2020-03-23T00:18:18.9420101Z    --> src/librustc_builtin_macros/proc_macro_harness.rs:220:51
2020-03-23T00:18:18.9420685Z     |
2020-03-23T00:18:18.9420685Z     |
2020-03-23T00:18:18.9421400Z 220 |             self.handler.span_err(self.source_map.def_span(item.span), msg);
2020-03-23T00:18:18.9427466Z     |                                                   ^^^^^^^^ method not found in `&'a rustc_span::source_map::SourceMap`
2020-03-23T00:18:18.9435849Z error[E0599]: no method named `def_span` found for reference `&'a rustc_span::source_map::SourceMap` in the current scope
2020-03-23T00:18:18.9436778Z    --> src/librustc_builtin_macros/proc_macro_harness.rs:239:51
2020-03-23T00:18:18.9437376Z     |
2020-03-23T00:18:18.9437376Z     |
2020-03-23T00:18:18.9438074Z 239 |             self.handler.span_err(self.source_map.def_span(item.span), msg);
2020-03-23T00:18:18.9439238Z     |                                                   ^^^^^^^^ method not found in `&'a rustc_span::source_map::SourceMap`
2020-03-23T00:18:18.9454515Z error[E0599]: no method named `def_span` found for reference `&'a rustc_span::source_map::SourceMap` in the current scope
2020-03-23T00:18:18.9455583Z    --> src/librustc_builtin_macros/proc_macro_harness.rs:250:55
2020-03-23T00:18:18.9456143Z     |
2020-03-23T00:18:18.9456143Z     |
2020-03-23T00:18:18.9456848Z 250 |                 self.handler.span_err(self.source_map.def_span(item.span), msg);
2020-03-23T00:18:18.9458067Z     |                                                       ^^^^^^^^ method not found in `&'a rustc_span::source_map::SourceMap`
2020-03-23T00:18:18.9488561Z error[E0599]: no method named `def_span` found for reference `&'a rustc_span::source_map::SourceMap` in the current scope
2020-03-23T00:18:18.9489504Z    --> src/librustc_builtin_macros/proc_macro_harness.rs:301:71
2020-03-23T00:18:18.9490049Z     |
2020-03-23T00:18:18.9490049Z     |
2020-03-23T00:18:18.9490790Z 301 |                 self.check_not_pub_in_root(&item.vis, self.source_map.def_span(item.span));
2020-03-23T00:18:18.9492190Z     |                                                                       ^^^^^^^^ method not found in `&'a rustc_span::source_map::SourceMap`
2020-03-23T00:18:18.9943429Z error: aborting due to 5 previous errors
2020-03-23T00:18:18.9943778Z 
2020-03-23T00:18:18.9946903Z For more information about this error, try `rustc --explain E0599`.
2020-03-23T00:18:19.0020983Z error: could not compile `rustc_builtin_macros`.
2020-03-23T00:18:19.0020983Z error: could not compile `rustc_builtin_macros`.
2020-03-23T00:18:19.0021443Z 
2020-03-23T00:18:19.0021957Z To learn more, run the command again with --verbose.
2020-03-23T00:18:19.0025300Z warning: build failed, waiting for other jobs to finish...
2020-03-23T00:18:41.1758018Z error: build failed
2020-03-23T00:18:41.1781943Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-23T00:18:41.1789683Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-23T00:18:41.1790279Z Build completed unsuccessfully in 0:04:31
2020-03-23T00:18:41.1841214Z == clock drift check ==
2020-03-23T00:18:41.1854807Z   local time: Mon Mar 23 00:18:41 UTC 2020
2020-03-23T00:18:41.1854807Z   local time: Mon Mar 23 00:18:41 UTC 2020
2020-03-23T00:18:41.4757447Z   network time: Mon, 23 Mar 2020 00:18:41 GMT
2020-03-23T00:18:41.4758102Z == end clock drift check ==
2020-03-23T00:18:42.2305604Z 
2020-03-23T00:18:42.2401242Z ##[error]Bash exited with code '1'.
2020-03-23T00:18:42.2417321Z ##[section]Finishing: Run build
2020-03-23T00:18:42.2484024Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-23T00:18:42.2489777Z Task         : Get sources
2020-03-23T00:18:42.2490174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T00:18:42.2490519Z Version      : 1.0.0
2020-03-23T00:18:42.2490766Z Author       : Microsoft
2020-03-23T00:18:42.2490766Z Author       : Microsoft
2020-03-23T00:18:42.2491167Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T00:18:42.2491617Z ==============================================================================
2020-03-23T00:18:42.6298299Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T00:18:42.6354222Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-23T00:18:42.6465346Z Cleaning up task key
2020-03-23T00:18:42.6466609Z Start cleaning up orphan processes.
2020-03-23T00:18:42.6691314Z Terminate orphan process: pid (3809) (python)
2020-03-23T00:18:42.6982130Z ##[section]Finishing: Finalize Job

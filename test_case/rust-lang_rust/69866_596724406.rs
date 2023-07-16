plain
2020-03-09T18:43:35.5514990Z Prepare build directory.
2020-03-09T18:43:35.5825428Z Set build variables.
2020-03-09T18:43:35.5859166Z Download all required tasks.
2020-03-09T18:43:35.5984200Z Downloading task: Bash (3.163.1)
2020-03-09T18:43:36.6909850Z Checking job knob settings.
2020-03-09T18:43:36.6929631Z Finished checking job knob settings.
2020-03-09T18:43:36.7556372Z ##[section]Finishing: Initialize job
2020-03-09T18:43:36.7891569Z ##[section]Starting: Configure Job Name
2020-03-09T18:43:36.8099122Z ==============================================================================
2020-03-09T18:43:36.8099951Z Task         : Bash
---
2020-03-09T18:43:38.1468322Z ========================== Starting Command Output ===========================
2020-03-09T18:43:38.1470531Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/59bb9c66-04ba-4103-91c9-81784ba9a1a5.sh
2020-03-09T18:43:38.1470756Z 
2020-03-09T18:43:38.1473970Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T18:43:38.1493307Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-09T18:43:38.1496876Z Task         : Get sources
2020-03-09T18:43:38.1497144Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T18:43:38.1497400Z Version      : 1.0.0
2020-03-09T18:43:38.1497573Z Author       : Microsoft
---
2020-03-09T18:43:39.1645815Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T18:43:39.1657638Z ##[command]git config gc.auto 0
2020-03-09T18:43:39.1667144Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T18:43:39.1671563Z ##[command]git config --get-all http.proxy
2020-03-09T18:43:39.1680900Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69866/merge:refs/remotes/pull/69866/merge
---
2020-03-09T18:50:25.4421901Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-09T18:50:26.2042561Z error[E0599]: no method named `def_span` found for reference `&rustc_span::source_map::SourceMap` in the current scope
2020-03-09T18:50:26.2043971Z     --> src/librustc_typeck/check/mod.rs:3874:74
2020-03-09T18:50:26.2044670Z      |
2020-03-09T18:50:26.2045723Z 3874 |             if let Some(def_s) = def_span.map(|sp| tcx.sess.source_map().def_span(sp)) {
2020-03-09T18:50:26.2047116Z      |                                                                          ^^^^^^^^ method not found in `&rustc_span::source_map::SourceMap`
2020-03-09T18:50:26.2876692Z error[E0599]: no method named `def_span` found for reference `&rustc_span::source_map::SourceMap` in the current scope
2020-03-09T18:50:26.2877827Z     --> src/librustc_typeck/check/mod.rs:4980:51
2020-03-09T18:50:26.2878896Z      |
2020-03-09T18:50:26.2878896Z      |
2020-03-09T18:50:26.2879709Z 4980 |                 let sp = self.sess().source_map().def_span(sp);
2020-03-09T18:50:26.2880935Z      |                                                   ^^^^^^^^ method not found in `&rustc_span::source_map::SourceMap`
2020-03-09T18:50:27.8918270Z error[E0599]: no method named `def_span` found for reference `&rustc_span::source_map::SourceMap` in the current scope
2020-03-09T18:50:27.8919135Z   --> src/librustc_typeck/coherence/orphan.rs:37:25
2020-03-09T18:50:27.8919616Z    |
2020-03-09T18:50:27.8919616Z    |
2020-03-09T18:50:27.8926150Z 37 |             let sp = sm.def_span(item.span);
2020-03-09T18:50:27.8927553Z    |                         ^^^^^^^^ method not found in `&rustc_span::source_map::SourceMap`
2020-03-09T18:50:28.3043083Z error: aborting due to 3 previous errors
2020-03-09T18:50:28.3047495Z 
2020-03-09T18:50:28.3054034Z For more information about this error, try `rustc --explain E0599`.
2020-03-09T18:50:28.3114290Z error: could not compile `rustc_typeck`.
2020-03-09T18:50:28.3114290Z error: could not compile `rustc_typeck`.
2020-03-09T18:50:28.3115146Z warning: build failed, waiting for other jobs to finish...
2020-03-09T18:50:37.5281432Z error: build failed
2020-03-09T18:50:37.5307457Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-09T18:50:37.5319929Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-09T18:50:37.5320312Z Build completed unsuccessfully in 0:04:22
2020-03-09T18:50:37.5364786Z == clock drift check ==
2020-03-09T18:50:37.5378930Z   local time: Mon Mar  9 18:50:37 UTC 2020
2020-03-09T18:50:37.5378930Z   local time: Mon Mar  9 18:50:37 UTC 2020
2020-03-09T18:50:37.6331149Z   network time: Mon, 09 Mar 2020 18:50:37 GMT
2020-03-09T18:50:37.6333953Z == end clock drift check ==
2020-03-09T18:50:38.3266383Z 
2020-03-09T18:50:38.3331103Z ##[error]Bash exited with code '1'.
2020-03-09T18:50:38.3343307Z ##[section]Finishing: Run build
2020-03-09T18:50:38.3385226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-09T18:50:38.3390508Z Task         : Get sources
2020-03-09T18:50:38.3390830Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T18:50:38.3391126Z Version      : 1.0.0
2020-03-09T18:50:38.3391345Z Author       : Microsoft
2020-03-09T18:50:38.3391345Z Author       : Microsoft
2020-03-09T18:50:38.3391768Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-09T18:50:38.3392123Z ==============================================================================
2020-03-09T18:50:38.6340967Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-09T18:50:38.6383237Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69866/merge to s
2020-03-09T18:50:38.6469070Z Cleaning up task key
2020-03-09T18:50:38.6470415Z Start cleaning up orphan processes.
2020-03-09T18:50:38.6636814Z Terminate orphan process: pid (3552) (python)
2020-03-09T18:50:38.6759290Z ##[section]Finishing: Finalize Job

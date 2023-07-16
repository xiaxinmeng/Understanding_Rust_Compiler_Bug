plain
2019-12-12T06:57:28.3946861Z [RUSTC-TIMING] libgit2_sys test:false 0.586
2019-12-12T06:57:32.7469790Z    Compiling git2-curl v0.11.0
2019-12-12T06:57:36.2759232Z [RUSTC-TIMING] git2_curl test:false 3.527
2019-12-12T06:57:36.2811341Z    Compiling cargo v0.42.0 (/checkout/src/tools/cargo)
2019-12-12T06:57:43.6837997Z warning: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
2019-12-12T06:57:43.6839091Z   --> src/tools/cargo/src/cargo/ops/cargo_doc.rs:97:23
2019-12-12T06:57:43.6840313Z 97 |                     e.description()
2019-12-12T06:57:43.6840897Z    |                       ^^^^^^^^^^^
2019-12-12T06:57:43.6841404Z    |
2019-12-12T06:57:43.6842168Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-12T07:06:04.6412001Z    Compiling cargo-test-macro v0.1.0 (/checkout/src/tools/cargo/crates/cargo-test-macro)
2019-12-12T07:06:04.6434276Z [RUSTC-TIMING] cargo_platform test:false 2.291
2019-12-12T07:06:04.6439149Z [RUSTC-TIMING] rustc_workspace_hack test:false 0.056
2019-12-12T07:06:04.6443861Z [RUSTC-TIMING] crates_io test:false 6.674
2019-12-12T07:06:04.6448997Z warning: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
2019-12-12T07:06:04.6449518Z   --> src/tools/cargo/src/cargo/ops/cargo_doc.rs:97:23
2019-12-12T07:06:04.6451051Z 97 |                     e.description()
2019-12-12T07:06:04.6451516Z    |                       ^^^^^^^^^^^
2019-12-12T07:06:04.6451870Z    |
2019-12-12T07:06:04.6452479Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-12T07:06:05.8789809Z    Compiling cargo-test-support v0.1.0 (/checkout/src/tools/cargo/crates/cargo-test-support)
2019-12-12T07:06:21.4575626Z [RUSTC-TIMING] cargo_test_support test:false 15.575
2019-12-12T07:06:21.4622232Z    Compiling cargo v0.42.0 (/checkout/src/tools/cargo)
2019-12-12T07:06:26.1558617Z [RUSTC-TIMING] build_std test:true 4.686
2019-12-12T07:06:28.7242861Z error: use of deprecated item 'std::error::Error::description': use the Display impl or to_string()
2019-12-12T07:06:28.7243767Z   --> src/tools/cargo/src/cargo/ops/cargo_doc.rs:97:23
2019-12-12T07:06:28.7248873Z 97 |                     e.description()
2019-12-12T07:06:28.7249275Z    |                       ^^^^^^^^^^^
2019-12-12T07:06:28.7249547Z    |
2019-12-12T07:06:28.7249777Z note: lint level defined here
---
2019-12-12T07:07:45.8942768Z 
2019-12-12T07:07:45.8942889Z 
2019-12-12T07:07:45.8956629Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-12-12T07:07:45.9008211Z Build completed unsuccessfully in 1:31:34
2019-12-12T07:07:45.9008724Z make: *** [check-aux] Error 1
2019-12-12T07:07:45.9010255Z Makefile:50: recipe for target 'check-aux' failed
2019-12-12T07:07:45.9033415Z   local time: Thu Dec 12 07:07:45 UTC 2019
2019-12-12T07:07:46.4292486Z   network time: Thu, 12 Dec 2019 07:07:46 GMT
2019-12-12T07:07:46.4293356Z == end clock drift check ==
2019-12-12T07:07:47.5033358Z 
2019-12-12T07:07:47.5033358Z 
2019-12-12T07:07:47.5142391Z ##[error]Bash exited with code '2'.
2019-12-12T07:07:47.5177091Z ##[section]Starting: Checkout
2019-12-12T07:07:47.5178962Z ==============================================================================
2019-12-12T07:07:47.5179039Z Task         : Get sources
2019-12-12T07:07:47.5179126Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

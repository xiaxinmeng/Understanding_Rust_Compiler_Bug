plain
2020-03-11T18:34:51.6325482Z 
2020-03-11T18:34:51.6325618Z failures:
2020-03-11T18:34:51.6325924Z 
2020-03-11T18:34:51.6326925Z ---- fix::fix_with_common stdout ----
2020-03-11T18:34:51.6327640Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --edition --allow-no-vcs`
2020-03-11T18:34:51.6328650Z thread 'fix::fix_with_common' panicked at 'assertion failed: `(left == right)`
2020-03-11T18:34:51.6329663Z   left: `"pub fn try() {}"`,
2020-03-11T18:34:51.6330441Z  right: `"pub fn r#try() {}"`', src/tools/cargo/tests/testsuite/fix.rs:1229:5
2020-03-11T18:34:51.6331189Z 
2020-03-11T18:34:51.6331282Z 
2020-03-11T18:34:51.6331414Z failures:
2020-03-11T18:34:51.6331611Z     fix::fix_with_common
---
2020-03-11T18:34:51.6374139Z 
2020-03-11T18:34:51.6374242Z 
2020-03-11T18:34:51.6382719Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-03-11T18:34:51.6383223Z Build completed unsuccessfully in 1:54:15
2020-03-11T18:34:51.6432343Z Makefile:50: recipe for target 'check-aux' failed
2020-03-11T18:34:51.6432744Z make: *** [check-aux] Error 1
2020-03-11T18:34:52.1827337Z   local time: Wed Mar 11 18:34:51 UTC 2020
2020-03-11T18:34:52.1827787Z   network time: Wed, 11 Mar 2020 18:34:51 GMT
2020-03-11T18:34:52.1828130Z == end clock drift check ==
2020-03-11T18:34:55.0281141Z 
2020-03-11T18:34:55.0281141Z 
2020-03-11T18:34:55.0347211Z ##[error]Bash exited with code '2'.
2020-03-11T18:34:55.0435289Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-11T18:34:55.0441483Z ==============================================================================
2020-03-11T18:34:55.0442050Z Task         : Get sources
2020-03-11T18:34:55.0442391Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

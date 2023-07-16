plain
2020-03-11T00:12:13.0591612Z 
2020-03-11T00:12:13.0592105Z failures:
2020-03-11T00:12:13.0592268Z 
2020-03-11T00:12:13.0593312Z ---- fix::fix_with_common stdout ----
2020-03-11T00:12:13.0594112Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo fix --edition --allow-no-vcs`
2020-03-11T00:12:13.0602858Z thread 'fix::fix_with_common' panicked at 'assertion failed: `(left == right)`
2020-03-11T00:12:13.0603229Z   left: `"pub fn try() {}"`,
2020-03-11T00:12:13.0603889Z  right: `"pub fn r#try() {}"`', src/tools/cargo/tests/testsuite/fix.rs:1229:5
2020-03-11T00:12:13.0604495Z 
2020-03-11T00:12:13.0604589Z 
2020-03-11T00:12:13.0604738Z failures:
2020-03-11T00:12:13.0604911Z     fix::fix_with_common
---
2020-03-11T00:12:13.0669767Z 
2020-03-11T00:12:13.0669865Z 
2020-03-11T00:12:13.0678429Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-03-11T00:12:13.0679306Z Build completed unsuccessfully in 2:00:11
2020-03-11T00:12:13.0755267Z Makefile:50: recipe for target 'check-aux' failed
2020-03-11T00:12:13.0756854Z make: *** [check-aux] Error 1
2020-03-11T00:12:13.0776180Z   local time: Wed Mar 11 00:12:13 UTC 2020
2020-03-11T00:12:13.3691959Z   network time: Wed, 11 Mar 2020 00:12:13 GMT
2020-03-11T00:12:13.3692293Z == end clock drift check ==
2020-03-11T00:12:16.1109048Z 
2020-03-11T00:12:16.1109048Z 
2020-03-11T00:12:16.1222530Z ##[error]Bash exited with code '2'.
2020-03-11T00:12:16.1308497Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-11T00:12:16.1327243Z ==============================================================================
2020-03-11T00:12:16.1327714Z Task         : Get sources
2020-03-11T00:12:16.1328171Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

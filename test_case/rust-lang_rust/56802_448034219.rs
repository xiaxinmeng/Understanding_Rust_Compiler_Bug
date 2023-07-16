plain
travis_time:end:1bd9e0ac:start=1545083970361818685,finish=1545084059795746139,duration=89433927454
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:48:02]     Checking core v0.0.0 (/checkout/src/libcore)
[00:48:33]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:48:33]     Checking compiler_builtins v0.1.2
[00:48:34]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:48:38] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:48:38]     |
[00:48:38]     |
[00:48:38] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:48:38]     |
[00:48:38]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:48:38]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:48:38] 
[00:48:38] 
[00:48:38] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:48:38]     |
[00:48:38]     |
[00:48:38] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:48:38]     |
[00:48:38]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:48:38] 
[00:48:39]     Finished release [optimized] target(s) in 37.61s
---
[00:49:14]     |              ^^^^^^^^^^^^^^^ cannot be resolved, ignoring
[00:49:14]     |
[00:49:14]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:14] 
[00:49:14] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:49:14]     |
[00:49:14]     |
[00:49:14] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:49:14]     |
[00:49:14]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:14] 
[00:49:19]     Finished release [optimized] target(s) in 18.45s
[00:49:19]     Finished release [optimized] target(s) in 18.45s
[00:49:19] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:49:20]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:49:20]     Checking term v0.0.0 (/checkout/src/libterm)
[00:49:20]     Checking getopts v0.2.17
[00:49:24]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:49:26] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:49:26]     |
[00:49:26]     |
[00:49:26] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:49:26]     |
[00:49:26]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:49:26]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:26] 
[00:49:26] 
[00:49:26] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:49:26]     |
[00:49:26]     |
[00:49:26] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:49:26]     |
[00:49:26]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:26] 
[00:49:26]     Finished release [optimized] target(s) in 6.53s
[00:49:26]     Finished release [optimized] target(s) in 6.53s
[00:49:26] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[00:49:27]  Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:49:30] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:49:30]     |
[00:49:30]     |
[00:49:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:49:30]     |
[00:49:30]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:49:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:30] 
[00:49:30] 
[00:49:30] warning: `[Iterator::nth]` cannot be resolved, ignoring it...
[00:49:30]     |
[00:49:30]     |
[00:49:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[00:49:30]     |
[00:49:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:49:30] 
[00:49:30]     Finished release [optimized] target(s) in 3.65s
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:17] 
[01:01:17] running 119 tests
[01:01:40] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[01:01:44] i......iii.i.....ii
[01:01:44] 
[01:01:44]  finished in 27.022
[01:01:44] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:19] 
[01:07:19] running 282 tests
[01:08:33] ........................i....................................................................FFF.... 100/282
[01:10:30] ..................................................................................
[01:10:30] failures:
[01:10:30] 
[01:10:30] ---- [rustdoc] rustdoc/intra-link-extern-crate.rs stdout ----
---
[01:10:30] 
[01:10:30] ------------------------------------------
[01:10:30] stderr:
[01:10:30] ------------------------------------------
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30] note: lint level defined here
[01:10:30]    --> /checkout/src/test/rustdoc/intra-link-extern-crate.rs:17:9
[01:10:30]     |
[01:10:30]     |
[01:10:30] 17  | #![deny(intra_doc_link_resolution_failure)]
[01:10:30]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] 
---
[01:10:30] ---- [rustdoc] rustdoc/intra-link-in-bodies.rs stdout ----
[01:10:30] 
[01:10:30] error: rustdoc failed!
[01:10:30] status: exit code: 1
[01:10:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-in-bodies/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-in-bodies" "/checkout/src/test/rustdoc/intra-link-in-bodies.rs"
[01:10:30] ------------------------------------------
[01:10:30] 
[01:10:30] ------------------------------------------
[01:10:30] stderr:
[01:10:30] stderr:
[01:10:30] ------------------------------------------
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30] note: lint level defined here
[01:10:30]    --> /checkout/src/test/rustdoc/intra-link-in-bodies.rs:13:9
[01:10:30]     |
[01:10:30]     |
[01:10:30] 13  | #![deny(intra_doc_link_resolution_failure)]
[01:10:30]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] 
---
[01:10:30] ---- [rustdoc] rustdoc/intra-link-private.rs stdout ----
[01:10:30] 
[01:10:30] error: rustdoc failed!
[01:10:30] status: exit code: 1
[01:10:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-private" "/checkout/src/test/rustdoc/intra-link-private.rs"
[01:10:30] ------------------------------------------
[01:10:30] 
[01:10:30] ------------------------------------------
[01:10:30] stderr:
[01:10:30] stderr:
[01:10:30] ------------------------------------------
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30] note: lint level defined here
[01:10:30]    --> /checkout/src/test/rustdoc/intra-link-private.rs:15:9
[01:10:30]     |
[01:10:30]     |
[01:10:30] 15  | #![deny(intra_doc_link_resolution_failure)]
[01:10:30]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] error: `[Iterator::nth]` cannot be resolved, ignoring it...
[01:10:30]     |
[01:10:30]     |
[01:10:30] 432 |     /// This is essentially the reversed version of [`nth`][Iterator::nth]. Although
[01:10:30]     |
[01:10:30]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:10:30] 
[01:10:30] 
---
[01:10:30] 
[01:10:30] test result: FAILED. 277 passed; 3 failed; 2 ignored; 0 measured; 0 filtered out
[01:10/bootstrap test
[01:10:30] Build completed unsuccessfully in 0:20:52
[01:10:30] make: *** [check] Error 1
[01:10:30] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:159ec8ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 23:11:41 UTC 2018
---
183740 ./obj/build/x86_64-unknown-linux-gnu/test/ui
160388 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7opx26amb-fdngv1-2sxk29axi4qzn
137420 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
128696 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128692 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
126304 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps

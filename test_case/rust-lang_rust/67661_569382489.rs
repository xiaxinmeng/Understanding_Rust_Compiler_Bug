plain
2019-12-28T01:54:55.1464353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-28T01:54:56.1638019Z ##[command]git config gc.auto 0
2019-12-28T01:54:56.1640201Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-28T01:54:56.1641961Z ##[command]git config --get-all http.proxy
2019-12-28T01:54:56.1644372Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67661/merge:refs/remotes/pull/67661/merge
---
2019-12-28T03:22:56.2427393Z  53 │ variables. [`UniversalRegions`] contains indices for all the free regions in
2019-12-28T03:22:56.2427644Z     │            ^ Server responded with 404 Not Found
2019-12-28T03:22:56.2428007Z     │
2019-12-28T03:22:56.2428040Z 
2019-12-28T03:22:56.2428714Z error: The server responded with 404 Not Found for "***/tree/master/src/librustc_mir/borrow_check/nll/region_infer/"
2019-12-28T03:22:56.2429012Z     ┌── borrow_check/region_inference.md:81:34 ───
2019-12-28T03:22:56.2429224Z     │
2019-12-28T03:22:56.2429447Z  81 │ for all regions is maintained in [the
2019-12-28T03:22:56.2429704Z     │                                  ^ Server responded with 404 Not Found
---
2019-12-28T03:33:41.8239235Z test [ui] ui/crashes/ice-2594.rs ... ok
2019-12-28T03:33:42.0153684Z test [ui] ui/crashes/ice-2727.rs ... ok
2019-12-28T03:33:42.3958384Z test [ui] ui/crashes/ice-2760.rs ... ok
2019-12-28T03:33:42.5626344Z normalized stderr:
2019-12-28T03:33:42.5627311Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-28T03:33:42.5627652Z 
2019-12-28T03:33:42.5627697Z error: internal compiler error: unexpected panic
2019-12-28T03:33:42.5627729Z 
2019-12-28T03:33:42.5627937Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5627937Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5627984Z 
2019-12-28T03:33:42.5628381Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-28T03:33:42.5628438Z 
2019-12-28T03:33:42.5628725Z note: Clippy version: clippy 0.0.212 (0fec590 2019-12-27)
2019-12-28T03:33:42.5628782Z 
2019-12-28T03:33:42.5628806Z 
2019-12-28T03:33:42.5628845Z expected stderr:
2019-12-28T03:33:42.5628888Z 
2019-12-28T03:33:42.5628888Z 
2019-12-28T03:33:42.5628912Z 
2019-12-28T03:33:42.5628950Z diff of stderr:
2019-12-28T03:33:42.5628976Z 
2019-12-28T03:33:42.5629280Z +thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-28T03:33:42.5629402Z +
2019-12-28T03:33:42.5629463Z +error: internal compiler error: unexpected panic
2019-12-28T03:33:42.5629502Z +
2019-12-28T03:33:42.5629544Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5629544Z +note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5629585Z +
2019-12-28T03:33:42.5630068Z +note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-28T03:33:42.5630119Z +
2019-12-28T03:33:42.5630400Z +note: Clippy version: clippy 0.0.212 (0fec590 2019-12-27)
2019-12-28T03:33:42.5630486Z +
2019-12-28T03:33:42.5630511Z 
2019-12-28T03:33:42.5630571Z The actual stderr differed from the expected stderr.
2019-12-28T03:33:42.5630957Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-473cade6e98745d4/out/test_build_base/crashes/ice-2774.stderr
---
2019-12-28T03:33:42.5634319Z 
2019-12-28T03:33:42.5634565Z ------------------------------------------
2019-12-28T03:33:42.5634612Z stderr:
2019-12-28T03:33:42.5635008Z ------------------------------------------
2019-12-28T03:33:42.5635647Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
2019-12-28T03:33:42.5635783Z 
2019-12-28T03:33:42.5635831Z error: internal compiler error: unexpected panic
2019-12-28T03:33:42.5635879Z 
2019-12-28T03:33:42.5635926Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5635926Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-28T03:33:42.5635957Z 
2019-12-28T03:33:42.5636294Z note: we would appreciate a bug report: ***-clippy/issues/new
2019-12-28T03:33:42.5636517Z 
2019-12-28T03:33:42.5636772Z note: Clippy version: clippy 0.0.212 (0fec590 2019-12-27)
2019-12-28T03:33:42.5636842Z 
2019-12-28T03:33:42.5637252Z ------------------------------------------
2019-12-28T03:33:42.5637289Z 
2019-12-28T03:33:42.5637514Z test [ui] ui/crashes/ice-2774.rs ... FAILED
---
2019-12-28T03:56:27.7479464Z Verifying status of rustfmt...
2019-12-28T03:56:27.7480214Z Verifying status of clippy-driver...
2019-12-28T03:56:27.7480523Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-28T03:56:27.7480556Z 
2019-12-28T03:56:27.7480775Z We detected that this PR updated 'clippy-driver', but its tests failed.
2019-12-28T03:56:27.7480806Z 
2019-12-28T03:56:27.7481042Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-12-28T03:56:27.7481086Z commit another update.
2019-12-28T03:56:27.7481109Z 
2019-12-28T03:56:27.7481317Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-12-28T03:56:27.7481544Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-12-28T03:56:27.7481593Z proper steps.
2019-12-28T03:56:27.7482973Z Build completed unsuccessfully in 0:00:01
2019-12-28T03:56:27.7537413Z == clock drift check ==
2019-12-28T03:56:27.7603815Z   local time: Sat Dec 28 03:56:27 UTC 2019
2019-12-28T03:56:28.0529191Z   network time: Sat, 28 Dec 2019 03:56:28 GMT
2019-12-28T03:56:28.0529191Z   network time: Sat, 28 Dec 2019 03:56:28 GMT
2019-12-28T03:56:28.0530641Z == end clock drift check ==
2019-12-28T03:56:29.3636516Z 
2019-12-28T03:56:29.3735359Z ##[error]Bash exited with code '1'.
2019-12-28T03:56:29.3800843Z ##[section]Starting: Checkout
2019-12-28T03:56:29.3802437Z ==============================================================================
2019-12-28T03:56:29.3802492Z Task         : Get sources
2019-12-28T03:56:29.3802532Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
[00:37:36]    Compiling parking_lot_core v0.3.0
[00:37:36]    Compiling tempfile v3.0.3
[00:37:37]    Compiling parking_lot v0.6.4
[00:37:38]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:37:44] error[E0004]: non-exhaustive patterns: `UnnormalizedProjection(_)` not covered
[00:37:44]     --> librustdoc/clean/mod.rs:2565:15
[00:37:44] 2565 |         match self.sty {
[00:37:44] 2565 |         match self.sty {
[00:37:44]      |               ^^^^^^^^ pattern `UnnormalizedProjection(_)` not covered
[00:37:44] error: aborting due to previous error
[00:37:44] 
[00:37:44] For more information about this error, try `rustc --explain E0004`.
[00:37:44] error: Could not compile `rustdoc`.
---
[00:37:44] 
[00:37:44] 
[00:37:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:44] Build completed unsuccessfully in 0:33:27
[00:37:44] Makefile:28: recipe for target 'all' failed
[00:37:44] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08851380
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

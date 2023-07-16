plain
[00:04:45]    Compiling libc v0.2.40
[00:04:45]    Compiling lazy_static v1.0.0
[00:04:45]    Compiling memoffset v0.2.1
[00:04:45]    Compiling scopeguard v0.3.3
[00:04:45]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:04:45]    Compiling smallvec v0.6.0
[00:04:45]    Compiling either v1.5.0
[00:04:46]    Compiling bitflags v1.0.1
[00:04:46]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:05:15]    Compiling rls-data v0.15.0
[00:05:17]    Compiling crossbeam-deque v0.2.0
[00:05:18]    Compiling parking_lot v0.5.5
[00:05:18]    Compiling flate2 v1.0.1
[00:05:25]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:05:32]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:05:34]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:34]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:38]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:22:04]    Compiling memoffset v0.2.1
[00:22:04]    Compiling scopeguard v0.3.3
[00:22:04]    Compiling lazy_static v1.0.0
[00:22:04]    Compiling libc v0.2.40
[00:22:04]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:22:05]    Compiling stable_deref_trait v1.0.0
[00:22:05]    Compiling either v1.5.0
[00:22:05]    Compiling bitflags v1.0.1
[00:22:05]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:22:26]    Compiling crossbeam-deque v0.2.0
[00:22:26]    Compiling parking_lot v0.5.5
[00:22:29]    Compiling rls-data v0.15.0
[00:22:31]    Compiling flate2 v1.0.1
[00:22:32]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:22:39]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:22:42]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:22:42]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:22:45]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:40:39]    Compiling crossbeam-epoch v0.3.1
[00:40:39]    Compiling crossbeam-deque v0.2.0
[00:40:40]    Compiling rand v0.3.22
[00:40:40]    Compiling parking_lot_core v0.2.14
[00:40:40]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:40:40]    Compiling parking_lot v0.5.5
[00:40:41]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:40:44]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:40:47]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:40:49]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:40:51]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[00:40:51]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[00:41:13]  Documenting proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:41:14] warning: [cfg] cannot be resolved, ignoring it...
[00:41:14] 
[00:41:14] warning: [rayon::prelude] cannot be resolved, ignoring it...
[00:41:14] warning: [Experimental] cannot be resolved, ignoring it...
[00:41:14] 
[00:41:14] 
[00:41:14] warning: [plumbing] cannot be resolved, ignoring it...
[00:41:14] warning: [Garbage] cannot be resolved, ignoring it...
[00:41:14] 
[00:41:15]     Finished release [optimized] target(s) in 37.63 secs
[00:41:15] Documenting stage2 compiler (x86_64-unknown-linux-gnu)
---
[01:19:11] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:11]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:19:13] error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
[01:19:13]    --> librustc_driver/test.rs:120:5
[01:19:13]     |
[01:19:13] 120 |     driver::spawn_thread_pool(&mut sess, || {
[01:19:13]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^            -- takes 0 arguments
[01:19:13]     |     |
[01:19:13]     |     expected closure that takes 1 argument
[01:19:13] error: aborting due to previous error
[01:19:13] 
[01:19:13] For more information about this error, try `rustc --explain E0593`.
[01:19:13] error: Could not compile `rustc_driver`.
[01:19:13] error: Could not compile `rustc_driver`.
[01:19:13] 
[01:19:13] To learn more, run the command again with --verbose.
[01:19:13] 
[01:19:13] 
[01:19:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:19:13] 
[01:19:13] 
[01:19:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:13] Build completed unsuccessfully in 0:37:51
[01:19:13] Build completed unsuccessfully in 0:37:51
[01:19:13] Makefile:58: recipe for target 'check' failed
[01:19:13] make: *** [check] Error 1

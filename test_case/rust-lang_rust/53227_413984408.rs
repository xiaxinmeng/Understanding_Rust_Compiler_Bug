plain
[00:47:55] ....................................................................................................
[00:47:59] ....................................................................................................
[00:48:01] i...................................................................................................
[00:48:04] ....................................................................................................
[00:48:07] .................................................iiiiiiiii..........................................
[00:48:13] ....................................................................................................
[00:48:16] ....................................................................................................
[00:48:19] ..............................i.....................................................................
[00:48:22] .................................i...............................................i.i..ii............
---
[01:02:23] 
[01:02:24] 
[01:02:24] running 402 tests
[01:02:37] ....................................................................................................
[01:02:48] ...................................................................i................F...............
[01:03:07] ....................................................................................................
[01:03:07] ..
[01:03:07] failures:
[01:03:07] 
[01:03:07] 
[01:03:07] ---- pin.rs - pin (line 37) stdout ----
[01:03:07] error: variable does not need to be mutable
[01:03:07]   --> pin.rs:80:5
[01:03:07]    |
[01:03:07] 46 | let mut still_unmoved = unmoved;
[01:03:07]    |     |
[01:03:07]    |     help: remove this `mut`
[01:03:07]    |
[01:03:07] note: lint level defined here
[01:03:07] note: lint level defined here
[01:03:07]   --> pin.rs:36:9
[01:03:07]    |
[01:03:07] 2  | #![deny(warnings)]
[01:03:07]    |         ^^^^^^^^
[01:03:07]    = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[01:03:07] 
[01:03:07] thread 'pin.rs - pin (line 37)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:07] 
[01:03:07] 
[01:03:07] failures:
[01:03:07]     pin.rs - pin (line 37)
[01:03:07]     pin.rs - pin (line 37)
[01:03:07] 
[01:03:07] test result: FAILED. 400 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:03:07] 
[01:03:07] error: test failed, to rerun pass '--doc'
[01:03:07] 
[01:03:07] 
[01:03:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:03:07] 
[01:03:07] 
[01:03:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:07] Build completed unsuccessfully in 0:19:16
[01:03:07] Build completed unsuccessfully in 0:19:16
[01:03:07] Makefile:58: recipe for target 'check' failed
[01:03:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ac71fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

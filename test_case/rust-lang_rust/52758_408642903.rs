plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:02] 
[01:10:02] running 189 tests
[01:10:30] .....................................................F..............................................
[01:11:24] ........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:12:19] failures:
[01:12:19] 
[01:12:19] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:12:19] 
[01:12:19] 
[01:12:19] error: make failed
[01:12:19] status: exit code: 2
[01:12:19] command: "make"
[01:12:19] stdout:
[01:12:19] ------------------------------------------
[01:12:19] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:12:19] /bin/echo || exit 0 # This test requires /bin/echo to exist
[01:12:19] 
[01:12:19] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[01:12:19]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[01:12:19] Makefile:4: recipe for target 'all' failed
[01:12:19] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:12:19] ------------------------------------------
[01:12:19] stderr:
[01:12:19] stderr:
[01:12:19] --android-cross-path" "" "--color" "always"
[01:12:19] 
[01:12:19] 
[01:12:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:19] Build completed unsuccessfully in 0:31:31
[01:12:19] Build completed unsuccessfully in 0:31:31
[01:12:19] make: *** [check] Error 1
[01:12:19] Makefile:58: recipe for target 'check' failed

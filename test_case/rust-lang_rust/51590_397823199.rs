plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:29:11] 
[01:29:11] running 186 tests
[01:29:56] ....................i..................................F............................................
[01:30:47] .....................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
g_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  some_crate.rs --crate-name some_crate --crate-type bin -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/some_crate \
[01:31:52]  -Z codegen-backend=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options
[01:31:52] Makefile:4: recipe for target 'all' failed
[01:31:52] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:31:52] ------------------------------------------
[01:31:52] stderr:
[01:31:52] ------------------------------------------
[01:31:52] ------------------------------------------
[01:31:52] warning: ignoring --out-dir flag due to -o flag
[01:31:52] 
[01:31:52] warning: ignoring --out-dir flag due to -o flag
[01:31:52] error[E0463]: can't find crate for `std`
[01:31:52] 
[01:31:52] error: aborting due to previous error
[01:31:52] 
[01:31:52] 
[01:31:52] For more information about this error, try `rustc --explain E0463`.
[01:31:52] make[1]: *** [all] Error 101

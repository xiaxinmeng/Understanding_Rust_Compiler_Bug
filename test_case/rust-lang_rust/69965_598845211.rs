
---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
/bin/echo || exit 0 # This test requires /bin/echo to exist

LD_LIBRARY_PATH="/nobackup/rust/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/nobackup/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/nobackup/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/nobackup/rust/build/x86_64-unknown-linux-gnu/stage0/lib" '/nobackup/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /nobackup/rust/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /nobackup/rust/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
	-o /nobackup/rust/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
Makefile:4: recipe for target 'all' failed

------------------------------------------
stderr:
------------------------------------------
error[E0433]: failed to resolve: maybe a missing crate `rustc_codegen_ssa`?
  --> the_backend.rs:23:5
   |
23 | use rustc_codegen_ssa::traits::CodegenBackend;
   |     ^^^^^^^^^^^^^^^^^ maybe a missing crate `rustc_codegen_ssa`?

error[E0405]: cannot find trait `CodegenBackend` in this scope
  --> the_backend.rs:44:6
   |
44 | impl CodegenBackend for TheBackend {
   |      ^^^^^^^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `CodegenBackend` in this scope
   --> the_backend.rs:111:45
    |
111 | pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    |                               -             ^^^^^^^^^^^^^^ not found in this scope
    |                               |
    |                               help: you might be missing a type parameter: `<CodegenBackend>`

warning: ignoring --out-dir flag due to -o flag

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0405, E0433.
For more information about an error, try `rustc --explain E0405`.
make: *** [all] Error 1

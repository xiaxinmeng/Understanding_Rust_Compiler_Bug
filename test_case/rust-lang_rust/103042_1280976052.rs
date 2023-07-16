plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
/bin/echo || exit 0 # This test requires /bin/echo to exist

LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
 -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

error[E0046]: not all trait items implemented, missing: `locale_resource`
error[E0046]: not all trait items implemented, missing: `locale_resource`
  --> the_backend.rs:29:1
   |
29 | impl CodegenBackend for TheBackend {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `locale_resource` in implementation
   |
   = help: implement the missing item: `fn locale_resource(&self) -> &'static str { todo!() }`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0046`.
make: *** [Makefile:11: all] Error 1

plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path  --crate-type lib dummy_core.rs --target target.json
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path" -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path my_crate.rs --target target.json
--- stderr -------------------------------
--- stderr -------------------------------
warning: target json file contains unused fields: has-elf-tls

warning: target json file contains unused fields: has-elf-tls

error[E0461]: couldn't find crate `dummy_core` with expected target triple target-7650165571033555380
 --> my_crate.rs:3:1
3 | extern crate dummy_core;
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: the following crate versions were found:
  = note: the following crate versions were found:
          crate `dummy_core`, target triple target-7650165571033555380: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path/libdummy_core.rlib
error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors


make: *** [Makefile:9: all] Error 1



failures:

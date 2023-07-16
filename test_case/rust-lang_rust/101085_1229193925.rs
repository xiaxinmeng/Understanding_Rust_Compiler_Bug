plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# We explicitly switch to a directory that is *not* a prefix of the directory our
# source code is contained in.
cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf && LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf  /checkout/src/test/run-make/remap-path-prefix-dwarf/src/quux.rs -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf/abs_input_outside_working_dir.rlib" -Cdebuginfo=2 --remap-path-prefix /checkout/src/test/run-make/remap-path-prefix-dwarf=REMAPPED --remap-path-prefix /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf=REMAPPED
"/usr/lib/llvm-13/bin"/llvm-dwarfdump /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf/abs_input_outside_working_dir.rlib | "/checkout/src/etc/cat-and-grep.sh" "REMAPPED/src/quux.rs"
[[[ begin stdout ]]]

[[[ end stdout ]]]
Error: cannot match: REMAPPED/src/quux.rs
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
warning: 1 warning emitted

error: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/remap-path-prefix-dwarf/remap-path-prefix-dwarf/abs_input_outside_working_dir.rlib(lib.rmeta): The file was not recognized as a valid object file
make: *** [Makefile:41: abs_input_outside_working_dir] Error 1



failures:

plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/print-rustc-path/print-rustc-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/print-rustc-path/print-rustc-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/print-rustc-path/print-rustc-path  --print cfg | "/checkout/src/etc/cat-and-grep.sh" bin/rustc
[[[ begin stdout ]]]
panic="unwind"
target_abi=""
target_arch="x86_64"
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="fxsr"
target_feature="sse"
target_feature="sse2"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_has_atomic_equal_alignment="16"
target_has_atomic_equal_alignment="32"
target_has_atomic_equal_alignment="64"
target_has_atomic_equal_alignment="8"
target_has_atomic_equal_alignment="ptr"
target_has_atomic_load_store="16"
target_has_atomic_load_store="32"
target_has_atomic_load_store="64"
target_has_atomic_load_store="8"
target_has_atomic_load_store="ptr"
target_os="linux"
target_pointer_width="64"
target_thread_local
target_vendor="unknown"


[[[ end stdout ]]]
Error: cannot match: bin/rustc
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:8: all] Error 1



failures:

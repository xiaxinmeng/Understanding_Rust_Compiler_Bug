plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path  --print rustc-path | "/checkout/src/etc/cat-and-grep.sh" LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path 
[[[ begin stdout ]]]


[[[ end stdout ]]]
Error: cannot match: LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib
Error: cannot match: --out-dir
Error: cannot match: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path
Error: cannot match: -L
Error: cannot match: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-rustc-path/print-rustc-path
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:4: all] Error 1



failures:

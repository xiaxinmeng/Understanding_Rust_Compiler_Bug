plain
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i...................................i...ii...............F
failures:

---- [run-make] src/test/run-make-fulldeps/core-no-128-bit stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit  --edition=2021 --crate-type=rlib --crate-name=core ../../../../library/core/src/lib.rs --cfg no_128_bit
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit  --edition=2021 --crate-type=staticlib --crate-name=demo --sysroot=. -C panic=abort lib.rs
# Expect that objdump succeeds and grep fails. The grep pattern is for names like __multi3.
# There is no pipefail on dash so echo a string that will fail the test if objump fails.
(objdump -t /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-128-bit/core-no-128-bit/libdemo.a || echo __objdumpfailedti) | (! grep -w '__[a-z]\+ti[0-9]\?')
0000000000000000         *UND* 0000000000000000 __muloti4
0000000000000000         *UND* 0000000000000000 __muloti4
--- stderr -------------------------------
make: *** [Makefile:8: all] Error 1
------------------------------------------


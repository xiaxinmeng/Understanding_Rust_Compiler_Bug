plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation  test.rs 2>&1 | grep "struct literal body without path"
error: struct literal body without path
error: struct literal body without path
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/translation/translation  test.rs -Ztranslate-additional-ftl=/checkout/src/test/run-make/translation/working.ftl 2>&1 | grep "this is a test message"
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:19: custom] Error 1



failures:

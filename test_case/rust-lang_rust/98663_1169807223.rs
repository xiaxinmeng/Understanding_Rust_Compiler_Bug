plain
test [run-make] src/test/run-make/emit-shared-files ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] src/test/run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] src/test/run-make/issue-88756-opt-help stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-88756-opt-help/issue-88756-opt-help:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc' -W help 2>&1 | diff - output-default.stdout
35d34
<     -W                           dwarf-version=val -- version of DWARF debug information to emit (default: 2 or 4, depending on platform)
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:4: all] Error 1



failures:

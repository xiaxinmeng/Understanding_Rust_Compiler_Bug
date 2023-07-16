plain
test [run-make] run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/foreign-double-unwind stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
arm-none-eabi-g++ -ffunction-sections -fdata-sections -mthumb -march=armv6s-m -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/foreign-double-unwind/foreign-double-unwind/libfoo.o foo.cpp
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
foo.cpp:1:10: fatal error: cstdio: No such file or directory
    1 | #include <cstdio>
compilation terminated.
compilation terminated.
make: *** [Makefile:10: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/foreign-double-unwind/foreign-double-unwind/libfoo.o] Error 1
------------------------------------------




plain
test [run-make] src/test/run-make/rustdoc-scrape-examples-ordering ... ok

failures:

---- [run-make] src/test/run-make/issue-36710 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
arm-linux-gnueabihf-g++ -ffunction-sections -fdata-sections -fPIC -march=armv6 -marm -mfpu=vfp -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o foo.cpp
--- stderr -------------------------------
--- stderr -------------------------------
/bin/dash: 1: arm-linux-gnueabihf-g++: not found
make: *** [Makefile:17: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o] Error 127



failures:

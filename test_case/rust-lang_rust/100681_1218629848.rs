plain
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iii...i.i.......iiii....F.......ii...iiiiiii.iiii...............
failures:

---- [run-make] src/test/run-make/print-rustc-path stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc -Zunstable-options --print rustc-path |  bin/rustc
--- stderr -------------------------------
--- stderr -------------------------------
/bin/sh: 1: bin/rustc: not found
make: *** [Makefile:4: all] Error 127



failures:

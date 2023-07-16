plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 58 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.i..i.F..iiii............ii...iiiiiii.iiii...............


---- [run-make] src/test/run-make/issue-88756-opt-help stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-88756-opt-help/issue-88756-opt-help:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -W help 2>&1 | diff - output-default.stdout
36d35
<     -W                           emit-thin-lto=val -- emit the bc module with thin LTO info (default: yes)
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:4: all] Error 1



failures:

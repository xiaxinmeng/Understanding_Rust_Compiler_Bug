plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 57 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.i.i..F.iiii............ii...iiiiiii.iiii...............

---- [run-make] src/test/run-make/issue-88756-default-output stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-88756-default-output/issue-88756-default-output:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' 2>&1 | sed -E 's@/nightly/|/beta/|/stable/|/1\.[0-9]+\.[0-9]+/@/$CHANNEL/@g' | diff - output-default.stdout
<         --diagnostic-width WIDTH
<                         Provide width of the output for truncated error
<                         messages
------------------------------------------
------------------------------------------
--- stderr -------------------------------
make: *** [Makefile:4: all] Error 1



failures:

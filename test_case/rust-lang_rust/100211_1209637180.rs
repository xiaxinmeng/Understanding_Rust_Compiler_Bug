plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 63 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iii...i.i......iiiiF...........ii...iiiiiii.iiii...............


---- [run-make] src/test/run-make/issue-85401-static-mir stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir  --crate-type rlib --crate-name foo -Crelocation-model=pic --edition=2018 foo.rs -Zalways-encode-mir=yes --emit metadata -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/libfoo.rmeta
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir  --crate-type rlib --crate-name bar -Crelocation-model=pic --edition=2018 bar.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/libbar.rlib --extern=foo=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/libfoo.rmeta
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir  --crate-type bin --crate-name baz -Crelocation-model=pic --edition=2018 baz.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/baz -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir --extern=bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/libbar.rlib > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/build-output 2>&1; [ $? -eq 1 ]
cat  /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/build-output
error: couldn't read baz.rs: No such file or directory (os error 2)
error: aborting due to previous error


"/checkout/src/etc/cat-and-grep.sh" 'crate `foo` required to be available in rlib format, but was not found in this form' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-85401-static-mir/issue-85401-static-mir/build-output
[[[ begin stdout ]]]
error: couldn't read baz.rs: No such file or directory (os error 2)
error: aborting due to previous error



[[[ end stdout ]]]
Error: cannot match: crate `foo` required to be available in rlib format, but was not found in this form
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
warning: 1 warning emitted

warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [Makefile:14: all] Error 1



failures:

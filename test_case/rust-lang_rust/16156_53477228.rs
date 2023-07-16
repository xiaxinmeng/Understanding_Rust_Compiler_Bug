
maketest: rustdoc-hidden-line
----- /home/cmr/src/rust2/src/test/run-make/rustdoc-hidden-line/ --------------------
------ stdout ---------------------------------------------
make[1]: Entering directory '/home/cmr/src/rust2/src/test/run-make/rustdoc-hidden-line'
LD_LIBRARY_PATH="/home/cmr/src/rust2/build/x86_64-unknown-linux-gnu/test/run-make/rustdoc-hidden-line:/home/cmr/src/rust2/build/x86_64-unknown-linux-gnu/stage2/lib:" /home/cmr/src/rust2/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --test foo.rs

running 1 test
test foo_0 ... FAILED

failures:

---- foo_0 stdout ----
    task 'foo_0' failed at 'couldn't run the test: no such file or directory', /home/cmr/src/rust2/src/librustdoc/test.rs:196



failures:
    foo_0

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

Makefile:7: recipe for target 'all' failed
make[1]: Leaving directory '/home/cmr/src/rust2/src/test/run-make/rustdoc-hidden-line'

------ stderr ---------------------------------------------
task '<unnamed>' failed at 'Some tests failed', /home/cmr/src/rust2/src/libtest/lib.rs:242
make[1]: *** [all] Error 101

------        ---------------------------------------------

/home/cmr/src/rust2/mk/tests.mk:1010: recipe for target 'x86_64-unknown-linux-gnu/test/run-make/rustdoc-hidden-line-2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu.ok' failed
make: *** [x86_64-unknown-linux-gnu/test/run-make/rustdoc-hidden-line-2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu.ok] Error 2

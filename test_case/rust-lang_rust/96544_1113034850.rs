plain
....................................ii................
failures:
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [run-make] src/test/run-make-fulldeps/issue-26092 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-26092/issue-26092:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-26092/issue-26092 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-26092/issue-26092  -o "" blank.rs 2>&1 | "/checkout/src/etc/cat-and-grep.sh" -i 'No such file or directory'
[[[ begin stdout ]]]
warning: ignoring --out-dir flag due to -o flag

error: couldn't create a temp dir: Read-only file system (os error 30) at path "/checkout/src/test/run-make-fulldeps/issue-26092/rmetaYAWqHi"
error: aborting due to previous error; 1 warning emitted



[[[ end stdout ]]]
Error: cannot match: No such file or directory
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:4: all] Error 1



failures:

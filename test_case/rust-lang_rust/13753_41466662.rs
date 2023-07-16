
remake[1]: Entering directory `/Users/fklock/Dev/Mozilla/rust-issue13732/src/test/run-make/bootstrap-from-c-with-green'
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
/Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/bin/rustc --out-dir /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green -L /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green lib.rs
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
ln -nsf /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/libboot-*.dylib /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/libboot.dylib
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
ccache clang -Qunused-arguments -Wall -Werror -g -fPIC -m64 -arch x86_64 -L /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green main.c -o /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/main -lboot
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
DYLD_LIBRARY_PATH=:/Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/main
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
hello
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
rm /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/libboot.dylib
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
##>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
gDYLD_LIBRARY_PATH=:/Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib /Users/fklock/Dev/Mozilla/rust-issue13732/objdir-check1-dbgopt/x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green/main && exit 1 || exit 0
##<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
hello

#0  all at /Users/fklock/Dev/Mozilla/rust-issue13732/src/test/run-make/bootstrap-from-c-with-green/Makefile:3
remake[1]: Leaving directory `/Users/fklock/Dev/Mozilla/rust-issue13732/src/test/run-make/bootstrap-from-c-with-green'
Command-line invocation:
    "/Users/fklock/opt/remake/bin/remake -C /Users/fklock/Dev/Mozilla/rust-issue13732/src/test/run-make/bootstrap-from-c-with-green/"

------ stderr ---------------------------------------------
Makefile:3: *** [all] Error 1

------        ---------------------------------------------

/Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:924: *** [x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green-1-T-x86_64-apple-darwin-H-x86_64-apple-darwin.ok] Error 2

#0  x86_64-apple-darwin/test/run-make/bootstrap-from-c-with-green-1-T-x86_64-apple-darwin-H-x86_64-apple-darwin.ok at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:924
#1  tmp/check-stage1-T-x86_64-apple-darwin-H-x86_64-apple-darwin-rmake.ok at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:925
#2  check-stage1-T-x86_64-apple-darwin-H-x86_64-apple-darwin-rmake-exec at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:925
#3  check-stage1-T-x86_64-apple-darwin-H-x86_64-apple-darwin-exec at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:332
#4  check-stage1-T-x86_64-apple-darwin-H-x86_64-apple-darwin at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:807
#5  check-stage1-H-x86_64-apple-darwin at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:847
#6  check-stage1 at /Users/fklock/Dev/Mozilla/rust-issue13732/mk/tests.mk:828
Command-line invocation:
    "/Users/fklock/opt/remake/bin/remake --trace -j1 check-stage1"



failures:
---- [run-make] run-make\issue-26092 stdout ----
	
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
PATH="<snip>" 'C:\projects\rust\build\x86_64-pc-windows-gnu\stage2\bin\rustc.exe' --out-dir /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/issue-26092.stage2-x86_64-pc-windows-gnu -L /c/projects/rust/build/x86_64-pc-windows-gnu/test/run-make/issue-26092.stage2-x86_64-pc-windows-gnu  -o "" blank.rs 2>&1 | \
		grep -i 'No such file or directory'
------------------------------------------
stderr:
------------------------------------------
make: *** [Makefile:4: all] Error 1
------------------------------------------
thread '[run-make] run-make\issue-26092' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:2479:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failures:
    [run-make] run-make\issue-26092

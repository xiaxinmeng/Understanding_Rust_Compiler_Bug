`

failures:

---- [run-make] run-make-fulldeps/long-linker-command-lines stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
DYLD_LIBRARY_PATH="/Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines -L /Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines  foo.rs -g -O
RUSTC="/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" DYLD_LIBRARY_PATH="/Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/Users/runner/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines/foo
attempt: 100
attempt: 200
attempt: 300

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'status: exit code: 1
stdout:

stderr:
error: couldn't read /Users/runner/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines/bar.rs: stream did not contain valid UTF-8

error: aborting due to previous error

', foo.rs:76:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [all] Error 101


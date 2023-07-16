
failures:

---- [run-make] run-make-fulldeps/long-linker-command-lines stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
DYLD_LIBRARY_PATH="/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines -L /Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines  foo.rs -g -O
RUSTC="/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" DYLD_LIBRARY_PATH="/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines:/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines/foo
attempt: 100
attempt: 200
attempt: 300

------------------------------------------
stderr:
------------------------------------------
warning: variable does not need to be mutable
  --> foo.rs:93:13
   |
93 |         for mut arg in linker_args.split('S') {
   |             ----^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: 1 warning emitted

thread 'main' panicked at 'status: exit code: 1
stdout:

stderr:
error: could not exec the linker `/Users/runner/runners/2.168.2/work/1/s/build/x86_64-apple-darwin/test/run-make-fulldeps/long-linker-command-lines/long-linker-command-lines/foo`
  |

', foo.rs:76:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
make: *** [all] Error 101

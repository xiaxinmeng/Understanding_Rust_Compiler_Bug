plain
[00:45:45] warning:  ^
[00:45:58] [RUSTC-TIMING] core test:false 24.473
[00:46:00] LLVM ERROR: Unexpected anonymous function when writing summary
[00:46:00] [RUSTC-TIMING] compiler_builtins test:false 2.402
[00:46:00] The following warnings were emitted during compilation:
[00:46:00] warning: ../../libcompiler_builtins/compiler-rt/lib/builtins/divdc3.c:23:1: warning: conflicting types for built-in function '__divdc3'
[00:46:00] warning:  __divdc3(double __a, double __b, double __c, double __d)
[00:46:00] warning:  ^
[00:46:00] warning: ../../libcompiler_builtins/compiler-rt/lib/builtins/divsc3.c:23:1: warning: conflicting types for built-in function '__divsc3'
---
[00:46:00] 
[00:46:00] error: Could not compile `compiler_builtins`.
[00:46:00] 
[00:46:00] To learn more, run the command again with --verbose.
[00:46:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "thumbv6m-none-eabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "c mem" "-p" "alloc" "-p" "compiler_builtins" "--manifest-path" "/checkout/src/rustc/compiler_builtins_shim/Cargo.toml" "--message-format" "json"
[00:46:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:46:00] Build completed unsuccessfully in 0:42:14
travis_time:end:01434e40:start=1543727275985231695,finish=1543730037202095022,duration=2761216863327
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:15500b1c:start=1543730038323319619,finish=1543730038331904784,duration=8585165
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b53f7c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06455fdc
travis_time:start:06455fdc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00e5f508
$ dmesg | grep -i kill

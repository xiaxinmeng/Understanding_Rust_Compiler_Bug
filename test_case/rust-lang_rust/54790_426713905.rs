plain
[00:07:34] [RUSTC-TIMING] syntax test:false 61.913
[00:07:34]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:42] [RUSTC-TIMING] proc_macro test:false 8.134
[00:07:42]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:42] error[E0523]: found two different crates with name `getopts` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[00:07:42]     |
[00:07:42] 119 | extern crate test;
[00:07:42]     | ^^^^^^^^^^^^^^^^^^
[00:07:42] 
---
[00:07:58] [RUSTC-TIMING] syntax_ext test:false 15.992
[00:07:58] error: build failed
[00:07:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:58] expected success, got: exit code: 101
[00:07:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:07:58] travis_fold:end:stage0-rustc

[00:07:58] travis_time:end:stage0-rustc:start=1538585624347499190,finish=1538585762699240976,duration=138351741786

---
travis_time:end:0b3a71a6:start=1538585763397753282,finish=1538585763407501091,duration=9747809
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d41eef0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0659d2d0
$ cat ./obj/build/x86_64-unknown-lin

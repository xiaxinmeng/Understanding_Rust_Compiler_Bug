plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0e1ddec8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:45:07]    Compiling strsim v0.7.0
[01:45:08]    Compiling hex v0.3.2
[01:45:08]    Compiling ansi_term v0.11.0
[01:45:08] error[E0282]: type annotations needed
[01:45:08]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/ansi_term-0.11.0/src/ansi.rs:30:39
[01:45:08]    |
[01:45:08] 29 |             let mut write_char = |c| {

[01:45:08]    |                 -------------- consider giving `write_char` a type
[01:45:08] 30 |                 if written_anything { write!(f, ";")?; }
[01:45:08]    |                                       ^^^^^^^^^^^^^^^
[01:45:08]    |                                       |
[01:45:08]    |                                       cannot infer type
[01:45:08]    |                                       cannot infer type
[01:45:08]    |                                       in this expansion of `desugaring of `?``
[01:45:08] 
[01:45:08] error: aborting due to previous error
[01:45:08] 
[01:45:08] For more information about this error, try `rustc --explain E0282`.
[01:45:08] For more information about this error, try `rustc --explain E0282`.
[01:45:08] error: Could not compile `ansi_term`.
[01:45:10] error: build failed
[01:45:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:45:10] expected success, got: exit code: 101
[01:45:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
---
travis_time:end:00e5df01:start=1558129959406603950,finish=1558129959431110558,duration=24506608
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11588250
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18753af9
travis_time:start:18753af9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e41aa2a
$ dmesg | grep -i kill

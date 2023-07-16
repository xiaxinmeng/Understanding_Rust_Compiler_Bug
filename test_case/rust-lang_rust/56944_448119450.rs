plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:08a1d7d3
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:25:10]   curl error: Could not resolve host: github.com
[01:25:10] ; class=Net (12)
[01:25:10] 
[01:25:10] 
[01:25:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[01:25:10] 
[01:25:10] 
[01:25:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:25:10] Build completed unsuccessfully in 1:20:37
---
travis_time:end:01d69427:start=1545115281612526283,finish=1545115281624430147,duration=11903864
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fc8be9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12446463
travis_time:start:12446463
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2182a530
$ dmesg | grep -i kill

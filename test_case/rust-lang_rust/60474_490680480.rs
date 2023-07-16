plain
[00:22:33]    Compiling serde_derive v1.0.81
[00:22:34]    Compiling serde_json v1.0.33
[00:22:34]    Compiling toml v0.4.10
[00:22:42]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:22:42] error: /usr/lib/x86_64-linux-gnu/libc.so: invalid ELF header
[00:22:42]    --> src/bootstrap/lib.rs:114:1
[00:22:42] 114 | extern crate serde_derive;
[00:22:42]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:22:42] 
[00:22:42] error: aborting due to previous error
---
[00:22:42] make: *** [prepare] Error 1
[00:22:42] Makefile:69: recipe for target 'prepare' failed
[00:22:43] Command failed. Attempt 2/5:
[00:22:44]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:22:44] error: /usr/lib/x86_64-linux-gnu/libc.so: invalid ELF header
[00:22:44]    --> src/bootstrap/lib.rs:114:1
[00:22:44] 114 | extern crate serde_derive;
[00:22:44]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:22:44] 
[00:22:44] error: aborting due to previous error
---
[00:22:44] make: *** [prepare] Error 1
[00:22:44] Makefile:69: recipe for target 'prepare' failed
[00:22:46] Command failed. Attempt 3/5:
[00:22:46]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:22:46] error: /usr/lib/x86_64-linux-gnu/libc.so: invalid ELF header
[00:22:46]    --> src/bootstrap/lib.rs:114:1
[00:22:46] 114 | extern crate serde_derive;
[00:22:46]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:22:46] 
[00:22:46] error: aborting due to previous error
---
[00:22:46] make: *** [prepare] Error 1
[00:22:46] Makefile:69: recipe for target 'prepare' failed
[00:22:49] Command failed. Attempt 4/5:
[00:22:50]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:22:50] error: /usr/lib/x86_64-linux-gnu/libc.so: invalid ELF header
[00:22:50]    --> src/bootstrap/lib.rs:114:1
[00:22:50] 114 | extern crate serde_derive;
[00:22:50]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:22:50] 
[00:22:50] error: aborting due to previous error
---
[00:22:50] make: *** [prepare] Error 1
[00:22:50] Makefile:69: recipe for target 'prepare' failed
[00:22:54] Command failed. Attempt 5/5:
[00:22:54]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:22:54] error: /usr/lib/x86_64-linux-gnu/libc.so: invalid ELF header
[00:22:54]    --> src/bootstrap/lib.rs:114:1
[00:22:54] 114 | extern crate serde_derive;
[00:22:54]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:22:54] 
[00:22:54] error: aborting due to previous error
---
travis_time:end:032126e8:start=1557355809455577958,finish=1557355809482933735,duration=27355777
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c306b24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a3fe2d0
travis_time:start:0a3fe2d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28fa1450
$ dmesg | grep -i kill

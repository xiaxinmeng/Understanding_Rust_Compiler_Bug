plain
[00:00:00] rm 'src/tools/lldb'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lldb.tar.gz &&         curl -sSL -o download-src-tools-lldb.tar.gz https://github.com/rust-lang-nursery/lldb/archive/fdea743be550ed8d7b61b2c908944cdd1290a6ad.tar.gz
[00:00:00] rm 'src/tools/clang'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-clang.tar.gz &&         curl -sSL -o download-src-tools-clang.tar.gz https://github.com/rust-lang-nursery/clang/archive/d0fc1788123de9844c8088b977cd142021cea1f2.tar.gz
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/doc/rustc-guide'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
---
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/doc/rustc-guide'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/clippy'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:06] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:06] Submodule path 'src/doc/nomicon': checked out 'f8a4e96feb2e5a6ed1ef170ad40e3509a7755cb4'
[00:00:06] Submodule path 'src/doc/reference': checked out '60077efda319c95a89fe39609803c5433567adbf'
[00:00:07] Submodule path 'src/doc/rustc-guide': checked out '3a804956e3c28d7e44e38804207a00013594e1d3'
[00:00:07] Submodule path 'src/libcompiler_builtins': checked out 'fe74674f6e4be76d47b66f67d529ebf4186f4eb1'
[00:00:07] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:00:07] Submodule 'libm' (https://github.com/rust-lang-nursery/libm) registered for path 'src/libcompiler_builtins/libm'
[00:00:07] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
---
[00:12:07] configure: 
[00:12:07] configure: run `python /checkout/obj/build/tmp/distcheck/x.py --help`
[00:12:07] configure: 
[00:12:07] Traceback (most recent call last):
[00:12:07]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 870, in <module>
[00:12:07]     main()
[00:12:07]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 853, in main
[00:12:07]     bootstrap(help_triggered)
[00:12:07]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 810, in bootstrap
[00:12:07]     data = stage0_data(build.rust_root)
[00:12:07]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 158, in stage0_data
[00:12:07]     with open(nightlies, 'r') as nightlies:
[00:12:07] IOError: [Errno 2] No such file or directory: '/checkout/obj/build/tmp/distcheck/stage0.txt'
[00:12:07] Makefile:58: recipe for target 'check' failed
[00:12:07] make: *** [check] Error 1
[00:12:07] 
[00:12:07] 
[00:12:07] command did not execute successfully: "make" "check"
[00:12:07] 
[00:12:07] 
[00:12:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:12:07] Build completed unsuccessfully in 0:09:44
---
travis_time:end:14591410:start=1543449932851960196,finish=1543449932859589395,duration=7629199
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07c89e64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05c21f7f
travis_time:start:05c21f7f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c48ebe8
$ dmesg | grep -i kill

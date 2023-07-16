plain
travis_time:end:02f89077:start=1542823082658892258,finish=1542823085575618150,duration=2916725892
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/e871c4598925594421d63e929fee292e6e071f97.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/bc342a475c09b6df8004d518382e6d5b6bcb49f7.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lldb' (https://github.com/rust-lang-nursery/lldb.git) registered for path 'src/tools/lldb'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/nomicon'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/rust-sgx'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rls'...
---
[00:01:12] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:01:13] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:13] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:13] From https://github.com/rust-lang/libc
[00:01:13]    438034c9..e85479ba  master     -> origin/master
[00:01:13] error: Server does not allow request for unadvertised object bc01bab69ad69606425b31c7fcca31c4d3479b22
[00:01:13] Fetched in submodule path 'src/liblibc', but it did not contain bc01bab69ad69606425b31c7fcca31c4d3479b22. Direct fetching of that commit failed.
[00:01:15] Cleared directory 'src/dlmalloc'
[00:01:15] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:15] Cleared directory 'src/doc/nomicon'
[00:01:15] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:15] Cleared directory 'src/libcompiler_builtins'
[00:01:15] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) unregistered for path 'src/libcompiler_builtins'
[00:01:15] Cleared directory 'src/liblibc'
[00:01:15] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) unregistered for path 'src/liblibc'
[00:01:15] Cleared directory 'src/rust-sgx'
[00:01:15] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) unregistered for path 'src/rust-sgx'
[00:01:15] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) unregistered for path 'src/stdsimd'
[00:01:15] Cleared directory 'src/tools/cargo'
[00:01:15] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) unregistered for path 'src/tools/cargo'
[00:01:15] Cleared directory 'src/tools/clang'
---
[00:01:15] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:01:15] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:01:15] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:01:15] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:01:15] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:01:15] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:01:15] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:01:15] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:01:15] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
---
[00:01:15] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
[00:01:15] Submodule path 'src/libcompiler_builtins': checked out 'e43c838450371008db60d552a586876855ba3d0d'
[00:01:15] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:15] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:16] error: Server does not allow request for unadvertised object bc01bab69ad69606425b31c7fcca31c4d3479b22
[00:01:16] Fetched in submodule path 'src/liblibc', but it did not contain bc01bab69ad69606425b31c7fcca31c4d3479b22. Direct fetching of that commit failed.
[00:01:18] Cleared directory 'src/dlmalloc'
[00:01:18] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:18] Cleared directory 'src/doc/nomicon'
[00:01:18] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:18] Cleared directory 'src/libcompiler_builtins'
[00:01:18] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) unregistered for path 'src/libcompiler_builtins'
[00:01:18] Cleared directory 'src/liblibc'
[00:01:18] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) unregistered for path 'src/liblibc'
[00:01:18] Cleared directory 'src/rust-sgx'
[00:01:18] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) unregistered for path 'src/rust-sgx'
[00:01:18] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) unregistered for path 'src/stdsimd'
[00:01:18] Cleared directory 'src/tools/cargo'
[00:01:18] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) unregistered for path 'src/tools/cargo'
[00:01:18] Cleared directory 'src/tools/clang'
---
[00:01:18] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:01:18] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:01:18] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:01:18] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:01:18] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:01:18] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:01:18] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:01:18] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:01:18] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
---
[00:01:19] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
[00:01:19] Submodule path 'src/libcompiler_builtins': checked out 'e43c838450371008db60d552a586876855ba3d0d'
[00:01:19] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:19] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:20] error: Server does not allow request for unadvertised object bc01bab69ad69606425b31c7fcca31c4d3479b22
[00:01:20] Fetched in submodule path 'src/liblibc', but it did not contain bc01bab69ad69606425b31c7fcca31c4d3479b22. Direct fetching of that commit failed.
[00:01:23] Cleared directory 'src/dlmalloc'
[00:01:23] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:23] Cleared directory 'src/doc/nomicon'
[00:01:23] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:23] Cleared directory 'src/libcompiler_builtins'
[00:01:23] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) unregistered for path 'src/libcompiler_builtins'
[00:01:23] Cleared directory 'src/liblibc'
[00:01:23] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) unregistered for path 'src/liblibc'
[00:01:23] Cleared directory 'src/rust-sgx'
[00:01:23] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) unregistered for path 'src/rust-sgx'
[00:01:23] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) unregistered for path 'src/stdsimd'
[00:01:23] Cleared directory 'src/tools/cargo'
[00:01:23] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) unregistered for path 'src/tools/cargo'
[00:01:23] Cleared directory 'src/tools/clang'
---
[00:01:23] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:01:23] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:01:23] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:01:23] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:01:23] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:01:23] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:01:23] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:01:23] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:01:23] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
---
[00:01:23] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
[00:01:23] Submodule path 'src/libcompiler_builtins': checked out 'e43c838450371008db60d552a586876855ba3d0d'
[00:01:24] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:24] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:24] error: Server does not allow request for unadvertised object bc01bab69ad69606425b31c7fcca31c4d3479b22
[00:01:24] Fetched in submodule path 'src/liblibc', but it did not contain bc01bab69ad69606425b31c7fcca31c4d3479b22. Direct fetching of that commit failed.
[00:01:28] Cleared directory 'src/dlmalloc'
[00:01:28] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:28] Cleared directory 'src/doc/nomicon'
[00:01:28] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:28] Cleared directory 'src/libcompiler_builtins'
[00:01:28] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) unregistered for path 'src/libcompiler_builtins'
[00:01:28] Cleared directory 'src/liblibc'
[00:01:28] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) unregistered for path 'src/liblibc'
[00:01:28] Cleared directory 'src/rust-sgx'
[00:01:28] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) unregistered for path 'src/rust-sgx'
[00:01:28] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) unregistered for path 'src/stdsimd'
[00:01:29] Cleared directory 'src/tools/cargo'
[00:01:29] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) unregistered for path 'src/tools/cargo'
[00:01:29] Cleared directory 'src/tools/clang'
---
[00:01:29] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:01:29] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:01:29] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:01:29] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:01:29] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:01:29] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:01:29] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:01:29] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:01:29] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
---
[00:01:29] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
[00:01:29] Submodule path 'src/libcompiler_builtins': checked out 'e43c838450371008db60d552a586876855ba3d0d'
[00:01:29] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:29] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:30] error: Server does not allow request for unadvertised object bc01bab69ad69606425b31c7fcca31c4d3479b22
[00:01:30] Fetched in submodule path 'src/liblibc', but it did not contain bc01bab69ad69606425b31c7fcca31c4d3479b22. Direct fetching of that commit failed.
travis_time:end:10c3160d:start=1542823099140706468,finish=1542823189705132535,duration=90564426067
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0098e324
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01e8f916:start=1542823189972728939,finish=1542823189979799471,duration=7070532
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06eda2a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02c09286
travis_time:start:02c09286
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0982a8bc
$ dmesg | grep -i kill

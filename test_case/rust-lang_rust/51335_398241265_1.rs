\n\nPaths in `use` statements are relative to the crate root. To import items\nrelative to the current and parent modules, use the `self::` and `super::e            768M     0  768M   0% /var/ramfs
31740 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects/pack
31668 ./obj/build/x86_64-unknown-linux-gnu/doc/src
31532 ./src/libcompiler_builtins/compiler-rt/test
27892 ./.git/modules/src/tools/lld
---
travis_time:end:201449f4:start=1529370078953385609,finish=1529370078960082113,duration=6696504
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:199f0db2
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05c06df6
$ dmesg | grep -i kill

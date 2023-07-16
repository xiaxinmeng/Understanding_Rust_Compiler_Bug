plain
    97% |███████████████████████████████▍| 532kB 49.1MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 44.3MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 2.4MB/s 
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2[K    14% |████▌                           | 10kB 41.1MB/s eta 0:00:01
    42% |█████████████▌                  | 30kB 50.8MB/s eta 0:00:01
    56% |██████████████████              | 40kB 53.6MB/s eta 0:00:01
    70% |██████████████████████▋         | 51kB 55.2MB/s eta 0:00:01
    84% |███████████████████████████     | 61kB 58.1MB/s eta 0:00:01
---
[00:19:53]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:19:53]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:19:58]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
2m...
[00:20:04] 100 |     });
[00:20:04]     |       - temporary value only lives until here
[00:20:04]     |
[00:20:04]     = note: borrowed value must be valid for the static lifetime...
[00:20:04]     = note: consider using a `let` binding to increase its lifetime
[00:20:04] 
[00:20:04] error[E0597]: borrowed value does not live long enough
[00:20:04]     |
[00:20:04]     |
[00:20:04] 218 |     let prev = LOCAL_STDERR.with(|s| s.borrow_mut().take());
[00:20:04]     |                ^^^^^^^^^^^^                                - temporary value only lives until here
[00:20:04]     |                |
[00:20:04]     |                temporary value does not live long enough
[00:20:04]     |
[00:20:04]     = note: borrowed value must be valid for the static lifetime...
[00:20:04]     = note:[00:20:04] error: aborting due to 3 previous errors
[00:20:04] For more information about this error, try `rustc --explain E0597`.
[00:20:04] error: Could not compile `std`.
[00:20:04] 
[00:20:04] Caused by:
[00:20:04] Caused by:
[00:20:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=03a946e021ed5d37 -C extra-filename=-03a946e021ed5d37 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-2bf84ccb717c3c05.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-b3b878d9b3919b74.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-e485a43971c20bb9.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcomp

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:046d7f6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:347a62e2:start=1530919042059136075,finish=1530919042066823128,duration=7687053
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c5d2869
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
tr

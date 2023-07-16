plain
travis_time:end:2060500d:start=1552287844715727058,finish=1552287846353950989,duration=1638223931
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:00:00] Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libunwind/libunwind' (https://git.llvm.org/git/libunwind.git) registered for path 'src/libunwind/libunwind'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
---

[00:03:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/docs/conf.py:251: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:111: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:122: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:124: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:125: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:127: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:131: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:236: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:237: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:240: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:243: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:245: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:246: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:247: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:254: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:256: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:258: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:259: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:261: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:265: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:310: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:312: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:314: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:318: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:323: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:340: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:341: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:342: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:343: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:347: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:348: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:352: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:375: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:376: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:379: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:382: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:383: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:387: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:390: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:391: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:405: trailing whitespace
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:427: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/include/mach-o/compact_unwind_encoding.h:428: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/config.h:69: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:479: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:481: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:483: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:492: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:494: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-seh.cpp:496: line longer than 100 chars
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-EHABI.cpp:79: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-EHABI.cpp:116: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-EHABI.cpp:120: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-EHABI.cpp:129: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/src/Unwind-EHABI.cpp:712: TODO is deprecated; use FIXME
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/test/libunwind/test/__init__.py: empty file
[00:03:59] tidy error: /checkout/src/libunwind/libunwind/test/libunwind/__init__.py: empty file
[00:04:01] some tidy checks failed
[00:04:01] 
[00:04:01] 
[00:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:01] 
[00:04:01] 
[00:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:01] Build completed unsuccessfully in 0:00:43
[00:04:01] Build completed unsuccessfully in 0:00:43
[00:04:01] make: *** [tidy] Error 1
[00:04:01] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c4c1b0a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 11 07:08:19 UTC 2019
---
travis_time:end:0cde8db0:start=1552288100375095301,finish=1552288100380276874,duration=5181573
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0564bd60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f918c3
travis_time:start:00f918c3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:018f50bb
$ dmesg | grep -i kill

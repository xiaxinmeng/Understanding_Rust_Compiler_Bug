plain
travis_time:end:282f309e:start=1560363086718034529,finish=1560363088879932910,duration=2161898381
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:48]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:14:43]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:43]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:15:07]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:15:18] error[E0599]: no method named `to_ptr` found for type `&mut interpret::memory::Memory<'mir, 'tcx, M>` in the current scope
[00:15:18]    --> src/librustc_mir/interpret/memory.rs:717:24
[00:15:18]     |
[00:15:18] 717 |         let src = self.to_ptr(src)?;
[00:15:18] 
[00:15:18] 
[00:15:18] error[E0599]: no method named `to_ptr` found for type `&mut interpret::memory::Memory<'mir, 'tcx, M>` in the current scope
[00:15:18]    --> src/librustc_mir/interpret/memory.rs:718:25
[00:15:18] 718 |         let dest = self.to_ptr(dest)?;
[00:15:18]     |                         ^^^^^^
[00:15:18] 
[00:15:20] error: aborting due to 2 previous errors
---
travis_time:end:0e0a32e0:start=1560364172144025475,finish=1560364172148822156,duration=4796681
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1542dcf8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22963cc4
travis_time:start:22963cc4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0afbc060
$ dmesg | grep -i kill

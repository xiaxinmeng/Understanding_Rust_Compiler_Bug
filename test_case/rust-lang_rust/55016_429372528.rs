plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:16ffcbc0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:14:10]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:14:12] error: doc comment not used by rustdoc
[00:14:12]   --> librustc_mir/interpret/traits.rs:52:9
[00:14:12]    |
[00:14:12] 52 |         /// If you touch this code, be sure to also make the corresponding changes to
[00:14:12]    |
[00:14:12]    = note: `-D unused-doc-comments` implied by `-D warnings`
[00:14:12] 
[00:14:23] error: aborting due to previous error
---
[00:15:23] travis_fold:end:stage0-rustc

[00:15:23] travis_time:end:stage0-rustc:start=1539358925796933868,finish=1539359437932254659,duration=512135320791

[00:15:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:15:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:15:23] Build completed unsuccessfully in 0:09:38
travis_time:end:0a6c36df:start=1539358513974048904,finish=1539359438194569767,duration=924220520863

---
travis_time:end:03bf0fca:start=1539359438672141567,finish=1539359438680370141,duration=8228574
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:009e5e46
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fc6de26
travis_time:start:0fc6de26
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2978f7c0
$ dmesg | grep -i kill

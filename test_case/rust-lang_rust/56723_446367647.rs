plain
travis_time:end:04b9a33c:start=1544558344989913189,finish=1544558420352388686,duration=75362475497
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:07] configure: build.locked-deps    := True
[00:07:07] configure: llvm.ccache          := sccache
[00:07:07] configure: build.cargo-native-static := True
[00:07:07] configure: dist.missing-tools   := True
[00:07:07] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:07:07] configure: writing `config.toml` in current directory
[00:07:07] configure: 
[00:07:07] configure: run `python /checkout/x.py --help`
[00:07:07] configure: 
---
[00:09:44]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:11:09] error[E0080]: could not evaluate static initializer
[00:11:09]     --> src/librustc/ty/sty.rs:1989:1
[00:11:09]      |
[00:11:09] 1989 | static_assert!(MEM_SIZE_OF_LAZY_CONST: ::std::mem::size_of::<LazyConst<'_>>() == 24);
[00:11:09]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
[00:11:09]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:11:09] 
[00:11:10] error: aborting due to previous error
[00:11:10] 
[00:11:10] 
[00:11:10] For more information about this error, try `rustc --explain E0080`.
[00:11:10] error: Could not compile `rustc`.
[00:11:10] 
[00:11:10] To learn more, run the command again with --verbose.
[00:11:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:11:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:11:10] Build completed unsuccessfully in 0:02:54
travis_time:end:0e63e16a:start=1544558428984192020,finish=1544559099453644431,duration=670469452411
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:02ca9624:start=1544559099902914048,finish=1544559099909545488,duration=6631440
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ba53794
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1441e3ec
travis_time:start:1441e3ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07329432
$ dmesg | grep -i kill

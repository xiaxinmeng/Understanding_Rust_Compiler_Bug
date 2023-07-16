plain
travis_time:end:074dec47:start=1549206021021908878,finish=1549206180419984500,duration=159398075622
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:25] configure: build.locked-deps    := True
[00:06:25] configure: llvm.ccache          := sccache
[00:06:25] configure: build.cargo-native-static := True
[00:06:25] configure: dist.missing-tools   := True
[00:06:25] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:25] configure: writing `config.toml` in current directory
[00:06:25] configure: 
[00:06:25] configure: run `python /checkout/x.py --help`
[00:06:25] configure: 
---
[00:08:14]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:08:14] error[E0433]: failed to resolve: use of undeclared type or module `libc`
[00:08:14]   --> src/libterm/win.rs:29:14
[00:08:14]    |
[00:08:14] 29 |     dwSize: [libc::c_short; 2],
[00:08:14]    |              ^^^^ use of undeclared type or module `libc`
[00:08:14] error[E0433]: failed to resolve: use of undeclared type or module `libc`
[00:08:14]   --> src/libterm/win.rs:30:24
[00:08:14]    |
[00:08:14]    |
[00:08:14] 30 |     dwCursorPosition: [libc::c_short; 2],
[00:08:14]    |                        ^^^^ use of undeclared type or module `libc`
[00:08:14] error[E0433]: failed to resolve: use of undeclared type or module `libc`
[00:08:14]   --> src/libterm/win.rs:32:16
[00:08:14]    |
[00:08:14]    |
[00:08:14] 32 |     srWindow: [libc::c_short; 4],
[00:08:14]    |                ^^^^ use of undeclared type or module `libc`
[00:08:14] error[E0433]: failed to resolve: use of undeclared type or module `libc`
[00:08:14]   --> src/libterm/win.rs:33:27
[00:08:14]    |
[00:08:14]    |
[00:08:14] 33 |     dwMaximumWindowSize: [libc::c_short; 2],
[00:08:14]    |                           ^^^^ use of undeclared type or module `libc`
[00:08:15] error: aborting due to 4 previous errors
[00:08:15] 
[00:08:15] For more information about this error, try `rustc --explain E0433`.
[00:08:15] error: Could not compile `term`.
[00:08:15] error: Could not compile `term`.
[00:08:15] warning: build failed, waiting for other jobs to finish...
[00:08:17] error: build failed
[00:08:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
[00:08:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:08:17] Build completed unsuccessfully in 0:00:41
[00:08:17] script exited with 1
travis_time:end:017459ca:start=1549206189305634936,finish=1549206687555406561,duration=498249771625
---
travis_time:end:0cdfa700:start=1549206688578682362,finish=1549206688583936767,duration=5254405
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fbbd11f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01aefd82
travis_time:start:01aefd82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1314445e
$ dmesg | grep -i kill

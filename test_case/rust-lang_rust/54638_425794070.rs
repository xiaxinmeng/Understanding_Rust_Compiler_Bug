plain
[00:01:39] configure: llvm.ccache          := sccache
[00:01:39] configure: build.openssl-static := True
[00:01:39] configure: build.build          := x86_64-unknown-linux-gnu
[00:01:39] configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-5.0/bin/l ...
[00:01:39] configure: dist.missing-tools   := True
[00:01:39] configure: 
[00:01:39] configure: writing `config.toml` in current directory
[00:01:39] configure: 
[00:01:39] configure: run `python /checkout/x.py --help`
---
[00:02:20]    Compiling toml v0.4.6
[00:02:20]    Compiling serde_json v1.0.26
[00:02:29]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:59]     Finished dev [unoptimized] target(s) in 1m 03s
[00:02:59] failed to parse TOML configuration 'config.toml': unknown field `missing-tools`, expected one of `sign-folder`, `gpg-password-file`, `upload-addr`, `src-tarball` for key `dist`
[00:02:59] Build completed unsuccessfully in 0:01:19
[00:02:59] Makefile:81: recipe for target 'prepare' failed
[00:02:59] make: *** [prepare] Error 1
[00:03:00] Command failed. Attempt 2/5:
[00:03:00] Command failed. Attempt 2/5:
[00:03:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:00]     Finished dev [unoptimized] target(s) in 0.23s
[00:03:00] failed to parse TOML configuration 'config.toml': unknown field `missing-tools`, expected one of `sign-folder`, `gpg-password-file`, `upload-addr`, `src-tarball` for key `dist`
[00:03:00] Build completed unsuccessfully in 0:00:00
[00:03:00] Makefile:81: recipe for target 'prepare' failed
[00:03:00] make: *** [prepare] Error 1
[00:03:02] Command failed. Attempt 3/5:
[00:03:02] Command failed. Attempt 3/5:
[00:03:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:03]     Finished dev [unoptimized] target(s) in 0.24s
[00:03:03] failed to parse TOML configuration 'config.toml': unknown field `missing-tools`, expected one of `sign-folder`, `gpg-password-file`, `upload-addr`, `src-tarball` for key `dist`
[00:03:03] Build completed unsuccessfully in 0:00:00
[00:03:03] make: *** [prepare] Error 1
[00:03:03] Makefile:81: recipe for target 'prepare' failed
[00:03:06] Command failed. Attempt 4/5:
[00:03:06] Command failed. Attempt 4/5:
[00:03:06] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:06] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:06] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:06] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:06] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:06]     Finished dev [unoptimized] target(s) in 0.24s
[00:03:06] failed to parse TOML configuration 'config.toml': unknown field `missing-tools`, expected one of `sign-folder`, `gpg-password-file`, `upload-addr`, `src-tarball` for key `dist`
[00:03:06] Build completed unsuccessfully in 0:00:00
[00:03:06] make: *** [prepare] Error 1
[00:03:06] Makefile:81: recipe for target 'prepare' failed
[00:03:10] Command failed. Attempt 5/5:
[00:03:10] Command failed. Attempt 5/5:
[00:03:10] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:10] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:10] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:10] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:10] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:10]     Finished dev [unoptimized] target(s) in 0.24s
[00:03:10] failed to parse TOML configuration 'config.toml': unknown field `missing-tools`, expected one of `sign-folder`, `gpg-password-file`, `upload-addr`, `src-tarball` for key `dist`
[00:03:10] Build completed unsuccessfully in 0:00:00
[00:03:10] make: *** [prepare] Error 1
[00:03:10] Makefile:81: recipe for target 'prepare' failed
[00:03:10] The command has failed after 5 attempts.
---
travis_time:end:07257294:start=1538371689863351411,finish=1538371689868265487,duration=4914076
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b6d824
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fbd3c80
travis_time:start:0fbd3c80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04786aa0
$ dmesg | grep -i kill

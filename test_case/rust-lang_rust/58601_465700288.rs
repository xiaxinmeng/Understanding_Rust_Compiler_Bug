plain
travis_time:end:0da2b216:start=1550686929924600452,finish=1550686930819087585,duration=894487133
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:10:24]    Compiling rustc-rayon v0.1.1
[00:10:27]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:10:27]    Compiling tempfile v3.0.5
[00:10:30]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:10:30] error: expected one of `,`, `.`, `?`, `}`, or an operator, found `;`
[00:10:30]      |
[00:10:30]      |
[00:10:30] 1188 |                     Ok(t) => return Ok(t);
[00:10:30]      |                           --             ^ expected one of `,`, `.`, `?`, `}`, or an operator here
[00:10:30]      |                           |
[00:10:30]      |                           while parsing the `match` arm starting here
[00:10:30]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:10:31] error: unreachable statement
[00:10:31]     --> src/librustc_target/spec/mod.rs:1194:17
[00:10:31]      |
[00:10:31]      |
[00:10:31] 1194 | /                 let path = {
[00:10:31] 1195 | |                     let mut target = target_triple.to_string();
[00:10:31] 1196 | |                     target.push_str(".json");
[00:10:31] 1197 | |                     PathBuf::from(target)
[00:10:31]      | |__________________^
[00:10:31]      |
[00:10:31]      = note: `-D unreachable-code` implied by `-D warnings`
[00:10:31] 
---
[00:10:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:10:34] expected success, got: exit code: 101
[00:10:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:10:34] Build completed unsuccessfully in 0:02:06
[00:10:34] make: *** [all] Error 1
[00:10:34] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05a93c0c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 18:32:56 UTC 2019
---
travis_time:end:074f3d04:start=1550687577217707182,finish=1550687577222406625,duration=4699443
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b7ea96
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:162fe958
travis_time:start:162fe958
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053245f2
$ dmesg | grep -i kill

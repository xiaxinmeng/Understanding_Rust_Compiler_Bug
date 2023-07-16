plain
travis_time:end:2bc5ddfd:start=1550682900316794114,finish=1550682901272244157,duration=955450043
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:08:50]    Compiling rustc-rayon v0.1.1
[00:08:54]    Compiling tempfile v3.0.5
[00:08:54]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:08:56]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:08:56] error: expected one of `,`, `.`, `?`, `}`, or an operator, found `;`
[00:08:56]      |
[00:08:56]      |
[00:08:56] 1188 |                     Ok(t) => return Ok(t);
[00:08:56]      |                           --             ^ expected one of `,`, `.`, `?`, `}`, or an operator here
[00:08:56]      |                           |
[00:08:56]      |                           while parsing the `match` arm starting here
[00:08:57]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:08:58] error: unreachable statement
[00:08:58]     --> src/librustc_target/spec/mod.rs:1194:17
[00:08:58]      |
[00:08:58]      |
[00:08:58] 1194 | /                 let path = {
[00:08:58] 1195 | |                     let mut target = target_triple.to_string();
[00:08:58] 1196 | |                     target.push_str(".json");
[00:08:58] 1197 | |                     PathBuf::from(target)
[00:08:58]      | |__________________^
[00:08:58]      |
[00:08:58]      = note: `-D unreachable-code` implied by `-D warnings`
[00:08:58] 
---
[00:09:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:01] expected success, got: exit code: 101
[00:09:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:01] Build completed unsuccessfully in 0:02:05
[00:09:01] make: *** [all] Error 1
[00:09:01] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2424f1ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 17:24:13 UTC 2019
---
travis_time:end:3677af2a:start=1550683453798676427,finish=1550683453803221705,duration=4545278
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:107f830b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05346fa8
travis_time:start:05346fa8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c1420d4
$ dmesg | grep -i kill

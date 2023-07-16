plain
travis_time:end:0fc08264:start=1559331014409966932,finish=1559331166350466336,duration=151940499404
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:21] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:9: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:72: TODO is deprecated; use FIXME
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:75: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:92: TODO is deprecated; use FIXME
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:97: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:108: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:115: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs:140: TODO is deprecated; use FIXME
[00:04:21] tidy error: /checkout/src/librustc_errors/annotate_rs_emitter.rs: too many trailing newlines (2)
[00:04:21] tidy error: /checkout/src/librustc/session/config.rs:1979: line longer than 100 chars
[00:04:21] tidy error: /checkout/src/librustc/session/config.rs:2016: line longer than 100 chars
[00:04:26] Dependencies not on the whitelist:
[00:04:26] * annotate-snippets 
[00:04:26] * ansi_term 
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:01:16
---
travis_time:end:0063792c:start=1559331442935453438,finish=1559331442941238311,duration=5784873
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a16972f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3781c4
travis_time:start:0c3781c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:134dc9c1
$ dmesg | grep -i kill

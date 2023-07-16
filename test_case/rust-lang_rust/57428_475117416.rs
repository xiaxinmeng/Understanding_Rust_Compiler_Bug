plain
travis_time:end:0ff77c26:start=1553145457236309382,finish=1553145459577327669,duration=2341018287
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:05:19] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-apit-fail.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-dyn-apit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/rpit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/duplicate.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-where.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/type-alias.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/entails-sized-object-safety.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/dyn-lcsit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/lcsit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/struct-bounds.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-inline.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/trait-params.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/existential-type.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/dyn-existential-type.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-apit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/union-bounds.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/enum-bounds.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/auxiliary/fn-aux.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/fn-wrap-apit.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs
[00:05:19] tidy error: binary checked into source: /checkout/src/test/ui/associated-type-bounds/dyn-rpit-and-let.rs
[00:05:20] tidy error: /checkout/src/librustc_passes/ast_validation.rs:734: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/librustc_typeck/collect.rs:706: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/librustc_typeck/astconv.rs:936: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/librustc_typeck/astconv.rs:937: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/librustc_typeck/astconv.rs:999: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/fn-apit-fail.rs:14: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/fn-dyn-apit.rs:30: TODO is deprecated; use FIXME
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:10: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:12: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:14: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:16: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:18: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:20: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:23: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:25: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:27: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:29: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:31: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:33: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:36: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:38: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:40: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:42: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:44: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:46: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:49: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:51: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:53: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:55: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:57: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:59: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:62: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:64: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:66: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:68: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:70: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:72: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:75: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:77: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:79: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:81: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:83: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:85: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:88: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:90: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:92: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:95: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:97: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:99: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:101: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:103: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:105: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:108: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:110: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:112: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:114: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:116: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:118: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:121: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:123: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:125: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:127: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:129: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:131: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:133: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:135: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:137: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:139: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:141: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:143: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:145: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:147: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:149: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:152: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:154: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:156: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs:38: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:7: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:10: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:13: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:17: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:20: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:23: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:27: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:30: line longer than 100 chars
[00:05:20] tidy error: /checkout/src/test/ui/associated-type-bounds/inside-adt.rs:33: line longer than 100 chars
[00:05:21] some tidy checks failed
[00:05:21] 
[00:05:21] 
[00:05:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:21] 
[00:05:21] 
[00:05:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:21] Build completed unsuccessfully in 0:00:46
[00:05:21] Build completed unsuccessfully in 0:00:46
[00:05:21] Makefile:67: recipe for target 'tidy' failed
[00:05:21] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00008a9e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 21 05:23:11 UTC 2019
---
travis_time:end:00593d4b:start=1553145791936635219,finish=1553145791941582417,duration=4947198
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05514dc0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03f7b1e0
travis_time:start:03f7b1e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b1206f1
$ dmesg | grep -i kill

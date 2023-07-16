plain
travis_time:end:0a6747fa:start=1558539228725843776,finish=1558539230930549063,duration=2204705287
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:13:53]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:53] error[E0252]: the name `Symbol` is defined multiple times
[00:13:53]   --> src/librustc_metadata/decoder.rs:36:68
[00:13:53]    |
[00:13:53] 33 | use syntax::symbol::{Symbol, sym};
[00:13:53]    |                      ------ previous import of the type `Symbol` here
[00:13:53] ...
[00:13:53] 36 | use syntax_pos::{self, Span, BytePos, Pos, DUMMY_SP, NO_EXPANSION, Symbol};
[00:13:53]    |                                                                    ^^^^^^ `Symbol` reimported here
[00:13:53]    = note: `Symbol` must be defined only once in the type namespace of this module
[00:13:53] 
[00:13:53] error: unused import: `Symbol`
[00:13:53]   --> src/librustc_metadata/decoder.rs:36:68
[00:13:53]   --> src/librustc_metadata/decoder.rs:36:68
[00:13:53]    |
[00:13:53] 36 | use syntax_pos::{self, Span, BytePos, Pos, DUMMY_SP, NO_EXPANSION, Symbol};
[00:13:53]    |
[00:13:53]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:53] 
[00:13:55] error: aborting due to 2 previous errors
---
travis_time:end:170f0cdc:start=1558540326056296542,finish=1558540326060685918,duration=4389376
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0420c318
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0074763a
travis_time:start:0074763a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:058b1a28
$ dmesg | grep -i kill

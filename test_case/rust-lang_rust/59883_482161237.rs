plain
travis_time:end:1273d20e:start=1554996018455684596,finish=1554996019182043995,duration=726359399
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:49]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:55] error[E0382]: borrow of moved value: `reader_metadata`
[00:04:55]    --> src/libstd/sys/unix/fs.rs:891:15
[00:04:55]     |
[00:04:55] 889 |     let (mut reader, reader_metadata) = open_from(from)?;
[00:04:55]     |                      --------------- move occurs because `reader_metadata` has type `fs::Metadata`, which does not implement the `Copy` trait
[00:04:55] 890 |     let (mut writer, _) = open_to_and_set_permissions(to, reader_metadata)?;
[00:04:55]     |                                                           --------------- value moved here
[00:04:55] 891 |     let len = reader_metadata.len();
[00:04:55]     |               ^^^^^^^^^^^^^^^ value borrowed here after move
[00:04:55] error: aborting due to previous error
[00:04:55] 
[00:04:55] For more information about this error, try `rustc --explain E0382`.
[00:04:55] error: Could not compile `std`.
---
travis_time:end:030ca053:start=1554996326731894733,finish=1554996326737708295,duration=5813562
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24dcc291
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:004b9537
travis_time:start:004b9537
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_fa

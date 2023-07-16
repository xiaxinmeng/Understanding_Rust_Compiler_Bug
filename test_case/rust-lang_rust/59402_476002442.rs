plain
travis_time:end:2089f16b:start=1553461422841257927,finish=1553461423787956486,duration=946698559
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:28:40]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:28:42] error: lifetime may not live long enough
[00:28:42]    --> src/librustc_data_structures/graph/implementation/mod.rs:240:9
[00:28:42]     |
[00:28:42] 236 |     pub fn successor_nodes<'a>(
[00:28:42]     |                            -- lifetime `'a` defined here
[00:28:42] ...
[00:28:42] 240 |         self.outgoing_edges(source).targets()
[00:28:42]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[00:28:42] help: to allow this impl Trait to capture borrowed data with lifetime `'a`, add `'a` as a constraint
[00:28:42]     |
[00:28:42] 239 |     ) -> impl Iterator<Item = NodeIndex> + 'a + 'a {
[00:28:42] 
[00:28:42] error: lifetime may not live long enough
[00:28:42]    --> src/librustc_data_structures/graph/implementation/mod.rs:247:9
[00:28:42]     |
[00:28:42]     |
[00:28:42] 243 |     pub fn predecessor_nodes<'a>(
[00:28:42]     |                              -- lifetime `'a` defined here
[00:28:42] ...
[00:28:42] 247 |         self.incoming_edges(target).sources()
[00:28:42]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[00:28:42] help: to allow this impl Trait to capture borrowed data with lifetime `'a`, add `'a` as a constraint
[00:28:42]     |
[00:28:42] 246 |     ) -> impl Iterator<Item = NodeIndex> + 'a + 'a {
[00:28:42] 
[00:28:43] error: aborting due to 2 previous errors
[00:28:43] 
[00:28:43] error: Could not compile `rustc_data_structures`.
---
travis_time:end:000583c2:start=1553463184039864065,finish=1553463184044554438,duration=4690373
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2841a47e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17084f4c
travis_time:start:17084f4c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e4a55b3
$ dmesg | grep -i kill

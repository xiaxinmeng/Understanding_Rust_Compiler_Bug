plain
travis_time:end:039da898:start=1560253851402060646,finish=1560253853676313846,duration=2274253200
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:02]    Compiling syn v0.15.22
[00:04:17]    Compiling serde_derive v1.0.81
[00:04:52]    Compiling serde_json v1.0.33
[00:04:55]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]  --> src/tools/tidy/src/errors.rs:6:23
[00:04:55] 6 | use std::collections::HashMap;
[00:04:55]   |                       ^^^^^^^ help: use: `FxHashMap`
[00:04:55]   |
[00:04:55]   = note: `-D default-hash-types` implied by `-D internal`
[00:04:55]   = note: `-D default-hash-types` implied by `-D internal`
[00:04:55]   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]   --> src/tools/tidy/src/errors.rs:13:18
[00:04:55]    |
[00:04:55] 13 |     let mut map: HashMap<_, Vec<_>> = HashMap::new();
[00:04:55]    |                  ^^^^^^^ help: use: `FxHashMap`
[00:04:55]    |
[00:04:55]    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]   --> src/tools/tidy/src/errors.rs:13:39
[00:04:55]    |
[00:04:55] 13 |     let mut map: HashMap<_, Vec<_>> = HashMap::new();
[00:04:55]    |                                       ^^^^^^^ help: use: `FxHashMap`
[00:04:55]    |
[00:04:55]    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]   --> src/tools/tidy/src/features.rs:12:23
[00:04:55] 12 | use std::collections::HashMap;
[00:04:55]    |                       ^^^^^^^ help: use: `FxHashMap`
[00:04:55]    |
[00:04:55]    |
[00:04:55]    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]   --> src/tools/tidy/src/features.rs:52:21
[00:04:55]    |
[00:04:55] 52 | pub type Features = HashMap<String, Feature>;
[00:04:55]    |                     ^^^^^^^ help: use: `FxHashMap`
[00:04:55]    |
[00:04:55]    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashSet over HashSet, it has better performance
[00:04:55]  --> src/tools/tidy/src/deps.rs:3:34
[00:04:55]   |
[00:04:55] 3 | use std::collections::{BTreeSet, HashSet, HashMap};
[00:04:55]   |                                  ^^^^^^^ help: use: `FxHashSet`
[00:04:55]   |
[00:04:55]   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]  --> src/tools/tidy/src/deps.rs:3:43
[00:04:55]   |
[00:04:55] 3 | use std::collections::{BTreeSet, HashSet, HashMap};
[00:04:55]   |                                           ^^^^^^^ help: use: `FxHashMap`
[00:04:55]   |
[00:04:55]   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashSet over HashSet, it has better performance
[00:04:55]    --> src/tools/tidy/src/deps.rs:265:20
[00:04:55]     |
[00:04:55] 265 |     let whitelist: HashSet<_> = WHITELIST.iter().cloned().collect();
[00:04:55]     |                    ^^^^^^^ help: use: `FxHashSet`
[00:04:55]     |
[00:04:55]     = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashSet over HashSet, it has better performance
[00:04:55]    --> src/tools/tidy/src/deps.rs:345:20
[00:04:55]     |
[00:04:55] 345 |     whitelist: &'a HashSet<Crate<'_>>,
[00:04:55]     |                    ^^^^^^^ help: use: `FxHashSet`
[00:04:55]     |
[00:04:55]     = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]    --> src/tools/tidy/src/deps.rs:395:25
[00:04:55]     |
[00:04:55] 395 |     let mut name_to_id: HashMap<_, Vec<_>> = HashMap::new();
[00:04:55]     |                         ^^^^^^^ help: use: `FxHashMap`
[00:04:55]     |
[00:04:55]     = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:55] 
[00:04:55] error: Prefer FxHashMap over HashMap, it has better performance
[00:04:55]    --> src/tools/tidy/src/deps.rs:395:46
[00:04:55]     |
[00:04:55] 395 |     let mut name_to_id: HashMap<_, Vec<_>> = HashMap::new();
[00:04:55]     |                                              ^^^^^^^ help: use: `FxHashMap`
[00:04:55]     |
[00:04:55]     = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[00:04:56] error: aborting due to 11 previous errors
[00:04:56] 
[00:04:56] 
[00:04:56] error: Could not compile `tidy`.
[00:04:56] To learn more, run the command again with --verbose.
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:04:56] expected success, got: exit code: 101
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
---
travis_time:end:01575a14:start=1560254161828335724,finish=1560254161833166700,duration=4830976
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08b1c5d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14b1b5f8
travis_time:start:14b1b5f8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fd56864
$ dmesg | grep -i kill

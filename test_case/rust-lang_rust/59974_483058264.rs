plain
travis_time:end:0d2add2a:start=1555275681232161794,finish=1555275682005096885,duration=772935091
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:06]    Compiling serde_derive v1.0.81
[00:02:08]    Compiling toml v0.4.10
[00:02:08]    Compiling serde_json v1.0.33
[00:02:16]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:17] error: the item `Seek` is imported redundantly
[00:02:17]     --> src/bootstrap/lib.rs:1128:23
[00:02:17]      |
[00:02:17] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:17]      |               ---- the item `Seek` is already imported here
[00:02:17] ...
[00:02:17] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:17]      |
[00:02:17] note: lint level defined here
[00:02:17]     --> src/bootstrap/lib.rs:107:9
[00:02:17]      |
[00:02:17]      |
[00:02:17] 107  | #![deny(warnings)]
[00:02:17]      |         ^^^^^^^^
[00:02:17]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:02:17] 
[00:02:17] error: the item `SeekFrom` is imported redundantly
[00:02:17]     --> src/bootstrap/lib.rs:1128:29
[00:02:17]      |
[00:02:17] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:17]      |                     -------- the item `SeekFrom` is already imported here
[00:02:17] ...
[00:02:17] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:17] 
[00:02:20] error: aborting due to 2 previous errors
[00:02:20] 
[00:02:20] error: Could not compile `bootstrap`.
---
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
[00:02:21] Command failed. Attempt 2/5:
[00:02:21]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:22] error: the item `Seek` is imported redundantly
[00:02:22]     --> src/bootstrap/lib.rs:1128:23
[00:02:22]      |
[00:02:22] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:22]      |               ---- the item `Seek` is already imported here
[00:02:22] ...
[00:02:22] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:22]      |
[00:02:22] note: lint level defined here
[00:02:22]     --> src/bootstrap/lib.rs:107:9
[00:02:22]      |
[00:02:22]      |
[00:02:22] 107  | #![deny(warnings)]
[00:02:22]      |         ^^^^^^^^
[00:02:22]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:02:22] 
[00:02:22] error: the item `SeekFrom` is imported redundantly
[00:02:22]     --> src/bootstrap/lib.rs:1128:29
[00:02:22]      |
[00:02:22] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:22]      |                     -------- the item `SeekFrom` is already imported here
[00:02:22] ...
[00:02:22] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:22] 
[00:02:25] error: aborting due to 2 previous errors
[00:02:25] 
[00:02:25] error: Could not compile `bootstrap`.
---
[00:02:25] Makefile:69: recipe for target 'prepare' failed
[00:02:25] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 3/5:
[00:02:28]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:29] error: the item `Seek` is imported redundantly
[00:02:29]     --> src/bootstrap/lib.rs:1128:23
[00:02:29]      |
[00:02:29] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:29]      |               ---- the item `Seek` is already imported here
[00:02:29] ...
[00:02:29] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:29]      |
[00:02:29] note: lint level defined here
[00:02:29]     --> src/bootstrap/lib.rs:107:9
[00:02:29]      |
[00:02:29]      |
[00:02:29] 107  | #![deny(warnings)]
[00:02:29]      |         ^^^^^^^^
[00:02:29]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:02:29] 
[00:02:29] error: the item `SeekFrom` is imported redundantly
[00:02:29]     --> src/bootstrap/lib.rs:1128:29
[00:02:29]      |
[00:02:29] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:29]      |                     -------- the item `SeekFrom` is already imported here
[00:02:29] ...
[00:02:29] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:29] 
[00:02:32] error: aborting due to 2 previous errors
[00:02:32] 
[00:02:32] error: Could not compile `bootstrap`.
---
[00:02:32] Makefile:69: recipe for target 'prepare' failed
[00:02:32] make: *** [prepare] Error 1
[00:02:35] Command failed. Attempt 4/5:
[00:02:35]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:36] error: the item `Seek` is imported redundantly
[00:02:36]     --> src/bootstrap/lib.rs:1128:23
[00:02:36]      |
[00:02:36] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:36]      |               ---- the item `Seek` is already imported here
[00:02:36] ...
[00:02:36] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:36]      |
[00:02:36] note: lint level defined here
[00:02:36]     --> src/bootstrap/lib.rs:107:9
[00:02:36]      |
[00:02:36]      |
[00:02:36] 107  | #![deny(warnings)]
[00:02:36]      |         ^^^^^^^^
[00:02:36]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:02:36] 
[00:02:36] error: the item `SeekFrom` is imported redundantly
[00:02:36]     --> src/bootstrap/lib.rs:1128:29
[00:02:36]      |
[00:02:36] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:36]      |                     -------- the item `SeekFrom` is already imported here
[00:02:36] ...
[00:02:36] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:36] 
[00:02:39] error: aborting due to 2 previous errors
[00:02:39] 
[00:02:39] error: Could not compile `bootstrap`.
---
[00:02:39] Makefile:69: recipe for target 'prepare' failed
[00:02:39] make: *** [prepare] Error 1
[00:02:43] Command failed. Attempt 5/5:
[00:02:44]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:44] error: the item `Seek` is imported redundantly
[00:02:44]     --> src/bootstrap/lib.rs:1128:23
[00:02:44]      |
[00:02:44] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:44]      |               ---- the item `Seek` is already imported here
[00:02:44] ...
[00:02:44] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:44]      |
[00:02:44] note: lint level defined here
[00:02:44]     --> src/bootstrap/lib.rs:107:9
[00:02:44]      |
[00:02:44]      |
[00:02:44] 107  | #![deny(warnings)]
[00:02:44]      |         ^^^^^^^^
[00:02:44]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:02:44] 
[00:02:44] error: the item `SeekFrom` is imported redundantly
[00:02:44]     --> src/bootstrap/lib.rs:1128:29
[00:02:44]      |
[00:02:44] 126  | use std::io::{Seek, SeekFrom, Write, Read};
[00:02:44]      |                     -------- the item `SeekFrom` is already imported here
[00:02:44] ...
[00:02:44] 1128 |         use std::io::{Seek, SeekFrom};
[00:02:44] 
[00:02:48] error: aborting due to 2 previous errors
[00:02:48] 
[00:02:48] error: Could not compile `bootstrap`.
---
travis_time:end:0e165f40:start=1555275861488183664,finish=1555275861493591185,duration=5407521
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fbcccfe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04b53932
travis_time:start:04b53932
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01c23237
$ dmesg | grep -i kill

plain
travis_time:end:077bfea8:start=1540759726713191818,finish=1540759778482466246,duration=51769274428
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:40:47] 2999 |             filename: FileName::Anon,
[00:40:47]      |                       ^^^^^^^^^^^^^^ expected enum `syntax_pos::FileName`, found fn item
[00:40:47]      |
[00:40:47]      = note: expected type `syntax_pos::FileName`
[00:40:47]                 found type `fn(u64) -> syntax_pos::FileName {syntax_pos::FileName::Anon}`
[00:40:49] error: aborting due to previous error
[00:40:49] 
[00:40:49] For more information about this error, try `rustc --explain E0308`.
[00:40:49] error: Could not compile `rustdoc`.
---
[00:40:49] 
[00:40:49] 
[00:40:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:40:49] Build completed unsuccessfully in 0:36:39
[00:40:49] make: *** [all] Error 1
[00:40:49] Makefile:28: recipe for target 'all' failed
1788272 ./obj
1788232 ./obj/build
1191180 ./.git
1159524 ./obj/build/x86_64-unknown-linux-gnu
---
151412 ./src/tools/clang
149644 ./obj/build/bootstrap/debug/incremental
149112 ./src/llvm-emscripten/test
134188 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134184 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f65jc4mlsp-r58geh-ycmuivyxm2px
121680 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-stage1-codegen
22656 ./src/llvm-emscripten/test/MC
travis_time:end:184c7889:start=1540762238094177208,finish=1540762238574498866,duration=480321658
travis_fold:end:after_failure.1
---
travis_time:end:0064149c:start=1540762238586866839,finish=1540762238590607110,duration=3740271
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00f5a630
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true

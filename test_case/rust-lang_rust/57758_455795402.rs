plain
travis_time:end:09602bcb:start=1547914667750290587,finish=1547914737519412162,duration=69769121575
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:40] 20  | fn main() {
[00:23:40]     |           - un-closed delimiter
[00:23:40] ...
[00:23:40] 269 |             } else {
[00:23:40]     |                    - this delimiter might not be properly closed...
[00:23:40] 270 |                 println!("cargo:rustc-link-lib={}", stdcppname);
[00:23:40] 271 |         }
[00:23:40]     |         - ...as it matches this but it has different indentation
[00:23:40] 281 | }
[00:23:40]     |   ^
[00:23:40] 
[00:23:40] error: aborting due to previous error
---
[00:23:40] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:40] Build completed unsuccessfully in 0:20:26
[00:23:40] Makefile:18: recipe for target 'all' failed
[00:23:40] make: *** [all] Error 1
74508 ./.git/modules/src
73896 ./src/llvm/lib
69912 ./src/llvm-emscripten/lib
69124 ./src/test
---
travis_time:end:279e286c:start=1547916167888057307,finish=1547916167892783822,duration=4726515
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049bd752
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aac56b1
travis_time:start:0aac56b1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0948518a
$ dmesg | grep -i kill

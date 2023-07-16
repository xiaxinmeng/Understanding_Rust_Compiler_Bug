plain
travis_time:end:09e065fb:start=1547750815188444421,finish=1547750817167062490,duration=1978618069
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:23:39] 20  | fn main() {
[00:23:39]     |           - un-closed delimiter
[00:23:39] ...
[00:23:39] 269 |             } else {
[00:23:39]     |                    - this delimiter might not be properly closed...
[00:23:39] 270 |                 println!("cargo:rustc-link-lib={}", stdcppname);
[00:23:39] 271 |         }
[00:23:39]     |         - ...as it matches this but it has different indentation
[00:23:39] 281 | }
[00:23:39]     |   ^
[00:23:39] 
[00:23:39] error: aborting due to previous error
---
[00:23:39] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:39] Build completed unsuccessfully in 0:19:53
[00:23:40] Makefile:18: recipe for target 'all' failed
[00:23:40] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f09bf28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 19:10:47 UTC 2019

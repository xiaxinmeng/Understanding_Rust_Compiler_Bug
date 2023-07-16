plain
travis_time:end:1af483e2:start=1548019910234785183,finish=1548019992659487595,duration=82424702412
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:13:56]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:58] error: unreachable pattern
[00:13:58]     --> src/librustc_metadata/encoder.rs:1224:17
[00:13:58]      |
[00:13:58] 1224 |                 hir::ItemKind::TraitAlias(..) => Some(self.encode_generics(def_id)),
[00:13:58]      |
[00:13:58]      = note: `-D unreachable-patterns` implied by `-D warnings`
[00:13:58] 
[00:13:58] error: aborting due to previous error
---
[00:18:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:32] expected success, got: exit code: 101
[00:18:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:32] Build completed unsuccessfully in 0:15:03
[00:18:32] make: *** [all] Error 1
[00:18:32] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08af59c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 21:51:53 UTC 2019

plain
travis_time:end:1617d87e:start=1555791129192816899,finish=1555791129946635688,duration=753818789
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:48]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:50]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:50]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:50]    Compiling rustc-demangle v0.1.10
[00:04:53] error: item has missing stability attribute
[00:04:53]     |
[00:04:53]     |
[00:04:53] 697 | /     pub fn insert_and_get_key_value(&mut self, key: K, value: V) -> (&K, &V) {
[00:04:53] 698 | |         match self.entry(key) {
[00:04:53] 699 | |             Occupied(mut entry) => {
[00:04:53] 700 | |                 entry.insert(value);
[00:04:53] 708 | |         }
[00:04:53] 709 | |     }
[00:04:53]     | |_____^
[00:04:53] 
[00:04:53] 
[00:04:53] error: item has missing stability attribute
[00:04:53]     |
[00:04:53]     |
[00:04:53] 714 | /     pub fn insert_and_get_mut_key_value(&mut self, key: K, value: V) -> (&K, &mut V) {
[00:04:53] 715 | |         match self.entry(key) {
[00:04:53] 716 | |             Occupied(mut entry) => {
[00:04:53] 717 | |                 entry.insert(value);
[00:04:53] 725 | |         }
[00:04:53] 726 | |     }
[00:04:53]     | |_____^
[00:04:53] 
---
205992 ./obj/build/cache/2019-04-11
157492 ./obj/build/bootstrap/debug/incremental
156496 ./src/llvm-project/clang
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc
142504 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc/s-fbhcnq774a-1ljttzl-3k7gym5px36ja
108532 ./src/llvm-project/lldb
100284 ./.git
97584 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen

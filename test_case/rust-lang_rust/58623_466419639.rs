plain
travis_time:end:08945f30:start=1550841979055942243,finish=1550842064074396060,duration=85018453817
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
################################################                          67.6%
######################################################################## 100.0%
[00:05:45] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:05:45]     Updating crates.io index
[00:05:56]     Updating git repository `https://github.com/Amanieu/hashbrown`
[00:05:58]   Downloaded cmake v0.1.33
[00:05:58]   Downloaded serde_derive v1.0.81
[00:05:58]   Downloaded getopts v0.2.17
[00:05:58]   Downloaded toml v0.4.10
---
[00:08:30]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:08:30]    Compiling rustc-demangle v0.1.10
[00:08:34]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:08:34]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:34]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:08:51] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:08:51] travis_fold:end:stage0-std

[00:08:51] travis_time:end:stage0-std:start=1550842548305667454,finish=1550842603804697380,duration=55499029926
---
[00:31:10]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:31:10]    Compiling rustc-demangle v0.1.10
[00:31:15]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:31:15]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:31:15]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:31:36] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:31:36] travis_fold:end:stage1-std

[00:31:36] travis_time:end:stage1-std:start=1550843900013830045,finish=1550843968736650178,duration=68722820133
---
[01:07:02]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:07:02]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:07:02]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:07:02]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[01:07:02]     Checking hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[01:07:23]     Finished release [optimized] target(s) in 26.61s
[01:07:23] Documenting stage2 test (x86_64-unknown-linux-gnu)
[01:07:24]     Checking getopts v0.2.17
[01:07:24]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
---
[01:12:59] .................................................................................................... 1900/5409
[01:13:02] .................................................................................................... 2000/5409
[01:13:05] ..................................................i................................................. 2100/5409
[01:13:09] .................................................................................................... 2200/5409
[01:13:13] ..................................F................................................................. 2300/5409
[01:13:21] .................................................................................................... 2500/5409
[01:13:25] .................................................................................................... 2600/5409
[01:13:28] .................................................................................................... 2700/5409
[01:13:33] .................................................................................................... 2800/5409
---
[01:15:08] ---- [ui] ui/issues/issue-21763.rs stdout ----
[01:15:08] diff of stderr:
[01:15:08] 
[01:15:08] 6    |
[01:15:08] 7    = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
[01:15:08] 8    = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`
[01:15:08] -    = note: required because of the requirements on the impl of `std::marker::Send` for `std::collections::hash::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`
[01:15:08] +    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`
[01:15:08] +    = note: required because it appears within the type `hashbrown::map::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`
[01:15:08] 10    = note: required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`
[01:15:08] 11 note: required by `foo`
[01:15:08] 
[01:15:08] 
[01:15:08] The actual stderr differed from the expected stderr.
[01:15:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
[01:15:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
[01:15:08] To update references, rerun the tests and pass the `--bless` flag
[01:15:08] To only update this specific test, also pass `--test-args issues/issue-21763.rs`
[01:15:08] error: 1 errors occurred comparing output.
[01:15:08] status: exit code: 1
[01:15:08] status: exit code: 1
[01:15:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21763.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/auxiliary" "-A" "unused"
[01:15:08] ------------------------------------------
[01:15:08] 
[01:15:08] ------------------------------------------
[01:15:08] stderr:
[01:15:08] stderr:
[01:15:08] ------------------------------------------
[01:15:08] {"message":"`std::rc::Rc<()>` cannot be sent between threads safely","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n
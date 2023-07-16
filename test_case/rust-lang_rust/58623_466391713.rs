plain
travis_time:end:00b4c8b5:start=1550836397319031263,finish=1550836695311659100,duration=297992627837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:49] 
######################################################################## 100.0%
[00:02:49] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:49]     Updating crates.io index
[00:03:24]     Updating git repository `https://github.com/Amanieu/hashbrown`
[00:03:25]   Downloaded petgraph v0.4.13
[00:03:25]   Downloaded serde_derive v1.0.81
[00:03:25]   Downloaded time v0.1.40
[00:03:25]   Downloaded getopts v0.2.17
---
[00:06:00]    Compiling rustc-demangle v0.1.10
[00:06:00]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:06:04]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:06:04]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:06:05]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:06:22] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:06:22] travis_fold:end:stage0-std

[00:06:22] travis_time:end:stage0-std:start=1550837029522207392,finish=1550837086562098635,duration=57039891243
---
[00:28:55]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:28:55]    Compiling rustc-demangle v0.1.10
[00:29:01]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:29:01]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:29:01]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:29:22] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:29:22] travis_fold:end:stage1-std

[00:29:22] travis_time:end:stage1-std:start=1550838395939538339,finish=1550838467114885749,duration=71175347410
---
[01:05:13]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:05:13]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:05:13]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:05:13]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[01:05:13]     Checking hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[01:05:36]     Finished release [optimized] target(s) in 29.02s
[01:05:36] Documenting stage2 test (x86_64-unknown-linux-gnu)
[01:05:37]     Checking term v0.0.0 (/checkout/src/libterm)
[01:05:37]     Checking getopts v0.2.17
---
[01:11:19] .................................................................................................... 1900/5409
[01:11:23] .................................................................................................... 2000/5409
[01:11:27] ..................................................i................................................. 2100/5409
[01:11:31] .................................................................................................... 2200/5409
[01:11:35] ..................................F................................................................. 2300/5409
[01:11:44] .................................................................................................... 2500/5409
[01:11:48] .................................................................................................... 2600/5409
[01:11:52] .................................................................................................... 2700/5409
[01:11:57] .................................................................................................... 2800/5409
---
[01:13:42] 
[01:13:42] ---- [ui] ui/issues/issue-21763.rs stdout ----
[01:13:42] diff of stderr:
[01:13:42] 
[01:13:42] 4 LL |     foo::<HashMap<Rc<()>, Rc<()>>>();
[01:13:42] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
[01:13:42] 6    |
[01:13:42] -    = help: within `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
[01:13:42] +    = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
[01:13:42] 8    = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`
[01:13:42] -    = note: required because it appears within the type `std::marker::PhantomData<(std::rc::Rc<()>, std::rc::Rc<()>)>`
[01:13:42] -    = note: required because it appears within the type `std::collections::hash::table::RawTable<std::rc::Rc<()>, std::rc::Rc<()>>`
[01:13:42] +    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`
[01:13:42] +    = note: required because it appears within the type `hashbrown::map::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`
[01:13:42] 11    = note: required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`
[01:13:42] 12 note: required by `foo`
[01:13:42] 
[01:13:42] 
[01:13:42] The actual stderr differed from the expected stderr.
[01:13:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
[01:13:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
[01:13:42] To update references, rerun the tests and pass the `--bless` flag
[01:13:42] To only update this specific test, also pass `--test-args issues/issue-21763.rs`
[01:13:42] error: 1 errors occurred comparing output.
[01:13:42] status: exit code: 1
[01:13:42] status: exit code: 1
[01:13:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21763.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/auxiliary" "-A" "unused"
[01:13:42] ------------------------------------------
[01:13:42] 
[01:13:42] ------------------------------------------
[01:13:42] stderr:
[01:13:42] stderr:
[01:13:42] ------------------------------------------
[01:13:42] {"message":"`std::rc::Rc<()>` cannot be sent between threads safely","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n
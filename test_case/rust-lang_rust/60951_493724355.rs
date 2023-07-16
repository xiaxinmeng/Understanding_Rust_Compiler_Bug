plain
travis_time:end:04844e10:start=1558236819886650896,finish=1558236820629847964,duration=743197068
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:48]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:55] error[E0412]: cannot find type `FnSig` in this scope
[00:08:55]   --> src/librustc/mir/interpret/error.rs:26:31
[00:08:55]    |
[00:08:55] 26 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
[00:08:55] help: possible candidate is found in another module, you can import it into scope
[00:08:55]    |
[00:08:55]    |
[00:08:55] 1  | use crate::ty::sty::FnSig;
[00:08:55] 
[00:08:55] error[E0412]: cannot find type `FnSig` in this scope
[00:08:55]   --> src/librustc/mir/interpret/error.rs:26:44
[00:08:55]    |
[00:08:55]    |
[00:08:55] 26 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
[00:08:55] help: possible candidate is found in another module, you can import it into scope
[00:08:55]    |
[00:08:55]    |
[00:08:55] 1  | use crate::ty::sty::FnSig;
[00:08:55] 
[00:08:56] error[E0412]: cannot find type `AccessKind` in this scope
[00:08:56]   --> src/librustc/mir/interpret/error.rs:65:17
[00:08:56]    |
---
[00:08:56]    |
[00:08:56] 1  | use rustc_data_structures::sync::Lock;
[00:08:56]    |
[00:08:56] 
[00:08:56] error[E0412]: cannot find type `Lrc` in this scope
[00:08:56]    --> src/librustc/mir/interpret/error.rs:111:24
[00:08:56]     |
[00:08:56] 111 |     ReferencedConstant(Lrc<ConstEvalErr<'tcx>>),
[00:08:56] help: possible candidate is found in another module, you can import it into scope
[00:08:56]     |
[00:08:56] 1   | use rustc_data_structures::sync::Lrc;
[00:08:56]     |
[00:08:56]     |
[00:08:56] 
[00:08:57] error: hidden lifetime parameters in types are deprecated
[00:08:57]    --> src/librustc/mir/interpret/error.rs:100:11
[00:08:57]     |
[00:08:57] 100 |     Panic(EvalErrorPanic),
[00:08:57] 
[00:09:00] error[E0106]: missing lifetime specifier
[00:09:00]    --> src/librustc/mir/interpret/error.rs:100:11
[00:09:00]     |
[00:09:00]     |
[00:09:00] 100 |     Panic(EvalErrorPanic),
[00:09:00] 
[00:09:00] error[E0107]: wrong number of type arguments: expected 1, found 0
[00:09:00]    --> src/librustc/mir/interpret/error.rs:100:11
[00:09:00]     |
[00:09:00]     |
[00:09:00] 100 |     Panic(EvalErrorPanic),
[00:09:00] 
[00:09:01] error: aborting due to 12 previous errors
[00:09:01] 
[00:09:01] Some errors occurred: E0106, E0107, E0412.
---
19256 ./src/llvm-project/lldb/www/cpp_reference
19252 ./src/llvm-project/lldb/www/cpp_reference/html
17940 ./src/llvm-project/lldb/www/python_reference
17148 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
travis_time:end:022e862e:start=1558237386506370987,finish=1558237387056180016,dur$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13c43f8a
$ dmesg | grep -i kill

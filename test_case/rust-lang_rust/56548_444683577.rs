plain
travis_time:end:35f709bc:start=1544050359345521619,finish=1544050360566614207,duration=1221092588
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:40]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:56]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:03:56]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:57]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:00] error[E0596]: cannot borrow `iterator` as mutable, as it is not declared as mutable
[00:04:00]      |
[00:04:00] 1735 |         let iterator = iter.into_iter();
[00:04:00]      |             -------- help: consider changing this to be mutable: `mut iterator`
[00:04:00] 1736 | 
[00:04:00] 1736 | 
[00:04:00] 1737 |         match iterator.next() {
[00:04:00]      |               ^^^^^^^^ cannot borrow as mutable
[00:04:00] 
[00:04:00] error[E0596]: cannot borrow `buf` as mutable, as it is not declared as mutable
[00:04:00]      |
[00:04:00] 1739 |             Some(buf) => {
[00:04:00]      |                  --- help: consider changing this to be mutable: `mut buf`
[00:04:00]      |                  --- help: consider changing this to be mutable: `mut buf`
[00:04:00] 1740 |                 buf.extend(iterator);

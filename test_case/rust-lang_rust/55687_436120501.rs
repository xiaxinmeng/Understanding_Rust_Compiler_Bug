plain
travis_time:end:00856f0e:start=1541474090098267045,finish=1541474146686383844,duration=56588116799
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:19:57]    Compiling cmake v0.1.33
[00:19:57]    Compiling std v0.0.0 (/checkout/src/libstd)
e trait `marker::Sized` is not implemented for `Self`
[00:20:03]    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:20:03]    = help: consider adding a `where Self: marker::Sized` bound
[00:20:03]    = note: required because of the requirements on the impl of `iter::traits::IntoIterator` for `Self`
[00:20:03] error[E0277]: `Self` is not an iterator
[00:20:03] error[E0277]: `Self` is not an iterator
[00:20:03]   --> libcore/num/dec2flt/num.rs:49:1
[00:20:03]    |
[00:20:03] 49 | / pub fn from_str_unchecked<'a, T>(bytes: T) -> u64 where T : IntoIterator<Item=&'a u8> {
[00:20:03] 50 | |     let mut result = 0;
[00:20:03] 51 | |     for &c in bytes {
[00:20:03] 52 | |         result = result * 10 + (c - b'0') as u64;
[00:20:03] 54 | |     result
[00:20:0

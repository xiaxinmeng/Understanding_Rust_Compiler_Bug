plain
travis_time:end:00432fe4:start=1557779926159424217,finish=1557779926949083226,duration=789659009
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:30:13]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:31:32]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:38:58]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:38:58]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:38:59] error[E0277]: `?` couldn't convert the error to `!`
[00:38:59]    --> src/librustc_metadata/encoder.rs:106:33
[00:38:59]     |
[00:38:59] 106 |         self.emit_usize(seq.len)?;
[00:38:59]     |                                 |
[00:38:59]     |                                 |
[00:38:59]     |                                 the trait `std::convert::From<()>` is not implemented for `!`
[00:38:59]     |                                 in this expansion of `desugaring of `?``
[00:38:59]     |
[00:38:59]     |
[00:38:59]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:38:59]     = note: required because of the requirements on the impl of `std::convert::Into<!>` for `()`
[00:38:59]     = note: required by `std::convert::Into::into`
[00:38:59] 
[00:38:59] error[E0277]: `?` couldn't convert the error to `!`
[00:38:59]    --> src/librustc_metadata/encoder.rs:129:27
[00:38:59]     |
[00:38:59] 129 |         krate.encode(self)?;
[00:38:59]     |                           |
[00:38:59]     |                           |
[00:38:59]     |                           the trait `std::convert::From<()>` is not implemented for `!`
[00:38:59]     |                           in this expansion of `desugaring of `?``
[00:38:59]     |
[00:38:59]     |
[00:38:59]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:38:59]     = note: required because of the requirements on the impl of `std::convert::Into<!>` for `()`
[00:38:59]     = note: required by `std::convert::Into::into`
[00:38:59] 
[00:38:59] error[E0277]: `?` couldn't convert the error to `!`
[00:38:59]    --> src/librustc_metadata/encoder.rs:164:36
[00:38:59]     |
[00:38:59] 164 |         TAG_VALID_SPAN.encode(self)?;
[00:38:59]     |                                    |
[00:38:59]     |                                    |
[00:38:59]     |                                    the trait `std::convert::From<()>` is not implemented for `!`
[00:38:59]     |                                    in this expansion of `desugaring of `?``
[00:38:59]     |
[00:38:59]     |
[00:38:59]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:38:59]     = note: required because of the requirements on the impl of `std::convert::Into<!>` for `()`
[00:38:59]     = note: required by `std::convert::Into::into`
[00:39:00] error: aborting due to 3 previous errors
[00:39:00] 
[00:39:00] For more information about this error, try `rustc --explain E0277`.
[00:39:00] error: Could not compile `rustc_metadata`.
---
travis_time:end:01fa57df:start=1557782524012055850,finish=1557782524019047332,duration=6991482
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a6a05e1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0560e290
$ dmesg | grep -i kill

plain
travis_time:end:1fa28a86:start=1559830134917475271,finish=1559830135730407821,duration=812932550
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:50]    Compiling cmake v0.1.38
[00:04:51] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:51]    --> src/libcore/ops/range.rs:834:17
[00:04:51]     |
[00:04:51] 834 | impl<T: ?Sized> RangeBounds<T> for (Bound<T>, Bound<T>) {
[00:04:51]     |
[00:04:51]     |
[00:04:51]     = help: within `ops::range::Bound<T>`, the trait `marker::Sized` is not implemented for `T`
[00:04:51]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]     = help: consider adding a `where T: marker::Sized` bound
[00:04:51]     = note: required because it appears within the type `ops::range::Bound<T>`
[00:04:51]     = note: only the last element of a tuple may have a dynamically sized type
[00:04:51] 
[00:04:51] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:51]   --> src/libcore/ops/range.rs:79:5
[00:04:51] 79 |     pub start: Idx,
[00:04:51]    |     ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:04:51]    |
[00:04:51]    = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:51]    = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:51]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]    = help: consider adding a `where Idx: marker::Sized` bound
[00:04:51]    = note: only the last field of a struct may have a dynamically sized type
[00:04:51] 
[00:04:51] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:51]    --> src/libcore/ops/range.rs:336:5
[00:04:51]     |
[00:04:51] 336 |     pub(crate) start: Idx,
[00:04:51]     |
[00:04:51]     = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:51]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]     = help: consider adding a `where Idx: marker::Sized` bound
[00:04:51]     = note: only the last field of a struct may have a dynamically sized type
[00:04:51] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:51]    --> src/libcore/ops/range.rs:690:73
[00:04:51]     |
[00:04:51]     |
[00:04:51] 690 |     Included(#[stable(feature = "collections_bound", since = "1.17.0")] T),
[00:04:51]     |                                                                         ^ doesn't have a size known at compile-time
[00:04:51]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:51]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:51]     = help: consider adding a `where T: marker::Sized` bound
[00:04:51]     = note: no field of an enum variant may have a dynamically sized type
[00:04:52]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:52]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:52] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:52]    --> src/libcore/ops/range.rs:469:5
[00:04:52] 469 | /     pub fn into_inner(self) -> (Idx, Idx) {
[00:04:52] 470 | |         (self.start, self.end)
[00:04:52] 471 | |     }
[00:04:52]     | |_____^ doesn't have a size known at compile-time
[00:04:52]     | |_____^ doesn't have a size known at compile-time
[00:04:52]     |
[00:04:52]     = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:52]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:52]     = help: consider adding a `where Idx: marker::Sized` bound
[00:04:52]     = note: only the last element of a tuple may have a dynamically sized type
[00:04:52] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:52]    --> src/libcore/ops/range.rs:835:5
[00:04:52]     |
[00:04:52]     |
[00:04:52] 835 | /     fn start_bound(&self) -> Bound<&T> {
[00:04:52] 836 | |         match *self {
[00:04:52] 837 | |             (Included(ref start), _) => Included(start),
[00:04:52] 838 | |             (Excluded(ref start), _) => Excluded(start),
[00:04:52] 839 | |             (Unbounded, _) => Unbounded,
[00:04:52] 841 | |     }
[00:04:52]     | |_____^ doesn't have a size known at compile-time
[00:04:52]     |
[00:04:52]     |
[00:04:52]     = help: within `ops::range::Bound<T>`, the trait `marker::Sized` is not implemented for `T`
[00:04:52]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:52]     = help: consider adding a `where T: marker::Sized` bound
[00:04:52]     = note: required because it appears within the type `ops::range::Bound<T>`
[00:04:52]     = note: only the last element of a tuple may have a dynamically sized type
[00:04:52] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:52]    --> src/libcore/ops/range.rs:843:5
[00:04:52]     |
[00:04:52]     |
[00:04:52] 843 | /     fn end_bound(&self) -> Bound<&T> {
[00:04:52] 844 | |         match *self {
[00:04:52] 845 | |             (_, Included(ref end)) => Included(end),
[00:04:52] 846 | |             (_, Excluded(ref end)) => Excluded(end),
[00:04:52] 847 | |             (_, Unbounded) => Unbounded,
[00:04:52] 849 | |     }
[00:04:52]     | |_____^ doesn't have a size known at compile-time
[00:04:52]     |
[00:04:52]     |
[00:04:52]     = help: within `ops::range::Bound<T>`, the trait `marker::Sized` is not implemented for `T`
[00:04:52]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:52]     = help: consider adding a `where T: marker::Sized` bound
[00:04:52]     = note: required because it appears within the type `ops::range::Bound<T>`
[00:04:52]     = note: only the last element of a tuple may have a dynamically sized type
[00:04:53] error: aborting due to 7 previous errors
[00:04:53] 
[00:04:53] For more information about this error, try `rustc --explain E0277`.
[00:04:53] error: Could not compile `core`.
---
travis_time:end:0025c63c:start=1559830446491539029,finish=1559830446496666209,duration=5127180
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2267a507
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10f9a140
travis_time:start:10f9a140
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:

plain
travis_time:end:2a0d9a64:start=1559824974243927155,finish=1559824975034404963,duration=790477808
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:55]    Compiling backtrace-sys v0.1.27
[00:04:55] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:55]    --> src/libcore/ops/range.rs:834:17
[00:04:55]     |
[00:04:55] 834 | impl<T: ?Sized> RangeBounds<T> for (Bound<T>, Bound<T>) {
[00:04:55]     |
[00:04:55]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:55]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:55]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:55]     = help: consider adding a `where T: marker::Sized` bound
[00:04:55] note: required by `ops::range::Bound`
[00:04:55]    --> src/libcore/ops/range.rs:687:1
[00:04:55]     |
[00:04:55] 687 | pub enum Bound<T> {
[00:04:55] 
[00:04:55] 
[00:04:56] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:56]   --> src/libcore/ops/range.rs:79:5
[00:04:56] 79 |     pub start: Idx,
[00:04:56]    |     ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
[00:04:56]    |
[00:04:56]    = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:56]    = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:56]    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:56]    = help: consider adding a `where Idx: marker::Sized` bound
[00:04:56]    = note: only the last field of a struct may have a dynamically sized type
[00:04:56] 
[00:04:56] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:56]    --> src/libcore/ops/range.rs:336:5
[00:04:56]     |
[00:04:56] 336 |     pub(crate) start: Idx,
[00:04:56]     |
[00:04:56]     = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:56]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:56]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:56]     = help: consider adding a `where Idx: marker::Sized` bound
[00:04:56]     = note: only the last field of a struct may have a dynamically sized type
[00:04:57]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:57]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:04:57] error[E0277]: the size for values of type `Idx` cannot be known at compilation time
[00:04:57]    --> src/libcore/ops/range.rs:469:5
[00:04:57] 469 | /     pub fn into_inner(self) -> (Idx, Idx) {
[00:04:57] 470 | |         (self.start, self.end)
[00:04:57] 471 | |     }
[00:04:57]     | |_____^ doesn't have a size known at compile-time
[00:04:57]     | |_____^ doesn't have a size known at compile-time
[00:04:57]     |
[00:04:57]     = help: the trait `marker::Sized` is not implemented for `Idx`
[00:04:57]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:57]     = help: consider adding a `where Idx: marker::Sized` bound
[00:04:57]     = note: only the last element of a tuple may have a dynamically sized type
[00:04:57] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:57]    --> src/libcore/ops/range.rs:835:5
[00:04:57]     |
[00:04:57]     |
[00:04:57] 835 | /     fn start_bound(&self) -> Bound<&T> {
[00:04:57] 836 | |         match *self {
[00:04:57] 837 | |             (Included(ref start), _) => Included(start),
[00:04:57] 838 | |             (Excluded(ref start), _) => Excluded(start),
[00:04:57] 839 | |             (Unbounded, _) => Unbounded,
[00:04:57] 841 | |     }
[00:04:57]     | |_____^ doesn't have a size known at compile-time
[00:04:57]     |
[00:04:57]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:57]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:57]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:57]     = help: consider adding a `where T: marker::Sized` bound
[00:04:57] note: required by `ops::range::Bound`
[00:04:57]    --> src/libcore/ops/range.rs:687:1
[00:04:57]     |
[00:04:57] 687 | pub enum Bound<T> {
[00:04:57] 
[00:04:57] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:04:57]    --> src/libcore/ops/range.rs:843:5
[00:04:57]     |
[00:04:57]     |
[00:04:57] 843 | /     fn end_bound(&self) -> Bound<&T> {
[00:04:57] 844 | |         match *self {
[00:04:57] 845 | |             (_, Included(ref end)) => Included(end),
[00:04:57] 846 | |             (_, Excluded(ref end)) => Excluded(end),
[00:04:57] 847 | |             (_, Unbounded) => Unbounded,
[00:04:57] 849 | |     }
[00:04:57]     | |_____^ doesn't have a size known at compile-time
[00:04:57]     |
[00:04:57]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:57]     = help: the trait `marker::Sized` is not implemented for `T`
[00:04:57]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:04:57]     = help: consider adding a `where T: marker::Sized` bound
[00:04:57] note: required by `ops::range::Bound`
[00:04:57]    --> src/libcore/ops/range.rs:687:1
[00:04:57]     |
[00:04:57] 687 | pub enum Bound<T> {
[00:04:57] 
[00:04:58] error: aborting due to 6 previous errors
[00:04:58] 
[00:04:58] For more information about this error, try `rustc --explain E0277`.

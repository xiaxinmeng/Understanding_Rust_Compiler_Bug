plain
travis_time:end:24647297:start=1559823304760332092,finish=1559823308249546529,duration=3489214437
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:01]    Compiling backtrace-sys v0.1.27
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:784:17
[00:05:02]     |
[00:05:02] 784 | impl<T: ?Sized> RangeBounds<T> for RangeFrom<T> {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::RangeFrom`
[00:05:02]    --> src/libcore/ops/range.rs:181:1
[00:05:02] 181 | pub struct RangeFrom<Idx> {
[00:05:02]     | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:02] 
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:794:17
[00:05:02]     |
[00:05:02] 794 | impl<T: ?Sized> RangeBounds<T> for RangeTo<T> {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::RangeTo`
[00:05:02]    --> src/libcore/ops/range.rs:265:1
[00:05:02] 265 | pub struct RangeTo<Idx> {
[00:05:02]     | ^^^^^^^^^^^^^^^^^^^^^^^
[00:05:02] 
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:804:17
[00:05:02]     |
[00:05:02] 804 | impl<T: ?Sized> RangeBounds<T> for Range<T> {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::Range`
[00:05:02]    --> src/libcore/ops/range.rs:76:1
[00:05:02] 76  | pub struct Range<Idx> {
[00:05:02]     | ^^^^^^^^^^^^^^^^^^^^^
[00:05:02] 
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:814:17
[00:05:02]     |
[00:05:02] 814 | impl<T: ?Sized> RangeBounds<T> for RangeInclusive<T> {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::RangeInclusive`
[00:05:02]    --> src/libcore/ops/range.rs:335:1
[00:05:02] 335 | pub struct RangeInclusive<Idx> {
[00:05:02]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:02] 
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:824:17
[00:05:02]     |
[00:05:02] 824 | impl<T: ?Sized> RangeBounds<T> for RangeToInclusive<T> {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::RangeToInclusive`
[00:05:02]    --> src/libcore/ops/range.rs:606:1
[00:05:02] 606 | pub struct RangeToInclusive<Idx> {
[00:05:02]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:02] 
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:02]    --> src/libcore/ops/range.rs:834:17
[00:05:02]     |
[00:05:02] 834 | impl<T: ?Sized> RangeBounds<T> for (Bound<T>, Bound<T>) {
[00:05:02]     |
[00:05:02]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:02]     = help: consider adding a `where T: marker::Sized` bound
[00:05:02] note: required by `ops::range::Bound`
[00:05:02]    --> src/libcore/ops/range.rs:687:1
[00:05:02]     |
[00:05:02] 687 | pub enum Bound<T> {
[00:05:02] 
[00:05:03]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:785:5
[00:05:03]    --> src/libcore/ops/range.rs:785:5
[00:05:03]     |
[00:05:03] 785 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 786 | |         Included(&self.start)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeFrom`
[00:05:03]    --> src/libcore/ops/range.rs:181:1
[00:05:03] 181 | pub struct RangeFrom<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:788:5
[00:05:03]     |
[00:05:03] 788 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 790 | |     }
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeFrom`
[00:05:03]    --> src/libcore/ops/range.rs:181:1
[00:05:03] 181 | pub struct RangeFrom<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:795:5
[00:05:03]     |
[00:05:03] 795 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 797 | |     }
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeTo`
[00:05:03]    --> src/libcore/ops/range.rs:265:1
[00:05:03] 265 | pub struct RangeTo<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:798:5
[00:05:03]     |
[00:05:03] 798 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 799 | |         Excluded(&self.end)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeTo`
[00:05:03]    --> src/libcore/ops/range.rs:265:1
[00:05:03] 265 | pub struct RangeTo<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:805:5
[00:05:03]     |
[00:05:03] 805 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 806 | |         Included(&self.start)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::Range`
[00:05:03]    --> src/libcore/ops/range.rs:76:1
[00:05:03] 76  | pub struct Range<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:808:5
[00:05:03]     |
[00:05:03] 808 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 809 | |         Excluded(&self.end)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::Range`
[00:05:03]    --> src/libcore/ops/range.rs:76:1
[00:05:03] 76  | pub struct Range<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:815:5
[00:05:03]     |
[00:05:03] 815 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 816 | |         Included(&self.start)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeInclusive`
[00:05:03]    --> src/libcore/ops/range.rs:335:1
[00:05:03] 335 | pub struct RangeInclusive<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:818:5
[00:05:03]     |
[00:05:03] 818 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 819 | |         Included(&self.end)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeInclusive`
[00:05:03]    --> src/libcore/ops/range.rs:335:1
[00:05:03] 335 | pub struct RangeInclusive<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:825:5
[00:05:03]     |
[00:05:03] 825 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 827 | |     }
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeToInclusive`
[00:05:03]    --> src/libcore/ops/range.rs:606:1
[00:05:03] 606 | pub struct RangeToInclusive<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:828:5
[00:05:03]     |
[00:05:03] 828 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 829 | |         Included(&self.end)
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::RangeToInclusive`
[00:05:03]    --> src/libcore/ops/range.rs:606:1
[00:05:03] 606 | pub struct RangeToInclusive<Idx> {
[00:05:03]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:835:5
[00:05:03]     |
[00:05:03] 835 | /     fn start_bound(&self) -> Bound<&T> {
[00:05:03] 836 | |         match *self {
[00:05:03] 837 | |             (Included(ref start), _) => Included(start),
[00:05:03] 838 | |             (Excluded(ref start), _) => Excluded(start),
[00:05:03] 839 | |             (Unbounded, _) => Unbounded,
[00:05:03] 841 | |     }
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::Bound`
[00:05:03]    --> src/libcore/ops/range.rs:687:1
[00:05:03]     |
[00:05:03] 687 | pub enum Bound<T> {
[00:05:03] 
[00:05:03] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:05:03]    --> src/libcore/ops/range.rs:843:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 843 | /     fn end_bound(&self) -> Bound<&T> {
[00:05:03] 844 | |         match *self {
[00:05:03] 845 | |             (_, Included(ref end)) => Included(end),
[00:05:03] 846 | |             (_, Excluded(ref end)) => Excluded(end),
[00:05:03] 847 | |             (_, Unbounded) => Unbounded,
[00:05:03] 849 | |     }
[00:05:03]     | |_____^ doesn't have a size known at compile-time
[00:05:03]     |
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = help: the trait `marker::Sized` is not implemented for `T`
[00:05:03]     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:05:03]     = help: consider adding a `where T: marker::Sized` bound
[00:05:03] note: required by `ops::range::Bound`
[00:05:03]    --> src/libcore/ops/range.rs:687:1
[00:05:03]     |
[00:05:03] 687 | pub enum Bound<T> {
[00:05:03] 
[00:05:04] error: aborting due to 18 previous errors
[00:05:04] 
[00:05:04] For more information about this error, try `rustc --explain E0277`.

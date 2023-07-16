plain
travis_time:end:03d5aeb5:start=1554489103055250039,finish=1554489105398275083,duration=2343025044
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:27]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:27]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a [T]>>::Searcher: needle::needle::Searcher<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 |  / macro_rules! forward_to_needle_api {
[00:04:28] 3791 |  |     ($(
[00:04:28] 3792 |  |         $(#[doc = $doc:tt])+
[00:04:28] 3793 |  |         #[self($stable:meta)]
[00:04:28] ...     |
[00:04:28] 3802 | /|         pub $struct $name<'a, T, P>
[00:04:28] 3803 | ||         where
[00:04:28] 3804 | ||             P: Needle<$hs>,
[00:04:28] 3805 | ||         {
[00:04:28] 3806 | ||             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28] 3807 | ||         }
[00:04:28]      | ||_________^ the trait `needle::needle::Searcher<[T]>` is not implemented for `<P as needle::needle::Needle<&'a [T]>>::Searcher`
[00:04:28] 3878 |  |
[00:04:28] 3879 |  | }
[00:04:28] 3879 |  | }
[00:04:28]      |  |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | /  forward_to_needle_api! {
[00:04:28] 3882 | |      /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |      ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |      struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28] 3971 | |  }
[00:04:28]      | |__- in this macro invocation
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a [T]>>::Searcher: needle::needle::Searcher<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a [T]>>::Consumer: needle::needle::Consumer<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 |  / macro_rules! forward_to_needle_api {
[00:04:28] 3791 |  |     ($(
[00:04:28] 3792 |  |         $(#[doc = $doc:tt])+
[00:04:28] 3793 |  |         #[self($stable:meta)]
[00:04:28] ...     |
[00:04:28] 3802 | /|         pub $struct $name<'a, T, P>
[00:04:28] 3803 | ||         where
[00:04:28] 3804 | ||             P: Needle<$hs>,
[00:04:28] 3805 | ||         {
[00:04:28] 3806 | ||             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28] 3807 | ||         }
[00:04:28]      | ||_________^ the trait `needle::needle::Consumer<[T]>` is not implemented for `<P as needle::needle::Needle<&'a [T]>>::Consumer`
[00:04:28] 3878 |  |
[00:04:28] 3879 |  | }
[00:04:28] 3879 |  | }
[00:04:28]      |  |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | /  forward_to_needle_api! {
[00:04:28] 3882 | |      /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |      ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |      struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28] 3971 | |  }
[00:04:28]      | |__- in this macro invocation
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a [T]>>::Consumer: needle::needle::Consumer<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a [T]>>::Searcher: needle::needle::Searcher<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 | / macro_rules! forward_to_needle_api {
[00:04:28] 3791 | |     ($(
[00:04:28] 3792 | |         $(#[doc = $doc:tt])+
[00:04:28] 3793 | |         #[self($stable:meta)]
[00:04:28] ...    |
[00:04:28] 3806 | |             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28]      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `needle::needle::Searcher<[T]>` is not implemented for `<P as needle::needle::Needle<&'a [T]>>::Searcher`
[00:04:28] 3878 | |
[00:04:28] 3879 | | }
[00:04:28] 3879 | | }
[00:04:28]      | |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | / forward_to_needle_api! {
[00:04:28] 3882 | |     /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |     ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |     struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28]      | |_- in this macro invocation
[00:04:28]      |
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a [T]>>::Searcher: needle::needle::Searcher<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a [T]>>::Consumer: needle::needle::Consumer<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 | / macro_rules! forward_to_needle_api {
[00:04:28] 3791 | |     ($(
[00:04:28] 3792 | |         $(#[doc = $doc:tt])+
[00:04:28] 3793 | |         #[self($stable:meta)]
[00:04:28] ...    |
[00:04:28] 3806 | |             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28]      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `needle::needle::Consumer<[T]>` is not implemented for `<P as needle::needle::Needle<&'a [T]>>::Consumer`
[00:04:28] 3878 | |
[00:04:28] 3879 | | }
[00:04:28] 3879 | | }
[00:04:28]      | |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | / forward_to_needle_api! {
[00:04:28] 3882 | |     /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |     ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |     struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28]      | |_- in this macro invocation
[00:04:28]      |
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a [T]>>::Consumer: needle::needle::Consumer<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a mut [T]>>::Searcher: needle::needle::Searcher<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 |  / macro_rules! forward_to_needle_api {
[00:04:28] 3791 |  |     ($(
[00:04:28] 3792 |  |         $(#[doc = $doc:tt])+
[00:04:28] 3793 |  |         #[self($stable:meta)]
[00:04:28] ...     |
[00:04:28] 3802 | /|         pub $struct $name<'a, T, P>
[00:04:28] 3803 | ||         where
[00:04:28] 3804 | ||             P: Needle<$hs>,
[00:04:28] 3805 | ||         {
[00:04:28] 3806 | ||             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28] 3807 | ||         }
[00:04:28]      | ||_________^ the trait `needle::needle::Searcher<[T]>` is not implemented for `<P as needle::needle::Needle<&'a mut [T]>>::Searcher`
[00:04:28] 3878 |  |
[00:04:28] 3879 |  | }
[00:04:28] 3879 |  | }
[00:04:28]      |  |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | /  forward_to_needle_api! {
[00:04:28] 3882 | |      /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |      ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |      struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28] 3971 | |  }
[00:04:28]      | |__- in this macro invocation
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a mut [T]>>::Searcher: needle::needle::Searcher<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a mut [T]>>::Consumer: needle::needle::Consumer<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 |  / macro_rules! forward_to_needle_api {
[00:04:28] 3791 |  |     ($(
[00:04:28] 3792 |  |         $(#[doc = $doc:tt])+
[00:04:28] 3793 |  |         #[self($stable:meta)]
[00:04:28] ...     |
[00:04:28] 3802 | /|         pub $struct $name<'a, T, P>
[00:04:28] 3803 | ||         where
[00:04:28] 3804 | ||             P: Needle<$hs>,
[00:04:28] 3805 | ||         {
[00:04:28] 3806 | ||             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28] 3807 | ||         }
[00:04:28]      | ||_________^ the trait `needle::needle::Consumer<[T]>` is not implemented for `<P as needle::needle::Needle<&'a mut [T]>>::Consumer`
[00:04:28] 3878 |  |
[00:04:28] 3879 |  | }
[00:04:28] 3879 |  | }
[00:04:28]      |  |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | /  forward_to_needle_api! {
[00:04:28] 3882 | |      /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |      ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |      struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28] 3971 | |  }
[00:04:28]      | |__- in this macro invocation
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a mut [T]>>::Consumer: needle::needle::Consumer<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a mut [T]>>::Searcher: needle::needle::Searcher<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 | / macro_rules! forward_to_needle_api {
[00:04:28] 3791 | |     ($(
[00:04:28] 3792 | |         $(#[doc = $doc:tt])+
[00:04:28] 3793 | |         #[self($stable:meta)]
[00:04:28] ...    |
[00:04:28] 3806 | |             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28]      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `needle::needle::Searcher<[T]>` is not implemented for `<P as needle::needle::Needle<&'a mut [T]>>::Searcher`
[00:04:28] 3878 | |
[00:04:28] 3879 | | }
[00:04:28] 3879 | | }
[00:04:28]      | |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | / forward_to_needle_api! {
[00:04:28] 3882 | |     /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |     ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |     struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28]      | |_- in this macro invocation
[00:04:28]      |
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a mut [T]>>::Searcher: needle::needle::Searcher<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
[00:04:28] 
[00:04:28] error[E0277]: the trait bound `<P as needle::needle::Needle<&'a mut [T]>>::Consumer: needle::needle::Consumer<[T]>` is not satisfied
[00:04:28]      |
[00:04:28]      |
[00:04:28] 3790 | / macro_rules! forward_to_needle_api {
[00:04:28] 3791 | |     ($(
[00:04:28] 3792 | |         $(#[doc = $doc:tt])+
[00:04:28] 3793 | |         #[self($stable:meta)]
[00:04:28] ...    |
[00:04:28] 3806 | |             inner: ext::$ext_name<$hs, P::Searcher>,
[00:04:28]      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `needle::needle::Consumer<[T]>` is not implemented for `<P as needle::needle::Needle<&'a mut [T]>>::Consumer`
[00:04:28] 3878 | |
[00:04:28] 3879 | | }
[00:04:28] 3879 | | }
[00:04:28]      | |_- in this expansion of `forward_to_needle_api!`
[00:04:28] 3880 | 
[00:04:28] 3881 | / forward_to_needle_api! {
[00:04:28] 3882 | |     /// An iterator over subslices separated by elements that match a predicate
[00:04:28] 3884 | |     ///
[00:04:28] ...    |
[00:04:28] ...    |
[00:04:28] 3970 | |     struct RSplitNMut<Searcher = ReverseSearcher, double_ended = false>(RSplit<&'a mut [T]>);
[00:04:28]      | |_- in this macro invocation
[00:04:28]      |
[00:04:28]      |
[00:04:28]      = help: consider adding a `where <P as needle::needle::Needle<&'a mut [T]>>::Consumer: needle::needle::Consumer<[T]>` bound
[00:04:28] note: required by `needle::needle::Needle`
[00:04:28]     --> src/libcore/needle/needle.rs:488:1
[00:04:28]      |
[00:04:28] 488  | / pub trait Needle<H: Haystack>: Sized
[00:04:28] 489  | | where H::Target: Hay // FIXME: RFC 2089 or 2289
[00:04:28] 490  | | {
[00:04:28] 491  | |     /// The searcher associated with this needle.
[00:04:28] 510  | |     fn into_consumer(self) -> Self::Consumer;
[00:04:28] 511  | | }
[00:04:28]      | |_^
[00:04:28] 
---
travis_time:end:050dbf14:start=1554489387755333673,finish=1554489387760689259,duration=5355586
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bbb6e48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f0a33d0
travis_time:start:1f0a33d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10baea3e
$ dmesg | grep -i kill

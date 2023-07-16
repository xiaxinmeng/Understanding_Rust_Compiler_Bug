plain
travis_time:end:02f2ff0d:start=1554110687908288597,finish=1554110688767636698,duration=859348101
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:14]    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14]     | |_- in this expansion of `count!`
[00:05:14] 738 | 
[00:05:14] 739 | / macro_rules! tuple {
[00:05:14] 740 | |     () => ();
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | |_- in this expansion of `tuple!`
[00:05:14]     | |_- in this expansion of `tuple!`
[00:05:14] 771 | 
[00:05:14] 772 |   tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     |   ------------------------------------------------------------ in this macro invocation
[00:05:14]     = note: `-D unused-parens` implied by `-D warnings`
[00:05:14] 
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:736:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:14] error: unnecessary parentheses around assigned value
[00:05:14]    --> src/libserialize/serialize.rs:735:32
[00:05:14]     |
[00:05:14]     |
[00:05:14] 722 | / macro_rules! peel {
[00:05:14] 723 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:05:14] 724 | | }
[00:05:14] 724 | | }
[00:05:14]     | |_- in this expansion of `peel!` (#2)
[00:05:14] 732 | / macro_rules! count {
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 733 | |     ()                     => (0usize);
[00:05:14] 734 | |     ($one:tt)              => (1usize);
[00:05:14] 735 | |     ($($pairs:tt $_p:tt)*) => ((count!($($pairs)*) << 1usize));
[00:05:14]     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
[00:05:14] 736 | |     ($odd:tt $($rest:tt)*) => ((count!($($rest)*) | 1usize));
[00:05:14] 737 | | }
[00:05:14]     | |_- in this expansion of `count!` (#4)
[00:05:14] 739 |   macro_rules! tuple {
[00:05:14]     |  _-
[00:05:14]     | |_|
[00:05:14]     | |
[00:05:14]     | |
[00:05:14] 740 | |     () => ();
[00:05:14] 741 | |     ( $($name:ident,)+ ) => (
[00:05:14] 742 | |         impl<$($name:Decodable),*> Decodable for ($($name,)*) {
[00:05:14] ...   |
[00:05:14] 745 | |                 let len: usize = count!($($name)*);
[00:05:14] ...   |
[00:05:14] ...   |
[00:05:14] 768 | |         peel! { $($name,)* }
[00:05:14] 769 | |     )
[00:05:14] 770 | | }
[00:05:14]     | | -
[00:05:14]     | |_|
[00:05:14]     | |_|
[00:05:14]     | |_in this expansion of `tuple!` (#1)
[00:05:14]     |   in this expansion of `tuple!` (#3)
[00:05:14] 771 | 
[00:05:14] 772 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:05:14]     | | ------------------------------------------------------------ in this macro invocation (#1)
[00:05:16] error: aborting due to 11 previous errors
[00:05:16] 
[00:05:16] error: Could not compile `serialize`.
[00:05:16] warning: build failed, waiting for other jobs to finish...

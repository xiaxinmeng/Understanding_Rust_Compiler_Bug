plain
travis_time:end:06d88562:start=1561211289165061878,finish=1561211379672885803,duration=90507823925
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:04]    Compiling autocfg v0.1.4
[00:04:04] error[E0658]: The attribute `sse4a_target_feature` is currently unknown to the compiler and may have meaning added to it in the future
[00:04:04]    --> src/libcore/lib.rs:127:24
[00:04:04]     |
[00:04:04] 127 | #![cfg_attr(bootstrap, sse4a_target_feature)]
[00:04:04]     |                        ^^^^^^^^^^^^^^^^^^^^ help: a built-in attribute with a similar name exists: `target_feature`
[00:04:04]     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
[00:04:04]     = help: add #![feature(custom_attribute)] to the crate attributes to enable
[00:04:04] 
[00:04:04] 
[00:04:04] error[E0658]: The attribute `tbm_target_feature` is currently unknown to the compiler and may have meaning added to it in the future
[00:04:04]     |
[00:04:04]     |
[00:04:04] 128 | #![cfg_attr(bootstrap, tbm_target_feature)]
[00:04:04]     |                        ^^^^^^^^^^^^^^^^^^ help: a built-in attribute with a similar name exists: `target_feature`
[00:04:04]     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
[00:04:04]     = help: add #![feature(custom_attribute)] to the crate attributes to enable
[00:04:04] 
[00:04:04] error[E0658]: The attribute `adx_target_feature` is currently unknown to the compiler and may have meaning added to it in the future
[00:04:04] error[E0658]: The attribute `adx_target_feature` is currently unknown to the compiler and may have meaning added to it in the future
[00:04:04]    --> src/libcore/lib.rs:129:24
[00:04:04]     |
[00:04:04] 129 | #![cfg_attr(bootstrap, adx_target_feature)]
[00:04:04]     |                        ^^^^^^^^^^^^^^^^^^ help: a built-in attribute with a similar name exists: `target_feature`
[00:04:04]     = note: for more information, see https://github.com/rust-lang/rust/issues/29642
[00:04:04]     = help: add #![feature(custom_attribute)] to the crate attributes to enable
[00:04:04] 
[00:04:04]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:04]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:05]    Compiling backtrace v0.3.29
[00:04:06] error: cannot determine resolution for the derive macro `Debug`
[00:04:06]   --> src/libcore/num/dec2flt/rawfp.rs:30:23
[00:04:06]    |
[00:04:06] 30 | #[derive(Copy, Clone, Debug)]
[00:04:06]    |
[00:04:06]    = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]    --> src/libcore/marker.rs:646:62
[00:04:06]     |
[00:04:06] 646 | #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
[00:04:06]     |
[00:04:06]     = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]   --> src/libcore/ops/range.rs:42:38
[00:04:06]    |
[00:04:06] 42 | #[derive(Copy, Clone, PartialEq, Eq, Hash)]
[00:04:06]    |
[00:04:06]    = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]   --> src/libcore/ops/range.rs:74:32
[00:04:06]    |
[00:04:06] 74 | #[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
[00:04:06]    |
[00:04:06]    = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]    --> src/libcore/ops/range.rs:179:32
[00:04:06]     |
[00:04:06] 179 | #[derive(Clone, PartialEq, Eq, Hash)] // not Copy -- see #27186
[00:04:06]     |
[00:04:06]     = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]    --> src/libcore/ops/range.rs:263:38
[00:04:06]     |
[00:04:06] 263 | #[derive(Copy, Clone, PartialEq, Eq, Hash)]
[00:04:06]     |
[00:04:06]     = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]    --> src/libcore/ops/range.rs:604:38
[00:04:06]     |
[00:04:06] 604 | #[derive(Copy, Clone, PartialEq, Eq, Hash)]
[00:04:06]     |
[00:04:06]     = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06] error: cannot determine resolution for the derive macro `Hash`
[00:04:06]    --> src/libcore/ops/range.rs:686:30
[00:04:06]     |
[00:04:06] 686 | #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
[00:04:06]     |
[00:04:06]     = note: import resolution is stuck, try simplifying macro imports
[00:04:06] 
[00:04:06] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:188:5
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:188:5
[00:04:07]     |
[00:04:07] 188 |       debug_assert_eq!(plus.e, minus.e);
[00:04:07]     |
[00:04:07]     |
[00:04:07]     = note: #[deny(macro_expanded_macro_exports_accessed_by_absolute_paths)] on by default
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:189:5
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:189:5
[00:04:07]     |
[00:04:07] 189 |       debug_assert_eq!(plus.e, v.e);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:268:13
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:268:13
[00:04:07]     |
[00:04:07] 268 |               debug_assert_eq!(ten_kappa, 1);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:269:13
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:269:13
[00:04:07]     |
[00:04:07] 269 |               debug_assert_eq!(kappa, 0);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:514:13
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:514:13
[00:04:07]     |
[00:04:07] 514 |               debug_assert_eq!(ten_kappa, 1);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:515:13
[00:04:07]    ::: src/libcore/num/flt2dec/strategy/grisu.rs:515:13
[00:04:07]     |
[00:04:07] 515 |               debug_assert_eq!(kappa, 0);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/num/flt2dec/mod.rs:633:17
[00:04:07]    ::: src/libcore/num/flt2dec/mod.rs:633:17
[00:04:07]     |
[00:04:07] 633 |                   debug_assert_eq!(len, 0);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/slice/sort.rs:345:9
[00:04:07]    ::: src/libcore/slice/sort.rs:345:9
[00:04:07]     |
[00:04:07] 345 |           debug_assert_eq!(width(l, r), block_l);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]    ::: src/libcore/slice/sort.rs:357:9
[00:04:07]    ::: src/libcore/slice/sort.rs:357:9
[00:04:07]     |
[00:04:07] 357 |           debug_assert_eq!(width(l, r), block_r);
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:07] error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
[00:04:07]     |
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 210 | / macro_rules! debug_assert_eq {
[00:04:07] 211 | |     ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
[00:04:07] 212 | | }
[00:04:07]     | |_- in this expansion of `debug_assert_eq!`
[00:04:07]     | 
[00:04:07]     | 
[00:04:07]    ::: src/libcore/hash/sip.rs:130:5
[00:04:07] 130 |       debug_assert_eq!(i, len);
[00:04:07]     |       ------------------------- in this macro invocation
[00:04:07]     |
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:07]     = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
[00:04:07] note: the macro is defined here
[00:04:07]    --> src/libcore/macros.rs:45:1
[00:04:07]     |
[00:04:07] 45  | / macro_rules! assert_eq {
[00:04:07] 46  | |     ($left:expr, $right:expr) => ({
[00:04:07] 47  | |         match (&$left, &$right) {
[00:04:07] 48  | |             (left_val, right_val) => {
[00:04:07] 77  | |     });
[00:04:07] 78  | | }
[00:04:07]     | |_^
[00:04:07] 
[00:04:07] 
[00:04:09] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:38:18
[00:04:09]    |
[00:04:09] 38 | #[target_feature(enable = "sse4a")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:55:18
[00:04:09]    |
[00:04:09] 55 | #[target_feature(enable = "sse4a")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:66:18
[00:04:09]    |
[00:04:09] 66 | #[target_feature(enable = "sse4a")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:77:18
[00:04:09]    |
[00:04:09] 77 | #[target_feature(enable = "sse4a")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:71:18
[00:04:09]    |
[00:04:09] 71 | #[target_feature(enable = "tbm")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:82:18
[00:04:09]    |
[00:04:09] 82 | #[target_feature(enable = "tbm")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:94:18
[00:04:09]    |
[00:04:09] 94 | #[target_feature(enable = "tbm")]
[00:04:09]    |
[00:04:09]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:105:18
[00:04:09]     |
[00:04:09] 105 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:117:18
[00:04:09]     |
[00:04:09] 117 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:128:18
[00:04:09]     |
[00:04:09] 128 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:141:18
[00:04:09]     |
[00:04:09] 141 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:153:18
[00:04:09]     |
[00:04:09] 153 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:165:18
[00:04:09]     |
[00:04:09] 165 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 
[00:04:09] 
[00:04:09] error[E0658]: the target feature `tbm` is currently unstable
[00:04:09]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:176:18
[00:04:09]     |
[00:04:09] 176 | #[target_feature(enable = "tbm")]
[00:04:09]     |
[00:04:09]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:09]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:09] 

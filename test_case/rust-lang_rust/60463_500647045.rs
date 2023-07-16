plain
travis_time:end:029ba130:start=1560208756719858608,finish=1560208757559025175,duration=839166567
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:28] 
[01:06:28] running 144 tests
[01:06:30] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:06:32] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:32] 
[01:06:32]  finished in 4.696
[01:06:32] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:34] 
[01:06:34] running 9 tests
[01:06:34] iiiiiiiii
[01:06:34] 
[01:06:34]  finished in 0.153
[01:06:34] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:51] 
[01:06:51] running 122 tests
[01:07:16] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:21] .i.i......iii.i.....ii
[01:07:21] 
[01:07:21]  finished in 30.125
[01:07:21] travis_fold:end:test_debuginfo

---
[01:46:24] ---- /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md - The_tracking_issue_for_this_feature_is_ (line 14) stdout ----
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:17:1
[01:46:24]   |
[01:46:24] 5 | / enum SingleFieldEnum {
[01:46:24] 6 | |     Variant(f32)
[01:46:24]   | |_^
[01:46:24]   |
[01:46:24]   = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]   = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24]   = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24] 
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:23:1
[01:46:24]    |
[01:46:24] 11 | / enum MultiFieldEnum {
[01:46:24] 12 | |     Variant { field: usize, nothing: () },
[01:46:24]    | |_^
[01:46:24]    |
[01:46:24]    = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]    = help: add #![feature(transparent_enums)] to the crate attributes to enable
---
[01:46:24] ---- /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md - The_tracking_issue_for_this_feature_is_ (line 32) stdout ----
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:40:1
[01:46:24]    |
[01:46:24] 10 | / pub enum BadEnum {
[01:46:24] 11 | |     Nothing(()),
[01:46:24]    | |_^
[01:46:24]    |
[01:46:24]    = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]    = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24]    = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24] 
[01:46:24] error[E0690]: the variant of a transparent enum needs exactly one non-zero-sized field, but has 0
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:40:1
[01:46:24]    |
[01:46:24] 10 | / pub enum BadEnum {
[01:46:24] 11 | |     Nothing(()),
[01:46:24]    | |_^
[01:46:24] 
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:46:1
---
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:69:1
[01:46:24]   |
[01:46:24] 4 |   #[repr(transparent)]
[01:46:24]   |   ^^^^^^^^^^^^^^^^^^^^
[01:46:24] 5 | / pub enum TooFewVariants {
[01:46:24]   | |_- zero-variant enum
[01:46:24] 
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:70:1
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:70:1
[01:46:24]   |
[01:46:24] 5 | / pub enum TooFewVariants {
[01:46:24]   | |_^
[01:46:24]   |
[01:46:24]   = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]   = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24]   = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24] 
[01:46:24] error[E0731]: transparent enum needs exactly one variant, but has 0
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:70:1
[01:46:24]   |
[01:46:24] 5 | / pub enum TooFewVariants {
[01:46:24]   | |_^
[01:46:24] 
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:75:1
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:75:1
[01:46:24]    |
[01:46:24] 10 | / pub enum TooManyVariants {
[01:46:24] 11 | |     First(usize),
[01:46:24] 13 | | }
[01:46:24]    | |_^
[01:46:24]    |
[01:46:24]    = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]    = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]    = help: add #![feature(transparent_enums)] to the crate attributes to enable
[01:46:24] 
[01:46:24] error[E0731]: transparent enum needs exactly one variant, but has 2
[01:46:24]   --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:75:1
[01:46:24]    |
[01:46:24] 10 | / pub enum TooManyVariants {
[01:46:24] 11 | |     First(usize),
[01:46:24] 13 | | }
[01:46:24]    | |_^
[01:46:24]    |
[01:46:24]    |
[01:46:24] note: the following variants exist on `main::TooManyVariants`
[01:46:24]    |
[01:46:24] 11 |     First(usize),
[01:46:24]    |     ^^^^^^^^^^^^
[01:46:24] 12 |     Second,
---
[01:46:24] ---- /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md - The_tracking_issue_for_this_feature_is_ (line 54) stdout ----
[01:46:24] error[E0658]: transparent enums are unstable
[01:46:24]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:57:1
[01:46:24]   |
[01:46:24] 5 | / pub enum GenericEnum<T> {
[01:46:24] 6 | |     Variant(T, ()),
[01:46:24]   | |_^
[01:46:24]   |
[01:46:24]   = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:46:24]   = help: add #![feature(transparent_enums)] to the crate attributes to enable

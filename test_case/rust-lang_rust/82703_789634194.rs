plain
.................................................................................................... 9300/11524
.................................................................................................... 9400/11524
.........................................................................i.......i.................. 9500/11524
.................................................................................................... 9600/11524
............iiiiiii..iiiiii.i....................................................................... 9700/11524
.................................................................................................... 9900/11524
.................................................................................................... 10000/11524
.................................................................................................... 10100/11524
.................................................................................................... 10200/11524
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.076 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.422 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1300/2876
.................................................................................................... 1400/2876
.................................................................................................... 1500/2876
.................................................................................................... 1600/2876
.....................................................................F..F.F.FFF..FFF..FF...F.F.FF.FF 1700/2876
FF.FF.FF.F.FFF.F.F.F.FF..FFFF....................................................................... 1800/2876
.................................................................................................... 2000/2876
.................................................................................................... 2100/2876
.................................................................................................... 2200/2876
.................................................................................................... 2300/2876
---
......i.....................i.....................i.....................i.....................i..... 2800/2876
............................................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(Some(two), one.checked_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(None, max.checked_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(Some(four), two.checked_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(None, max.checked_mul(two));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::checked_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(Some(twenty_seven), three.checked_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(None, half_max.checked_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(two, one.saturating_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(max, max.saturating_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(four, two.saturating_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(max, four.saturating_mul(max));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU128::saturating_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(twenty_seven, three.saturating_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(max, max.saturating_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(Some(two), one.checked_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(None, max.checked_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(Some(twenty_seven), three.checked_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(None, half_max.checked_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::checked_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(Some(four), two.checked_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(None, max.checked_mul(two));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(two, one.saturating_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(max, max.saturating_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(four, two.saturating_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(max, four.saturating_mul(max));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::saturating_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(twenty_seven, three.saturating_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(max, max.saturating_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(Some(two), one.checked_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(None, max.checked_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(Some(four), two.checked_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(None, max.checked_mul(two));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::checked_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(Some(twenty_seven), three.checked_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(None, half_max.checked_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(twenty_seven, three.saturating_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(max, max.saturating_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(four, two.saturating_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(max, four.saturating_mul(max));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(Some(two), one.checked_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(None, max.checked_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::saturating_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(two, one.saturating_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(max, max.saturating_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(Some(four), two.checked_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(None, max.checked_mul(two));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::checked_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(Some(twenty_seven), three.checked_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(None, half_max.checked_pow(3));
   |
   |
   = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_add (line 351) stdout ----
error[E0658]: use of unstable library feature 'nonzero_add'
  |
  |
9 | assert_eq!(two, one.saturating_add(1));
  |
  |
  = help: add `#![feature(nonzero_add)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_add'
   |
   |
10 | assert_eq!(max, max.saturating_add(1));
   |
   |
   = help: add `#![feature(nonzero_add)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_mul (line 418) stdout ----
error[E0658]: use of unstable library feature 'nonzero_mul'
  |
  |
9 | assert_eq!(four, two.saturating_mul(two));
  |
  |
  = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_mul'
   |
   |
10 | assert_eq!(max, four.saturating_mul(max));
   |
   |
   = help: add `#![feature(nonzero_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::saturating_pow (line 483) stdout ----
error[E0658]: use of unstable library feature 'nonzero_pow'
  |
  |
9 | assert_eq!(twenty_seven, three.saturating_pow(3));
  |
  |
  = help: add `#![feature(nonzero_pow)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'nonzero_pow'
   |
   |
10 | assert_eq!(max, max.saturating_pow(3));
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:12

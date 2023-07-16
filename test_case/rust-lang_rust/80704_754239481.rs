plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.24s

 finished in 2.320 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/infer-static-outlives-requirements.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/inline-const.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/language-features/inline-const.md" "--test-args" ""

stdout ----

running 3 tests
running 3 tests
test /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 32) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 21) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 10) ... ok

failures:

---- /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 32) stdout ----
warning: the feature `inline_const` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:32:12
2 | #![feature(inline_const)]
  |            ^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76001 <https://github.com/rust-lang/rust/issues/76001> for more information

error[E0284]: type annotations needed
 --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:39:15
  |
9 |     const { 1 + 2 } => println!("Matched 1 + 2"),
  |               ^ cannot infer type for type `{integer}`
  |
  = note: cannot satisfy `<{integer} as Add<{integer}>>::Output == {integer}`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0284`.
Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/inline-const.md - The_tracking_issue_for_this_feature_is__ (line 21) stdout ----
warning: the feature `inline_const` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:22:12
2 | #![feature(inline_const)]
  |            ^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76001 <https://github.com/rust-lang/rust/issues/76001> for more information

error[E0284]: type annotations needed
 --> /checkout/src/doc/unstable-book/src/language-features/inline-const.md:26:31
  |
6 |     let x = add_one(const { 1 + 2 * 3 / 4 });
  |                               ^ cannot infer type for type `{integer}`
  |
  = note: cannot satisfy `<{integer} as Add<{integer}>>::Output == {integer}`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0284`.
Couldn't compile the test.

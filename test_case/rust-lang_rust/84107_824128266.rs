plain
.................................................................................................... 9400/11763
.................................................................................................... 9500/11763
............................................................................................i....... 9600/11763
i................................................................................................... 9700/11763
......................................iiiiiii..iiiiii.i............................................. 9800/11763
.................................................................................................... 10000/11763
.................................................................................................... 10100/11763
.................................................................................................... 10200/11763
.................................................................................................... 10300/11763
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.154 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.403 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/library-features/format-args-capture.md
doc tests for: /checkout/src/doc/unstable-book/src/library-features/global-asm.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/library-features/global-asm.md" "--test-args" ""

stdout ----

running 4 tests
running 4 tests
test /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 24) ... ignored
test /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 77) ... FAILED
test /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 32) ... ok
test /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 94) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 77) stdout ----
error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:79:1
4 | global_asm!(
  | ^^^^^^^^^^
  |
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = help: add `#![feature(global_asm)]` to the crate attributes to enable
error: non-statement macro in statement position: global_asm
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:79:1
  |
  |
4 | / global_asm!(
5 | |     ".global bar",
6 | |     "bar: .word {c}",
7 | |     c = const C,
  | |__^

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 94) stdout ----
error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:95:1
  |
3 | global_asm!("movl ${}, %ecx", const 5, options(att_syntax));
  |
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = help: add `#![feature(global_asm)]` to the crate attributes to enable
error: non-statement macro in statement position: global_asm
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:95:1
  |
  |
3 | global_asm!("movl ${}, %ecx", const 5, options(att_syntax));


error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:97:1
  |
5 | global_asm!("mov ecx, {}", const 5);
  |
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = help: add `#![feature(global_asm)]` to the crate attributes to enable
error: non-statement macro in statement position: global_asm
 --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:97:1
  |
  |
5 | global_asm!("mov ecx, {}", const 5);

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.

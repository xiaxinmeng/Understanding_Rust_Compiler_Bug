plain
.................................................................................................... 9400/11722
.................................................................................................... 9500/11722
................................................................i......i............................ 9600/11722
.................................................................................................... 9700/11722
..........iiiiiii..iiiiii.i......................................................................... 9800/11722
.................................................................................................... 10000/11722
.................................................................................................... 10100/11722
.................................................................................................... 10200/11722
.................................................................................................... 10300/11722
---
 finished in 0.422 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.113 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.28s

 finished in 2.343 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"src/doc"}) not skipped for "bootstrap::doc::Standalone" -- not in ["src/tools/tidy"]
Documenting stage2 std (x86_64-unknown-linux-gnu)
Set({"library/alloc", "library/core", "library/panic_abort", "library/panic_unwind", "library/proc_macro", "library/std", "library/term", "library/test", "library/unwind"}) not skipped for "bootstrap::doc::Std" -- not in ["src/tools/tidy"]
 Documenting core v0.0.0 (/checkout/library/core)
error: documentation for `MIN` links to item `i128::MIN` which will not have documentation generated
   |
   |
6  | /         #[doc = concat!(
7  | |             "The smallest value that can be represented by this integer type. Use ",
8  | |             "[`", stringify!($T), "::MIN", "`] instead."
9  | |         )]
19 | |         /// 
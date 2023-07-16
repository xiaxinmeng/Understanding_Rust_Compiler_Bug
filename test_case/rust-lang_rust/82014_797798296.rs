plain
.................................................................................................... 9300/11545
.................................................................................................... 9400/11545
..........................................................................i......i.................. 9500/11545
.................................................................................................... 9600/11545
.....................iiiiiii.iiiiii.i............................................................... 9700/11545
.................................................................................................... 9900/11545
.................................................................................................... 10000/11545
.................................................................................................... 10100/11545
.................................................................................................... 10200/11545
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.058 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i....iii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.422 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"src/doc"}) not skipped for "bootstrap::doc::Standalone" -- not in ["src/tools/tidy"]
Set({"library/alloc", "library/core", "library/panic_abort", "library/panic_unwind", "library/proc_macro", "library/std", "library/term", "library/test", "library/unwind"}) not skipped for "bootstrap::doc::Std" -- not in ["src/tools/tidy"]
Documenting stage2 std (x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: documentation for `MIN` links to item `i128::MIN` which will not have documentation generated
   |
   |
6  | /         #[doc = concat!(
7  | |             "The smallest value that can be represented by this integer type. Use ",
8  | |             "[`", stringify!($T), "::MIN", "`] instead."
9  | |         )]
19 | |         /// 
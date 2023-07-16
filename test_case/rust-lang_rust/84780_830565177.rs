plain
.................................................................................................... 9400/11812
.................................................................................................... 9500/11812
.................................................................................................... 9600/11812
..................................i......i.......................................................... 9700/11812
................................................................................iiiiiii..iiiiii.i... 9800/11812
.................................................................................................... 10000/11812
.................................................................................................... 10100/11812
.................................................................................................... 10200/11812
.................................................................................................... 10300/11812
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.175 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.008 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.475 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
test builder::tests::dist::test_with_no_doc_stage0 ... ok

failures:

---- builder::tests::dist::dist_only_cross_host stdout ----
thread 'main' panicked at 'assertion failed: !use_snapshot || stage == 0 || self.local_rebuild', src/bootstrap/builder.rs:1024:9


failures:
    builder::tests::dist::dist_only_cross_host

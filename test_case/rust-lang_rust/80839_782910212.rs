plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.069 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i.....ii.........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.435 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiii.........

 finished in 0.608 seconds
Build completed successfully in 0:32:10
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
 finished in 2.163 seconds
Build completed successfully in 0:01:58
+ python2.7 ../x.py --stage 2 test src/test/ui --pass=check --target=i686-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
thread 'main' panicked at 'Linkcheck currently does not support builds with different hosts and targets.
You can skip linkcheck with --exclude src/tools/linkchecker', src/bootstrap/test.rs:134:13
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:00:00

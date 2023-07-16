plain
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
................................................................................iiiiii..iiiiii.i.... 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.068 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.37s

 finished in 2.438 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling rand_xorshift v0.2.0
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: call to noop method
    --> library/alloc/src/collections/btree/map/tests.rs:1636:14
     |
1636 |     a.insert(key.clone(), value.clone());
     |
     |
     = note: `-D noop-method-call` implied by `-D warnings`

error: call to noop method
    --> library/alloc/src/collections/btree/map/tests.rs:1636:27
     |
1636 |     a.insert(key.clone(), value.clone());


error: call to noop method
    --> library/alloc/src/collections/btree/map/tests.rs:1640:19
     |
1640 |     match a.entry(key.clone()) {


error: call to noop method
    --> library/alloc/src/collections/btree/map/tests.rs:1656:19
     |
1656 |     match a.entry(key.clone()) {


error: call to noop method
    --> library/alloc/src/collections/btree/map/tests.rs:1660:22
     |
1660 |             e.insert(value.clone());

error: aborting due to 5 previous errors

error: could not compile `alloc`
error: could not compile `alloc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:00

plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 53 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.i..Fiiii...........ii...iiiiiii.iiii...............

---- [run-make] src/test/run-make/const_fn_mir stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2022-04-29 11:49:53.837235899 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2022-04-29 12:23:06.413702946 +0000
@@ -23,10 +23,6 @@
         _0 = move (_1.0: i32);           // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
-
-
-    bb2 (cleanup): {
-        resume;                          // scope 0 at main.rs:4:1: 6:2
 }
 
 fn main() -> () {
------------------------------------------
------------------------------------------
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [Makefile:5: all] Error 1



failures:

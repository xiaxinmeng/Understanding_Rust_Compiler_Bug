plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
diff -u "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2"
--- /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1 2022-04-02 22:33:23.818036379 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2 2022-04-02 22:33:24.074039341 +0000
@@ -1,5 +1,5 @@
 -m64
-/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/rustc7diUY1/symbols.o: 13188796287369540740
+/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/rustcfaJC5k/symbols.o: 13188796287369540740
 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.99fb15fb-cgu.0.rcgu.o: 15740180347004849190
 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.99fb15fb-cgu.1.rcgu.o: 8640485513335145010
 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.99fb15fb-cgu.2.rcgu.o: 15719987896575683549
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:31: smoke] Error 1



failures:

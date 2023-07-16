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
--- /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1 2022-03-28 21:15:29.166655203 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2 2022-03-28 21:15:29.446655247 +0000
@@ -1,4 +1,4 @@
-/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/rustceuvcP1/forcelink.ld: 5325194396573141467
+/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/rustcOTsm1W/forcelink.ld: 5325194396573141467
 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.99fb15fb-cgu.0.rcgu.o: 17935545186115717501
 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.99fb15fb-cgu.1.rcgu.o: 12472619423567504001
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:31: smoke] Error 1



failures:

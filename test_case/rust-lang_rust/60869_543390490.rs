
---- [run-make] run-make-fulldeps/reproducible-build stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
rm -rf /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
diff -u "/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1" "/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2"
--- /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1	2019-10-18 09:31:45.248924701 +1100
+++ /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2	2019-10-18 09:31:45.456923894 +1100
@@ -5,7 +5,7 @@
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.0.rcgu.o: 9024332235029870339
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.1.rcgu.o: 8252971569739609253
-/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.2.rcgu.o: 3534221490709993710
+/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.2.rcgu.o: 1155913470043241769
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.3.rcgu.o: 10237418006570950210
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.4.rcgu.o: 15369966276953066318
 /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/reproducible-build.reproducible_build.7rcbfp3g-cgu.5.rcgu.o: 320128326741088814

------------------------------------------
stderr:
------------------------------------------
make: *** [Makefile:21: smoke] Error 1

------------------------------------------


---- [run-make] run-make-fulldeps/reproducible-build-2 stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
rm -rf /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 && mkdir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build-aux.rs
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs -C lto=fat
cp /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build-a
LD_LIBRARY_PATH="/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage0/lib:/home/njn/local/lib:" '/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs -C lto=fat
cmp "/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build-a" "/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build" || exit 1
/home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build-a /home/njn/moz/rust3/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build differ: byte 781, line 1

------------------------------------------
stderr:
------------------------------------------
make: *** [Makefile:17: fat_lto] Error 1

------------------------------------------

failures:
    [run-make] run-make-fulldeps/reproducible-build
    [run-make] run-make-fulldeps/reproducible-build-2

test result: FAILED. 0 passed; 2 failed; 199 ignored; 0 measured; 0 filtered out


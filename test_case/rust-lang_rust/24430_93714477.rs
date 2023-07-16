
maketest: trace-macros-flag
----- /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-make/trace-macros-flag/ --------------------
------ stdout ---------------------------------------------
make[1]: Entering directory `/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-make/trace-macros-flag'
LD_LIBRARY_PATH="/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag:/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/stage2/lib:/home/rustbuild/gcc-4.7.4/lib64:/home/rustbuild/gcc-4.7.4/lib" /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/stage2/bin/rustc --out-dir /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag -L /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag -Z trace-macros hello.rs &> /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag/hello.out
diff -u /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag/hello.out hello.trace
--- /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/obj/x86_64-unknown-linux-gnu/test/run-make/trace-macros-flag/hello.out  2015-04-16 11:27:50.638159976 +0000
+++ hello.trace 2015-04-16 10:35:56.314159976 +0000
@@ -0,0 +1,2 @@
+println! { "Hello, World!" }
+print! { concat ! ( "Hello, World!" , "\n" ) }
make[1]: Leaving directory `/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-make/trace-macros-flag'
println! { "Hello, World!" }
print! { concat ! ( "Hello, World!" , "\n" ) }

------ stderr ---------------------------------------------
make[1]: *** [all] Error 1


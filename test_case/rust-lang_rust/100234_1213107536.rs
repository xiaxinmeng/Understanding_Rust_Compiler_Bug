plain
 ---> 2afb3e7bef8f
Step 3/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   python3.9   git   cmake   sudo   gdb   llvm-12-tools   llvm-12-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
 ---> Using cache
 ---> 357fae1e02d2
Step 4/8 : RUN apt-get update &&     apt-get install -y apt-transport-https software-properties-common &&     curl -s "https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb" > packages-microsoft-prod.deb &&     dpkg -i packages-microsoft-prod.deb &&     apt-get update &&     apt-get install -y powershell
 ---> 080d1843107f
Step 5/8 : COPY scripts/sccache.sh /scripts/
 ---> Using cache
 ---> 4a6c76c56ba3
---

---- [ui] src/test/ui/proc-macro/span-absolute-posititions.rs stdout ----
diff of stderr:

- error: line/column mismatch: (0, 35) != (21, 35)
+ error: line/column mismatch: (5, 35) != (5, 36)
+    |
+    |
+ LL | assert_span_pos::assert_span_pos!(5, 35);
+ 
+ 
+ error: line/column mismatch: (8, 39) != (8, 40)
+    |
+    |
+ LL |     assert_span_pos::assert_span_pos!(8, 39);
+ 
+ 
+ error: line/column mismatch: (10, 36) != (10, 37)
+    |
+    |
+ LL |     assert_span_pos::assert_span_pos!(10, 36);
+ 
+ 
+ error: line/column mismatch: (16, 40) != (16, 41)
+    |
+    |
+ LL | /*🌈*/assert_span_pos::assert_span_pos!(16, 40);
+ 
+ 
+ error: line/column mismatch: (18, 43) != (18, 44)
+    |
+    |
+ LL | /*🏳️🌈*/assert_span_pos::assert_span_pos!(18, 43);
+ 
+ 
+ error: line/column mismatch: (0, 35) != (21, 36)
3    |
3    |
4 LL | assert_span_pos::assert_span_pos!(0, 35);
5    |                                   ^
6 
6 
- error: line/column mismatch: (22, 0) != (22, 35)
+ error: line/column mismatch: (22, 0) != (22, 36)
9    |
9    |
10 LL | assert_span_pos::assert_span_pos!(22, 0);
11    |                                   ^^
12 
- error: aborting due to 2 previous errors
+ error: aborting due to 7 previous errors
---
To only update this specific test, also pass `--test-args proc-macro/span-absolute-posititions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/span-absolute-posititions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-absolute-posititions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-absolute-posititions/auxiliary"
stdout: none
--- stderr -------------------------------
error: line/column mismatch: (5, 35) != (5, 36)
   |
   |
LL | assert_span_pos::assert_span_pos!(5, 35);


error: line/column mismatch: (8, 39) != (8, 40)
   |
   |
LL |     assert_span_pos::assert_span_pos!(8, 39);


error: line/column mismatch: (10, 36) != (10, 37)
   |
   |
LL |     assert_span_pos::assert_span_pos!(10, 36);


error: line/column mismatch: (16, 40) != (16, 41)
   |
   |
LL | /*🌈*/assert_span_pos::assert_span_pos!(16, 40);


error: line/column mismatch: (18, 43) != (18, 44)
   |
   |
LL | /*🏳️🌈*/assert_span_pos::assert_span_pos!(18, 43);

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: line/column mismatch: (0, 35) != (21, 36)
   |
   |
LL | assert_span_pos::assert_span_pos!(0, 35); //~ ERROR line/column mismatch: (0, 35) != (21, 35)


error: line/column mismatch: (22, 0) != (22, 36)
   |
   |
LL | assert_span_pos::assert_span_pos!(22, 0); //~ ERROR line/column mismatch: (22, 0) != (22, 35)

error: aborting due to 7 previous errors
------------------------------------------


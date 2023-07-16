plain
........................................................................................ 4664/13094
........................................................................................ 4752/13094
........................................................................................ 4840/13094
........................................................................................ 4928/13094
................................................................................i..F.... 5016/13094
........................................................................................ 5192/13094
........................................................................................ 5280/13094
........................................................................................ 5368/13094
........................................................................................ 5456/13094
---
- error[E0158]: const parameters cannot be referenced in patterns
+ error: constant pattern depends on a generic parameter
2   --> $DIR/const-match-pat-generic.rs:8:9
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
4 LL |         const { V } => {},
11    |         ^^^^^^^^^^^^^^
12 
13 error: constant pattern depends on a generic parameter
+   --> $DIR/const-match-pat-generic.rs:8:9
+   --> $DIR/const-match-pat-generic.rs:8:9
+    |
+ LL |         const { V } => {},
+ 
+ error: constant pattern depends on a generic parameter
14   --> $DIR/const-match-pat-generic.rs:20:9
15    |
15    |
16 LL |         const { f(V) } => {},
17    |         ^^^^^^^^^^^^^^
18 
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args inline-const/const-match-pat-generic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-match-pat-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-generic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-generic/auxiliary"
stdout: none
--- stderr -------------------------------
error: constant pattern depends on a generic parameter
   |
   |
LL |         const { V } => {},

error: constant pattern depends on a generic parameter
  --> /checkout/src/test/ui/inline-const/const-match-pat-generic.rs:20:9
   |
   |
LL |         const { f(V) } => {},

error: constant pattern depends on a generic parameter
  --> /checkout/src/test/ui/inline-const/const-match-pat-generic.rs:8:9
   |
   |
LL |         const { V } => {},

error: constant pattern depends on a generic parameter
  --> /checkout/src/test/ui/inline-const/const-match-pat-generic.rs:20:9
   |
   |
LL |         const { f(V) } => {},

error: aborting due to 4 previous errors
------------------------------------------


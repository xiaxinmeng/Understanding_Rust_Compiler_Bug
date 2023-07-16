plain

4 LL |     it();
5    |     ^^^^^
6    |
+    = note: iterators are lazy and do nothing unless consumed
8   --> $DIR/unused-supertrait.rs:1:9
9    |

10 LL | #![deny(unused_must_use)]
10 LL | #![deny(unused_must_use)]
11    |         ^^^^^^^^^^^^^^^
-    = note: iterators are lazy and do nothing unless consumed
14 error: aborting due to previous error
15 


---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-supertrait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-supertrait/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused implementer of `Iterator` that must be used
   |
LL |     it();
   |     ^^^^^
   |
   |
   = note: iterators are lazy and do nothing unless consumed
  --> /checkout/src/test/ui/lint/unused/unused-supertrait.rs:1:9
   |
LL | #![deny(unused_must_use)]
   |         ^^^^^^^^^^^^^^^

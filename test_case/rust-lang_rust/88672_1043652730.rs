plain

---- [ui] ui/associated-types/issue-36499.rs stdout ----
diff of stderr:

- error: leading `+` is not supported
-   --> $DIR/issue-36499.rs:4:9
+ error: Rust has no postfix increment operator
3    |
4 LL |     2 + +2;
-    |         ^ unexpected `+`
+    |       ^^^ not a valid postfix operator
+    |       ^^^ not a valid postfix operator
6    |
- help: try removing the `+`
+ help: use `+= 1` instead
8    |
+ LL |     { let tmp = 2 ; 2 += 1; tmp }2;
+    |     +++++++++++   ~~~~~~~~~~~~~~~
+ help: or, if you don't need to use it as an expression, change it to this
9 LL -     2 + +2;
- LL +     2 + 2;
- LL +     2 + 2;
+ LL +     2  += 12;
12 
13 error: aborting due to previous error


---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-36499.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-36499" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-36499/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/associated-types/issue-36499.rs:4:7
   |
LL |     2 + +2;
   |       ^^^ not a valid postfix operator
   |
help: use `+= 1` instead
   |
LL |     { let tmp = 2 ; 2 += 1; tmp }2;
   |     +++++++++++   ~~~~~~~~~~~~~~~
help: or, if you don't need to use it as an expression, change it to this
LL -     2 + +2;
LL +     2  += 12;
   | 


error: aborting due to previous error


------------------------------------------


---- [ui] ui/parser/increment-autofix.rs stdout ----
thread '[ui] ui/parser/increment-autofix.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/parser/increment-autofix.rs" with rustfix: Cannot replace slice of data that was already replaced', src/tools/compiletest/src/runtest.rs:3131:17
---- [ui] ui/parser/issues/issue-88276-unary-plus.rs stdout ----
diff of stderr:

10 LL +     let _ = 1;
10 LL +     let _ = 1;
11    | 
12 
- error: leading `+` is not supported
-   --> $DIR/issue-88276-unary-plus.rs:5:20
+ error: Rust has no postfix increment operator
15    |
15    |
16 LL |     let _ = (1.0 + +2.0) * +3.0;
-    |                    ^ unexpected `+`
+    |                  ^^^ not a valid postfix operator
- help: try removing the `+`
- help: try removing the `+`
+ help: use `+= 1` instead
20    |
+ LL |     let _ = ({ let tmp = 1.0 ; 1.0 += 1; tmp }2.0) * +3.0;
+    |              +++++++++++     ~~~~~~~~~~~~~~~~~
+ help: or, if you don't need to use it as an expression, change it to this
+    |
21 LL -     let _ = (1.0 + +2.0) * +3.0;
- LL +     let _ = (1.0 + 2.0) * +3.0;
+ LL +     let _ = (1.0  += 12.0) * +3.0;
24 
24 
25 error: leading `+` is not supported

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-88276-unary-plus/issue-88276-unary-plus.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-88276-unary-plus/issue-88276-unary-plus.stderr
thread '[ui] ui/parser/issues/issue-88276-unary-plus.rs' panicked at 'failed to apply suggestions for "/checkout/src/test/ui/parser/issues/issue-88276-unary-plus.rs" with rustfix: Cannot replace slice of data that was already replaced', src/tools/compiletest/src/runtest.rs:3131:17

failures:
    [ui] ui/associated-types/issue-36499.rs
    [ui] ui/parser/increment-autofix.rs

plain
---- [ui] ui/parser/issues/issue-15980.rs stdout ----
diff of stderr:

- error: expected identifier, found keyword `return`
-   --> $DIR/issue-15980.rs:8:13
+ error: expected one of `!`, `.`, `::`, `=>`, `?`, or an operator, found `{`
3    |
3    |
4 LL |         Err(ref e) if e.kind == io::EndOfFile {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                                 ------------- while parsing this struct
- LL |
- LL |             return
-    |             ^^^^^^ expected identifier, found keyword
+    |                                               ^ expected one of `!`, `.`, `::`, `=>`, `?`, or an operator
9    |
- help: escape `return` to use it as an identifier
+ help: try adding a fat arrow here
11    |
- LL |             r#return
-    |             ++
+ LL |         Err(ref e) if e.kind == io::EndOfFile => {
14 
14 
- error: expected one of `.`, `=>`, `?`, or an operator, found reserved identifier `_`
-   --> $DIR/issue-15980.rs:13:9
- LL |         }
- LL |         }
-    |          - expected one of `.`, `=>`, `?`, or an operator
- LL |
- LL |         _ => {}
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
25 
---
To only update this specific test, also pass `--test-args parser/issues/issue-15980.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-15980.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-15980" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-15980/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `!`, `.`, `::`, `=>`, `?`, or an operator, found `{`
   |
   |
LL |         Err(ref e) if e.kind == io::EndOfFile {
   |                                               ^ expected one of `!`, `.`, `::`, `=>`, `?`, or an operator
   |
help: try adding a fat arrow here
   |
LL |         Err(ref e) if e.kind == io::EndOfFile => {

error: aborting due to previous error
------------------------------------------



---- [ui] ui/parser/struct-literal-in-match-guard.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/struct-literal-in-match-guard.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-in-match-guard" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-in-match-guard/auxiliary"
stdout: none
--- stderr -------------------------------
error: struct literals are not allowed here
   |
   |
LL |         () if f == Foo { x: 42 } => {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |         () if f == (Foo { x: 42 }) => {}

error: aborting due to previous error
------------------------------------------


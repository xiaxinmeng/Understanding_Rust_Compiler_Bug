plain
To only update this specific test, also pass `--test-args closures/old-closure-expression-remove-semicolon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/old-closure-expression-remove-semicolon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/old-closure-expression-remove-semicolon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/old-closure-expression-remove-semicolon/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/old-closure-expression-remove-semicolon.rs:8:19
   |
LL |       let _x: i32 = {
   |  ___________________^
   |  ___________________^
LL | |         //~^ ERROR mismatched types
LL | |         foo(); //~ HELP remove this semicolon
   | |              - help: remove this semicolon to return this value
LL | |     };
   | |_____^ expected `i32`, found `()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
27 LL | (), w20);
-    |         ^ help: remove this semicolon to return this value
+    |         ^ help: remove this semicolon
29 
30 error[E0412]: cannot find type `isizee` in this scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62895/issue-62895.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62895/issue-62895.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-62895.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-62895.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62895" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62895/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found reserved identifier `_`
   |
   |
LL | mod _ { //~ ERROR expected identifier

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/parser/issues/issue-62895.rs:6:5
   |
   |
LL | mod _ { //~ ERROR expected identifier

error: missing `fn` for function definition
  --> /checkout/src/test/ui/parser/issues/issue-62895.rs:7:4
   |
   |
LL | pub    g() -> is //~ ERROR missing `fn` for function definition
   |
   |
help: add `fn` here to parse `g` as a public function
   |
LL | pub fn g() -> is //~ ERROR missing `fn` for function definition

error: expected item, found `;`
  --> /checkout/src/test/ui/parser/issues/issue-62895.rs:10:9
   |
   |
LL | (), w20); //~ ERROR expected item, found `;`


error[E0412]: cannot find type `isizee` in this scope
   |
   |
LL | pub fn g() -> isizee { //~ ERROR cannot find type `isizee` in this scope
   |               ^^^^^^ help: a builtin type with a similar name exists: `isize`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/issues/issue-62895.rs:3:11
   |
   |
LL | fn v() -> isize { //~ ERROR mismatched types
   |    -      ^^^^^ expected `isize`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0412.
For more information about an error, try `rustc --explain E0308`.

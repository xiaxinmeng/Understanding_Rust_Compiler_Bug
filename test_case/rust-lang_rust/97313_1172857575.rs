plain

---- [ui] src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs stdout ----
diff of stderr:

33 LL | fn bar<'b, L: X<'b, &'b Nested<i32>>>(){}
35 
35 
- error[E0477]: the type `&'b Nested<K>` does not fulfill the required lifetime
-   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:19
+ error[E0477]: the type `&'b Nested<i32>` does not fulfill the required lifetime
38    |
38    |
- LL |     fn foo<'b, L: X<&'b Nested<K>>>();
-    |                   ^^^^^^^^^^^^^^^^
+ LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
41    |
42 note: type must satisfy the static lifetime as required by this binding
43   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:8:16


45 LL | trait X<'a, K: 'a> {
47 
47 
- error[E0477]: the type `&'b Nested<i32>` does not fulfill the required lifetime
-   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:14:15
+ error[E0477]: the type `&'b Nested<K>` does not fulfill the required lifetime
50    |
50    |
- LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
-    |               ^^^^^^^^^^^^^^^^^^
+ LL |     fn foo<'b, L: X<&'b Nested<K>>>();
53    |
54 note: type must satisfy the static lifetime as required by this binding
55   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:8:16

---
To only update this specific test, also pass `--test-args generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | fn should_error<T>() where T : Into<&u32> {}
   |                                     ^ explicit lifetime name needed here
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:20
   |
   |
LL |     fn foo<'b, L: X<&'b Nested<K>>>();
   |                    ^ expected named lifetime parameter
   |
note: these named lifetimes are available to use
   |
   |
LL | trait X<'a, K: 'a> {
   |         ^^
LL |     fn foo<'b, L: X<&'b Nested<K>>>();
help: consider using one of the available lifetimes here
   |
   |
LL |     fn foo<'b, L: X<'lifetime, &'b Nested<K>>>();

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:14:16
   |
   |
LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
   |                ^ expected named lifetime parameter
   |
help: consider using the `'b` lifetime
   |
LL | fn bar<'b, L: X<'b, &'b Nested<i32>>>(){}


error[E0477]: the type `&'b Nested<i32>` does not fulfill the required lifetime
   |
   |
LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
   |
note: type must satisfy the static lifetime as required by this binding
  --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:8:16
   |
   |
LL | trait X<'a, K: 'a> {


error[E0477]: the type `&'b Nested<K>` does not fulfill the required lifetime
   |
   |
LL |     fn foo<'b, L: X<&'b Nested<K>>>();
   |
note: type must satisfy the static lifetime as required by this binding
  --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:8:16
   |
   |
LL | trait X<'a, K: 'a> {

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0106, E0477, E0637.

plain
........................................................................................ 1232/14189
........................................................................................ 1320/14189
.................................................................................i...... 1408/14189
........................................................................................ 1496/14189
......F.................F................................i.............................. 1584/14189
........................................................................................ 1760/14189
........................................................................................ 1848/14189
.....i...................i...........ii................................................. 1936/14189
........................................................................................ 2024/14189
---
---- [ui] checkout/tests/ui/cast/cast-as-bool.rs stdout ----
diff of stderr:

18    |
19 help: consider using the `is_empty` method on `&'static str` to determine if it contains anything
20    |
- LL -     let v = "hello" as bool;
- LL +     let v = !"hello".is_empty();
-    |
+ LL |     let v = !"hello".is_empty();
24 
25 error: aborting due to 3 previous errors
26 

---
To only update this specific test, also pass `--test-args cast/cast-as-bool.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/cast/cast-as-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-as-bool" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-as-bool/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0054]: cannot cast as `bool`
   |
   |
LL |     let u = 5 as bool; //~ ERROR cannot cast as `bool`
   |             ^^^^^^^^^ help: compare with zero instead: `5 != 0`
error[E0054]: cannot cast as `bool`
  --> /checkout/tests/ui/cast/cast-as-bool.rs:6:13
   |
   |
LL |     let t = (1 + 2) as bool; //~ ERROR cannot cast as `bool`
   |             ^^^^^^^^^^^^^^^ help: compare with zero instead: `(1 + 2) != 0`

error[E0606]: casting `&'static str` as `bool` is invalid
   |
   |
LL |     let v = "hello" as bool;
   |
   |
help: consider using the `is_empty` method on `&'static str` to determine if it contains anything
   |
LL |     let v = !"hello".is_empty();

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0054, E0606.
Some errors have detailed explanations: E0054, E0606.
For more information about an error, try `rustc --explain E0054`.
------------------------------------------


---- [ui] checkout/tests/ui/cast/issue-106883-is-empty.rs stdout ----

6    |
6    |
7 help: consider using the `is_empty` method on `&'static str` to determine if it contains anything
8    |
- LL -     let _ = "foo" as bool;
- LL +     let _ = !"foo".is_empty();
-    |
+ LL |     let _ = !"foo".is_empty();
12 
12 
13 error[E0605]: non-primitive cast: `String` as `bool`
14   --> $DIR/issue-106883-is-empty.rs:17:13
23    |             ^^^^^^^^^^^^^^^^^^^
23    |             ^^^^^^^^^^^^^^^^^^^
24 help: consider using the `is_empty` method on `String` to determine if it contains anything
25    |
- LL -     let _ = String::from("foo") as bool;
- LL +     let _ = !String::from("foo").is_empty();
-    |
+ LL |     let _ = !String::from("foo").is_empty();
29 
29 
30 error[E0605]: non-primitive cast: `Foo` as `bool`
31   --> $DIR/issue-106883-is-empty.rs:20:13
40    |             ^^^
40    |             ^^^
41 help: consider using the `is_empty` method on `Foo` to determine if it contains anything
42    |
- LL -     let _ = Foo as bool;
- LL +     let _ = !Foo.is_empty();
-    |
+ LL |     let _ = !Foo.is_empty();
+    |             +   ~~~~~~~~~~~
46 
47 error[E0606]: casting `&[i32]` as `bool` is invalid
48   --> $DIR/issue-106883-is-empty.rs:25:5
52    |
52    |
53 help: consider using the `is_empty` method on `&[i32]` to determine if it contains anything
54    |
- LL -     bar as bool
- LL +     !bar.is_empty()
-    |
+ LL |     !bar.is_empty()
+    |     +   ~~~~~~~~~~~
59 error: aborting due to 4 previous errors
60 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-106883-is-empty/issue-106883-is-empty.stderr
To only update this specific test, also pass `--test-args cast/issue-106883-is-empty.rs`

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/cast/issue-106883-is-empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-106883-is-empty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-106883-is-empty/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0606]: casting `&'static str` as `bool` is invalid
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:14:13
   |
LL |     let _ = "foo" as bool;
   |
   |
help: consider using the `is_empty` method on `&'static str` to determine if it contains anything
   |
LL |     let _ = !"foo".is_empty();


error[E0605]: non-primitive cast: `String` as `bool`
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:17:13
   |
LL |     let _ = String::from("foo") as bool;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
   |
note: this expression `Deref`s to `str` which implements `is_empty`
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:17:13
   |
LL |     let _ = String::from("foo") as bool;
   |             ^^^^^^^^^^^^^^^^^^^
help: consider using the `is_empty` method on `String` to determine if it contains anything
   |
LL |     let _ = !String::from("foo").is_empty();


error[E0605]: non-primitive cast: `Foo` as `bool`
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:20:13
LL |     let _ = Foo as bool;
LL |     let _ = Foo as bool;
   |             ^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
   |
note: this expression `Deref`s to `[u8]` which implements `is_empty`
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:20:13
LL |     let _ = Foo as bool;
   |             ^^^
   |             ^^^
help: consider using the `is_empty` method on `Foo` to determine if it contains anything
   |
LL |     let _ = !Foo.is_empty();
   |             +   ~~~~~~~~~~~

error[E0606]: casting `&[i32]` as `bool` is invalid
  --> /checkout/tests/ui/cast/issue-106883-is-empty.rs:25:5
LL |     bar as bool
   |     ^^^^^^^^^^^
   |
   |
help: consider using the `is_empty` method on `&[i32]` to determine if it contains anything
   |
LL |     !bar.is_empty()
   |     +   ~~~~~~~~~~~
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0605, E0606.
For more information about an error, try `rustc --explain E0605`.

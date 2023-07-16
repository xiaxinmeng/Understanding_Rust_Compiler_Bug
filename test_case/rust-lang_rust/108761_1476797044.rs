plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 2376/14677
........................................................................................ 2464/14677
........................................................................................ 2552/14677
........................................................................................ 2640/14677
.......................F............F................................................... 2728/14677
........................................................................................ 2904/14677
........................................................................................ 2992/14677
........................................................................................ 3080/14677
........................................................................................ 3168/14677
---

2   --> $DIR/unused-substs-3.rs:16:9
3    |
4 LL |     t = foo;
-    |         ^^^ cyclic type of infinite size
- help: try using a conversion method
-    |
- LL |     t = foo.to_vec();
-    |            +++++++++
-    |            +++++++++
- LL |     t = foo.to_vec_co();
-    |            ++++++++++++
+    |         ^^^- help: try using a conversion method: `.to_vec()`
+    |         |
+    |         cyclic type of infinite size
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3/unused-substs-3.stderr
To only update this specific test, also pass `--test-args const-generics/occurs-check/unused-substs-3.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/occurs-check/unused-substs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/const-generics/occurs-check/unused-substs-3.rs:16:9
   |
LL |     t = foo;
LL |     t = foo;
   |         ^^^- help: try using a conversion method: `.to_vec()`
   |         |
   |         cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/const-generics/occurs-check/unused-substs-5.rs stdout ----
diff of stderr:

2   --> $DIR/unused-substs-5.rs:15:9
3    |
4 LL |     x = q::<_, N>(x);
-    |         ^^^^^^^^^^^^ cyclic type of infinite size
- help: try using a conversion method
-    |
-    |
- LL |     x = q::<_, N>(x).to_vec();
-    |                     +++++++++
- LL |     x = q::<_, N>(x).to_vec_co();
-    |                     ++++++++++++
+    |         ^^^^^^^^^^^^- help: try using a conversion method: `.to_vec()`
+    |         |
+    |         cyclic type of infinite size
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/unused-substs-5.stderr
To only update this specific test, also pass `--test-args const-generics/occurs-check/unused-substs-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/occurs-check/unused-substs-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/const-generics/occurs-check/unused-substs-5.rs:15:9
   |
   |
LL |     x = q::<_, N>(x); //~ ERROR mismatched types
   |         ^^^^^^^^^^^^- help: try using a conversion method: `.to_vec()`
   |         |
   |         cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/suggestions/issue-53692.rs stdout ----
diff of stderr:

2   --> $DIR/issue-53692.rs:6:33
3    |
4 LL |     let items_clone: Vec<i32> = ref_items.clone();
-    |                      --------   ^^^^^^^^^^^^^^^^^ expected `Vec<i32>`, found `&[i32]`
+    |                      --------   ^^^^^^^^^^-----^^
+    |                      |          |         |
+    |                      |          |         help: try using a conversion method: `to_vec`
+    |                      |          |         help: try using a conversion method: `to_vec`
+    |                      |          expected `Vec<i32>`, found `&[i32]`
8    |
9    = note: expected struct `Vec<i32>`

10            found reference `&[i32]`
10            found reference `&[i32]`
- help: try using a conversion method
-    |
- LL |     let items_clone: Vec<i32> = ref_items.to_vec();
-    |                                           ~~~~~~
- LL |     let items_clone: Vec<i32> = ref_items.to_vec_co();
17 
18 error[E0308]: mismatched types
19   --> $DIR/issue-53692.rs:13:26

---
To only update this specific test, also pass `--test-args suggestions/issue-53692.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/issue-53692.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-53692" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-53692/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/suggestions/issue-53692.rs:6:33
   |
   |
LL |     let items_clone: Vec<i32> = ref_items.clone();
   |                      |          |         |
   |                      |          |         help: try using a conversion method: `to_vec`
   |                      |          |         help: try using a conversion method: `to_vec`
   |                      |          expected `Vec<i32>`, found `&[i32]`
   |
   = note: expected struct `Vec<i32>`
           found reference `&[i32]`

---
---- [ui] tests/ui/typeck/conversion-methods.rs stdout ----
diff of stderr:

39    |
40 LL |     let _prove_piercing_earnest: Vec<usize> = (&[1, 2, 3]).to_vec();
41    |                                               +          ++++++++++
- LL |     let _prove_piercing_earnest: Vec<usize> = (&[1, 2, 3]).to_vec_co();
44 
45 error: aborting due to 4 previous errors
46 

---
To only update this specific test, also pass `--test-args typeck/conversion-methods.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/conversion-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/conversion-methods" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/conversion-methods/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/typeck/conversion-methods.rs:5:41
   |
   |
LL |     let _tis_an_instants_play: String = "'Tis a fond Ambush—"; //~ ERROR mismatched types
   |                                ------   ^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                                |        expected `String`, found `&str`
   |                                expected due to this

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/typeck/conversion-methods.rs:6:40
   |
LL |     let _just_to_make_bliss: PathBuf = Path::new("/ern/her/own/surprise");
   |                              -------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_path_buf()`
   |                              |         expected `PathBuf`, found `&Path`
   |                              expected due to this

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/typeck/conversion-methods.rs:9:40
   |
LL |     let _but_should_the_play: String = 2; // Perhaps surprisingly, we suggest .to_string() here
   |                               ------   ^- help: try using a conversion method: `.to_string()`
   |                               |        expected `String`, found integer
   |                               expected due to this

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/typeck/conversion-methods.rs:12:47
   |
LL |     let _prove_piercing_earnest: Vec<usize> = &[1, 2, 3]; //~ ERROR mismatched types
   |                                  ----------   ^^^^^^^^^^ expected `Vec<usize>`, found `&[{integer}; 3]`
   |                                  expected due to this
   |
   = note: expected struct `Vec<usize>`
   = note: expected struct `Vec<usize>`
           found reference `&[{integer}; 3]`
   |
   |
LL |     let _prove_piercing_earnest: Vec<usize> = (&[1, 2, 3]).to_vec(); //~ ERROR mismatched types

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

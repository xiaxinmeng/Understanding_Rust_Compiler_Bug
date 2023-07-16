plain
44    |           - this expression has type `char`
45 LL |         "A" => {}
46    |         ^^^ expected `char`, found `&str`
-    |
- help: if you meant to write a `char` literal, use single quotes
-    |
- LL |         'A' => {}
52 
53 error: aborting due to 4 previous errors
54 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/char-as-str-single/char-as-str-single.stderr
diff of fixed:

15 #[allow(dead_code)]
16 fn convert_c_to_str(c: char) {
17     match c {
-         'A' => {} //~ ERROR mismatched types
+         "A" => {} //~ ERROR mismatched types
20     }
21 }



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/char-as-str-single/char-as-str-single.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/char-as-str-single.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/inference/char-as-str-single.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/char-as-str-single" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/char-as-str-single/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/inference/char-as-str-single.rs:9:19
   |
   |
LL |     let _: char = "a"; //~ ERROR mismatched types
   |            ----   ^^^ expected `char`, found `&str`
   |            expected due to this
   |
   |
help: if you meant to write a `char` literal, use single quotes
   |
LL |     let _: char = 'a'; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/inference/char-as-str-single.rs:10:19
   |
   |
LL |     let _: char = "人"; //~ ERROR mismatched types
   |            ----   ^^^^ expected `char`, found `&str`
   |            expected due to this
   |
   |
help: if you meant to write a `char` literal, use single quotes
   |
LL |     let _: char = '人'; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/inference/char-as-str-single.rs:11:19
   |
   |
LL |     let _: char = "'"; //~ ERROR mismatched types
   |            ----   ^^^ expected `char`, found `&str`
   |            expected due to this
   |
   |
help: if you meant to write a `char` literal, use single quotes
   |
LL |     let _: char = '\''; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> fake-test-src-base/inference/char-as-str-single.rs:18:9
   |
   |
LL |     match c {
   |           - this expression has type `char`
LL |         "A" => {} //~ ERROR mismatched types
   |         ^^^ expected `char`, found `&str`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------

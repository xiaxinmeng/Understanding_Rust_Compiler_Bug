plain
diff of stderr:

2   --> $DIR/coerce-block-tail-83850.rs:5:7
3    |
4 LL |     f(&Box::new([1, 2]));
-    |       ^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found struct `Box`
+    |     - ^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found struct `Box`
+    |     arguments to this function are incorrect
6    |
7    = note: expected reference `&[i32]`
7    = note: expected reference `&[i32]`
8               found reference `&Box<[{integer}; 2]>`
+ note: function defined here
+   --> $DIR/coerce-block-tail-83850.rs:2:4
+    |
+    |
+ LL | fn f(_: &[i32]) {}
+    |    ^ ---------
10 error: aborting due to previous error
11 



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-block-tail-83850/coerce-block-tail-83850.stderr
To only update this specific test, also pass `--test-args coercion/coerce-block-tail-83850.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-block-tail-83850.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-block-tail-83850" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-block-tail-83850/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/coercion/coerce-block-tail-83850.rs:5:7
   |
   |
LL |     f(&Box::new([1, 2]));
   |     - ^^^^^^^^^^^^^^^^^ expected slice `[i32]`, found struct `Box`
   |     arguments to this function are incorrect
   |
   = note: expected reference `&[i32]`
   = note: expected reference `&[i32]`
              found reference `&Box<[{integer}; 2]>`
  --> /checkout/src/test/ui/coercion/coerce-block-tail-83850.rs:2:4
   |
   |
LL | fn f(_: &[i32]) {}
   |    ^ ---------
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
25 error[E0308]: mismatched types
-   --> $DIR/brackets-to-braces-single-element.rs:7:27
+   --> $DIR/brackets-to-braces-single-element.rs:7:23
27    |
28 LL | const C: &&[u32; 1] = &&{ 1 };
-    |                           ^ expected array `[u32; 1]`, found integer
+    |                       ^^^^^^^ expected array `[u32; 1]`, found integer
30    |
+    = note: expected reference `&'static &'static [u32; 1]`
+               found reference `&&{integer}`
31 help: to create an array, use square brackets instead of curly braces
32    |
33 LL | const C: &&[u32; 1] = &&[ 1 ];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/brackets-to-braces-single-element/brackets-to-braces-single-element.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/brackets-to-braces-single-element.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/brackets-to-braces-single-element.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/brackets-to-braces-single-element" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/brackets-to-braces-single-element/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/did_you_mean/brackets-to-braces-single-element.rs:1:24
   |
   |
LL | const A: [&str; 1] = { "hello" };
   |                        ^^^^^^^ expected array `[&'static str; 1]`, found `&str`
   |
help: to create an array, use square brackets instead of curly braces
   |
LL | const A: [&str; 1] = [ "hello" ];

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/brackets-to-braces-single-element.rs:4:19
   |
   |
LL | const B: &[u32] = &{ 1 };
   |                   ^^^^^^ expected slice `[u32]`, found integer
   = note: expected reference `&'static [u32]`
              found reference `&{integer}`
              found reference `&{integer}`
help: to create an array, use square brackets instead of curly braces
   |
LL | const B: &[u32] = &[ 1 ];

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/brackets-to-braces-single-element.rs:7:23
   |
   |
LL | const C: &&[u32; 1] = &&{ 1 };
   |                       ^^^^^^^ expected array `[u32; 1]`, found integer
   |
   = note: expected reference `&'static &'static [u32; 1]`
              found reference `&&{integer}`
help: to create an array, use square brackets instead of curly braces
   |
LL | const C: &&[u32; 1] = &&[ 1 ];

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

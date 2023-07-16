plain
........................................................................................ 14432/14969
........................................................................................ 14520/14969
........................................................................................ 14608/14969
........................................................................................ 14696/14969
..........................F.F........................................................i.. 14784/14969
........................................................................................ 14960/14969
.........

failures:
failures:

---- [ui] tests/ui/unsized-locals/borrow-after-move.rs stdout ----
diff of stderr:

58 error[E0382]: borrow of moved value: `x`
59   --> $DIR/borrow-after-move.rs:40:24
60    |
+ LL |         let x = "hello".to_owned().into_boxed_str();
+    |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
61 LL |         x.foo();
-    |         ------- value moved here
63 LL |         println!("{}", &x);
64    |                        ^^ value borrowed here after move
65    |


-    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
+ help: consider cloning the value if the performance cost is acceptable
+    |
+ LL |         x.clone().foo();
67 
68 error: aborting due to 5 previous errors; 1 warning emitted
69 

---
To only update this specific test, also pass `--test-args unsized-locals/borrow-after-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unsized-locals/borrow-after-move.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:1:12
LL | #![feature(unsized_locals, unsized_fn_params)]
   |            ^^^^^^^^^^^^^^
   |
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0382]: borrow of moved value: `x`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:21:24
   |
LL |         let y = *x;
   |                 -- value moved here
LL |         drop_unsized(y);
LL |         println!("{}", &x);
   |
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
error[E0382]: borrow of moved value: `y`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:23:24
   |
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         drop_unsized(y);
   |         --------------- value moved here
LL |         println!("{}", &y);
   |                        ^^ value borrowed here after move

error[E0382]: borrow of moved value: `x`
error[E0382]: borrow of moved value: `x`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:31:24
   |
LL |         y.foo();
   |           ----- `*x` moved due to this method call
LL |         println!("{}", &x);
   |
   |
note: `Foo::foo` takes ownership of the receiver `self`, which moves `*x`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:5:12
Build completed unsuccessfully in 0:12:55
LL |     fn foo(self) -> String;
   |            ^^^^
   |            ^^^^
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
error[E0382]: borrow of moved value: `y`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:33:24
   |
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         y.foo();
   |         ------- value moved here
LL |         println!("{}", &y);
   |                        ^^ value borrowed here after move

error[E0382]: borrow of moved value: `x`
error[E0382]: borrow of moved value: `x`
  --> fake-test-src-base/unsized-locals/borrow-after-move.rs:40:24
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         x.foo();
LL |         println!("{}", &x);
   |                        ^^ value borrowed here after move
   |
help: consider cloning the value if the performance cost is acceptable
help: consider cloning the value if the performance cost is acceptable
   |
LL |         x.clone().foo();

error: aborting due to 5 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] tests/ui/unsized-locals/double-move.rs stdout ----
diff of stderr:

47 LL |         y.foo();
48    |         ^^^^^^^ value used here after move
- error[E0382]: use of moved value: `*x`
+ error[E0382]: use of moved value: `x`
51   --> $DIR/double-move.rs:46:9
52    |
52    |
53 LL |         let _y = *x;
54    |                  -- value moved here
54    |                  -- value moved here
55 LL |         x.foo();
-    |         ^^^^^^^ value used here after move
+    |         ^ value used here after move
57    |
58    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait

60 error[E0382]: use of moved value: `*x`
61   --> $DIR/double-move.rs:52:18
62    |
62    |
+ LL |         let x = "hello".to_owned().into_boxed_str();
+    |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
63 LL |         x.foo();
-    |         ------- value moved here
+    |         - value moved here
65 LL |         let _y = *x;
66    |                  ^^ value used here after move
-    |
-    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
70 error: aborting due to 6 previous errors; 1 warning emitted
71 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move/double-move.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized-locals/double-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unsized-locals/double-move.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/unsized-locals/double-move.rs:1:12
LL | #![feature(unsized_locals, unsized_fn_params)]
   |            ^^^^^^^^^^^^^^
   |
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information
   = note: `#[warn(incomplete_features)]` on by default

error[E0382]: use of moved value: `y`
  --> fake-test-src-base/unsized-locals/double-move.rs:21:9
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         drop_unsized(y);
   |         --------------- value moved here
LL |         drop_unsized(y); //~ERROR use of moved value
   |         ^^^^^^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `x`
  --> fake-test-src-base/unsized-locals/double-move.rs:27:22
   |
   |
LL |         let _y = *x;
   |                  -- value moved here
LL |         drop_unsized(x); //~ERROR use of moved value
   |                      ^ value used here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `*x`
  --> fake-test-src-base/unsized-locals/double-move.rs:33:18
   |
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         drop_unsized(x);
   |                      - value moved here
LL |         let _y = *x; //~ERROR use of moved value
   |                  ^^ value used here after move
error[E0382]: use of moved value: `y`
  --> fake-test-src-base/unsized-locals/double-move.rs:40:9
   |
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         y.foo();
   |         ------- value moved here
LL |         y.foo(); //~ERROR use of moved value
   |         ^^^^^^^ value used here after move
error[E0382]: use of moved value: `x`
  --> fake-test-src-base/unsized-locals/double-move.rs:46:9
   |
   |
LL |         let _y = *x;
   |                  -- value moved here
LL |         x.foo(); //~ERROR use of moved value
   |         ^ value used here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `*x`
  --> fake-test-src-base/unsized-locals/double-move.rs:52:18
   |
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         x.foo();
   |         - value moved here
LL |         let _y = *x; //~ERROR use of moved value
   |                  ^^ value used here after move
error: aborting due to 6 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
------------------------------------------

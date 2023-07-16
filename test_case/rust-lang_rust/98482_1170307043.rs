plain
diff of stderr:

10    |
11    = note: expected unit type `()`
12                    found type `!`
-    = note: required for the cast from `[closure@$DIR/fallback-closure-wrap.rs:18:40: 21:6]` to the object type `dyn FnMut()`
+    = note: required for the cast from `[closure@$DIR/fallback-closure-wrap.rs:18:40: 18:47]` to the object type `dyn FnMut()`
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback/fallback-closure-wrap.fallback.stderr
To only update this specific test, also pass `--test-args never_type/fallback-closure-wrap.rs`


error in revision `fallback`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/fallback-closure-wrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/never_type/fallback-closure-wrap.rs:18:40: 18:47] as FnOnce<()>>::Output == ()`
   |
   |
LL |       let error = Closure::wrap(Box::new(move || {
   |  _______________________________^
LL | |         //[fallback]~^ ERROR type mismatch resolving
LL | |         panic!("Can't connect to server.");
LL | |     }) as Box<dyn FnMut()>);
   | |______^ expected `()`, found `!`
   = note: expected unit type `()`
                   found type `!`
                   found type `!`
   = note: required for the cast from `[closure@/checkout/src/test/ui/never_type/fallback-closure-wrap.rs:18:40: 18:47]` to the object type `dyn FnMut()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/tuple/wrong_argument_ice-4.rs stdout ----
diff of stderr:

6 LL | |
7 LL | |         let b = 1;
8 LL | |     });
-    | |_____- argument of type `[closure@$DIR/wrong_argument_ice-4.rs:2:13: 5:6]` unexpected
+    | |_____- argument of type `[closure@$DIR/wrong_argument_ice-4.rs:2:13: 2:15]` unexpected
11 note: closure defined here
12   --> $DIR/wrong_argument_ice-4.rs:2:6



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice-4/wrong_argument_ice-4.stderr
To only update this specific test, also pass `--test-args tuple/wrong_argument_ice-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple/wrong_argument_ice-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice-4/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/tuple/wrong_argument_ice-4.rs:2:5
   |
   |
LL |       (|| {})(|| {
   |  _____^^^^^^^_-
LL | |         //~^ ERROR this function takes 0 arguments but 1 argument was supplied
LL | |         let b = 1;
LL | |     });
   | |_____- argument of type `[closure@/checkout/src/test/ui/tuple/wrong_argument_ice-4.rs:2:13: 2:15]` unexpected
note: closure defined here
  --> /checkout/src/test/ui/tuple/wrong_argument_ice-4.rs:2:6
   |
   |
LL |     (|| {})(|| {
help: remove the extra argument
   |
   |
LL |     (|| {})();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0057`.

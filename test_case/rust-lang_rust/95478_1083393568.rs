plain

15    |
16 LL | #![deny(large_assignments)]
17    |         ^^^^^^^^^^^^^^^^^
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
20 error: moving 10024 bytes
21   --> $DIR/large_moves.rs:18:14
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


23 LL |     let z = (x, 42);
24    |              ^ value moved from here
25    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
28 error: moving 10024 bytes
29   --> $DIR/large_moves.rs:18:13


31 LL |     let z = (x, 42);
32    |             ^^^^^^^ value moved from here
33    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
36 error: moving 10024 bytes
37   --> $DIR/large_moves.rs:20:13

39 LL |     let a = z.0;
39 LL |     let a = z.0;
40    |             ^^^ value moved from here
41    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
44 error: aborting due to 4 previous errors
45 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option/large_moves.option.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/large_moves.rs`

error in revision `option`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "option" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmove-size-limit=1000" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option/auxiliary"
stdout: none
--- stderr -------------------------------
error: moving 10024 bytes
   |
   |
LL |       let x = async { //~ ERROR large_assignments
   |  _____________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };
   | |_____^ value moved from here
note: the lint level is defined here
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
LL | #![deny(large_assignments)]
LL | #![deny(large_assignments)]
   |         ^^^^^^^^^^^^^^^^^
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:14
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |             ^^^^^^^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:20:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |             ^^^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] ui/async-await/large_moves.rs#attribute stdout ----
diff of stderr:

15    |
16 LL | #![deny(large_assignments)]
17    |         ^^^^^^^^^^^^^^^^^
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
20 error: moving 10024 bytes
21   --> $DIR/large_moves.rs:18:14


23 LL |     let z = (x, 42);
24    |              ^ value moved from here
25    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
28 error: moving 10024 bytes
29   --> $DIR/large_moves.rs:18:13


31 LL |     let z = (x, 42);
32    |             ^^^^^^^ value moved from here
33    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
36 error: moving 10024 bytes
37   --> $DIR/large_moves.rs:20:13

39 LL |     let a = z.0;
39 LL |     let a = z.0;
40    |             ^^^ value moved from here
41    |
-    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute
+    = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
44 error: aborting due to 4 previous errors
45 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute/large_moves.attribute.stderr
To only update this specific test, also pass `--test-args async-await/large_moves.rs`


error in revision `attribute`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "attribute" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute/auxiliary"
stdout: none
--- stderr -------------------------------
error: moving 10024 bytes
   |
   |
LL |       let x = async { //~ ERROR large_assignments
   |  _____________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };
   | |_____^ value moved from here
note: the lint level is defined here
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
LL | #![deny(large_assignments)]
LL | #![deny(large_assignments)]
   |         ^^^^^^^^^^^^^^^^^
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:14
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |             ^^^^^^^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:20:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |             ^^^ value moved from here
   |
   = note: The current maximum size is 1000, it can be customized by modifying the move_size_limit attribute: (#![move_size_limit = "1000"])
error: aborting due to 4 previous errors
------------------------------------------



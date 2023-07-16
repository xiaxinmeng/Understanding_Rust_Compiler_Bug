plain
46 LL |             writeln!(f, "{}",)?;
47    |                          ^^
48 
- error: aborting due to 8 previous errors
+ error[E0522]: definition of an unknown language item: `eh_catch_typeinfo`
+    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | #[cfg(core)] #[lang = "eh_catch_typeinfo"] static EH_CATCH_TYPEINFO: u8 = 0;
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `eh_catch_typeinfo`
+ error: aborting due to 9 previous errors
+ 
+ For more information about this error, try `rustc --explain E0522`.
51 
51 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/macro-comma-behavior.core.stderr
To only update this specific test, also pass `--test-args macros/macro-comma-behavior.rs`


error in revision `core`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior.core/auxiliary"
stdout: none
--- stderr -------------------------------
error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_eq!(1, 1, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     debug_assert_ne!(1, 2, "{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |     format_args!("{}",);
   |                   ^^


error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     unimplemented!("{}",);


error: 1 positional argument in format string, but no arguments were given
   |
LL |             write!(f, "{}",)?;
   |                        ^^


error: 1 positional argument in format string, but no arguments were given
   |
LL |             writeln!(f, "{}",)?;
   |                          ^^


error[E0522]: definition of an unknown language item: `eh_catch_typeinfo`
   |
   |
LL | #[cfg(core)] #[lang = "eh_catch_typeinfo"] static EH_CATCH_TYPEINFO: u8 = 0;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `eh_catch_typeinfo`
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0522`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/range/issue-54505-no-std.rs stdout ----
diff of stderr:

- error: `#[panic_handler]` function required, but not found
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:27:16
-   --> $DIR/issue-54505-no-std.rs:27:16
+ error[E0425]: cannot find function `take_range` in this scope
5    |
5    |
6 LL |     take_range(0..1);
-    |                |
-    |                expected reference, found struct `Range`
-    |                help: consider borrowing here: `&(0..1)`
-    |
-    |
-    = note: expected reference `&_`
-                  found struct `Range<{integer}>`
+    |     ^^^^^^^^^^ not found in this scope
14 
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:32:16
+ error[E0425]: cannot find function `take_range` in this scope
17    |
17    |
18 LL |     take_range(1..);
-    |                |
-    |                expected reference, found struct `RangeFrom`
-    |                help: consider borrowing here: `&(1..)`
-    |
-    |
-    = note: expected reference `&_`
-                  found struct `RangeFrom<{integer}>`
+    |     ^^^^^^^^^^ not found in this scope
26 
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:37:16
+ error[E0425]: cannot find function `take_range` in this scope
29    |
29    |
30 LL |     take_range(..);
-    |                |
-    |                expected reference, found struct `RangeFull`
-    |                help: consider borrowing here: `&(..)`
-    |
-    |
-    = note: expected reference `&_`
-                  found struct `RangeFull`
+    |     ^^^^^^^^^^ not found in this scope
38 
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:42:16
+ error[E0425]: cannot find function `take_range` in this scope
41    |
41    |
42 LL |     take_range(0..=1);
-    |                |
-    |                expected reference, found struct `RangeInclusive`
-    |                expected reference, found struct `RangeInclusive`
-    |                help: consider borrowing here: `&(0..=1)`
-    = note: expected reference `&_`
-                  found struct `RangeInclusive<{integer}>`
+    |     ^^^^^^^^^^ not found in this scope
50 
50 
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:47:16
+ error[E0425]: cannot find function `take_range` in this scope
53    |
53    |
54 LL |     take_range(..5);
-    |                |
-    |                expected reference, found struct `RangeTo`
-    |                help: consider borrowing here: `&(..5)`
-    |
-    |
-    = note: expected reference `&_`
-                  found struct `RangeTo<{integer}>`
+    |     ^^^^^^^^^^ not found in this scope
62 
- error[E0308]: mismatched types
-   --> $DIR/issue-54505-no-std.rs:52:16
+ error[E0425]: cannot find function `take_range` in this scope
65    |
65    |
66 LL |     take_range(..=42);
-    |                |
-    |                expected reference, found struct `RangeToInclusive`
-    |                expected reference, found struct `RangeToInclusive`
-    |                help: consider borrowing here: `&(..=42)`
-    = note: expected reference `&_`
-                  found struct `RangeToInclusive<{integer}>`
+    |     ^^^^^^^^^^ not found in this scope
74 
74 
+ error: `#[panic_handler]` function required, but not found
75 error: aborting due to 7 previous errors
76 
- For more information about this error, try `rustc --explain E0308`.
+ For more information about this error, try `rustc --explain E0425`.
---
To only update this specific test, also pass `--test-args range/issue-54505-no-std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/issue-54505-no-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(0..1);


error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(1..);


error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(..);


error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(0..=1);


error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(..5);


error[E0425]: cannot find function `take_range` in this scope
   |
   |
LL |     take_range(..=42);


error: `#[panic_handler]` function required, but not found
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------

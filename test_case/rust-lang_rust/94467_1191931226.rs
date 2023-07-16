plain
.........iii............................................................................ 13200/13228
............................
failures:

---- [ui] src/test/ui/modules/special_module_name.rs stdout ----

14    |
14    |
15    = help: to create the module `main`, create file "$DIR/main.rs" or "$DIR/main/mod.rs"
- warning: found module declaration for lib.rs
-   --> $DIR/special_module_name.rs:1:1
-    |
-    |
- LL | mod lib;
-    |
-    |
-    = note: `#[warn(special_module_name)]` on by default
-    = note: lib.rs is the root of this crate's library target
-    = help: to refer to it from other targets, use the library's name as the path
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- warning: found module declaration for main.rs
-   --> $DIR/special_module_name.rs:4:1
-    |
-    |
- LL | mod main;
-    |
-    = note: a binary crate cannot be used as library
- 
- error: aborting due to 2 previous errors; 2 warnings emitted
---
To only update this specific test, also pass `--test-args modules/special_module_name.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/modules/special_module_name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/special_module_name" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/special_module_name/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0583]: file not found for module `lib`
  --> /checkout/src/test/ui/modules/special_module_name.rs:1:1
LL | mod lib;
   | ^^^^^^^^
   |
   |
   = help: to create the module `lib`, create file "/checkout/src/test/ui/modules/lib.rs" or "/checkout/src/test/ui/modules/lib/mod.rs"
error[E0583]: file not found for module `main`
  --> /checkout/src/test/ui/modules/special_module_name.rs:4:1
   |
   |
LL | mod main;
   |
   |
   = help: to create the module `main`, create file "/checkout/src/test/ui/modules/main.rs" or "/checkout/src/test/ui/modules/main/mod.rs"
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0583`.
------------------------------------------

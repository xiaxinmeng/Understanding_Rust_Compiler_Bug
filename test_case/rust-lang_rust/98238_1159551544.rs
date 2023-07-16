plain

---- [ui] src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
diff of 32bit.stderr:

+ warning: the type `!` does not permit zero-initialization
+    |
+ LL |     unsafe { std::mem::transmute(()) }
+    |              ^^^^^^^^^^^^^^^^^^^^^^^
+    |              |
+    |              |
+    |              this code causes undefined behavior when executed
+    |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
+    = note: `#[warn(invalid_value)]` on by default
+    = note: `#[warn(invalid_value)]` on by default
+    = note: the `!` type has no valid value
1 error[E0080]: evaluation of constant value failed
2   --> $DIR/validate_uninhabited_zsts.rs:4:14
3    |


18    |
19    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
20    = note: the raw bytes of the constant (size: 0, align: 1) {}
- 
- warning: the type `!` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:4:14
- LL |     unsafe { std::mem::transmute(()) }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
-    |              ^^^^^^^^^^^^^^^^^^^^^^^
-    |              |
-    |              |
-    |              this code causes undefined behavior when executed
-    |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
-    = note: `#[warn(invalid_value)]` on by default
-    = note: `#[warn(invalid_value)]` on by default
-    = note: the `!` type has no valid value
33 
34 warning: the type `empty::Empty` does not permit zero-initialization


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the type `!` does not permit zero-initialization
   |
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              |
   |              this code causes undefined behavior when executed
   |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: `#[warn(invalid_value)]` on by default
   = note: `#[warn(invalid_value)]` on by default
   = note: the `!` type has no valid value
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
LL |     unsafe { std::mem::transmute(()) }
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:4:14
...
LL | const FOO: [empty::Empty; 3] = [foo(); 3];
   |                                 ----- inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:20:33
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:23:1
   |
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0].0: encountered a value of uninhabited type empty::Void
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}

warning: the type `empty::Empty` does not permit zero-initialization
   |
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
note: enums with no variants have no valid value (in this struct field)
   |
LL |     pub struct Empty(Void);
   |                      ^^^^


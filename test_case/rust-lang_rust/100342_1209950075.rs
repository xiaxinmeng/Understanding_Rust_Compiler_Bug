plain
normalized stderr:
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> $DIR/issue-61422.rs:11:33
   |
LL |         let array: [u8; SIZE] = mem::uninitialized();
   |
   |
   = note: `#[warn(mem_uninitialized)]` on by default
   = note: for more information, see FIXME: fill this in
warning: 1 warning emitted

Future incompatibility report: Future breakage diagnostic:
warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> $DIR/issue-61422.rs:11:33
   |
LL |         let array: [u8; SIZE] = mem::uninitialized();
   |
   |
   = note: `#[warn(mem_uninitialized)]` on by default
   = note: for more information, see FIXME: fill this in



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61422/issue-61422.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-61422.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61422.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61422" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61422/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of mem::uninitialized is very likely to be undefined behavior
   |
   |
LL |         let array: [u8; SIZE] = mem::uninitialized();
   |
   |
   = note: `#[warn(mem_uninitialized)]` on by default
   = note: for more information, see FIXME: fill this in
warning: 1 warning emitted

Future incompatibility report: Future breakage diagnostic:
warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/const-generics/issues/issue-61422.rs:11:33
   |
LL |         let array: [u8; SIZE] = mem::uninitialized();
   |
   |
   = note: `#[warn(mem_uninitialized)]` on by default
   = note: for more information, see FIXME: fill this in


---- [ui] src/test/ui/lint/uninitialized-zeroed.rs stdout ----
diff of stderr:
diff of stderr:

+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:41:32
+    |
+ LL |         let _val: &'static T = mem::uninitialized();
+    |                                ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:44:38
+    |
+    |
+ LL |         let _val: Wrap<&'static T> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
---
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:55:30
+    |
+ LL |         let _val: (i32, !) = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
---
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:70:32
+    |
+ LL |         let _val: Wrap<fn()> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:73:36
+    |
+ LL |         let _val: WrapEnum<fn()> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:76:42
+    |
+ LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:79:34
+    |
+ LL |         let _val: NonNull<i32> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:82:37
+    |
+ LL |         let _val: *const dyn Send = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:85:31
+    |
+ LL |         let _val: [fn(); 2] = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
---
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:92:32
+    |
+ LL |         let _val: Wrap<char> = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
---
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:98:27
+    |
+ LL |         let _val: Fruit = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:101:31
+    |
+ LL |         let _val: [bool; 2] = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
---
+ 
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:124:30
+    |
+ LL |         let _val: OneFruit = mem::uninitialized();
+    |
+    = note: for more information, see FIXME: fill this in
+ 
+ 
1 error: the type `&T` does not permit zero-initialization
3    |

480    |
480    |
481    = note: booleans must be either `true` or `false`
- error: aborting due to 39 previous errors
+ error: aborting due to 39 previous errors; 21 warnings emitted
+ 
+ 
+ Future incompatibility report: Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+    |
+ LL |         let _val: &'static T = mem::uninitialized();
+    |                                ^^^^^^^^^^^^^^^^^^
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:44:38
+    |
+    |
+ LL |         let _val: Wrap<&'static T> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:52:23
+    |
+    |
+ LL |         let _val: ! = mem::uninitialized();
+    |                       ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:55:30
+    |
+    |
+ LL |         let _val: (i32, !) = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:58:26
+    |
+    |
+ LL |         let _val: Void = mem::uninitialized();
+    |                          ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:61:34
+    |
+    |
+ LL |         let _val: &'static i32 = mem::uninitialized();
+    |                                  ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:64:25
+    |
+    |
+ LL |         let _val: Ref = mem::uninitialized();
+    |                         ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:67:26
+    |
+    |
+ LL |         let _val: fn() = mem::uninitialized();
+    |                          ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:70:32
+    |
+    |
+ LL |         let _val: Wrap<fn()> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:73:36
+    |
+    |
+ LL |         let _val: WrapEnum<fn()> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:76:42
+    |
+    |
+ LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:79:34
+    |
+    |
+ LL |         let _val: NonNull<i32> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:82:37
+    |
+    |
+ LL |         let _val: *const dyn Send = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:85:31
+    |
+    |
+ LL |         let _val: [fn(); 2] = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:89:26
+    |
+    |
+ LL |         let _val: bool = mem::uninitialized();
+    |                          ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:92:32
+    |
+    |
+ LL |         let _val: Wrap<char> = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:95:28
+    |
+    |
+ LL |         let _val: NonBig = mem::uninitialized();
+    |                            ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:98:27
+    |
+    |
+ LL |         let _val: Fruit = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:101:31
+    |
+    |
+ LL |         let _val: [bool; 2] = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:123:25
+    |
+    |
+ LL |         let _val: i32 = mem::uninitialized();
+    |                         ^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
+ Future breakage diagnostic:
+ warning: use of mem::uninitialized is very likely to be undefined behavior
+   --> $DIR/uninitialized-zeroed.rs:124:30
+    |
+    |
+ LL |         let _val: OneFruit = mem::uninitialized();
+    |
+    |
+    = note: `#[warn(mem_uninitialized)]` on by default
+    = note: for more information, see FIXME: fill this in
485 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/uninitialized-zeroed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of mem::uninitialized is very likely to be undefined behavior
   |
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   |
   = note: `#[warn(mem_uninitialized)]` on by default
   = note: for more information, see FIXME: fill this in
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:44:38
   |
   |
LL |         let _val: Wrap<&'static T> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:52:23
   |
LL |         let _val: ! = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:55:30
   |
LL |         let _val: (i32, !) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:58:26
   |
LL |         let _val: Void = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:61:34
   |
LL |         let _val: &'static i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:64:25
   |
LL |         let _val: Ref = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:67:26
   |
LL |         let _val: fn() = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:70:32
   |
LL |         let _val: Wrap<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:73:36
   |
LL |         let _val: WrapEnum<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:76:42
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:79:34
   |
LL |         let _val: NonNull<i32> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:82:37
   |
LL |         let _val: *const dyn Send = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:85:31
   |
LL |         let _val: [fn(); 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:89:26
   |
LL |         let _val: bool = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:92:32
   |
LL |         let _val: Wrap<char> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:95:28
   |
LL |         let _val: NonBig = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:98:27
   |
LL |         let _val: Fruit = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:101:31
   |
LL |         let _val: [bool; 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   = note: for more information, see FIXME: fill this in

warning: use of mem::uninitialized is very likely to be undefined behavior
---

warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:124:30
   |
LL |         let _val: OneFruit = mem::uninitialized();
   |
   = note: for more information, see FIXME: fill this in


error: the type `&T` does not permit zero-initialization
   |
   |
LL |         let _val: &'static T = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:6:9
   |
LL | #![deny(invalid_value)]
LL | #![deny(invalid_value)]
   |         ^^^^^^^^^^^^^
   = note: references must be non-null

error: the type `&T` does not permit being left uninitialized
   |
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `Wrap<&T>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<&'static T> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<&T>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<&'static T> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `!` does not permit zero-initialization
   |
   |
LL |         let _val: ! = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                       |
   |                       this code causes undefined behavior when executed
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the `!` type has no valid value

error: the type `!` does not permit being left uninitialized
   |
   |
LL |         let _val: ! = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                       |
   |                       this code causes undefined behavior when executed
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit zero-initialization
   |
   |
LL |         let _val: (i32, !) = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                              |
   |                              this code causes undefined behavior when executed
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit being left uninitialized
   |
   |
LL |         let _val: (i32, !) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                              |
   |                              this code causes undefined behavior when executed
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the `!` type has no valid value

error: the type `Void` does not permit zero-initialization
   |
   |
LL |         let _val: Void = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: enums with no variants have no valid value


error: the type `Void` does not permit being left uninitialized
   |
   |
LL |         let _val: Void = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: enums with no variants have no valid value


error: the type `&i32` does not permit zero-initialization
   |
   |
LL |         let _val: &'static i32 = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `&i32` does not permit being left uninitialized
   |
   |
LL |         let _val: &'static i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `Ref` does not permit zero-initialization
   |
   |
LL |         let _val: Ref = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                         |
   |                         this code causes undefined behavior when executed
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:14:12
   |
   |
LL | struct Ref(&'static i32);


error: the type `Ref` does not permit being left uninitialized
   |
   |
LL |         let _val: Ref = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                         |
   |                         this code causes undefined behavior when executed
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:14:12
   |
   |
LL | struct Ref(&'static i32);


error: the type `fn()` does not permit zero-initialization
   |
   |
LL |         let _val: fn() = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: function pointers must be non-null


error: the type `fn()` does not permit being left uninitialized
   |
   |
LL |         let _val: fn() = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: function pointers must be non-null


error: the type `Wrap<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `WrapEnum<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: function pointers must be non-null (in this enum field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:18:28
   |
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `WrapEnum<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: function pointers must be non-null (in this enum field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:18:28
   |
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `Wrap<(RefPair, i32)>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:15:16
   |
   |
LL | struct RefPair((&'static i32, i32));


error: the type `Wrap<(RefPair, i32)>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:15:16
   |
   |
LL | struct RefPair((&'static i32, i32));


error: the type `NonNull<i32>` does not permit zero-initialization
   |
   |
LL |         let _val: NonNull<i32> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `NonNull<i32>` does not permit being left uninitialized
   |
   |
LL |         let _val: NonNull<i32> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `*const dyn Send` does not permit zero-initialization
   |
   |
LL |         let _val: *const dyn Send = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `*const dyn Send` does not permit being left uninitialized
   |
   |
LL |         let _val: *const dyn Send = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `[fn(); 2]` does not permit zero-initialization
   |
   |
LL |         let _val: [fn(); 2] = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                               |
   |                               this code causes undefined behavior when executed
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: function pointers must be non-null


error: the type `[fn(); 2]` does not permit being left uninitialized
   |
   |
LL |         let _val: [fn(); 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: function pointers must be non-null


error: the type `bool` does not permit being left uninitialized
   |
   |
LL |         let _val: bool = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: booleans must be either `true` or `false`

error: the type `Wrap<char>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<char> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
note: characters must be a valid Unicode codepoint (in this struct field)
   |
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `NonBig` does not permit being left uninitialized
   |
   |
LL |         let _val: NonBig = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                            |
   |                            this code causes undefined behavior when executed
   |                            this code causes undefined behavior when executed
   |                            help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `NonBig` must be initialized inside its custom valid range

error: the type `Fruit` does not permit being left uninitialized
   |
   |
LL |         let _val: Fruit = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                           |
   |                           this code causes undefined behavior when executed
   |                           this code causes undefined behavior when executed
   |                           help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
note: enums have to be initialized to a variant
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:26:1
   |
   |
LL | enum Fruit {


error: the type `[bool; 2]` does not permit being left uninitialized
   |
   |
LL |         let _val: [bool; 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: booleans must be either `true` or `false`

error: the type `&i32` does not permit zero-initialization
   |
   |
LL |         let _val: &'static i32 = mem::transmute(0usize); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `&[i32]` does not permit zero-initialization
   |
   |
LL |         let _val: &'static [i32] = mem::transmute((0usize, 0usize)); //~ ERROR: does not permit zero-initialization
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: references must be non-null


error: the type `NonZeroU32` does not permit zero-initialization
   |
   |
LL |         let _val: NonZeroU32 = mem::transmute(0); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   = note: `std::num::NonZeroU32` must be non-null


error: the type `NonNull<i32>` does not permit zero-initialization
   |
   |
LL |         let _val: NonNull<i32> = MaybeUninit::zeroed().assume_init(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `NonNull<i32>` does not permit being left uninitialized
   |
   |
LL |         let _val: NonNull<i32> = MaybeUninit::uninit().assume_init(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `bool` does not permit being left uninitialized
   |
   |
LL |         let _val: bool = MaybeUninit::uninit().assume_init(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: booleans must be either `true` or `false`
error: aborting due to 39 previous errors; 21 warnings emitted

Future incompatibility report: Future breakage diagnostic:
warning: use of mem::uninitialized is very likely to be undefined behavior
warning: use of mem::uninitialized is very likely to be undefined behavior
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:41:32
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |
   |

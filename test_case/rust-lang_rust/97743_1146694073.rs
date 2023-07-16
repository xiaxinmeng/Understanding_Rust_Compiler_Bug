plain
..............................................................................i......... 6688/13299
.............................................ii.ii........i...i......................... 6776/13299
........................................................................................ 6864/13299
........................................................................................ 6952/13299
.............................i....i...............F.........................i........... 7040/13299
......i.............i........................FF..F..........................i........... 7128/13299
.....i.................................................................................. 7304/13299
........................................................................................ 7392/13299
...........................................................................ii........... 7480/13299
..........................ii...........................................................i 7568/13299
---

23 error: aborting due to 2 previous errors
24 
25 For more information about this error, try `rustc --explain E0080`.
+ Future incompatibility report: Future breakage diagnostic:
+ error: erroneous constant used
+    |
+    |
+ LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
+    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
+    |
+    = note: `#[deny(const_err)]` on by default
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
26 
---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-55878/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ inside `std::mem::size_of::<[u8; 18446744073709551615]>` at /checkout/library/core/src/mem/mod.rs:311:5
   |
   |
  ::: /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());

error: erroneous constant used
  --> /checkout/src/test/ui/limits/issue-55878.rs:7:26
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: erroneous constant used
   |
   |
LL |     println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
------------------------------------------



---- [ui] src/test/ui/lint/force-warn/allowed-deny-by-default-lint.rs stdout ----
diff of stderr:

12 
13 warning: 1 warning emitted
14 
+ Future incompatibility report: Future breakage diagnostic:
+ warning: any use of this value will cause an error
+    |
+    |
+ LL | const C: i32 = 1 / 0;
+    |                |
+    |                |
+    |                attempt to divide `1_i32` by zero
+    |
+    = note: requested on the command line with `--force-warn const-err`
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
15 

---
To only update this specific test, also pass `--test-args lint/force-warn/allowed-deny-by-default-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/allowed-deny-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/allowed-deny-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warn" "const_err" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/allowed-deny-by-default-lint/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/force-warn/allowed-deny-by-default-lint.rs:7:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: 1 warning emitted


Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/lint/force-warn/allowed-deny-by-default-lint.rs:7:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint.rs stdout ----
diff of stderr:

12 
13 warning: 1 warning emitted
14 
+ Future incompatibility report: Future breakage diagnostic:
+ warning: any use of this value will cause an error
+    |
+    |
+ LL | const C: i32 = 1 / 0;
+    |                |
+    |                |
+    |                attempt to divide `1_i32` by zero
+    |
+    = note: requested on the command line with `--force-warn const-err`
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
15 

---
To only update this specific test, also pass `--test-args lint/force-warn/allowed-cli-deny-by-default-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "const_err" "--force-warn" "const_err" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint.rs:6:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: 1 warning emitted


Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/lint/force-warn/allowed-cli-deny-by-default-lint.rs:6:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/lint/force-warn/deny-by-default-lint.rs stdout ----
diff of stderr:

12 
13 warning: 1 warning emitted
14 
+ Future incompatibility report: Future breakage diagnostic:
+ warning: any use of this value will cause an error
+   --> $DIR/deny-by-default-lint.rs:5:16
+    |
+ LL | const C: i32 = 1 / 0;
+    |                |
+    |                |
+    |                attempt to divide `1_i32` by zero
+    |
+    = note: requested on the command line with `--force-warn const-err`
+    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ 
15 

---
To only update this specific test, also pass `--test-args lint/force-warn/deny-by-default-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/deny-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/deny-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warn" "const_err" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/deny-by-default-lint/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/force-warn/deny-by-default-lint.rs:5:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: 1 warning emitted


Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/lint/force-warn/deny-by-default-lint.rs:5:16
   |
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: requested on the command line with `--force-warn const-err`
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



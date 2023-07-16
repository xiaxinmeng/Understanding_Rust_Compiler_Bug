plain
---- [ui] src/test/ui/asm/issue-99122-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-99122-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99122-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99122-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
LL | /     core::arch::asm!(
LL | |         "nop",
LL | |         "nop",
LL | |         in("eax") pointer,
LL | |     );
   |
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error: invalid register `eax`: unknown register
   |
   |
LL |         in("eax") pointer,

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
------------------------------------------


---- [ui] src/test/ui/asm/issue-99122.rs stdout ----
diff of stderr:

+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+ LL | /     core::arch::asm!(
+ LL | /     core::arch::asm!(
+ LL | |         "nop",
+ LL | |         in("eax") pointer,
+ LL | |     );
+    |
+    = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
+    = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error: invalid register `eax`: unknown register
+    |
+    |
+ LL |         in("eax") pointer,
+ 
1 error[E0641]: cannot cast to a pointer of an unknown kind
2   --> $DIR/issue-99122.rs:2:27
3    |
3    |

6    |
7    = note: the type information given here is insufficient to check whether the pointer cast is valid
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
10 
- For more information about this error, try `rustc --explain E0641`.
---
To only update this specific test, also pass `--test-args asm/issue-99122.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-99122.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99122" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99122/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
LL | /     core::arch::asm!(
LL | |         "nop",
LL | |         "nop",
LL | |         in("eax") pointer,
LL | |     );
   |
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error: invalid register `eax`: unknown register
   |
   |
LL |         in("eax") pointer,

error[E0641]: cannot cast to a pointer of an unknown kind
  --> /checkout/src/test/ui/asm/issue-99122.rs:2:27
   |
   |
LL |     let pointer = 1u32 as *const _;
   |                           ^^^^^^^^ needs more type information
   |
   = note: the type information given here is insufficient to check whether the pointer cast is valid
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0641, E0658.
For more information about an error, try `rustc --explain E0641`.

plain
test [ui] ui/feature-gates/feature-gate-allow-internal-unstable-nested-macro.rs ... ok
test [ui] ui/extern/issue-10764-rpass.rs ... ok
test [ui] ui/feature-gates/feature-gate-allow_fail.rs ... ok
test [ui] ui/extoption_env-not-defined.rs ... ok
test [ui] ui/feature-gates/feature-gate-asm_experimental_arch.rs ... ok
test [ui] ui/feature-gates/feature-gate-assoc-type-defaults.rs ... ok
test [ui] ui/feature-gates/feature-gate-arbitrary_self_types-raw-pointer.rs ... ok
test [ui] ui/feature-gates/feature-gate-arbitrary-self-types.rs ... ok
test [ui] ui/feature-gates/feature-gate-box_patterns.rs ... ok
---

---- [ui] ui/asm/issue-72570.rs stdout ----
diff of stderr:

+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL |         asm!("", in("invalid") "".len());
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
1 error: invalid register `invalid`: unknown register
3    |


4 LL |         asm!("", in("invalid") "".len());
6 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
8 
---
To only update this specific test, also pass `--test-args asm/issue-72570.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-72570.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-72570" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-72570/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         asm!("", in("invalid") "".len());
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error: invalid register `invalid`: unknown register
   |
   |
LL |         asm!("", in("invalid") "".len());

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui/asm/issue-89305.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-89305.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-89305" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-89305/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         let x: () = asm!("nop");
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/asm/naked-functions-ffi.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions-ffi.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-ffi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-ffi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     unsafe { asm!("", options(noreturn)); }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/asm/naked-invalid-attr.rs stdout ----
diff of stderr:

+ error[E0658]: inline assembly is not stable yet on this architecture
+   --> $DIR/naked-invalid-attr.rs:28:18
+    |
+ LL |         unsafe { asm!("", options(noreturn)) }
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error[E0658]: inline assembly is not stable yet on this architecture
+   --> $DIR/naked-invalid-attr.rs:34:14
+    |
+ LL |     unsafe { asm!("", options(noreturn)) }
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error[E0658]: inline assembly is not stable yet on this architecture
+   --> $DIR/naked-invalid-attr.rs:40:18
+    |
+ LL |         unsafe { asm!("", options(noreturn)) }
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error[E0658]: inline assembly is not stable yet on this architecture
+   --> $DIR/naked-invalid-attr.rs:45:18
+    |
+ LL |         unsafe { asm!("", options(noreturn)) }
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
1 error: attribute should be applied to a function definition
2   --> $DIR/naked-invalid-attr.rs:13:1
3    |


38 LL | #![naked]
40 
- error: aborting due to 5 previous errors
+ error: aborting due to 9 previous errors
42 
---
To only update this specific test, also pass `--test-args asm/naked-invalid-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-invalid-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-invalid-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-invalid-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         unsafe { asm!("", options(noreturn)) }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     unsafe { asm!("", options(noreturn)) }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         unsafe { asm!("", options(noreturn)) }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         unsafe { asm!("", options(noreturn)) }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:13:1
   |
   |
LL |   #[naked] //~ ERROR should be applied to a function definition
   |   ^^^^^^^^
LL |   #[repr(C)]
LL | / struct S {
LL | |     a: u32,
LL | |     b: u32,
LL | | }
   | |_- not a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:50:5
   |
   |
LL |     #[naked] || {}; //~ ERROR should be applied to a function definition

error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:21:5
   |
   |
LL |     #[naked] //~ ERROR should be applied to a function definition
LL |     extern "C" fn invoke(&self);
   |     ---------------------------- not a function definition

error: attribute should be applied to a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:9:5
   |
LL |     #[naked] //~ ERROR should be applied to a function definition
LL |     fn f();
   |     ------- not a function definition

error: attribute should be applied to a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:6:1
   |
LL | #![naked] //~ ERROR should be applied to a function definition

error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui/asm/noreturn.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/noreturn.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/noreturn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/noreturn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     let _: () = asm!("");
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     let _: ! = asm!("", options(noreturn));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("", options(noreturn));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/consts/inline_asm.rs stdout ----
diff of stderr:

- error[E0015]: inline assembly is not allowed in constants
+ error[E0658]: inline assembly is not stable yet on this architecture
3    |
3    |
4 LL | const _: () = unsafe { asm!("nop") };
5    |                        ^^^^^^^^^^^
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args consts/inline_asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/inline_asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/inline_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/inline_asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL | const _: () = unsafe { asm!("nop") };
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-global_asm.rs stdout ----
diff of stderr:

7    = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
8    = help: add `#![feature(global_asm)]` to the crate attributes to enable
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL | global_asm!("");
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+    = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 2 previous errors
11 
12 For more information about this error, try `rustc --explain E0658`.
13 
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm/feature-gate-global_asm.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-global_asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-global_asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
   |
   |
LL | global_asm!(""); //~ ERROR `global_asm!` is not stable
   |
   = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
   = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
   = help: add `#![feature(global_asm)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL | global_asm!(""); //~ ERROR `global_asm!` is not stable
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
   = note: this error originates in the macro `global_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.

---
16    = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
17    = help: add `#![feature(naked_functions)]` to the crate attributes to enable
18 
- error: aborting due to 2 previous errors
+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL |     asm!("", options(noreturn))
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL |     asm!("", options(noreturn))
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ error: aborting due to 4 previous errors
20 
21 For more information about this error, try `rustc --explain E0658`.
22 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-naked_functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-naked_functions.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-naked_functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-naked_functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[naked]` attribute is an experimental feature
   |
   |
LL | #[naked]
   |
   = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
   = help: add `#![feature(naked_functions)]` to the crate attributes to enable


error[E0658]: the `#[naked]` attribute is an experimental feature
   |
   |
LL | #[naked]
   |
   = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
   = help: add `#![feature(naked_functions)]` to the crate attributes to enable


error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("", options(noreturn))
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("", options(noreturn))
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/macros/macro-expanded-include/test.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-expanded-include/test.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-expanded-include/test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-expanded-include/test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     () => { unsafe { asm!(include_str!("file.txt")); } }
   |
  ::: /checkout/src/test/ui/macros/macro-expanded-include/test.rs:10:10
   |
   |
LL | fn f() { n!(); }
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/rfc-2091-track-caller/error-with-naked.rs stdout ----
diff of stderr:

+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL |     asm!("", options(noreturn));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
+ error[E0658]: inline assembly is not stable yet on this architecture
+    |
+    |
+ LL |         asm!("", options(noreturn));
+    |
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
+ 
1 error[E0736]: cannot use `#[track_caller]` with `#[naked]`
3    |


10 LL |     #[track_caller]
12 
- error: aborting due to 2 previous errors
+ error: aborting due to 4 previous errors
14 
---
To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-naked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-naked.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("", options(noreturn));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         asm!("", options(noreturn));
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL | #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`


error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL |     #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0658, E0736.
---

---- [ui] ui/unsafe/inline_asm.rs#mir stdout ----
diff of stderr:

- error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
+ error[E0658]: inline assembly is not stable yet on this architecture
3    |
3    |
4 LL |     asm!("nop");
-    |     ^^^^^^^^^^^ use of inline assembly
+    |     ^^^^^^^^^^^
6    |
6    |
-    = note: inline assembly is entirely unchecked and can cause undefined behavior
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
8 
- error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:11:5
-    |
- LL |     llvm_asm!("nop");
-    |     ^^^^^^^^^^^^^^^^ use of inline assembly
-    |
-    = note: inline assembly is entirely unchecked and can cause undefined behavior
-    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- For more information about this error, try `rustc --explain E0133`.
+ For more information about this error, try `rustc --explain E0658`.
+ For more information about this error, try `rustc --explain E0658`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/inline_asm.mir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/unsafe/inline_asm.rs#thir stdout ----
diff of stderr:

- error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
+ error[E0658]: inline assembly is not stable yet on this architecture
3    |
3    |
4 LL |     asm!("nop");
-    |     ^^^^^^^^^^^ use of inline assembly
+    |     ^^^^^^^^^^^
6    |
6    |
-    = note: inline assembly is entirely unchecked and can cause undefined behavior
+    = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
+    = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
8 
- error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:11:5
-    |
- LL |     llvm_asm!("nop");
-    |     ^^^^^^^^^^^^^^^^ use of inline assembly
-    |
-    = note: inline assembly is entirely unchecked and can cause undefined behavior
-    = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- For more information about this error, try `rustc --explain E0133`.
+ For more information about this error, try `rustc --explain E0658`.
+ For more information about this error, try `rustc --explain E0658`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/inline_asm.thir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11717 passed; 12 failed; 645 ignored; 0 measured; 0 filtered out; finished in 84.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:56

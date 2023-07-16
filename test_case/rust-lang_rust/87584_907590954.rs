plain
test [ui] ui/array-slice-vec/vec-matching-autoslice.rs ... ok
test [ui] ui/asm/bad-arch.rs#mirunsafeck ... ok
test [ui] ui/asm/bad-template.rs#aarch64_mirunsafeck ... ok
test [ui] ui/asm/bad-arch.rs#thirunsafeck ... ok
test [ui] ui/asm/bad-template.rs#x86_64_mirunsafeck ... ok
test [ui] ui/asm/bad-template.rs#aarch64_thirunsafeck ... ok
test [ui] ui/asm/issue-87802.rs ... ignored
test [ui] ui/asm/naked-functions-ffi.rs ... ignored
test [ui] ui/asm/naked-functions-unused.rs ... ignored
test [ui] ui/asm/naked-functions-unused.rs ... ignored
test [ui] ui/array-slice-vec/vec-macro-with-trailing-comma.rs ... ok
test [ui] ui/array-slice-vec/subslice-patterns-const-eval.rs ... ok
test [ui] ui/asm/named-asm-labels.rs ... ignored
test [ui] ui/asm/bad-template.rs#x86_64_thirunsafeck ... ok
test [ui] ui/asm/inline-syntax.rs#x86_64 ... ok
test [ui] ui/asm/noreturn.rs ... ok
test [ui] ui/asm/naked-invalid-attr.rs ... ok
test [ui] ui/asm/x86_64/bad-options.rs ... ignored
test [ui] ui/asm/x86_64/bad-reg.rs ... ignored
test [ui] ui/asm/x86_64/const.rs#mirunsafeck ... ignored
test [ui] ui/asm/x86_64/const.rs#thirunsafeck ... ignored
test [ui] ui/asm/x86_64/duplicate-options.rs ... ignored
test [ui] ui/asm/x86_64/interpolated-idents.rs ... ignored
test [ui] ui/asm/x86_64/issue-82869.rs ... ignored
test [ui] ui/asm/x86_64/parse-error.rs ... ignored
test [ui] ui/asm/x86_64/srcloc.rs ... ignored
test [ui] ui/asm/x86_64/sym.rs ... ignored
test [ui] ui/asm/x86_64/type-check-2.rs ... ignored
test [ui] ui/asm/x86_64/type-check-3.rs ... ignored
test [ui] ui/array-slice-vec/vec-repeat-with-cast.rs ... ok
test [ui] ui/asm/naked-functions.rs ... FAILED
test [ui] ui/asm/inline-syntax.rs#arm ... ok
test [ui] ui/array-slice-vec/vec-macro-rvalue-scope.rs ... ok
---

---- [ui] ui/asm/naked-functions.rs stdout ----
diff of stderr:

4 LL |     asm!("", options(readonly, nostack), options(pure));
6 
6 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:36:23
+    |
+ LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:63:10
+    |
+ LL |          in(reg) a,
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:66:10
+    |
+ LL |          inlateout(reg) b,
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:67:10
+    |
+ LL |          inout(reg) c,
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:68:10
+    |
+ LL |          lateout(reg) d,
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+   --> $DIR/naked-functions.rs:69:10
+    |
+ LL |          out(reg) e,
+ 
7 error: patterns not allowed in naked function parameters
8   --> $DIR/naked-functions.rs:14:5
9    |
---
To only update this specific test, also pass `--test-args asm/naked-functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: asm with the `pure` option must have at least one output
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));


error: invalid register class `reg`: unknown register class
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));


error: invalid register class `reg`: unknown register class
   |
   |
LL |          in(reg) a,


error: invalid register class `reg`: unknown register class
   |
   |
LL |          inlateout(reg) b,


error: invalid register class `reg`: unknown register class
   |
   |
LL |          inout(reg) c,


error: invalid register class `reg`: unknown register class
   |
   |
LL |          lateout(reg) d,


error: invalid register class `reg`: unknown register class
   |
   |
LL |          out(reg) e,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:14:5
   |
   |
LL |     mut a: u32,
   |     ^^^^^

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:16:5
   |
LL |     &b: &i32,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:18:6
   |
   |
LL |     (None | Some(_)): Option<std::ptr::NonNull<u8>>,

error: patterns not allowed in naked function parameters
  --> /checkout/src/test/ui/asm/naked-functions.rs:20:5
   |
   |
LL |     P { x, y }: P,

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:30:5
   |
   |
LL |     a + 1
   |     ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     a + 1
   | |     ----- non-asm is unsupported in naked functions
LL | |     //~^ ERROR referencing function parameters is not allowed in naked functions
LL | | }
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:36:31
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   |
   = help: follow the calling convention in asm block to use parameters

warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |     asm!("/* {0} */", in(reg) a, options(noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     (|| a + 1)()
   | |     ------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: only `const` and `sym` operands are supported in naked functions
   |
   |
LL |          in(reg) a,
...
...
LL |          inlateout(reg) b,
   |          ^^^^^^^^^^^^^^^^
LL |          inout(reg) c,
   |          ^^^^^^^^^^^^
LL |          lateout(reg) d,
   |          ^^^^^^^^^^^^^^
LL |          out(reg) e,
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL | /     asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
LL | |          //~^ WARN asm in naked functions must use `noreturn` option
LL | |          //~| WARN this was previously accepted
LL | |          in(reg) a,
LL | |          sym G,
LL | |     );
   | |______^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub unsafe extern "C" fn unsupported_operands() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     let mut a = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut b = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut c = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut d = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     let mut e = 0usize;
   | |     ------------------- non-asm is unsupported in naked functions
LL | |     );
LL | | }
   | |_^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn missing_assembly() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions must contain a single asm block
   |
   |
LL | / pub extern "C" fn too_many_asm_blocks() {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     asm!("");
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("");
   | |     --------- multiple asm blocks are unsupported in naked functions
...  |
LL | |     asm!("", options(noreturn));
   | |     ---------------------------- multiple asm blocks are unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


error: referencing function parameters is not allowed in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:102:11
   |
LL |         *&y
   |           ^
   |
   = help: follow the calling convention in asm block to use parameters

warning: naked functions must contain a single asm block
   |
   |
LL | /     pub extern "C" fn inner(y: usize) -> usize {
LL | |         //~^ WARN naked functions must contain a single asm block
LL | |         //~| WARN this was previously accepted
LL | |         *&y
   | |         --- non-asm is unsupported in naked functions
LL | |         //~^ ERROR referencing function parameters is not allowed in naked functions
LL | |     }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: the LLVM-style inline assembly is unsupported in naked functions
   |
   |
LL |     llvm_asm!("");
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = help: use the new asm! syntax specified in RFC 2873
   = note: this warning originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: naked functions must contain a single asm block
   |
   |
LL | / unsafe extern "C" fn llvm() -> ! {
LL | |     //~^ WARN naked functions must contain a single asm block
LL | |     //~| WARN this was previously accepted
LL | |     llvm_asm!("");
LL | |     core::hint::unreachable_unchecked();
LL | |     core::hint::unreachable_unchecked();
   | |     ------------------------------------ non-asm is unsupported in naked functions
LL | | }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nomem`, `preserves_flags`
   |
   |
LL |     asm!("", options(nomem, preserves_flags, noreturn));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm options unsupported in naked functions: `nostack`, `pure`, `readonly`
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: asm in naked functions must use `noreturn` option
   |
   |
LL |     asm!("", options(readonly, nostack), options(pure));
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:136:15
   |
LL | pub unsafe fn default_abi() {
   |
   |
   = note: `#[warn(undefined_naked_function_abi)]` on by default
warning: Rust ABI is unsupported in naked functions
  --> /checkout/src/test/ui/asm/naked-functions.rs:142:29
   |
   |
LL | pub unsafe extern "Rust" fn rust_abi() {

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:176:1
   |
---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:184:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:192:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:203:1
   |
LL | #[inline(always)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions cannot be inlined
  --> /checkout/src/test/ui/asm/naked-functions.rs:206:1
   |
LL | #[inline(never)]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

---- [ui] ui/asm/type-check-4.rs stdout ----
diff of stderr:

- error[E0506]: cannot assign to `a` because it is borrowed
-   --> $DIR/type-check-4.rs:11:9
+ error: invalid register class `reg`: unknown register class
3    |
- LL |         let p = &a;
- LL |         let p = &a;
-    |                 -- borrow of `a` occurs here
6 LL |         asm!("{}", out(reg) a);
-    |         ^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `a` occurs here
- LL |
- LL |         println!("{}", p);
-    |                        - borrow later used here
11 
11 
- error[E0503]: cannot use `a` because it was mutably borrowed
-   --> $DIR/type-check-4.rs:19:28
+ error: invalid register class `reg`: unknown register class
14    |
- LL |         let p = &mut a;
- LL |         let p = &mut a;
-    |                 ------ borrow of `a` occurs here
17 LL |         asm!("{}", in(reg) a);
-    |                            ^ use of borrowed `a`
- LL |
- LL |         println!("{}", p);
-    |                        - borrow later used here
22 
23 error: aborting due to 2 previous errors
24 


- Some errors have detailed explanations: E0503, E0506.
- For more information about an error, try `rustc --explain E0503`.
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-4/type-check-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/type-check-4.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-4.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", out(reg) a);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", in(reg) a);

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/asm/type-check-1.rs stdout ----
diff of stderr:

25 LL |         asm!("{}", const const_bar(x));
26    |                                    ^ non-constant value
27 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", in(reg) 1 + 2);
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", out(reg) 1 + 2);
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", inout(reg) 1 + 2);
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", in(reg) v[..]);
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", out(reg) v[..]);
+ 
+ 
+ error: invalid register class `reg`: unknown register class
+    |
+    |
+ LL |         asm!("{}", inout(reg) v[..]);
+ 
28 error: invalid asm output
29   --> $DIR/type-check-1.rs:10:29
30    |
30    |

106    = note:     expected type `{integer}`
107            found raw pointer `*mut u8`
- error: aborting due to 13 previous errors
+ error: aborting due to 19 previous errors
110 
111 Some errors have detailed explanations: E0277, E0308, E0435.
111 Some errors have detailed explanations: E0277, E0308, E0435.
112 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/type-check-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/type-check-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:34:26
   |
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const x);
   |                          ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:37:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_foo(x));
   |                                    ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:40:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_bar(x));
   |                                    ^ non-constant value

error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", in(reg) 1 + 2);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", out(reg) 1 + 2);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", inout(reg) 1 + 2);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", in(reg) v[..]);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", out(reg) v[..]);


error: invalid register class `reg`: unknown register class
   |
   |
LL |         asm!("{}", inout(reg) v[..]);

error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:10:29
   |
   |
LL |         asm!("{}", out(reg) 1 + 2);
   |                             ^^^^^ cannot assign to this expression
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:12:31
   |
   |
LL |         asm!("{}", inout(reg) 1 + 2);
   |                               ^^^^^ cannot assign to this expression
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:18:28
   |
   |
LL |         asm!("{}", in(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:20:29
   |
   |
LL |         asm!("{}", out(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:22:31
   |
   |
LL |         asm!("{}", inout(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:48:26
   |
   |
LL |         asm!("{}", const 0f32);
   |                          ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:50:26
   |
   |
LL |         asm!("{}", const 0 as *mut u8);
   |                          ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:52:26
   |
   |
LL |         asm!("{}", const &0);
   |                          ^^ expected integer, found `&{integer}`
help: consider removing the borrow
   |
   |
LL -         asm!("{}", const &0);
LL +         asm!("{}", const 0);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:62:25
   |
   |
LL | global_asm!("{}", const 0f32);
   |                         ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:64:25
   |
   |
LL | global_asm!("{}", const 0 as *mut u8);
   |                         ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error: aborting due to 19 previous errors

Some errors have detailed explanations: E0277, E0308, E0435.
For more information about an error, try `rustc --explain E0277`.
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:03

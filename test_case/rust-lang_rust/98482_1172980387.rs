plain
---- [ui] src/test/ui/asm/aarch64/type-check-2.rs stdout ----
diff of stderr:

22    |
23    = note: `SimdNonCopy` does not implement the Copy trait
24 
- error: cannot use value of type `[closure@$DIR/type-check-2.rs:41:28: 41:38]` for inline assembly
+ error: cannot use value of type `[closure@$DIR/type-check-2.rs:41:28: 41:36]` for inline assembly
27    |
27    |
28 LL |         asm!("{}", in(reg) |x: i32| x);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `sym` operand
   |
   |
LL | global_asm!("{}", sym C);
   |                   ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static

error: invalid `sym` operand
   |
   |
LL |         asm!("{}", sym C);
   |                    ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static
error: arguments for inline assembly must be copyable
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:29:31
   |
   |
LL |         asm!("{:v}", in(vreg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/aarch64/type-check-2.rs:41:28: 41:36]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `Vec<i32>` for inline assembly
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:43:28
   |
   |
LL |         asm!("{}", in(reg) vec![0]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot use value of type `(i32, i32, i32)` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) (1, 2, 3));
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `[i32; 3]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) [1, 2, 3]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `fn() {main}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `&mut i32` for inline assembly
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:58:31
   |
   |
LL |         asm!("{}", inout(reg) r);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: aborting due to 9 previous errors
------------------------------------------



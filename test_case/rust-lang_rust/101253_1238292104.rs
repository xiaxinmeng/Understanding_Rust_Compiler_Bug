plain
---- [ui] src/test/ui/asm/type-check-1.rs stdout ----
diff of stderr:

96    |
97    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
- warning: formatting may not be suitable for sub-register argument
-   --> $DIR/type-check-1.rs:67:27
-    |
-    |
- LL |         asm!( "movd xmm0, {0}", in(reg) val, out("xmm0") _,);
-    |                           ^^^           --- for this argument
-    |
-    = note: `#[warn(asm_sub_register)]` on by default
-    = help: use the `0:e` modifier to have the register formatted as `eax`
-    = help: or use `0:r` modifier to keep the default formatting of `rax`
109 error[E0308]: mismatched types
110   --> $DIR/type-check-1.rs:58:26
111    |


148    = note:     expected type `{integer}`
149            found raw pointer `*mut u8`
- error: aborting due to 17 previous errors; 1 warning emitted
+ error: aborting due to 17 previous errors
152 
153 Some errors have detailed explanations: E0277, E0308, E0435.
153 Some errors have detailed explanations: E0277, E0308, E0435.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
154 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/type-check-1.stderr
To only update this specific test, also pass `--test-args asm/type-check-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/asm/type-check-1.rs:42:26
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
LL |         asm!("{}", const x);
   |                          ^ non-constant value

error[E0435]: attempt to use a non-constant value in a constant
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:45:36
   |
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_foo(x));
   |                                    ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:48:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_bar(x));
   |                                    ^ non-constant value

error: invalid `sym` operand
   |
   |
LL |         asm!("{}", sym x);
   |                        ^ is a local variable
   |
   = help: `sym` operands must refer to either a function or a static
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:15:29
   |
   |
LL |         asm!("{}", out(reg) 1 + 2);
   |                             ^^^^^ cannot assign to this expression
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:17:31
   |
   |
LL |         asm!("{}", inout(reg) 1 + 2);
   |                               ^^^^^ cannot assign to this expression
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:23:28
   |
   |
LL |         asm!("{}", in(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size


error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:26:29
   |
LL |         asm!("{}", out(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size


error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:29:31
   |
LL |         asm!("{}", inout(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size


error: cannot use value of type `[u64]` for inline assembly
  --> /checkout/src/test/ui/asm/type-check-1.rs:23:28
   |
LL |         asm!("{}", in(reg) v[..]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `[u64]` for inline assembly
  --> /checkout/src/test/ui/asm/type-check-1.rs:26:29
   |
   |
LL |         asm!("{}", out(reg) v[..]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `[u64]` for inline assembly
  --> /checkout/src/test/ui/asm/type-check-1.rs:29:31
   |
   |
LL |         asm!("{}", inout(reg) v[..]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:58:26
   |
LL |         asm!("{}", const 0f32);
LL |         asm!("{}", const 0f32);
   |                          ^^^^ expected integer, found `f32`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:60:26
   |
LL |         asm!("{}", const 0 as *mut u8);
   |                          ^^^^^^^^^^^^ expected integer, found `*mut u8`
   = note:     expected type `{integer}`
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:62:26
   |
LL |         asm!("{}", const &0);
LL |         asm!("{}", const &0);
   |                          ^^ expected integer, found `&{integer}`
   |
help: consider removing the borrow
   |
LL -         asm!("{}", const &0);
LL +         asm!("{}", const 0);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:81:25
   |
   |
LL | global_asm!("{}", const 0f32);
   |                         ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:83:25
   |
   |
LL | global_asm!("{}", const 0 as *mut u8);
   |                         ^^^^^^^^^^^^ expected integer, found `*mut u8`
   = note:     expected type `{integer}`
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0277, E0308, E0435.
For more information about an error, try `rustc --explain E0277`.

plain

---- [ui] ui/asm/aarch64/bad-reg.rs stdout ----
diff of stderr:

+ '+fp' is not a recognized feature for this target (ignoring feature)
1 error: invalid register class `foo`: unknown register class
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg/bad-reg.stderr
To only update this specific test, also pass `--test-args asm/aarch64/bad-reg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/bad-reg.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+fp" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg/auxiliary"
stdout: none
--- stderr -------------------------------
'+fp' is not a recognized feature for this target (ignoring feature)
error: invalid register class `foo`: unknown register class
   |
   |
LL |         asm!("{}", in(foo) foo);


error: invalid register `foo`: unknown register
   |
   |
LL |         asm!("", in("foo") foo);

error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:18:15
   |
   |
LL |         asm!("{:z}", in(reg) foo);
   |               ^^^^   ----------- argument
   |               template modifier
   |
   |
   = note: the `reg` register class supports the following template modifiers: `w`, `x`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:20:15
   |
   |
LL |         asm!("{:r}", in(vreg) foo);
   |               ^^^^   ------------ argument
   |               template modifier
   |
   |
   = note: the `vreg` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:22:15
   |
   |
LL |         asm!("{:r}", in(vreg_low16) foo);
   |               ^^^^   ------------------ argument
   |               template modifier
   |
   |
   = note: the `vreg_low16` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`

error: asm template modifiers are not allowed for `const` arguments
   |
   |
LL |         asm!("{:a}", const 0);
   |               ^^^^   ------- argument
   |               template modifier


error: asm template modifiers are not allowed for `sym` arguments
   |
   |
LL |         asm!("{:a}", sym main);
   |               ^^^^   -------- argument
   |               template modifier


error: invalid register `x29`: the frame pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x29") foo);


error: invalid register `sp`: the stack pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("sp") foo);


error: invalid register `xzr`: the zero register cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("xzr") foo);


error: invalid register `x19`: x19 is used internally by LLVM and cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x19") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("p0") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(preg) foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(preg) _);


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, in("w0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `x0`
   |                  register `x0`


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `x0`
   |                  register `x0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, in("q0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `v0`
   |                  register `v0`


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `v0`
   |                  register `v0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);

error: aborting due to 18 previous errors
------------------------------------------


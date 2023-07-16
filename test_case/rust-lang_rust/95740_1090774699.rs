plain

---- [ui] ui/asm/x86_64/bad-reg.rs stdout ----
diff of stderr:

64 LL |         asm!("", in("ip") foo);
66 
66 
- error: invalid register `k0`: the k0 AVX mask register cannot be used as an operand for inline asm
-   --> $DIR/bad-reg.rs:32:18
-    |
- LL |         asm!("", in("k0") foo);
- 
- 
73 error: register class `x87_reg` can only be used as a clobber, not as an input or output
75    |


150 LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
152 
- error: aborting due to 20 previous errors
+ error: aborting due to 19 previous errors
154 
154 
155 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg/bad-reg.stderr
To only update this specific test, also pass `--test-args asm/x86_64/bad-reg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/bad-reg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-reg/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid register class `foo`: unknown register class
   |
   |
LL |         asm!("{}", in(foo) foo);


error: invalid register `foo`: unknown register
   |
   |
LL |         asm!("", in("foo") foo);

error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/x86_64/bad-reg.rs:18:15
   |
   |
LL |         asm!("{:z}", in(reg) foo);
   |               ^^^^   ----------- argument
   |               template modifier
   |
   |
   = note: the `reg` register class supports the following template modifiers: `l`, `x`, `e`, `r`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/x86_64/bad-reg.rs:20:15
   |
   |
LL |         asm!("{:r}", in(xmm_reg) foo);
   |               ^^^^   --------------- argument
   |               template modifier
   |
   |
   = note: the `xmm_reg` register class supports the following template modifiers: `x`, `y`, `z`

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


error: invalid register `ebp`: the frame pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("ebp") foo);


error: invalid register `rsp`: the stack pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("rsp") foo);


error: invalid register `ip`: the instruction pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("ip") foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("st(2)") foo);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("mm0") foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(x87_reg) foo);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(mmx_reg) foo);


error: register class `x87_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(x87_reg) _);


error: register class `mmx_reg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(mmx_reg) _);


error: register `al` conflicts with register `ax`
   |
   |
LL |         asm!("", in("eax") foo, in("al") bar);
   |                  -------------  ^^^^^^^^^^^^ register `al`
   |                  register `ax`


error: register `ax` conflicts with register `ax`
   |
   |
LL |         asm!("", in("rax") foo, out("rax") bar);
   |                  -------------  ^^^^^^^^^^^^^^ register `ax`
   |                  register `ax`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("rax") foo, out("rax") bar);


error: register `ymm0` conflicts with register `xmm0`
   |
   |
LL |         asm!("", in("xmm0") foo, in("ymm0") bar);
   |                  --------------  ^^^^^^^^^^^^^^ register `ymm0`
   |                  register `xmm0`


error: register `ymm0` conflicts with register `xmm0`
   |
   |
LL |         asm!("", in("xmm0") foo, out("ymm0") bar);
   |                  --------------  ^^^^^^^^^^^^^^^ register `ymm0`
   |                  register `xmm0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("xmm0") foo, out("ymm0") bar);

error: aborting due to 19 previous errors
------------------------------------------


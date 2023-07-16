
The job x86_64-gnu-llvm-13 failed! Check out the build log: [(web)](https://github.com/rust-lang/rust/runs/8209338744?check_suite_focus=true) [(plain)](https://github.com/rust-lang/rust/commit/aa662c2ee0ab30df02114622ee3f5e9508814c9e/checks/8209338744/logs)

Click to see the possible cause of the failure (guessed by this bot)
---- [ui] src/test/ui/asm/[type-check-1.rs](http://type-check-1.rs/) stdout ----
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
-    = help: use `0:e` modifier to have the register formatted as `eax`
-    = help: or use `0:r` modifier to keep the default formatting of `rax`


 99 warning: formatting may not be suitable for sub-register argument
100   --> $DIR/type-check-1.rs:67:27
101    |
102 LL |         asm!( "movd xmm0, {0}", in(reg) val, out("xmm0") _,);
103    |                           ^^^           --- for this argument
104    |
105    = note: `#[warn(asm_sub_register)]` on by default
106    = help: use `0:e` modifier to have the register formatted as `eax`
107    = help: or use `0:r` modifier to keep the default formatting of `rax`

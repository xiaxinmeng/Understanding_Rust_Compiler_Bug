
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:51
#1  0x00007ffff6dd5801 in __GI_abort () at abort.c:79
#2  0x00007ffff6dc539a in __assert_fail_base (fmt=0x7ffff6f4c7d8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", 
    assertion=assertion@entry=0x7fffe8b466d6 <.str.llvm> "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"", 
    file=file@entry=0x7fffe831cf85 <.str.98.llvm> "/checkout/src/llvm-project/llvm/include/llvm/Support/Casting.h", line=line@entry=255, 
    function=function@entry=0x7fffe8412fab "typename cast_retty<X, Y *>::ret_type llvm::cast(Y *) [X = llvm::GlobalValue, Y = llvm::Value]")
    at assert.c:92
#3  0x00007ffff6dc5412 in __GI___assert_fail (
    assertion=0x7fffe8b466d6 <.str.llvm> "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"", 
    file=0x7fffe831cf85 <.str.98.llvm> "/checkout/src/llvm-project/llvm/include/llvm/Support/Casting.h", line=255, 
    function=0x7fffe8412fab "typename cast_retty<X, Y *>::ret_type llvm::cast(Y *) [X = llvm::GlobalValue, Y = llvm::Value]") at assert.c:101
#4  0x00007fffe9c11f94 in LLVMSetLinkage ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8-rust-1.35.0-nightly.so
#5  0x00007fffece481fd in rustc_codegen_llvm::consts::check_and_apply_linkage ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffecd811f4 in rustc_codegen_llvm::consts::<impl rustc_codegen_llvm::context::CodegenCx>::get_static ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffecd039e2 in rustc_codegen_ssa::mir::place::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_place ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffecd07a7d in rustc_codegen_ssa::mir::operand::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_consume ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffecd07b25 in rustc_codegen_ssa::mir::operand::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_operand ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffecd055e3 in rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_rvalue_operand ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffeccfb3e3 in rustc_codegen_ssa::mir::codegen_mir ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffecd9e507 in rustc_codegen_ssa::base::codegen_instance ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffeccf3f43 in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen ()
   from /home/nikic/.rustup/toolchains/3750348daff89741e3153e0e120aa70a45ff5b68-alt/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so

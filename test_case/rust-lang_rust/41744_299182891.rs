
(lldb) r
Process 34235 launched: '~/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc' (x86_64)
Assertion failed: (C->getType()->getScalarSizeInBits() > Ty->getScalarSizeInBits()&& "SrcTy must be larger than DestTy for Trunc!"), function getTrunc, file /Users/travis/build/rust-lang/rust/src/llvm/lib/IR/Constants.cpp, line 1568.
Process 34235 stopped
* thread #2, name = 'rustc', stop reason = signal SIGABRT
    frame #0: 0x00007fffc99b9d42 libsystem_kernel.dylib`__pthread_kill + 10
libsystem_kernel.dylib`__pthread_kill:
->  0x7fffc99b9d42 <+10>: jae    0x7fffc99b9d4c            ; <+20>
    0x7fffc99b9d44 <+12>: movq   %rax, %rdi
    0x7fffc99b9d47 <+15>: jmp    0x7fffc99b2caf            ; cerror_nocancel
    0x7fffc99b9d4c <+20>: retq   
(lldb) bt
* thread #2, name = 'rustc', stop reason = signal SIGABRT
  * frame #0: 0x00007fffc99b9d42 libsystem_kernel.dylib`__pthread_kill + 10
    frame #1: 0x00007fffc9aa75bf libsystem_pthread.dylib`pthread_kill + 90
    frame #2: 0x00007fffc991f420 libsystem_c.dylib`abort + 129
    frame #3: 0x00007fffc98e6893 libsystem_c.dylib`__assert_rtn + 320
    frame #4: 0x0000000103f45f84 librustc_llvm-a020c9c42ea0ddd7.dylib`llvm::ConstantExpr::getTrunc(llvm::Constant*, llvm::Type*, bool) + 484
    frame #5: 0x0000000100fd13ec librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::constant::MirConstContext::const_lvalue::hcabbdbff5e8ff317 + 2668
    frame #6: 0x0000000100fd2422 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::constant::MirConstContext::const_rvalue::h19d240f0e896d085 + 418
    frame #7: 0x0000000100fcfad0 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::constant::MirConstContext::trans::ha4a4f90ba0446139 + 1840
    frame #8: 0x0000000100fd6749 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::constant::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_constant::hfab8591e311015c1 + 697
    frame #9: 0x0000000100fd9f91 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::operand::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_operand::he38cb0ddc21903f4 + 49
    frame #10: 0x0000000100fdb6f1 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::rvalue::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_rvalue_operand::hee4d435691371db0 + 129
    frame #11: 0x0000000100fc6944 librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::block::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_block::he55852e7fc7fd602 + 4404
    frame #12: 0x0000000100fc351c librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::mir::trans_mir::h2b1fef795fef4867 + 16268
    frame #13: 0x0000000100fe875e librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::trans_item::TransItem::define::h66a94f3ed0cbd750 + 2414
    frame #14: 0x0000000100f8190b librustc_trans-2a23ec83470b8da3.dylib`rustc_trans::base::trans_crate::hc8538e6b8fc181e8 + 12955
    frame #15: 0x000000010019706f librustc_driver-45e2d96c004bc54e.dylib`rustc_driver::driver::phase_4_translate_to_llvm::h8794d833f6109935 + 1567
<snip>

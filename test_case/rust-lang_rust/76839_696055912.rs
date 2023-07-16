gdb
warning: used $at (currently $1) without ".set noat"
  --> /home/lzutao/fork/rust/worktree/mips/src/test/assembly/asm/mips-types.rs:52:15
   |
52 |         asm!("move {}, {}", out($class) y, in($class) x);
   |               ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:11
   |
1  |     move $2, $1
   |              ^

rustc: /home/lzutao/fork/rust/worktree/mips/src/llvm-project/llvm/include/llvm/CodeGen/TargetLowering.h:831: virtual const llvm::TargetRegisterClass* llvm::TargetLoweringBase::getRegClassFor(llvm::MVT, bool) const: Assertion `RC && "This value type is not natively supported!"' failed.
Process 5450 stopped
* thread #5, name = 'rustc', stop reason = hit program assert
    frame #4: 0x00007ffff04c26a3 librustc_driver-ff4867624e9288b7.so`llvm::TargetLoweringBase::getRegClassFor(isDivergent=<unavailable>, VT=<unavailable>, this=<unavailable>) const at TargetLowering.h:831:5
   828    virtual const TargetRegisterClass *getRegClassFor(MVT VT, bool isDivergent = false) const {
   829      (void)isDivergent;
   830      const TargetRegisterClass *RC = RegClassForVT[VT.SimpleTy];
-> 831      assert(RC && "This value type is not natively supported!");
   832      return RC;
   833    }
   834

 shell
rustc: /rust-master/src/llvm/lib/Transforms/Utils/PromoteMemoryToRegister.cpp:531: void {anonymous}::PromoteMem2Reg::run(): Assertion `isAllocaPromotable(AI) && "Cannot promote non-promotable alloca!"' failed.
(gdb) bt
#0  __libc_do_syscall () at ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S:44
#1  0xb69b1f0e in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#2  0xb69b4766 in __GI_abort () at abort.c:89
#3  0xb69ad150 in __assert_fail_base (fmt=0x1 <error: Cannot access memory at address 0x1>, 
    assertion=0xb5d72c78 "isAllocaPromotable(AI) && \"Cannot promote non-promotable alloca!\"", assertion@entry=0x0, 
    file=0xb5d72b28 "/rust-master/src/llvm/lib/Transforms/Utils/PromoteMemoryToRegister.cpp", file@entry=0xb1f882b0 "\001", line=531, line@entry=3064397996, 
    function=function@entry=0xb5d6fe50 "void {anonymous}::PromoteMem2Reg::run()") at assert.c:92
#4  0xb69ad1e6 in __GI___assert_fail (assertion=0x0, file=0xb1f882b0 "\001", line=3064397996, 
    function=0xb5d6fe50 "void {anonymous}::PromoteMem2Reg::run()") at assert.c:101
#5  0xb4e45db0 in ?? () from /home/odroid/rust-nightly-sysalloc-assert/bin/../lib/librustc_llvm-9026086f.so

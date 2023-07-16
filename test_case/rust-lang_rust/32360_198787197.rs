 shell
src/llvm/lib/IR/Attributes.cpp:876: llvm::AttributeSet llvm::AttributeSet::removeAttributes(llvm::LLVMContext&, unsigned int, llvm::AttributeSet) const: Assertion `!Attrs.hasAttribute(Index, Attribute::Alignment) && "Attempt to change alignment!"' failed.

(gdb) bt
#0  __libc_do_syscall () at ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S:44
#1  0xb6a30f0e in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#2  0xb6a33766 in __GI_abort () at abort.c:89
#3  0xb6a2c150 in __assert_fail_base (fmt=0x1 <error: Cannot access memory at address 0x1>, 
    assertion=0xb41c8e88 "!Attrs.hasAttribute(Index, Attribute::Alignment) && \"Attempt to change alignment!\"", assertion@entry=0x0, 
    file=0xb41c8434 "/src/llvm/lib/IR/Attributes.cpp", file@entry=0xb244f2a0 "\001", line=876, line@entry=3064918188, 
    function=function@entry=0xb4c744f0 "llvm::AttributeSet llvm::AttributeSet::removeAttributes(llvm::LLVMContext&, unsigned int, llvm::AttributeSet) const") at assert.c:92
#4  0xb6a2c1e6 in __GI___assert_fail (assertion=0x0, file=0xb244f2a0 "\001", line=3064918188, 
    function=0xb4c744f0 "llvm::AttributeSet llvm::AttributeSet::removeAttributes(llvm::LLVMContext&, unsigned int, llvm::AttributeSet) const") at assert.c:101
#5  0xb3dc95cc in llvm::AttributeSet::removeAttributes(llvm::LLVMContext&, unsigned int, llvm::AttributeSet) const ()
   from /arm-unknown-linux-gnueabihf/stage1/bin/../lib/librustc_llvm-9026086f.so
#6  0xb3e09170 in LLVMRemoveFunctionAttr ()
   from /arm-unknown-linux-gnueabihf/stage1/bin/../lib/librustc_llvm-9026086f.so
#7  0xb628e5ec in trans::attributes::from_fn_attrs::h75cc605b937fa118Rlh ()
   from /arm-unknown-linux-gnueabihf/stage1/bin/../lib/librustc_trans-9026086f.so
#8  0xb6258b7c in ?? ()
   from /arm-unknown-linux-gnueabihf/stage1/bin/../lib/librustc_trans-9026086f.so


rustc: malloc.c:4028: _int_malloc: Assertion `(unsigned long) (size) >= (unsigned long) (nb)' failed.
malloc(): invalid size (unsorted)
[New Thread 0xae0fa100 (LWP 20685)]

Thread 34 "rustc" received signal SIGABRT, Aborted.
[Switching to Thread 0xae4fc100 (LWP 20681)]
__GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
50	../sysdeps/unix/sysv/linux/raise.c: No such file or directory.
(gdb) 
(gdb) where
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0xb0e41230 in __GI_abort () at abort.c:79
#2  0xb0e97fb0 in __malloc_assert (assertion=<optimized out>, file=<optimized out>, 
    line=line@entry=4028, 
    function=0xb0f58514 <__PRETTY_FUNCTION__.17241> "_int_malloc") at malloc.c:298
#3  0xb0e9bc6c in _int_malloc (av=av@entry=0xabc00010, bytes=bytes@entry=24)
    at malloc.c:4028
#4  0xb0e9d318 in __GI___libc_malloc (bytes=24) at malloc.c:3057
#5  0xb3ac75ac in operator new (sz=24)
    at /tmp/build/.build/src/gcc-5.2.0/libstdc++-v3/libsupc++/new_op.cc:50
#6  0xb367f750 in llvm::Attribute::get(llvm::LLVMContext&, llvm::Attribute::AttrKind, unsigned long long) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#7  0xb367faf4 in llvm::Attribute::getWithDereferenceableBytes(llvm::LLVMContext&, unsigned long long) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#8  0xb3685330 in llvm::AttributeSetNode::get(llvm::LLVMContext&, llvm::AttrBuilder const&) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#9  0xb36853ac in llvm::AttributeSet::get(llvm::LLVMContext&, llvm::AttrBuilder const&) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#10 0xb36868e0 in llvm::AttributeList::get(llvm::LLVMContext&, unsigned int, llvm::AttrBuilder const&) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#11 0xb3637b6c in (anonymous namespace)::BitcodeReader::parseAttributeGroupBlock()
    ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib/librustc_driver-f52089ea7176cf29.so
#12 0xb364d5a4 in (anonymous namespace)::BitcodeReader::parseModule(unsigned long long, bool) ()
   from /home/joshua/.rustup/toolchains/stable-arm-unknown-linux-gnueabihf/bin/../lib--Type <RET> for more, q to quit, c to continue without paging--
/librustc_driver-f52089ea7176cf29.so
Backtrace stopped: previous frame inner to this frame (corrupt stack?)

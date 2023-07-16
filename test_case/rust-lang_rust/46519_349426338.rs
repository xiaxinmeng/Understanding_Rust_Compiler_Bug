rust
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:51
#1  0x00007ffff726ff5d in __GI_abort () at abort.c:90
#2  0x00007ffff7265f17 in __assert_fail_base (fmt=<optimized out>, 
    assertion=assertion@entry=0x7fffef3ea0e0 "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"", 
    file=file@entry=0x7fffef547a20 "/checkout/src/llvm/include/llvm/Support/Casting.h", line=line@entry=236, 
    function=function@entry=0x7ffff0539ba0 <_ZZN4llvm4castINS_11IntegerTypeENS_4TypeEEENS_10cast_rettyIT_PT0_E8ret_typeES6_E19__PRETTY_FUNCTION__> "typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::IntegerType; Y = llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = llvm::IntegerType*]") at assert.c:92
#3  0x00007ffff7265fc2 in __GI___assert_fail (
    assertion=0x7fffef3ea0e0 "isa<X>(Val) && \"cast<Ty>() argument of incompatible type!\"", 
    file=0x7fffef547a20 "/checkout/src/llvm/include/llvm/Support/Casting.h", line=236, 
    function=0x7ffff0539ba0 <_ZZN4llvm4castINS_11IntegerTypeENS_4TypeEEENS_10cast_rettyIT_PT0_E8ret_typeES6_E19__PRETTY_FUNCTION__> "typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::IntegerType; Y = llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = llvm::IntegerType*]") at assert.c:101
#4  0x00007fffef0f5eca in LLVMConstIntOfArbitraryPrecision ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/../lib/librustc_llvm-fe79b73fb19ea62e.so
#5  0x00007ffff5a400c7 in rustc_trans::mir::place::PlaceRef::trans_set_discr ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so
#6  0x00007ffff5a51546 in rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_rvalue ()
   from /home/simon/tmp/servo-share/rust/1956d5535ad77ddf46e4b29ba089a8b4a73cfaea-alt/bin/../lib/../lib/librustc_trans-84cc35c4fa80a856.so

[…]

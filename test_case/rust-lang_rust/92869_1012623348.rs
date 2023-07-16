asm
0000000000007010 <std::thread::local::LocalKey<T>::with>:
    7010:	48 83 ec 18          	sub    $0x18,%rsp
    7014:	ff 17                	call   *(%rdi)
    7016:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
    701b:	48 83 f8 00          	cmp    $0x0,%rax
    701f:	75 2a                	jne    704b <std::thread::local::LocalKey<T>::with+0x3b>
    7021:	48 8d 3d 2c 50 08 00 	lea    0x8502c(%rip),%rdi        # 8c054 <anon.dda6e9c32c792d26f11e51339977f165.0.llvm.9859745662360967200>
    7028:	48 8d 0d 21 04 09 00 	lea    0x90421(%rip),%rcx        # 97450 <anon.dda6e9c32c792d26f11e51339977f165.3.llvm.9859745662360967200>
    702f:	4c 8d 05 02 04 09 00 	lea    0x90402(%rip),%r8        # 97438 <anon.dda6e9c32c792d26f11e51339977f165.2.llvm.9859745662360967200>
    7036:	48 8d 05 33 2f 00 00 	lea    0x2f33(%rip),%rax        # 9f70 <core::result::unwrap_failed>
    703d:	be 46 00 00 00       	mov    $0x46,%esi
    7042:	48 8d 54 24 10       	lea    0x10(%rsp),%rdx
    7047:	ff d0                	call   *%rax
    7049:	0f 0b                	ud2    
    704b:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi

0000000000007050 <alloc::raw_vec::RawVec<T,A>::current_memory>:
[...]

0000000000007980 <bug::thread_func>:
    7980:	48 8d 3d e9 fa 08 00 	lea    0x8fae9(%rip),%rdi        # 97470 <anon.dda6e9c32c792d26f11e51339977f165.3.llvm.9859745662360967200+0x20>
    7987:	e9 84 f6 ff ff       	jmp    7010 <std::thread::local::LocalKey<T>::with>
    798c:	0f 1f 40 00          	nopl   0x0(%rax)


     _ZN4main23count_non_comment_bytesE():
       nop
                 (State::Inside, b'*') => state = State::EndAsterisk,
300:   cmp    $0x2a,%al
       sete   %r12b
       or     $0x2,%r12b
     _ZN3std2io8buffered8{{impl}}23fill_buf<std::fs::File>E():
30a:   mov    -0x40(%rbp),%r14
       mov    -0x38(%rbp),%r15
     _ZN3std2io8buffered8{{impl}}19read<std::fs::File>E():
       cmp    %r15,%r14
     ↓ jne    360
     _ZN3std2io8buffered8{{impl}}23fill_buf<std::fs::File>E():
       mov    -0x48(%rbp),%rcx
     _ZN3std2io8buffered8{{impl}}19read<std::fs::File>E():
       cmp    $0x1,%rcx
     ↓ jbe    3c0
     _ZN3std2io8buffered8{{impl}}23fill_buf<std::fs::File>E():
       mov    -0x50(%rbp),%rdx
       lea    -0x130(%rbp),%rdi
       lea    -0x58(%rbp),%rsi
     → callq  _$LT$std..fs..File$u20$as$u20$std..io..Read$GT$::read::h68b68e96a3ee717e
       mov    -0x128(%rbp),%r15
     _ZN4core3ops8{{impl}}89translate<usize,std::io::error::Error,core::result::Result<usize, std::io::error::Error>>E():
       cmpq   $0x1,-0x130(%rbp)
     ↓ je     540
     _ZN3std2io8buffered8{{impl}}23fill_buf<std::fs::File>E():
       mov    %r15,-0x38(%rbp)
       movq   $0x0,-0x40(%rbp)
     _ZN3std2io8buffered8{{impl}}22consume<std::fs::File>E():
       xor    %r14d,%r14d
     ↓ jmp    369
     _ZN4core5slice8{{impl}}9index<u8>E():
       nop
360:   cmp    %r14,%r15
     ↓ jb     62c
     _ZN3std2io8buffered8{{impl}}23fill_buf<std::fs::File>E():
369:   mov    -0x48(%rbp),%rsi
     _ZN4core5slice8{{impl}}9index<u8>E():
       cmp    %rsi,%r15
     ↓ ja     624
       mov    -0x50(%rbp),%rsi
       add    %r14,%rsi
     _ZN4core3cmp5impls8{{impl}}2leE():
       xor    %ebx,%ebx
       cmp    %r14,%r15
       setne  %bl
       lea    -0x29(%rbp),%rdi
     _ZN4core5slice8{{impl}}19copy_from_slice<u8>E():
       mov    %rbx,%rdx
     → callq  memcpy@plt # <--- Note the call here
     _ZN3std2io8buffered8{{impl}}22consume<std::fs::File>E():
       add    %rbx,%r14
     _ZN4core3cmp5impls8{{impl}}2leE():
       cmp    %r15,%r14

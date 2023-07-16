
(lldb) bt
* thread #2, name = 'rustc', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x000000010162c7e8 librustc-863c36ebb34cad14.dylib`rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::h8d48a4ca8ec2db40 + 760
    frame #1: 0x00000001016373a9 librustc-863c36ebb34cad14.dylib`rustc::ich::impls_syntax::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$$u5b$syntax..ast..Attribute$u5d$$GT$::hash_stable::hcbeec66e1d823fad + 2681
    frame #2: 0x00000001015fe649 librustc-863c36ebb34cad14.dylib`_$LT$rustc..hir..map..collector..HirItemLike$LT$T$GT$$u20$as$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$hir$GT$$GT$$GT$::hash_stable::hbcbbaf6c54b1cc3e + 201
    frame #3: 0x000000010156ea4c librustc-863c36ebb34cad14.dylib`rustc::dep_graph::graph::DepGraph::with_task::h2676a93b883dbd65 + 2764
    frame #4: 0x00000001015fbee2 librustc-863c36ebb34cad14.dylib`_$LT$rustc..hir..map..collector..NodeCollector$LT$$u27$a$C$$u20$$u27$hir$GT$$u20$as$u20$rustc..hir..intravisit..Visitor$LT$$u27$hir$GT$$GT$::visit_item::h8804047228cc6c2e + 338
    frame #5: 0x000000010160de38 librustc-863c36ebb34cad14.dylib`rustc::hir::map::map_crate::h81da5cba736e888a + 4744
    frame #6: 0x00000001001623a3 librustc_driver-316df4eccf83bc20.dylib`rustc_driver::driver::compile_input::h50b15af9e8f0bc9e + 9907
    frame #7: 0x000000010017f60f librustc_driver-316df4eccf83bc20.dylib`rustc_driver::run_compiler::he5fca2bf13208946 + 4479
    frame #8: 0x00000001000a7b56 librustc_driver-316df4eccf83bc20.dylib`std::sys_common::backtrace::__rust_begin_short_backtrace::he6dd637274e4e34c + 1046
    frame #9: 0x0000000102d997ff libstd-ab14014bac7f480c.dylib`__rust_maybe_catch_panic + 31
    frame #10: 0x00000001000e4366 librustc_driver-316df4eccf83bc20.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h3dd84feee2236c8c + 134
    frame #11: 0x0000000102d8e41c libstd-ab14014bac7f480c.dylib`std::sys::unix::thread::Thread::new::thread_start::h8d626b697ba2edfb + 140
    frame #12: 0x00007fff7a5406c1 libsystem_pthread.dylib`_pthread_body + 340
    frame #13: 0x00007fff7a54056d libsystem_pthread.dylib`_pthread_start + 377
    frame #14: 0x00007fff7a53fc5d libsystem_pthread.dylib`thread_start + 13
(lldb) disass
librustc-863c36ebb34cad14.dylib`rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::h8d48a4ca8ec2db40:
    0x10162c4f0 <+0>:    push   rbp
    0x10162c4f1 <+1>:    mov    rbp, rsp
    0x10162c4f4 <+4>:    push   r15
    0x10162c4f6 <+6>:    push   r14
    0x10162c4f8 <+8>:    push   r13
    0x10162c4fa <+10>:   push   r12
    0x10162c4fc <+12>:   push   rbx
    0x10162c4fd <+13>:   sub    rsp, 0xc8
    0x10162c504 <+20>:   mov    r14, rdx
    0x10162c507 <+23>:   mov    r12, rsi
    0x10162c50a <+26>:   mov    rbx, rdi
    0x10162c50d <+29>:   mov    qword ptr [rbp - 0x50], r12
    0x10162c511 <+33>:   cmp    byte ptr [r12 + 0xb8], 0x0
    0x10162c51a <+42>:   je     0x10162c656               ; <+358>
    0x10162c520 <+48>:   lea    rsi, [rip + 0x448879]     ; str.1zW + 33680
    0x10162c527 <+55>:   mov    rdi, rbx
    0x10162c52a <+58>:   call   0x1019dcc2e               ; symbol stub for: _$LT$syntax_pos..span_encoding..Span$u20$as$u20$core..cmp..PartialEq$GT$::eq::h0e56e2d603ee2cd2
    0x10162c52f <+63>:   test   al, al
    0x10162c531 <+65>:   jne    0x10162c637               ; <+327>
    0x10162c537 <+71>:   mov    esi, dword ptr [rbx]
    0x10162c539 <+73>:   lea    rdi, [rbp - 0x48]
    0x10162c53d <+77>:   call   0x1013c2410               ; syntax_pos::span_encoding::Span::data::h6f31e1db7305e513
    0x10162c542 <+82>:   mov    r15d, dword ptr [rbp - 0x48]
    0x10162c546 <+86>:   mov    r13d, dword ptr [rbp - 0x44]
    0x10162c54a <+90>:   cmp    r13d, r15d
    0x10162c54d <+93>:   jb     0x10162c637               ; <+327>
    0x10162c553 <+99>:   lea    rbx, [r12 + 0x30]
    0x10162c558 <+104>:  cmp    qword ptr [r12 + 0x30], 0x0
    0x10162c55e <+110>:  jne    0x10162c5cf               ; <+223>
    0x10162c560 <+112>:  mov    rsi, qword ptr [r12 + 0x28]
    0x10162c565 <+117>:  lea    rdi, [rbp - 0xd8]
    0x10162c56c <+124>:  call   0x10162ba30               ; rustc::ich::caching_codemap_view::CachingCodemapView::new::h8caa73032c4551e5
    0x10162c571 <+129>:  cmp    qword ptr [r12 + 0x30], 0x0
    0x10162c577 <+135>:  je     0x10162c5b1               ; <+193>
    0x10162c579 <+137>:  lea    rax, [r12 + 0x60]
    0x10162c57e <+142>:  mov    qword ptr [rbp - 0x30], rax
    0x10162c582 <+146>:  lea    rdi, [r12 + 0x48]
    0x10162c587 <+151>:  call   0x10141e640               ; core::ptr::drop_in_place::h756bf3ffd5133862
    0x10162c58c <+156>:  lea    rax, [r12 + 0x88]
    0x10162c594 <+164>:  mov    qword ptr [rbp - 0x30], rax
    0x10162c598 <+168>:  lea    rdi, [r12 + 0x70]
    0x10162c59d <+173>:  call   0x10141e640               ; core::ptr::drop_in_place::h756bf3ffd5133862
    0x10162c5a2 <+178>:  add    r12, 0x98
    0x10162c5a9 <+185>:  mov    rdi, r12
    0x10162c5ac <+188>:  call   0x10141e640               ; core::ptr::drop_in_place::h756bf3ffd5133862
    0x10162c5b1 <+193>:  lea    rsi, [rbp - 0xd8]
    0x10162c5b8 <+200>:  mov    edx, 0x88
    0x10162c5bd <+205>:  mov    rdi, rbx
    0x10162c5c0 <+208>:  call   0x1019dd246               ; symbol stub for: memcpy
    0x10162c5c5 <+213>:  cmp    qword ptr [rbx], 0x0
    0x10162c5c9 <+217>:  je     0x10162c8bd               ; <+973>
    0x10162c5cf <+223>:  lea    rdi, [rbp - 0xf0]
    0x10162c5d6 <+230>:  mov    rsi, rbx
    0x10162c5d9 <+233>:  mov    edx, r15d
    0x10162c5dc <+236>:  call   0x10162bbd0               ; rustc::ich::caching_codemap_view::CachingCodemapView::byte_pos_to_line_and_col::hf3c0193086c7751b
    0x10162c5e1 <+241>:  mov    r12, qword ptr [rbp - 0xf0]
    0x10162c5e8 <+248>:  test   r12, r12
    0x10162c5eb <+251>:  je     0x10162c637               ; <+327>
    0x10162c5ed <+253>:  mov    rbx, qword ptr [rbp - 0xe8]
    0x10162c5f4 <+260>:  movzx  eax, byte ptr [rbp - 0xe0]
    0x10162c5fb <+267>:  mov    qword ptr [rbp - 0x38], r12
    0x10162c5ff <+271>:  cmp    dword ptr [r12 + 0x104], r13d
    0x10162c607 <+279>:  ja     0x10162c613               ; <+291>
    0x10162c609 <+281>:  cmp    dword ptr [r12 + 0x108], r13d
    0x10162c611 <+289>:  jae    0x10162c668               ; <+376>
    0x10162c613 <+291>:  mov    byte ptr [rbp - 0xd8], 0x1
    0x10162c61a <+298>:  lea    rsi, [rbp - 0xd8]
    0x10162c621 <+305>:  mov    edx, 0x1
    0x10162c626 <+310>:  mov    rdi, r14
    0x10162c629 <+313>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c62e <+318>:  inc    qword ptr [r14 + 0x48]
    0x10162c632 <+322>:  jmp    0x10162c7cc               ; <+732>
    0x10162c637 <+327>:  mov    byte ptr [rbp - 0xd8], 0x1
    0x10162c63e <+334>:  lea    rsi, [rbp - 0xd8]
    0x10162c645 <+341>:  mov    edx, 0x1
    0x10162c64a <+346>:  mov    rdi, r14
    0x10162c64d <+349>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c652 <+354>:  inc    qword ptr [r14 + 0x48]
    0x10162c656 <+358>:  add    rsp, 0xc8
    0x10162c65d <+365>:  pop    rbx
    0x10162c65e <+366>:  pop    r12
    0x10162c660 <+368>:  pop    r13
    0x10162c662 <+370>:  pop    r14
    0x10162c664 <+372>:  pop    r15
    0x10162c666 <+374>:  pop    rbp
    0x10162c667 <+375>:  ret    
    0x10162c668 <+376>:  mov    dword ptr [rbp - 0x30], eax
    0x10162c66b <+379>:  mov    byte ptr [rbp - 0xd8], 0x0
    0x10162c672 <+386>:  lea    rsi, [rbp - 0xd8]
    0x10162c679 <+393>:  mov    edx, 0x1
    0x10162c67e <+398>:  mov    rdi, r14
    0x10162c681 <+401>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c686 <+406>:  inc    qword ptr [r14 + 0x48]
    0x10162c68a <+410>:  mov    rax, qword ptr [r12 + 0xf0]
    0x10162c692 <+418>:  mov    qword ptr [rbp - 0xd8], rax
    0x10162c699 <+425>:  lea    rsi, [rbp - 0xd8]
    0x10162c6a0 <+432>:  mov    edx, 0x8
    0x10162c6a5 <+437>:  mov    rdi, r14
    0x10162c6a8 <+440>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c6ad <+445>:  add    qword ptr [r14 + 0x48], 0x8
    0x10162c6b2 <+450>:  mov    edi, r13d
    0x10162c6b5 <+453>:  mov    esi, r15d
    0x10162c6b8 <+456>:  call   0x1019dcc04               ; symbol stub for: _$LT$syntax_pos..BytePos$u20$as$u20$core..ops..arith..Sub$GT$::sub::hdb689f5a05767d93
    0x10162c6bd <+461>:  and    rbx, 0xffffff
    0x10162c6c4 <+468>:  shl    rbx, 0x8
    0x10162c6c8 <+472>:  mov    ecx, dword ptr [rbp - 0x30]
    0x10162c6cb <+475>:  or     rcx, rbx
    0x10162c6ce <+478>:  shl    rax, 0x20
    0x10162c6d2 <+482>:  or     rax, rcx
    0x10162c6d5 <+485>:  mov    qword ptr [rbp - 0xd8], rax
    0x10162c6dc <+492>:  lea    rsi, [rbp - 0xd8]
    0x10162c6e3 <+499>:  mov    edx, 0x8
    0x10162c6e8 <+504>:  mov    rdi, r14
    0x10162c6eb <+507>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c6f0 <+512>:  add    qword ptr [r14 + 0x48], 0x8
    0x10162c6f5 <+517>:  cmp    dword ptr [rbp - 0x40], 0x0
    0x10162c6f9 <+521>:  je     0x10162c7a8               ; <+696>
    0x10162c6ff <+527>:  mov    byte ptr [rbp - 0xd8], 0x0
    0x10162c706 <+534>:  lea    rsi, [rbp - 0xd8]
    0x10162c70d <+541>:  mov    edx, 0x1
    0x10162c712 <+546>:  mov    rdi, r14
    0x10162c715 <+549>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c71a <+554>:  inc    qword ptr [r14 + 0x48]
    0x10162c71e <+558>:  lea    rdi, [rip + 0x4f723b]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c725 <+565>:  call   qword ptr [rdi]
    0x10162c727 <+567>:  cmp    byte ptr [rax + 0x21], 0x0
    0x10162c72b <+571>:  jne    0x10162c8cb               ; <+987>
    0x10162c731 <+577>:  lea    rdi, [rip + 0x4f7228]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c738 <+584>:  call   qword ptr [rdi]
    0x10162c73a <+586>:  cmp    byte ptr [rax + 0x20], 0x0
    0x10162c73e <+590>:  jne    0x10162c765               ; <+629>
    0x10162c740 <+592>:  lea    rdi, [rip + 0x4f7219]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c747 <+599>:  call   qword ptr [rdi]
    0x10162c749 <+601>:  lea    rsi, [rip - 0x2513f0]     ; std::thread::local::fast::destroy_value::h795e0dcafb24f001
    0x10162c750 <+608>:  mov    rdi, rax
    0x10162c753 <+611>:  call   0x1019dcdc0               ; symbol stub for: std::sys::unix::fast_thread_local::register_dtor::h4e4cabb233ebf8c9
    0x10162c758 <+616>:  lea    rdi, [rip + 0x4f7201]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c75f <+623>:  call   qword ptr [rdi]
    0x10162c761 <+625>:  mov    byte ptr [rax + 0x20], 0x1
    0x10162c765 <+629>:  lea    rdi, [rip + 0x4f71f4]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c76c <+636>:  call   qword ptr [rdi]
    0x10162c76e <+638>:  cmp    qword ptr [rax + 0x18], 0x0
    0x10162c773 <+643>:  je     0x10162c7da               ; <+746>
    0x10162c775 <+645>:  lea    rdi, [rip + 0x4f71e4]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c77c <+652>:  call   qword ptr [rdi]
    0x10162c77e <+654>:  lea    rdi, [rbp - 0x48]
    0x10162c782 <+658>:  lea    rsi, [rbp - 0x50]
    0x10162c786 <+662>:  mov    rdx, rax
    0x10162c789 <+665>:  call   0x10162c940               ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::_$u7b$$u7b$closure$u7d$$u7d$::h3094acc762e79e5f
    0x10162c78e <+670>:  mov    qword ptr [rbp - 0xd8], rax
    0x10162c795 <+677>:  mov    ebx, 0x8
    0x10162c79a <+682>:  lea    rsi, [rbp - 0xd8]
    0x10162c7a1 <+689>:  mov    edx, 0x8
    0x10162c7a6 <+694>:  jmp    0x10162c7c0               ; <+720>
    0x10162c7a8 <+696>:  mov    byte ptr [rbp - 0xd8], 0x1
    0x10162c7af <+703>:  mov    ebx, 0x1
    0x10162c7b4 <+708>:  lea    rsi, [rbp - 0xd8]
    0x10162c7bb <+715>:  mov    edx, 0x1
    0x10162c7c0 <+720>:  mov    rdi, r14
    0x10162c7c3 <+723>:  call   0x1013cea80               ; rustc_data_structures::sip128::SipHasher128::short_write::hddbede9409661ad5
    0x10162c7c8 <+728>:  add    qword ptr [r14 + 0x48], rbx
    0x10162c7cc <+732>:  lea    rdi, [rbp - 0x38]
    0x10162c7d0 <+736>:  call   0x10141e640               ; core::ptr::drop_in_place::h756bf3ffd5133862
    0x10162c7d5 <+741>:  jmp    0x10162c656               ; <+358>
    0x10162c7da <+746>:  call   0x1019dcd30               ; symbol stub for: std::collections::hash::map::DefaultResizePolicy::new::h7e59c95758f35a6c
    0x10162c7df <+751>:  lea    rdi, [rip + 0x4f717a]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c7e6 <+758>:  call   qword ptr [rdi]
->  0x10162c7e8 <+760>:  movdqa xmm0, xmmword ptr [rax]
    0x10162c7ec <+764>:  movdqa xmm1, xmmword ptr [rax + 0x10]
    0x10162c7f1 <+769>:  mov    rcx, -0x1
    0x10162c7f8 <+776>:  movq   xmm2, rcx
    0x10162c7fd <+781>:  pslldq xmm2, 0x8                 ; xmm2 = zero,zero,zero,zero,zero,zero,zero,zero,xmm2[0,1,2,3,4,5,6,7] 
    0x10162c802 <+786>:  movdqa xmmword ptr [rax], xmm2
    0x10162c806 <+790>:  mov    ecx, 0x1
    0x10162c80b <+795>:  movq   xmm2, rcx
    0x10162c810 <+800>:  pslldq xmm2, 0x8                 ; xmm2 = zero,zero,zero,zero,zero,zero,zero,zero,xmm2[0,1,2,3,4,5,6,7] 
    0x10162c815 <+805>:  movdqa xmmword ptr [rax + 0x10], xmm2
    0x10162c81a <+810>:  pshufd xmm1, xmm1, 0x4e          ; xmm1 = xmm1[2,3,0,1] 
    0x10162c81f <+815>:  movq   rbx, xmm1
    0x10162c824 <+820>:  test   rbx, rbx
    0x10162c827 <+823>:  je     0x10162c775               ; <+645>
    0x10162c82d <+829>:  pshufd xmm0, xmm0, 0x4e          ; xmm0 = xmm0[2,3,0,1] 
    0x10162c832 <+834>:  movq   rcx, xmm0
    0x10162c837 <+839>:  inc    rcx
    0x10162c83a <+842>:  je     0x10162c775               ; <+645>
    0x10162c840 <+848>:  lea    rsi, [8*rcx]
    0x10162c848 <+856>:  shl    rcx, 0x4
    0x10162c84c <+860>:  lea    rdi, [rbp - 0xd8]
    0x10162c853 <+867>:  mov    edx, 0x8
    0x10162c858 <+872>:  mov    r8d, 0x8
    0x10162c85e <+878>:  call   0x1019dcd36               ; symbol stub for: std::collections::hash::table::calculate_allocation::h80671a357bb87ea0
    0x10162c863 <+883>:  mov    rdx, qword ptr [rbp - 0xd8]
    0x10162c86a <+890>:  mov    rsi, qword ptr [rbp - 0xd0]
    0x10162c871 <+897>:  mov    rax, rdx
    0x10162c874 <+900>:  neg    rax
    0x10162c877 <+903>:  cmp    rsi, rax
    0x10162c87a <+906>:  ja     0x10162c8d2               ; <+994>
    0x10162c87c <+908>:  lea    rax, [rdx - 0x1]
    0x10162c880 <+912>:  mov    rcx, rdx
    0x10162c883 <+915>:  or     rcx, -0x80000000
    0x10162c88a <+922>:  and    rcx, rax
    0x10162c88d <+925>:  jne    0x10162c8d2               ; <+994>
    0x10162c88f <+927>:  and    rbx, -0x2
    0x10162c893 <+931>:  mov    rdi, rbx
    0x10162c896 <+934>:  call   0x1019bf7e0               ; __rust_dealloc
    0x10162c89b <+939>:  lea    rdi, [rip + 0x4f70be]     ; rustc::ich::hcx::_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc..ich..hcx..StableHashingContext$LT$$u27$a$GT$$GT$$u20$for$u20$syntax_pos..span_encoding..Span$GT$::hash_stable::CACHE::__getit::__KEY::h765db91289e6600a
    0x10162c8a2 <+946>:  call   qword ptr [rdi]
    0x10162c8a4 <+948>:  cmp    qword ptr [rax + 0x18], 0x0
    0x10162c8a9 <+953>:  jne    0x10162c775               ; <+645>
    0x10162c8af <+959>:  lea    rdi, [rip + 0x4e3ada]
    0x10162c8b6 <+966>:  call   0x1019dd04e               ; symbol stub for: core::panicking::panic::h09bcda4b57fcbe39
    0x10162c8bb <+971>:  jmp    0x10162c8de               ; <+1006>
    0x10162c8bd <+973>:  lea    rdi, [rip + 0x4e3acc]
    0x10162c8c4 <+980>:  call   0x1019dd04e               ; symbol stub for: core::panicking::panic::h09bcda4b57fcbe39
    0x10162c8c9 <+985>:  ud2    
    0x10162c8cb <+987>:  call   0x101493c10               ; core::result::unwrap_failed::hdc4d27fde799a2d6
    0x10162c8d0 <+992>:  jmp    0x10162c8de               ; <+1006>
    0x10162c8d2 <+994>:  lea    rdi, [rip + 0x4e3ab7]
    0x10162c8d9 <+1001>: call   0x1019dd04e               ; symbol stub for: core::panicking::panic::h09bcda4b57fcbe39
    0x10162c8de <+1006>: ud2    
    0x10162c8e0 <+1008>: jmp    0x10162c8e4               ; <+1012>
    0x10162c8e2 <+1010>: jmp    0x10162c8e4               ; <+1012>
    0x10162c8e4 <+1012>: mov    r14, rax
    0x10162c8e7 <+1015>: lea    rdi, [rbp - 0x38]
    0x10162c8eb <+1019>: call   0x10141e640               ; core::ptr::drop_in_place::h756bf3ffd5133862
    0x10162c8f0 <+1024>: jmp    0x10162c932               ; <+1090>
    0x10162c8f2 <+1026>: jmp    0x10162c91b               ; <+1067>
    0x10162c8f4 <+1028>: mov    r15, qword ptr [rbp - 0x30]
    0x10162c8f8 <+1032>: mov    r14, rax
    0x10162c8fb <+1035>: add    r12, 0xb0
    0x10162c902 <+1042>: mov    rdi, r15
    0x10162c905 <+1045>: call   0x1014095b0               ; core::ptr::drop_in_place::h307e3bf3d12d15ea
    0x10162c90a <+1050>: mov    rax, r15
    0x10162c90d <+1053>: add    rax, 0x28
    0x10162c911 <+1057>: mov    r15, rax
    0x10162c914 <+1060>: cmp    r12, rax
    0x10162c917 <+1063>: jne    0x10162c902               ; <+1042>
    0x10162c919 <+1065>: jmp    0x10162c91e               ; <+1070>
    0x10162c91b <+1067>: mov    r14, rax
    0x10162c91e <+1070>: lea    rsi, [rbp - 0xd8]
    0x10162c925 <+1077>: mov    edx, 0x88
    0x10162c92a <+1082>: mov    rdi, rbx
    0x10162c92d <+1085>: call   0x1019dd246               ; symbol stub for: memcpy
    0x10162c932 <+1090>: mov    rdi, r14
    0x10162c935 <+1093>: call   0x1019dd1b0               ; symbol stub for: _Unwind_Resume
    0x10162c93a <+1098>: ud2    
    0x10162c93c <+1100>: nop    dword ptr [rax]

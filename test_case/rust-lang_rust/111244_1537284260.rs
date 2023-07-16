=================================================================
==3909474==ERROR: AddressSanitizer: attempting double-free on 0x607000000020 in thread T0:
    #0 0x55954b5c87b6 in realloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:85:3
    #1 0x55954b5f4c53 in alloc::alloc::realloc::hff458273b20becbe /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:132:14
    #2 0x55954b5f4c53 in alloc::alloc::Global::grow_impl::h51f841a52cb1d214 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:209:31
    #3 0x55954b5f5f11 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::grow::he6ab02d051088968 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:262:18
    #4 0x55954b5f99eb in alloc::raw_vec::finish_grow::h75847b4942b1fd66 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:466:13
    #5 0x55954b5fafdf in alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_exact::hb1a5309d5baa7051 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:419:19
    #6 0x55954b5fecbe in alloc::raw_vec::RawVec$LT$T$C$A$GT$::try_reserve_exact::h34fe28ae0ba6ef5b /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:338:50
    #7 0x55954b5fc976 in alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_exact::hd47cc53060e4d144 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:329:24
    #8 0x55954b605ff1 in alloc::vec::Vec$LT$T$C$A$GT$::reserve_exact::h239d3dd74a145960 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/vec/mod.rs:937:9
    #9 0x55954b601b14 in _$LT$alloc..collections..vec_deque..VecDeque$LT$T$C$A$GT$$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$T$C$A$GT$$GT$$GT$::from::he00045a99c5f260a /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/collections/vec_deque/mod.rs:3063:17
    #10 0x55954b608aef in vec_deque_vuln::into_iter_helper::hd48e5b4f49c469aa

0x607000000020 is located 0 bytes inside of 80-byte region [0x607000000020,0x607000000070)
freed by thread T0 here:
    #0 0x55954b5c87b6 in realloc /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:85:3
    #1 0x55954b5f4c53 in alloc::alloc::realloc::hff458273b20becbe /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:132:14
    #2 0x55954b5f4c53 in alloc::alloc::Global::grow_impl::h51f841a52cb1d214 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:209:31
    #3 0x55954b5f5f11 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::grow::he6ab02d051088968 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/alloc.rs:262:18
    #4 0x55954b5f99eb in alloc::raw_vec::finish_grow::h75847b4942b1fd66 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:466:13
    #5 0x55954b5fafdf in alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_exact::hb1a5309d5baa7051 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:419:19
    #6 0x55954b5fecbe in alloc::raw_vec::RawVec$LT$T$C$A$GT$::try_reserve_exact::h34fe28ae0ba6ef5b /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:338:50
    #7 0x55954b5fc976 in alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_exact::hd47cc53060e4d144 /rustc/a28f3c88e50a77bc2a91889241248c4543854e61/library/alloc/src/raw_vec.rs:329:24
    #8 0x55954b607a91 in vec_deque_vuln::into_iter_helper::h7ac7f51feb689e89 
SUMMARY: AddressSanitizer: double-free /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:85:3 in realloc
==3909474==ABORTING

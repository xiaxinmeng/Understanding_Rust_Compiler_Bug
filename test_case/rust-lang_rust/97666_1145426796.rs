
root@ede627ca950e:~/sync# valgrind --leak-check=full --show-leak-kinds=all target/debug/sync
==6328== Memcheck, a memory error detector
==6328== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==6328== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==6328== Command: target/debug/sync
==6328== 
==6328== 
==6328== HEAP SUMMARY:
==6328==     in use at exit: 10 bytes in 1 blocks
==6328==   total heap usage: 80 allocs, 79 frees, 36,635 bytes allocated
==6328== 
==6328== 10 bytes in 1 blocks are still reachable in loss record 1 of 1
==6328==    at 0x4848899: malloc (in /usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so)
==6328==    by 0x12954B: alloc::alloc::alloc (alloc.rs:89)
==6328==    by 0x1295D6: alloc::alloc::Global::alloc_impl (alloc.rs:171)
==6328==    by 0x129CF9: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:231)
==6328==    by 0x11B518: alloc::raw_vec::finish_grow (raw_vec.rs:468)
==6328==    by 0x11C547: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:400)
==6328==    by 0x11CA27: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:285)
==6328==    by 0x11C9CB: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:289)
==6328==    by 0x11843F: alloc::vec::Vec<T,A>::reserve (mod.rs:815)
==6328==    by 0x117E5B: alloc::vec::Vec<T,A>::append_elements (mod.rs:1802)
==6328==    by 0x117CC9: <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter<T>>>::spec_extend (spec_extend.rs:85)
==6328==    by 0x118244: alloc::vec::Vec<T,A>::extend_from_slice (mod.rs:2251)
==6328== 
==6328== LEAK SUMMARY:
==6328==    definitely lost: 0 bytes in 0 blocks
==6328==    indirectly lost: 0 bytes in 0 blocks
==6328==      possibly lost: 0 bytes in 0 blocks
==6328==    still reachable: 10 bytes in 1 blocks
==6328==         suppressed: 0 bytes in 0 blocks
==6328== 
==6328== For lists of detected and suppressed errors, rerun with: -s
==6328== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)


==3885223== Memcheck, a memory error detector
==3885223== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==3885223== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==3885223== Command: /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh
==3885223== 
==3885223== Warning: set address range perms: large range [0x59c8e000, 0x179c8f000) (noaccess)
==3885223== Warning: client switching stacks?  SP change: 0x79c8ddc0 --> 0x179c8ddd8
==3885223==          to suppress, use: --max-stackframe=4294967320 or greater
==3885223== Warning: client switching stacks?  SP change: 0x179c8dde8 --> 0x79c8ddd0
==3885223==          to suppress, use: --max-stackframe=4294967320 or greater
==3885223== Thread 2:
==3885223== Invalid read of size 4
==3885223==    at 0x110A91: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==  Address 0x4a92090 is 0 bytes inside a block of size 4 free'd
==3885223==    at 0x4846CC3: realloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==3885223==    by 0x10F191: alloc::raw_vec::finish_grow (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==    by 0x10F281: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==    by 0x110A87: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==  Block was alloc'd at
==3885223==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==3885223==    by 0x110A53: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223== 
==3885223== Invalid free() / delete / delete[] / realloc()
==3885223==    at 0x484426F: free (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==3885223==    by 0x110AA9: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==  Address 0x4a92090 is 0 bytes inside a block of size 4 free'd
==3885223==    at 0x4846CC3: realloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==3885223==    by 0x10F191: alloc::raw_vec::finish_grow (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==    by 0x10F281: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==    by 0x110A87: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223==  Block was alloc'd at
==3885223==    at 0x4841888: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==3885223==    by 0x110A53: scratchIyxkUIBfh::perform_double_free (in /home/jess/.cache/cargo/target/release/scratchIyxkUIBfh)
==3885223== 
==3885223== Warning: client switching stacks?  SP change: 0x79c8ddd0 --> 0x179c8dde8
==3885223==          to suppress, use: --max-stackframe=4294967320 or greater
==3885223==          further instances of this message will not be shown.
==3885223== Warning: set address range perms: large range [0x59c8e000, 0x179c8f000) (noaccess)
==3885223== 
==3885223== HEAP SUMMARY:
==3885223==     in use at exit: 4 bytes in 1 blocks
==3885223==   total heap usage: 22 allocs, 22 frees, 2,853 bytes allocated
==3885223== 
==3885223== LEAK SUMMARY:
==3885223==    definitely lost: 4 bytes in 1 blocks
==3885223==    indirectly lost: 0 bytes in 0 blocks
==3885223==      possibly lost: 0 bytes in 0 blocks
==3885223==    still reachable: 0 bytes in 0 blocks
==3885223==         suppressed: 0 bytes in 0 blocks
==3885223== Rerun with --leak-check=full to see details of leaked memory
==3885223== 
==3885223== For lists of detected and suppressed errors, rerun with: -s
==3885223== ERROR SUMMARY: 2 errors from 2 contexts (suppressed: 0 from 0)

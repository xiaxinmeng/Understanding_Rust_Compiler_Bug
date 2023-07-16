
valgrind --leak-check=full --show-reachable=yes target/debug/sandbox
==55274== Memcheck, a memory error detector
==55274== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==55274== Using Valgrind-3.12.0 and LibVEX; rerun with -h for copyright info
==55274== Command: target/debug/sandbox
==55274==
==55274==
==55274== HEAP SUMMARY:
==55274==     in use at exit: 23,333 bytes in 187 blocks
==55274==   total heap usage: 275 allocs, 88 frees, 29,733 bytes allocated
==55274==
==55274== 112 bytes in 1 blocks are still reachable in loss record 39 of 64
==55274==    at 0x10005C681: malloc (in /usr/local/Cellar/valgrind/3.12.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==55274==    by 0x10019CD7C: tlv_allocate_and_initialize_for_key (in /usr/lib/system/libdyld.dylib)
==55274==    by 0x10019D52B: tlv_get_addr (in /usr/lib/system/libdyld.dylib)
==55274==    by 0x10000541D: std::sys_common::thread_info::set::h3ee71b197ed00dc8 (in target/debug/sandbox)
==55274==    by 0x100006782: std::rt::lang_start::hca48e539ce72a288 (in target/debug/sandbox)
==55274==    by 0x100001639: main (in target/debug/sandbox)
==55274==
==55274== 2,064 bytes in 1 blocks are possibly lost in loss record 61 of 64
==55274==    at 0x10005C942: malloc_zone_malloc (in /usr/local/Cellar/valgrind/3.12.0/lib/valgrind/vgpreload_memcheck-amd64-darwin.so)
==55274==    by 0x10054EEFD: _objc_copyClassNamesForImage (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x100542182: protocols() (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x100542093: readClass(objc_class*, bool, bool) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x10053FC13: gc_init (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x10054724E: objc_initializeClassPair_internal(objc_class*, char const*, objc_class*, objc_class*) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x100554132: layout_string_create (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x10054283C: realizeClass(objc_class*) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x100542300: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x1005422E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x1005422E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==55274==    by 0x1005422E9: copySwiftV1MangledName(char const*, bool) (in /usr/lib/libobjc.A.dylib)
==55274==
==55274== LEAK SUMMARY:
==55274==    definitely lost: 0 bytes in 0 blocks
==55274==    indirectly lost: 0 bytes in 0 blocks
==55274==      possibly lost: 2,064 bytes in 1 blocks
==55274==    still reachable: 112 bytes in 1 blocks
==55274==         suppressed: 21,157 bytes in 185 blocks
==55274==
==55274== For counts of detected and suppressed errors, rerun with: -v
==55274== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 15 from 15)

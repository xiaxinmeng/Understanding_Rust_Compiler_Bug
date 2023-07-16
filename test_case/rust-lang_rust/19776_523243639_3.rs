bash
$ LD_LIBRARY_PATH=../target/debug/ valgrind --leak-check=full --show-leak-kinds=all --trace-children=yes ./test.out
==9724== Memcheck, a memory error detector
==9724== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==9724== Using Valgrind-3.14.0 and LibVEX; rerun with -h for copyright info
==9724== Command: ./test.out
==9724== 
==9724== 
==9724== HEAP SUMMARY:
==9724==     in use at exit: 1,200 bytes in 7 blocks
==9724==   total heap usage: 8 allocs, 1 frees, 1,232 bytes allocated
==9724== 
==9724== 8 bytes in 1 blocks are still reachable in loss record 1 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484A7F8: alloc (alloc.rs:81)
==9724==    by 0x484A7F8: exchange_malloc (alloc.rs:203)
==9724==    by 0x484A7F8: new<closure> (boxed.rs:113)
==9724==    by 0x484A7F8: at_exit<closure> (mod.rs:123)
==9724==    by 0x484A7F8: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:48)
==9724==    by 0x484A7F8: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A7F8: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 8 bytes in 1 blocks are still reachable in loss record 2 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484A973: alloc (alloc.rs:81)
==9724==    by 0x484A973: exchange_malloc (alloc.rs:203)
==9724==    by 0x484A973: new<alloc::sync::Arc<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>>> (boxed.rs:113)
==9724==    by 0x484A973: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:60)
==9724==    by 0x484A973: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A973: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 16 bytes in 1 blocks are still reachable in loss record 3 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484CA25: alloc (alloc.rs:81)
==9724==    by 0x484CA25: alloc (alloc.rs:169)
==9724==    by 0x484CA25: reserve_internal<alloc::boxed::Box<FnOnce<()>>,alloc::alloc::Global> (raw_vec.rs:668)
==9724==    by 0x484CA25: reserve<alloc::boxed::Box<FnOnce<()>>,alloc::alloc::Global> (raw_vec.rs:491)
==9724==    by 0x484CA25: reserve<alloc::boxed::Box<FnOnce<()>>> (vec.rs:457)
==9724==    by 0x484CA25: push<alloc::boxed::Box<FnOnce<()>>> (vec.rs:1033)
==9724==    by 0x484CA25: std::sys_common::at_exit_imp::push (at_exit_imp.rs:69)
==9724==    by 0x484A813: at_exit<closure> (mod.rs:123)
==9724==    by 0x484A813: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:48)
==9724==    by 0x484A813: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A813: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 24 bytes in 1 blocks are still reachable in loss record 4 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484C94A: alloc (alloc.rs:81)
==9724==    by 0x484C94A: exchange_malloc (alloc.rs:203)
==9724==    by 0x484C94A: init (at_exit_imp.rs:30)
==9724==    by 0x484C94A: std::sys_common::at_exit_imp::push (at_exit_imp.rs:66)
==9724==    by 0x484A813: at_exit<closure> (mod.rs:123)
==9724==    by 0x484A813: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:48)
==9724==    by 0x484A813: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A813: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 40 bytes in 1 blocks are still reachable in loss record 5 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484A862: alloc (alloc.rs:81)
==9724==    by 0x484A862: exchange_malloc (alloc.rs:203)
==9724==    by 0x484A862: new<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>> (remutex.rs:54)
==9724==    by 0x484A862: stdout_init (stdio.rs:480)
==9724==    by 0x484A862: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:58)
==9724==    by 0x484A862: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A862: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 80 bytes in 1 blocks are still reachable in loss record 6 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484A8DE: alloc (alloc.rs:81)
==9724==    by 0x484A8DE: exchange_malloc (alloc.rs:203)
==9724==    by 0x484A8DE: new<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (sync.rs:288)
==9724==    by 0x484A8DE: stdout_init (stdio.rs:480)
==9724==    by 0x484A8DE: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:58)
==9724==    by 0x484A8DE: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A8DE: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== 1,024 bytes in 1 blocks are still reachable in loss record 7 of 7
==9724==    at 0x483577F: malloc (vg_replace_malloc.c:299)
==9724==    by 0x484A826: alloc (alloc.rs:81)
==9724==    by 0x484A826: alloc (alloc.rs:169)
==9724==    by 0x484A826: allocate_in<u8,alloc::alloc::Global> (raw_vec.rs:95)
==9724==    by 0x484A826: with_capacity<u8> (raw_vec.rs:139)
==9724==    by 0x484A826: with_capacity<u8> (vec.rs:355)
==9724==    by 0x484A826: with_capacity<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>> (buffered.rs:484)
==9724==    by 0x484A826: with_capacity<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>> (buffered.rs:857)
==9724==    by 0x484A826: new<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>> (buffered.rs:836)
==9724==    by 0x484A826: stdout_init (stdio.rs:480)
==9724==    by 0x484A826: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:58)
==9724==    by 0x484A826: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::buffered::LineWriter<std::io::stdio::Maybe<std::io::stdio::StdoutRaw>>>>> (lazy.rs:34)
==9724==    by 0x484A826: std::io::stdio::stdout (stdio.rs:470)
==9724==    by 0x484AF17: {{closure}}<std::io::stdio::Stdout> (stdio.rs:786)
==9724==    by 0x484AF17: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:299)
==9724==    by 0x484AF17: print_to<std::io::stdio::Stdout> (stdio.rs:780)
==9724==    by 0x484AF17: std::io::stdio::_print (stdio.rs:802)
==9724==    by 0x48475A3: print_things (lib.rs:3)
==9724==    by 0x1086CD: main (in /home/proxmox/Projects/test-things/src/test.out)
==9724== 
==9724== LEAK SUMMARY:
==9724==    definitely lost: 0 bytes in 0 blocks
==9724==    indirectly lost: 0 bytes in 0 blocks
==9724==      possibly lost: 0 bytes in 0 blocks
==9724==    still reachable: 1,200 bytes in 7 blocks
==9724==         suppressed: 0 bytes in 0 blocks
==9724== 
==9724== For counts of detected and suppressed errors, rerun with: -v
==9724== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

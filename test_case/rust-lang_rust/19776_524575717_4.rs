
==975== Memcheck, a memory error detector
==975== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==975== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==975== Command: ./run
==975== 
something
==975== 
==975== HEAP SUMMARY:
==975==     in use at exit: 144 bytes in 6 blocks
==975==   total heap usage: 7 allocs, 1 frees, 176 bytes allocated
==975== 
==975== 8 bytes in 1 blocks are still reachable in loss record 1 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB0196: alloc (alloc.rs:82)
==975==    by 0x4EB0196: exchange_malloc (alloc.rs:204)
==975==    by 0x4EB0196: new<closure> (boxed.rs:116)
==975==    by 0x4EB0196: at_exit<closure> (mod.rs:117)
==975==    by 0x4EB0196: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:48)
==975==    by 0x4EB0196: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB0196: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== 8 bytes in 1 blocks are still reachable in loss record 2 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB02AD: alloc (alloc.rs:82)
==975==    by 0x4EB02AD: exchange_malloc (alloc.rs:204)
==975==    by 0x4EB02AD: new<alloc::sync::Arc<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>>> (boxed.rs:116)
==975==    by 0x4EB02AD: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:60)
==975==    by 0x4EB02AD: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB02AD: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== 16 bytes in 1 blocks are still reachable in loss record 3 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB2105: alloc (alloc.rs:82)
==975==    by 0x4EB2105: alloc (alloc.rs:170)
==975==    by 0x4EB2105: reserve_internal<alloc::boxed::Box<FnOnce<()>>,alloc::alloc::Global> (raw_vec.rs:668)
==975==    by 0x4EB2105: reserve<alloc::boxed::Box<FnOnce<()>>,alloc::alloc::Global> (raw_vec.rs:491)
==975==    by 0x4EB2105: reserve<alloc::boxed::Box<FnOnce<()>>> (vec.rs:457)
==975==    by 0x4EB2105: push<alloc::boxed::Box<FnOnce<()>>> (vec.rs:1102)
==975==    by 0x4EB2105: std::sys_common::at_exit_imp::push (at_exit_imp.rs:69)
==975==    by 0x4EB01B1: at_exit<closure> (mod.rs:117)
==975==    by 0x4EB01B1: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:48)
==975==    by 0x4EB01B1: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB01B1: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== 24 bytes in 1 blocks are still reachable in loss record 4 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB202A: alloc (alloc.rs:82)
==975==    by 0x4EB202A: exchange_malloc (alloc.rs:204)
==975==    by 0x4EB202A: init (at_exit_imp.rs:30)
==975==    by 0x4EB202A: std::sys_common::at_exit_imp::push (at_exit_imp.rs:66)
==975==    by 0x4EB01B1: at_exit<closure> (mod.rs:117)
==975==    by 0x4EB01B1: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:48)
==975==    by 0x4EB01B1: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB01B1: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== 40 bytes in 1 blocks are still reachable in loss record 5 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB01C4: alloc (alloc.rs:82)
==975==    by 0x4EB01C4: exchange_malloc (alloc.rs:204)
==975==    by 0x4EB01C4: new<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>> (remutex.rs:54)
==975==    by 0x4EB01C4: stderr_init (stdio.rs:639)
==975==    by 0x4EB01C4: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:58)
==975==    by 0x4EB01C4: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB01C4: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== 48 bytes in 1 blocks are still reachable in loss record 6 of 6
==975==    at 0x4C2FB0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
==975==    by 0x4EB0243: alloc (alloc.rs:82)
==975==    by 0x4EB0243: exchange_malloc (alloc.rs:204)
==975==    by 0x4EB0243: new<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (sync.rs:288)
==975==    by 0x4EB0243: stderr_init (stdio.rs:639)
==975==    by 0x4EB0243: init<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:58)
==975==    by 0x4EB0243: get<std::sys_common::remutex::ReentrantMutex<core::cell::RefCell<std::io::stdio::Maybe<std::io::stdio::StderrRaw>>>> (lazy.rs:34)
==975==    by 0x4EB0243: std::io::stdio::stderr (stdio.rs:629)
==975==    by 0x4EB0772: {{closure}}<std::io::stdio::Stderr> (stdio.rs:786)
==975==    by 0x4EB0772: try_with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure,core::result::Result<(), std::io::error::Error>> (local.rs:257)
==975==    by 0x4EB0772: print_to<std::io::stdio::Stderr> (stdio.rs:780)
==975==    by 0x4EB0772: std::io::stdio::_eprint (stdio.rs:811)
==975==    by 0x4E4A1B3: print_something (embeddings_wrap.rs:147)
==975==    by 0x108662: main (main.cpp:13)
==975== 
==975== LEAK SUMMARY:
==975==    definitely lost: 0 bytes in 0 blocks
==975==    indirectly lost: 0 bytes in 0 blocks
==975==      possibly lost: 0 bytes in 0 blocks
==975==    still reachable: 144 bytes in 6 blocks
==975==         suppressed: 0 bytes in 0 blocks
==975== 
==975== For counts of detected and suppressed errors, rerun with: -v
==975== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

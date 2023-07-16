
info threads
  6 Thread 2012.0x334  0x76f7000d in ntdll!LdrFindResource_U () from ...\ntdll.dll
  5 Thread 2012.0x5dc  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
  4 Thread 2012.0xb68  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
  3 Thread 2012.0x520  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
  2 Thread 2012.0x354  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
  1 Thread 2012.0xb78  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll

thread 6:

#0  0x76f7000d in ntdll!LdrFindResource_U () from ...\ntdll.dll
#1  0x76fff896 in ntdll!RtlQueryTimeZoneInformation () from ...\ntdll.dll
#2  0x741b1d99 in ?? ()
#3  0x00000000 in ?? ()

thread 5:

#0  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#1  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#2  0x74bb0a91 in WaitForSingleObjectEx () from ...\KernelBase.dll
#3  0x00000110 in ?? () at ...stl/stl_tree.h:1244
#4  0x74e91194 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#5  0x00000110 in ?? () at ...stl/stl_tree.h:1244
#6  0x74e91148 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#7  0x00000110 in ?? () at ...stl/stl_tree.h:1244
#8  0x61901730 in lock_and_signal::timed_wait (this=0x1b6e0f4, timeout_in_ms=10) at .../sync/lock_and_signal.cpp:77
#9  0x619058fb in rust_scheduler::start_main_loop (this=0x1b6d838) at .../rust_scheduler.cpp:269
#10 0x619014e0 in rust_thread_start (ptr=0x1b6d838) at .../sync/sync.cpp:34
#11 0x74e9339a in KERNEL32!BaseCleanupAppcompatCacheSupport () from ...\kernel32.dll
#12 0x01b6d838 in ?? ()
#13 0x76f99ef2 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#14 0x01b6d838 in ?? ()
#15 0x76f99ec5 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#16 0x619014d0 in timer::timer() () from ...\rustrt.dll

thread 4:

#0  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#1  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#2  0x74bb0a91 in WaitForSingleObjectEx () from ...\KernelBase.dll
#3  0x00000108 in ?? () at ...stl/stl_tree.h:1244
#4  0x74e91194 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#5  0x00000108 in ?? () at ...stl/stl_tree.h:1244
#6  0x74e91148 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#7  0x00000108 in ?? () at ...stl/stl_tree.h:1244
#8  0x61901730 in lock_and_signal::timed_wait (this=0x13d61c4, timeout_in_ms=10) at .../sync/lock_and_signal.cpp:77
#9  0x619058fb in rust_scheduler::start_main_loop (this=0x13d5908) at .../rust_scheduler.cpp:269
#10 0x619014e0 in rust_thread_start (ptr=0x13d5908) at .../sync/sync.cpp:34
#11 0x74e9339a in KERNEL32!BaseCleanupAppcompatCacheSupport () from ...\kernel32.dll
#12 0x013d5908 in ?? ()
#13 0x76f99ef2 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#14 0x013d5908 in ?? ()
#15 0x76f99ec5 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#16 0x619014d0 in timer::timer() () from ...\rustrt.dll
#17 0x00000000 in ?? ()

thread 3:

#0  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#1  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#2  0x74bb0a91 in WaitForSingleObjectEx () from ...\KernelBase.dll
#3  0x00000100 in ?? () at ...stl/stl_tree.h:1244
#4  0x74e91194 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#5  0x00000100 in ?? () at ...stl/stl_tree.h:1244
#6  0x74e91148 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#7  0x00000100 in ?? () at ...stl/stl_tree.h:1244
#8  0x61901730 in lock_and_signal::timed_wait (this=0x13d587c, timeout_in_ms=10) at .../sync/lock_and_signal.cpp:77
#9  0x619058fb in rust_scheduler::start_main_loop (this=0x13d4fc0) at .../rust_scheduler.cpp:269
#10 0x619014e0 in rust_thread_start (ptr=0x13d4fc0) at .../sync/sync.cpp:34
#11 0x74e9339a in KERNEL32!BaseCleanupAppcompatCacheSupport () from ...\kernel32.dll
#12 0x013d4fc0 in ?? ()
#13 0x76f99ef2 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#14 0x013d4fc0 in ?? ()
#15 0x76f99ec5 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#16 0x619014d0 in timer::timer() () from ...\rustrt.dll
#17 0x00000000 in ?? ()


thread 2:

#0  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#1  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#2  0x74bb0a91 in WaitForSingleObjectEx () from ...\KernelBase.dll
#3  0x000000e8 in ?? () at ...stl/stl_tree.h:1274
#4  0x74e91194 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#5  0x000000e8 in ?? () at ...stl/stl_tree.h:1274
#6  0x74e91148 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#7  0x000000e8 in ?? () at ...stl/stl_tree.h:1274
#8  0x61901730 in lock_and_signal::timed_wait (this=0x13d4e14, timeout_in_ms=10) at .../sync/lock_and_signal.cpp:77
#9  0x619058fb in rust_scheduler::start_main_loop (this=0x13d4558) at .../rust_scheduler.cpp:269
#10 0x619014e0 in rust_thread_start (ptr=0x13d4558) at .../sync/sync.cpp:34
#11 0x74e9339a in KERNEL32!BaseCleanupAppcompatCacheSupport () from ...\kernel32.dll
#12 0x013d4558 in ?? ()
#13 0x76f99ef2 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#14 0x013d4558 in ?? ()
#15 0x76f99ec5 in ntdll!RtlpNtSetValueKey () from ...\ntdll.dll
#16 0x619014d0 in timer::timer() () from ...\rustrt.dll
#17 0x00000000 in ?? ()


thread 1:

#0  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#1  0x76f7f8b1 in ntdll!RtlUpdateClonedSRWLock () from ...\ntdll.dll
#2  0x74bb0a91 in WaitForSingleObjectEx () from ...\KernelBase.dll
#3  0x0000011c in ?? () at ...stl/stl_tree.h:1263
#4  0x74e91194 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#5  0x0000011c in ?? () at ...stl/stl_tree.h:1263
#6  0x74e91148 in KERNEL32!GetPrivateProfileStructA () from ...\kernel32.dll
#7  0x0000011c in ?? () at ...stl/stl_tree.h:1263
#8  0x619015c1 in rust_thread::join (this=0x13d4558) at .../sync/sync.cpp:56
#9  0x6190c104 in rust_kernel::start_task_threads (this=0x13d3c90) at .../rust_kernel.cpp:132
#10 0x61901c3b in rust_start (main_fn=4207600, argc=4194304, argv=0x0, crate_map=0x406000) at .../rust.cpp:106
#11 0x00403461 in WinMain@16 ()
#12 0x004033f0 in main::_6178e9efa23b8931 ()
#13 0x00400000 in ?? ()
#14 0x00405896 in main ()



Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000

Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   librustrt-4e7c5e5c.dylib        0x0000000103941f6b stack_overflow::imp::signal_handler::term::h4e1f7a7875fabf2cTLa + 59
1   librustrt-4e7c5e5c.dylib        0x0000000103941ee5 stack_overflow::imp::signal_handler::h49baa5350251116fALa + 101
2   libsystem_platform.dylib        0x00007fff91953f1a _sigtramp + 26
3   librustrt-4e7c5e5c.dylib        0x00000001039664f8 je_extent_tree_ad_remove + 216
4   librustrt-4e7c5e5c.dylib        0x0000000103967224 je_huge_dalloc + 68
5   librustrt-4e7c5e5c.dylib        0x0000000103971c16 je_sdallocx + 534
6   libstd-4e7c5e5c.dylib           0x00000001036ee4f4 thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h14050218259616233623 + 52
7   libstd-4e7c5e5c.dylib           0x00000001036ee5d4 rt::start::closure.32341 + 164
8   librustrt-4e7c5e5c.dylib        0x00000001039aaafc rust_try_inner + 12
9   librustrt-4e7c5e5c.dylib        0x00000001039aaae6 rust_try + 6
10  librustrt-4e7c5e5c.dylib        0x00000001039458c7 unwind::try::hc34b1ff6cb3d2cbdquc + 71
11  librustrt-4e7c5e5c.dylib        0x000000010394566c task::Task::run::h5eecf441ff21ebbepLb + 124
12  libstd-4e7c5e5c.dylib           0x00000001036ee3af rt::start::h98e85f0930041adcIZx + 511
13  libstd-4e7c5e5c.dylib           0x00000001036ee19d rt::lang_start::hdf0f81f47bc259ddZYx + 109
14  libdyld.dylib                   0x00007fff93b1a5c9 start + 1

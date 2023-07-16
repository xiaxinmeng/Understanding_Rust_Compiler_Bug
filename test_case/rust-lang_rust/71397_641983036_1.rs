
Sampling process 35504 for 3 seconds with 1 millisecond of run time between samples
Sampling completed, processing symbols...
Analysis of sampling pallet_contracts_rpc-d3910674823302e5 (pid 35504) every 1 millisecond
Process:         pallet_contracts_rpc-d3910674823302e5 [35504]
Path:            /Users/USER/*/pallet_contracts_rpc-d3910674823302e5
Load Address:    0x10601a000
Identifier:      pallet_contracts_rpc-d3910674823302e5
Version:         0
Code Type:       X86-64
Parent Process:  cargo [35503]

Date/Time:       2020-06-10 14:37:01.469 +0200
Launch Time:     2020-06-10 14:36:40.831 +0200
OS Version:      Mac OS X 10.15.5 (19F96)
Report Version:  7
Analysis Tool:   /usr/bin/sample

Physical footprint:         12.4M
Physical footprint (peak):  12.4M
----

Call graph:
    2798 Thread_1449457   DispatchQueue_1: com.apple.main-thread  (serial)
    + 2798 start  (in libdyld.dylib) + 1  [0x7fff6871dcc9]
    +   2798 main  (in pallet_contracts_rpc-d3910674823302e5) + 34  [0x106022df2]
    +     2798 std::rt::lang_start::h2982339634add2f5  (in pallet_contracts_rpc-d3910674823302e5) + 65  [0x10601ae81]  rt.rs:67
    +       2798 std::rt::lang_start_internal::h1069bf3e81ece2dd  (in pallet_contracts_rpc-d3910674823302e5) + 441  [0x1060a69c9]  rt.rs:51
    +         2798 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h17d40798dcc92eee  (in pallet_contracts_rpc-d3910674823302e5) + 14  [0x10601ae9e]  rt.rs:67
    +           2798 pallet_contracts_rpc::main::h5b73f23169ae7275  (in pallet_contracts_rpc-d3910674823302e5) + 24  [0x106022dc8]
    +             2798 test::test_main_static::h5ec0d2acdadcabb7  (in pallet_contracts_rpc-d3910674823302e5) + 286  [0x10604c5ce]  lib.rs:139
    +               2798 test::test_main::ha0ba06cde89e658c  (in pallet_contracts_rpc-d3910674823302e5) + 298  [0x10604b28a]  lib.rs:120
    +                 2798 test::console::run_tests_console::hb1588804e66cb577  (in pallet_contracts_rpc-d3910674823302e5) + 9643  [0x10603ee5b]  console.rs:280
    +                   2798 std::sync::mpsc::shared::Packet$LT$T$GT$::recv::he5639701e5f48e33  (in pallet_contracts_rpc-d3910674823302e5) + 347  [0x10602e38b]  shared.rs:229
    +                     2798 std::sync::mpsc::blocking::WaitToken::wait_max_until::h4078089a70fd63c5  (in pallet_contracts_rpc-d3910674823302e5) + 286  [0x1060a32be]  blocking.rs:75
    +                       2798 std::thread::park_timeout::h5ad21fe3fd74daf8  (in pallet_contracts_rpc-d3910674823302e5) + 270  [0x10609b4ce]  mod.rs:1014
    +                         2798 std::sys::unix::condvar::Condvar::wait_timeout::h17d79840596d07fd  (in pallet_contracts_rpc-d3910674823302e5) + 249  [0x1060a6f29]  condvar.rs:153
    +                           2798 _pthread_cond_wait  (in libsystem_pthread.dylib) + 698  [0x7fff68922425]
    +                             2798 __psynch_cvwait  (in libsystem_kernel.dylib) + 10  [0x7fff68861882]
    2798 Thread_1449458: tests::call_request_should_serialize_deserialize_properly
      2798 thread_start  (in libsystem_pthread.dylib) + 15  [0x7fff6891db8b]
        2798 _pthread_start  (in libsystem_pthread.dylib) + 148  [0x7fff68922109]
          2798 std::sys::unix::thread::Thread::new::thread_start::h2b28b74d30bce841  (in pallet_contracts_rpc-d3910674823302e5) + 45  [0x1060ac60d]  thread.rs:87
            2798 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hd45267100ae6c7ce  (in pallet_contracts_rpc-d3910674823302e5) + 117  [0x1060303a5]  function.rs:232
              2798 std::sys_common::backtrace::__rust_begin_short_backtrace::hffd4a983e423c33e  (in pallet_contracts_rpc-d3910674823302e5) + 43  [0x10602aecb]  backtrace.rs:130
                2798 test::run_test::run_test_inner::_$u7b$$u7b$closure$u7d$$u7d$::hf35455f67ec1e4ed  (in pallet_contracts_rpc-d3910674823302e5) + 819  [0x106050ca3]  lib.rs:450
                  2798 core::ops::function::FnOnce::call_once::h14427d74c038af42  (in pallet_contracts_rpc-d3910674823302e5) + 17  [0x106023511]  function.rs:232
                    2798 pallet_contracts_rpc::tests::call_request_should_serialize_deserialize_properly::_$u7b$$u7b$closure$u7d$$u7d$::h88024b7db0e1a095  (in pallet_contracts_rpc-d3910674823302e5) + 17  [0x106023301]  lib.rs:295
                      2798 pallet_contracts_rpc::tests::call_request_should_serialize_deserialize_properly::h65dce4213e60d6a0  (in pallet_contracts_rpc-d3910674823302e5) + 80  [0x106025f80]  lib.rs:297
                        2798 core::result::Result$LT$T$C$E$GT$::unwrap::h4e51363d6979532e  (in pallet_contracts_rpc-d3910674823302e5) + 102  [0x10602a406]  result.rs:1005
                          2798 core::option::expect_none_failed::h07b7c1add83ba342  (in pallet_contracts_rpc-d3910674823302e5) + 117  [0x1060d2635]  option.rs:1268
                            2798 core::panicking::panic_fmt::h299f54c72477a62a  (in pallet_contracts_rpc-d3910674823302e5) + 47  [0x1060d272f]  panicking.rs:111
                              2798 rust_begin_unwind  (in pallet_contracts_rpc-d3910674823302e5) + 130  [0x1060a6232]  panicking.rs:419
                                2798 std::panicking::rust_panic_with_hook::h2cd47f71d6d55501  (in pallet_contracts_rpc-d3910674823302e5) + 296  [0x1060a6668]  panicking.rs:511
                                  2798 std::panicking::default_hook::h722aa3f5c1c31788  (in pallet_contracts_rpc-d3910674823302e5) + 664  [0x1060a5d78]  panicking.rs:215
                                    2798 std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::ha991e4eca34b4afa  (in pallet_contracts_rpc-d3910674823302e5) + 474  [0x1060a609a]  panicking.rs:198
                                      2798 std::io::impls::_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..boxed..Box$LT$W$GT$$GT$::write_fmt::h352c9db3a02449c0  (in pallet_contracts_rpc-d3910674823302e5) + 76  [0x10609e36c]  impls.rs:156
                                        2798 std::io::Write::write_fmt::h53fe50e3fff0275d  (in pallet_contracts_rpc-d3910674823302e5) + 89  [0x10602b9c9]  mod.rs:1504
                                          2798 core::fmt::write::hf81c429634e1f3ed  (in pallet_contracts_rpc-d3910674823302e5) + 510  [0x1060c94ae]  mod.rs:1069
                                            2798 _$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$::fmt::h83d53b696ac99295  (in pallet_contracts_rpc-d3910674823302e5) + 319  [0x1060a3f9f]  backtrace.rs:59
                                              2798 _Unwind_Backtrace  (in libunwind.dylib) + 78  [0x7fff6895413f]
                                                2798 backtrace::backtrace::libunwind::trace::trace_fn::h10f4b899671fce38  (in pallet_contracts_rpc-d3910674823302e5) + 35  [0x1060add93]  libunwind.rs:98
                                                  2798 std::sys_common::backtrace::_print_fmt::_$u7b$$u7b$closure$u7d$$u7d$::h56579608189a9677  (in pallet_contracts_rpc-d3910674823302e5) + 110  [0x1060a462e]  backtrace.rs:85
                                                    2798 backtrace::symbolize::libbacktrace::resolve::h39ea9ff39d2c28cb  (in pallet_contracts_rpc-d3910674823302e5) + 231  [0x1060ae107]  libbacktrace.rs:469
                                                      2798 __rdos_backtrace_syminfo  (in pallet_contracts_rpc-d3910674823302e5) + 40  [0x1060bbe68]
                                                        2798 fileline_initialize  (in pallet_contracts_rpc-d3910674823302e5) + 450  [0x1060bbda2]
                                                          2798 __rdos_backtrace_initialize  (in pallet_contracts_rpc-d3910674823302e5) + 278  [0x1060bca16]
                                                            2798 __rdos_macho_add  (in pallet_contracts_rpc-d3910674823302e5) + 2166,2160  [0x1060bd386,0x1060bd380]

Total number in stack (recursive counted multiple, when >=5):

Sort by top of stack, same collapsed (when >= 5):
        __psynch_cvwait  (in libsystem_kernel.dylib)        2798
        __rdos_macho_add  (in pallet_contracts_rpc-d3910674823302e5)        2798


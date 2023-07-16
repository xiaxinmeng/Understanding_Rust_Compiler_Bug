
(lldb) bt
* thread #3, stop reason = signal SIGUSR1
  * frame #0: 0x00007fff72696062 libsystem_kernel.dylib`__psynch_mutexwait + 10
    frame #1: 0x00007fff72754917 libsystem_pthread.dylib`_pthread_mutex_firstfit_lock_wait + 83
    frame #2: 0x00007fff72752937 libsystem_pthread.dylib`_pthread_mutex_firstfit_lock_slow + 222
    frame #3: 0x00007fff727574b9 libsystem_pthread.dylib`_pthread_cond_wait + 846
    frame #4: 0x000000010089383e cargo`jobserver::HelperState::for_each_request::hd7676203155a2e1f + 478
    frame #5: 0x0000000100893bac cargo`std::sys_common::backtrace::__rust_begin_short_backtrace::h8fcdd033f1d0a3b3 + 60
    frame #6: 0x000000010089470e cargo`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::he7c0c0a7015d0f87 + 142
    frame #7: 0x00000001008df98d cargo`std::sys::unix::thread::Thread::new::thread_start::h93dd3097fa4fa219 + 45
    frame #8: 0x00007fff72757109 libsystem_pthread.dylib`_pthread_start + 148
    frame #9: 0x00007fff72752b8b libsystem_pthread.dylib`thread_start + 15

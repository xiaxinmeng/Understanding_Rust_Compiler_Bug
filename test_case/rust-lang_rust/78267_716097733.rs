bash
$ LLVM_PROFILE_FILE=try_error_result.profraw ./try_error_result
thread 'main' panicked at 'explicit panic', src/test/run-make-fulldeps/coverage/try_error_result.rs:6:9
stack backtrace:
   0:     0x5608e9f59e31 - std::backtrace_rs::backtrace::libunwind::trace::he2381431404d861d
                               at /usr/local/google/home/richkadel/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:100:5

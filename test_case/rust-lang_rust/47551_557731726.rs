
Hello, world! stack backtrace:
   0: unwind_repro::main
             at src/main.rs:19
   1: std::rt::lang_start::{{closure}}
             at /rustc/5c5b8afd80e6fa1d24632153cb2257c686041d41/src/libstd/rt.rs:61
   2: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:48
      std::panicking::try::do_call
             at src/libstd/panicking.rs:287
   3: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
   4: std::panicking::try
             at src/libstd/panicking.rs:265
      std::panic::catch_unwind
             at src/libstd/panic.rs:395
      std::rt::lang_start_internal
             at src/libstd/rt.rs:47
   5: std::rt::lang_start
             at /rustc/5c5b8afd80e6fa1d24632153cb2257c686041d41/src/libstd/rt.rs:61
   6: main

1122334455667788

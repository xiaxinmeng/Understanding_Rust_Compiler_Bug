
  = note: /tmp/rustc.xZWN2thLlEHU/librustc_lsan-8d532cbe1107932c.rlib(lsan.cc.o): In function `InitializeFlags':
          /usr/ports/lang/rust-nightly/work/rustc-nightly-src/src/libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc:53: undefined reference to `__lsan::lsan_flags'
          /usr/ports/lang/rust-nightly/work/rustc-nightly-src/src/libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc:53: undefined reference to `__lsan::Flags::SetDefaults()'
          /usr/ports/lang/rust-nightly/work/rustc-nightly-src/src/libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc:56: undefined reference to `__lsan::RegisterLsanFlags(__sanitizer::FlagParser*, __lsan::Flags*)'
          /tmp/rustc.xZWN2thLlEHU/librustc_lsan-8d532cbe1107932c.rlib(lsan_allocator.cc.o): In function `RegisterAllocation':
          /usr/ports/lang/rust-nightly/work/rustc-nightly-src/src/libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.cc:84: undefined reference to `__lsan::DisabledInThisThread()'
          /usr/ports/lang/rust-nightly/work/rustc-nightly-src/src/libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.cc:84: undefined reference to `__lsan::DisabledInThisThread()'

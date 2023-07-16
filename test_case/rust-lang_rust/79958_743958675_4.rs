text
===================================================================================================

Failure due to:
   'found env value "__LLVM_PROFILE_RT_INIT_ONCE" "__LLVM_PROFILE_RT_INIT_ONCE"'
I assume LLVM's compiler or profiler runtime sets this, and the test would simply need to be
updated to expect it.

src/test/ui/env-funky-keys.rs

See:
src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c:381:30:
    #define LPROF_INIT_ONCE_ENV "__LLVM_PROFILE_RT_INIT_ONCE"
src/llvm-project/compiler-rt/lib/profile/InstrProfilingFile.c:402 and 404
    _putenv(LPROF_INIT_ONCE_ENV "=" LPROF_INIT_ONCE_ENV);
    setenv(LPROF_INIT_ONCE_ENV, LPROF_INIT_ONCE_ENV, 1);


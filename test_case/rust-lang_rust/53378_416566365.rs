
==55743==AddressSanitizer: failed to intercept '__isoc99_printf'
==55743==AddressSanitizer: failed to intercept '__isoc99_sprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_snprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_fprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_vprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_vsprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_vsnprintf'
==55743==AddressSanitizer: failed to intercept '__isoc99_vfprintf'
==55743==AddressSanitizer: failed to intercept '__cxa_throw'
==55743==AddressSanitizer: libc interceptors initialized
|| `[0x10007fff8000, 0x7fffffffffff]` || HighMem    ||
|| `[0x02008fff7000, 0x10007fff7fff]` || HighShadow ||
|| `[0x00008fff7000, 0x02008fff6fff]` || ShadowGap  ||
|| `[0x00007fff8000, 0x00008fff6fff]` || LowShadow  ||
|| `[0x000000000000, 0x00007fff7fff]` || LowMem     ||
MemToShadow(shadow): 0x00008fff7000 0x000091ff6dff 0x004091ff6e00 0x02008fff6fff
redzone=16
max_redzone=2048
quarantine_size_mb=256M
thread_local_quarantine_size_kb=1024K
malloc_context_size=30
SHADOW_SCALE: 3
SHADOW_GRANULARITY: 8
SHADOW_OFFSET: 0x7fff8000
==55743==Installed the sigaction for signal 11
==55743==Installed the sigaction for signal 7
==55743==Installed the sigaction for signal 8
==55743==T0: stack [0x7ffc053e5000,0x7ffc05be5000) size 0x800000; local=0x7ffc05be291c
==55743==AddressSanitizer Init done

running 1 test
test ffi::vote::tests::ffi_vote_new ... ==55743==T1: stack [0x7fae5e5ff000,0x7fae5e7feec0) size 0x1ffec0; local=0x7fae5e7feddc

[ffi_vote_new] Memory before: 2028, memory after: 3734
[ffi_vote_new] Warning: memory grew during the execution of this test. This is expected if running sanitizers, otherwise there is probably a leak.
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

==55743==T1 TSDDtor
==55743==T1 exited
==55745==Could not attach to thread 55743 (errno 1).
==55745==Failed suspending threads.
==55743==LeakSanitizer has encountered a fatal error.
==55743==HINT: For debugging, try setting environment variable LSAN_OPTIONS=verbosity=1:log_threads=1
==55743==HINT: LeakSanitizer does not work under ptrace (strace, gdb, etc)

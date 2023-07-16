
error: failed to run custom build command for `pulldown-cmark v0.0.15 (https://github.com/google/pulldown-cmark.git#13918e7d)`
process didn't exit successfully: `/home/tester/targets/target/debug/build/pulldown-cmark-04a92855a8df93af/build-script-build` (exit code: 1)
--- stderr
==21919==AddressSanitizer: failed to intercept '__isoc99_printf'
==21919==AddressSanitizer: failed to intercept '__isoc99_sprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_snprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_fprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_vprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_vsprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_vsnprintf'
==21919==AddressSanitizer: failed to intercept '__isoc99_vfprintf'
==21919==AddressSanitizer: failed to intercept '__cxa_throw'
==21919==AddressSanitizer: libc interceptors initialized
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
==21919==Installed the sigaction for signal 11
==21919==Installed the sigaction for signal 7
==21919==Installed the sigaction for signal 8
==21919==T0: stack [0x7ffe66357000,0x7ffe66b57000) size 0x800000; local=0x7ffe66b54e70
==21919==AddressSanitizer Init done
==21938==Could not attach to thread 21919 (errno 1).
==21938==Failed suspending threads.
==21919==LeakSanitizer has encountered a fatal error.
==21919==HINT: For debugging, try setting environment variable LSAN_OPTIONS=verbosity=1:log_threads=1
==21919==HINT: LeakSanitizer does not work under ptrace (strace, gdb, etc)

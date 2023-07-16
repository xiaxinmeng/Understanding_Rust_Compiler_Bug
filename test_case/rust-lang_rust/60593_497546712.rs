
running 1 test
[
    0x00007fdd79d6b730,
    0x00007fdd79d6b290,
    0x00007fdd79d6a320,
]
0x00007fdd7bb6a7d8
[src/libproc_macro/bridge/client.rs:249] "$method" = "$method"
[src/libproc_macro/bridge/client.rs:319] p_with = 0x00007fdd7be330a0
[src/libproc_macro/bridge/client.rs:356] "__run_expand1" = "__run_expand1"
[src/libproc_macro/bridge/client.rs:295] p_enter = 0x00007fdd7a09fd00
[src/libproc_macro/bridge/client.rs:319] p_with = 0x00007fdd7a09fd00
[src/libproc_macro/bridge/client.rs:249] "$method" = "$method"
[src/libproc_macro/bridge/client.rs:319] p_with = 0x00007fdd7be330a0
thread 'test_getset_expansion' panicked at 'procedural macro API is used outside of a procedural macro', src/libproc_macro/bridge/client.rs:324:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/edwin/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/libunwind.rs:97
   1: backtrace::backtrace::trace_unsynchronized
             at /home/edwin/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47

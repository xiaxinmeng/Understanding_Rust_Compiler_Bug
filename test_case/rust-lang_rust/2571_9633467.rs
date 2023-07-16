
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so
rustc: /home/rustbuild/src/rustbot/workspace-snap-stage3-x86_64-unknown-linux-gnu/src/src/rt/rust_upcall.cpp:53: void upcall_call_shim_on_c_stack(void*, void*): Assertion `false && "Foreign code threw an exception"' failed.
Stack dump:
0.  Running pass 'Function Pass Manager' on module 'rustc.rc'.
1.  Running pass 'Live IR Variables' on function '@_ZN6middle6typeck5infer7combine16eq_regions_500564anon13expr_fn_50063E'
Aborted (core dumped)
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so] Error 134

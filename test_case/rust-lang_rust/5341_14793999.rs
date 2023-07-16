
#0  __GI_getenv (name=0xf7e094c2 <str15142+2> "ST_DEBUG_MEM") at getenv.c:90
#1  0xf7d1fcce in getenv__c_stack_shim () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#2  0xf77aa24f in __morestack () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/librustrt.so
#3  0xf77998eb in call_on_c_stack (fn_ptr=0xf7d1fc90 <getenv__c_stack_shim>, args=0xf2c12fe0, this=0xf1609880) at /home/brian/dev/rust2/src/rt/rust_task.h:478
#4  upcall_call_shim_on_c_stack (args=0xf2c12fe0, fn_ptr=0xf7d1fc90 <getenv__c_stack_shim>) at /home/brian/dev/rust2/src/rt/rust_upcall.cpp:64
#5  0xf7d1fc81 in libc::funcs::c95::stdlib::getenv::_8cacb51579a97611::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#6  0xf7d803c1 in cleanup::debug_mem::anon::expr_fn_15143 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#7  0xf7d3dd2a in os::as_c_charp_13493::anon::expr_fn_13496 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#8  0xf7d3d553 in str::as_c_str_13479::anon::expr_fn_13482 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#9  0xf7cc794f in str::as_buf_9428::_76f8bd1621acf844::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#10 0xf7d3d2ed in str::as_c_str_13479::_52cafc0fb95fe1::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#11 0xf7d3dcc4 in os::as_c_charp_13493::_4913e1ce74acafa9::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#12 0xf7e04fb8 in __morestack () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#13 0xf7d80353 in cleanup::debug_mem::_7df31a6dbb47b7c::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#14 0xf7d80529 in cleanup::annihilate::_826321676dae553::_06 () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.6.so
#15 0xf77aa24f in __morestack () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/librustrt.so
#16 0xf7797cc5 in call_on_rust_stack (fn_ptr=0x8051900 <_ZN7cleanup10annihilate16_826321676dae5533_06E@plt>, args=0x0, this=0xf1609880) at /home/brian/dev/rust2/src/rt/rust_task.h:516
#17 cleanup_task (args=0xf16153b8) at /home/brian/dev/rust2/src/rt/rust_task.cpp:142
#18 0xf77aa24f in __morestack () from /opt/dev/rust2/build/i686-unknown-linux-gnu/test/../stage2/lib/rustc/i686-unknown-linux-gnu/lib/librustrt.so
#19 0xf779895c in call_on_c_stack (fn_ptr=0xf7797b70 <cleanup_task(cleanup_args*)>, args=0xf16153b8, this=0xf1609880) at /home/brian/dev/rust2/src/rt/rust_task.h:478
#20 task_start_wrapper (a=0xf16153f4) at /home/brian/dev/rust2/src/rt/rust_task.cpp:189
#21 0x00000000 in ?? ()


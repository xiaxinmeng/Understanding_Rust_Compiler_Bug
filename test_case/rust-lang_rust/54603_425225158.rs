plain
[00:23:13]    Compiling rustc-rayon-core v0.1.1
[00:23:13]    Compiling libc v0.2.43
[00:23:14] error: linking with `cc` failed: exit code: 1
[00:23:14]   |
[00:23:14]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/rustc-rayon-core-0b41451cbfe510bc/build_script_build-0b41451cbfe510bc.build_script_build.aq0c5hq9-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/rustc-rayon-core-0b41451cbfe510bc/build_script_build-0b41451cbfe510bc.build_script_build.aq0c5hq9-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/rustc-rayon-core-0b41451cbfe510bc/build_script_build-0b41451cbfe510bc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/rustc-rayon-core-0b41451cbfe510bc/build_script_build-0b41451cbfe510bc.1jzcqtcalyz2v6uy.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5dbb56afb46dab3a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-68abf883c39f5e5e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-d88ca52f0823481c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-fd856600a74ec610.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-0a7474373c7b460e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a00992e63ad2cea8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-9e29571bd610130b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-aae624166adf9237.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-a70f4a2a2c46367f.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
[00:23:14]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5dbb56afb46dab3a.rlib(std-5dbb56afb46dab3a.std.8tcrkv5i-cgu.3.rcgu.o): In function `std::sys_common::at_exit_imp::push::hcb0e5b87a46b6ea9':
[00:23:14]           std.8tcrkv5i-cgu.3:(.text._ZN3std10sys_common11at_exit_imp4push17hcb0e5b87a46b6ea9E+0x261): undefined reference to `alloc::handle_alloc_error::h3247bee8acd4f146'
[00:23:14]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5dbb56afb46dab3a.rlib(std-5dbb56afb46dab3a.std.8tcrkv5i-cgu.4.rcgu.o): In function `std::io::stdio::stdout::stdout_init::he43717ecbc65409a':
[00:23:14]           std.8tcrkv5i-cgu.4:(.text._ZN3std2io5stdio6stdout11stdout_init17he43717ecbc65409aE+0x158): undefined reference to `alloc::handle_alloc_error::h3247bee8acd4f146'
[00:23:14]           std.8tcrkv5i-cgu.4:(.text._ZN3std2io5stdio6stdout11stdout_init17he43717ecbc65409aE+0x170): undefined reference to `alloc::handle_alloc_error::h3247bee8acd4f146'
[00:23:14]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5dbb56afb46dab3a.rlib(std-5dbb56afb46dab3a.std.8tcrkv5i-cgu.6.rcgu.o): In function `std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h691e2667111da562':
[00:23:14]           std.8tcrkv5i-cgu.6:(.text._ZN3std3sys4unix9backtrace7tracing3imp16unwind_backtrace17h691e2667111da562E+0x112): undefined reference to `alloc::handle_alloc_error::h3247bee8acd4f146'
[00:23:14]           std.8tcrkv5i-cgu.6:(.text._ZN3std3sys4unix9backtrace7tracing3imp16unwind_backtrace17h691e2667111da562E+0x123): undefined reference to `alloc::handle_alloc_error::h3247bee8acd4f146'
[00:23:14]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5dbb56afb46dab3a.rlib(std-5dbb56afb46dab3a.std.8tcrkv5i-cgu.7.rcgu.o):std.8tcrkv5i-cgu.7:(.text._ZN3std3ffi5c_str7CString3new17h3ee05fc805b6eaffE+0x32): more undefined references to `alloc::handle_alloc_error::h3247bee8acd4f146' follow
[00:23:14]           
[00:23:14] 
[00:23:14] error: aborting due to previous error
[00:23:14] 
[00:23:14] 
[00:23:14] error: Could not compile `rustc-rayon-core`.
[00:23:14] warning: build failed, waiting for other jobs to finish...
[00:23:15] Makefile:28: recipe for target 'all' failed
[00:23:15] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0100ea71
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

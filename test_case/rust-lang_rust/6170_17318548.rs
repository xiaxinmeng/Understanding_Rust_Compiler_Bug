
/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/usr/bin/arm-apple-darwin10-llvm-g++-4.2  -o rt/arm-apple-darwin/librustrt.dylib -dynamiclib -lpthread -framework CoreServices -Wl,-no_compact_unwind
  -Wl,-exported_symbols_list,rt/rustrt.darwin.def  rt/arm-apple-darwin/sync/timer.o rt/arm-apple-darwin/sync/lock_and_signal.o rt/arm-apple-darwin/sync/rust_thread.o rt/arm-apple-darwin/rust.o
 rt/arm-apple-darwin/rust_builtin.o rt/arm-apple-darwin/rust_run_program.o rt/arm-apple-darwin/rust_env.o
 rt/arm-apple-darwin/rust_rng.o rt/arm-apple-darwin/rust_sched_loop.o rt/arm-apple-darwin/rust_sched_launcher.o
 rt/arm-apple-darwin/rust_sched_driver.o rt/arm-apple-darwin/rust_scheduler.o rt/arm-apple-darwin/rust_sched_reaper.o
 rt/arm-apple-darwin/rust_task.o rt/arm-apple-darwin/rust_stack.o rt/arm-apple-darwin/rust_upcall.o
 rt/arm-apple-darwin/rust_uv.o rt/arm-apple-darwin/rust_crate_map.o rt/arm-apple-darwin/rust_log.o
 rt/arm-apple-darwin/rust_gc_metadata.o rt/arm-apple-darwin/rust_util.o rt/arm-apple-darwin/rust_exchange_alloc.o
 rt/arm-apple-darwin/isaac/randport.o rt/arm-apple-darwin/miniz.o rt/arm-apple-darwin/rust_kernel.o
 rt/arm-apple-darwin/rust_abi.o rt/arm-apple-darwin/rust_debug.o rt/arm-apple-darwin/memory_region.o
 rt/arm-apple-darwin/boxed_region.o rt/arm-apple-darwin/arch/arm/context.o rt/arm-apple-darwin/arch/arm/gpr.o
 rt/arm-apple-darwin/rust_android_dummy.o rt/arm-apple-darwin/linenoise/linenoise.o rt/arm-apple-darwin/linenoise/utf8.o
 rt/arm-apple-darwin/arch/arm/_context.o rt/arm-apple-darwin/arch/arm/ccall.o rt/arm-apple-darwin/arch/arm/record_sp.o
  rt/arm-apple-darwin/libuv/libuv.a
  -Wl,-install_name,@rpath/librustrt.dylib
ld: warning: ld: warning: ignoring file /usr/lib/dylib1.o,
 missing required architecture armv7 in file /usr/lib/dylib1.o
 (2 slices)ignoring file /usr/lib/libpthread.dylib, 
missing required architecture armv7 in file /usr/lib/libpthread.dylib (2 slices)

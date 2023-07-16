
Starting program: /home/roughl/.cargo/bin/rustc --crate-name water_sensor_firmware src/main.rs --crate-type bin --emit=dep-info,link -C opt-level=3 -C lto -C debuginfo=2 -C metadata=f52654b5cf7ba5eb -C extra-filename=-f52654b5cf7ba5eb --out-dir /home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps --target thumbv6m-none-eabi -L dependency=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps -L dependency=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/release/deps --extern cortex_m_semihosting=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps/libcortex_m_semihosting-d4bd87d810ba6491.rlib --extern lpc11uxx=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps/liblpc11uxx-12094799941f5318.rlib --extern cortex_m_rt=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps/libcortex_m_rt-60430ea2cdd7c81a.rlib --extern cortex_m=/home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/deps/libcortex_m-8bf6d3b61cf96d39.rlib -C link-arg=-Tlink.x -C linker=arm-none-eabi-ld -Z linker-flavor=ld --sysroot /home/roughl/.xargo -L /home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/build/water-sensor-firmware-5dc893357f99df90/out -L /home/roughl/proggen/projects/coredump/water-sensor/water-sensor-firmware-rs/target/thumbv6m-none-eabi/release/build/cortex-m-rt-d65db69c9e56ca7e/out
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
process 26084 is executing new program: /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
[New Thread 0x7fffecfff700 (LWP 26088)]
warning: unused `#[macro_use]` import
 --> src/main.rs:5:1
  |
5 | #[macro_use]
  | ^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused import: `cortex_m_semihosting::hio`
  --> src/main.rs:14:5
   |
14 | use cortex_m_semihosting::hio;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

[New Thread 0x7fffe83ff700 (LWP 26089)]
[New Thread 0x7fffe7dff700 (LWP 26090)]
[New Thread 0x7fffe77ff700 (LWP 26091)]
[Thread 0x7fffe77ff700 (LWP 26091) exited]
[New Thread 0x7fffe77ff700 (LWP 26092)]
[New Thread 0x7fffe6fff700 (LWP 26093)]
[New Thread 0x7fffe69ff700 (LWP 26094)]
[New Thread 0x7fffe63ff700 (LWP 26095)]
[New Thread 0x7fffe5dff700 (LWP 26096)]
[Thread 0x7fffe63ff700 (LWP 26095) exited]
[Thread 0x7fffe5dff700 (LWP 26096) exited]
[Thread 0x7fffe69ff700 (LWP 26094) exited]
[Thread 0x7fffe6fff700 (LWP 26093) exited]
[Thread 0x7fffe77ff700 (LWP 26092) exited]
[New Thread 0x7fffe77ff700 (LWP 26097)]
[New Thread 0x7fffe6fff700 (LWP 26098)]
[New Thread 0x7fffe5dff700 (LWP 26099)]
[New Thread 0x7fffe69ff700 (LWP 26100)]
[New Thread 0x7fffe63ff700 (LWP 26101)]
[New Thread 0x7fffe53ff700 (LWP 26102)]
[New Thread 0x7fffe51fe700 (LWP 26103)]
[New Thread 0x7fffe4bff700 (LWP 26104)]
[New Thread 0x7fffe43ff700 (LWP 26105)]
[New Thread 0x7fffe35ff700 (LWP 26106)]
[New Thread 0x7fffe27ff700 (LWP 26107)]
[New Thread 0x7fffe1fff700 (LWP 26108)]
[Thread 0x7fffe63ff700 (LWP 26101) exited]
[New Thread 0x7fffe63ff700 (LWP 26109)]
[Thread 0x7fffe51fe700 (LWP 26103) exited]
[New Thread 0x7fffe51fe700 (LWP 26110)]
[New Thread 0x7fffe17ff700 (LWP 26111)]
[Thread 0x7fffe5dff700 (LWP 26099) exited]
[New Thread 0x7fffe5dff700 (LWP 26112)]
[New Thread 0x7fffe0fff700 (LWP 26113)]
[New Thread 0x7fffe09ff700 (LWP 26114)]
[Thread 0x7fffe4bff700 (LWP 26104) exited]
[New Thread 0x7fffe4bff700 (LWP 26115)]
[Thread 0x7fffe77ff700 (LWP 26097) exited]
[New Thread 0x7fffe77ff700 (LWP 26116)]
[Thread 0x7fffe43ff700 (LWP 26105) exited]
[Thread 0x7fffe69ff700 (LWP 26100) exited]
[Thread 0x7fffe6fff700 (LWP 26098) exited]
[New Thread 0x7fffe6fff700 (LWP 26117)]
[Thread 0x7fffe53ff700 (LWP 26102) exited]
[New Thread 0x7fffe53ff700 (LWP 26118)]
[Thread 0x7fffe35ff700 (LWP 26106) exited]
[New Thread 0x7fffe35ff700 (LWP 26119)]
[New Thread 0x7fffe69ff700 (LWP 26120)]
[New Thread 0x7fffe43ff700 (LWP 26121)]
[Thread 0x7fffe0fff700 (LWP 26113) exited]
[New Thread 0x7fffdfdff700 (LWP 26122)]
[Thread 0x7fffe27ff700 (LWP 26107) exited]
[New Thread 0x7fffe27ff700 (LWP 26123)]
[Thread 0x7fffe69ff700 (LWP 26120) exited]
[Thread 0x7fffe5dff700 (LWP 26112) exited]
[Thread 0x7fffe17ff700 (LWP 26111) exited]
[Thread 0x7fffe51fe700 (LWP 26110) exited]
[New Thread 0x7fffe69ff700 (LWP 26124)]
[Thread 0x7fffe53ff700 (LWP 26118) exited]
[New Thread 0x7fffe53ff700 (LWP 26125)]
[Thread 0x7fffe6fff700 (LWP 26117) exited]
[New Thread 0x7fffe6fff700 (LWP 26126)]
[Thread 0x7fffe63ff700 (LWP 26109) exited]
[New Thread 0x7fffe63ff700 (LWP 26127)]
[New Thread 0x7fffe51fe700 (LWP 26128)]
[New Thread 0x7fffe17ff700 (LWP 26129)]
[Thread 0x7fffe35ff700 (LWP 26119) exited]
[New Thread 0x7fffe35ff700 (LWP 26130)]
[New Thread 0x7fffe5dff700 (LWP 26131)]
[Thread 0x7fffe69ff700 (LWP 26124) exited]
[Thread 0x7fffe27ff700 (LWP 26123) exited]
[Thread 0x7fffe43ff700 (LWP 26121) exited]
[New Thread 0x7fffe69ff700 (LWP 26132)]
[Thread 0x7fffe4bff700 (LWP 26115) exited]
[New Thread 0x7fffe4bff700 (LWP 26133)]
[Thread 0x7fffe63ff700 (LWP 26127) exited]
[Thread 0x7fffe09ff700 (LWP 26114) exited]
[New Thread 0x7fffe09ff700 (LWP 26134)]
[Thread 0x7fffe51fe700 (LWP 26128) exited]
[New Thread 0x7fffe51fe700 (LWP 26135)]
[New Thread 0x7fffe63ff700 (LWP 26136)]
[Thread 0x7fffe1fff700 (LWP 26108) exited]
[New Thread 0x7fffe1fff700 (LWP 26137)]
[New Thread 0x7fffe43ff700 (LWP 26138)]
[Thread 0x7fffe63ff700 (LWP 26136) exited]
[Thread 0x7fffe17ff700 (LWP 26129) exited]
[Thread 0x7fffdfdff700 (LWP 26122) exited]
[Thread 0x7fffe77ff700 (LWP 26116) exited]
[New Thread 0x7fffdfdff700 (LWP 26139)]
[Thread 0x7fffe43ff700 (LWP 26138) exited]
[Thread 0x7fffe1fff700 (LWP 26137) exited]
[Thread 0x7fffe51fe700 (LWP 26135) exited]
[Thread 0x7fffe09ff700 (LWP 26134) exited]
[Thread 0x7fffe4bff700 (LWP 26133) exited]
[Thread 0x7fffe69ff700 (LWP 26132) exited]
[Thread 0x7fffe53ff700 (LWP 26125) exited]
[New Thread 0x7fffe4bff700 (LWP 26140)]
[New Thread 0x7fffe51fe700 (LWP 26141)]
[New Thread 0x7fffe09ff700 (LWP 26142)]
[Thread 0x7fffe6fff700 (LWP 26126) exited]
[New Thread 0x7fffe6fff700 (LWP 26143)]
[Thread 0x7fffe09ff700 (LWP 26142) exited]
[Thread 0x7fffe51fe700 (LWP 26141) exited]
[Thread 0x7fffe4bff700 (LWP 26140) exited]
[New Thread 0x7fffe51fe700 (LWP 26144)]
[New Thread 0x7fffe4bff700 (LWP 26145)]
[New Thread 0x7fffe09ff700 (LWP 26146)]
[New Thread 0x7fffe43ff700 (LWP 26147)]
[Thread 0x7fffe09ff700 (LWP 26146) exited]
[New Thread 0x7fffe09ff700 (LWP 26148)]
[New Thread 0x7fffe69ff700 (LWP 26149)]
[Thread 0x7fffe4bff700 (LWP 26145) exited]
[Thread 0x7fffe51fe700 (LWP 26144) exited]
[Thread 0x7fffe6fff700 (LWP 26143) exited]
[New Thread 0x7fffe4bff700 (LWP 26150)]
[New Thread 0x7fffe51fe700 (LWP 26151)]
[New Thread 0x7fffe6fff700 (LWP 26152)]
[Thread 0x7fffe51fe700 (LWP 26151) exited]
[Thread 0x7fffe4bff700 (LWP 26150) exited]
[Thread 0x7fffe69ff700 (LWP 26149) exited]
[Thread 0x7fffe09ff700 (LWP 26148) exited]
[Thread 0x7fffe43ff700 (LWP 26147) exited]
[Thread 0x7fffdfdff700 (LWP 26139) exited]
[Thread 0x7fffe5dff700 (LWP 26131) exited]
[New Thread 0x7fffe4bff700 (LWP 26153)]
[New Thread 0x7fffe51fe700 (LWP 26154)]
[New Thread 0x7fffdfdff700 (LWP 26155)]
[New Thread 0x7fffe69ff700 (LWP 26156)]
[New Thread 0x7fffe43ff700 (LWP 26157)]
[Thread 0x7fffdfdff700 (LWP 26155) exited]
[Thread 0x7fffe51fe700 (LWP 26154) exited]
[Thread 0x7fffe69ff700 (LWP 26156) exited]
[Thread 0x7fffe43ff700 (LWP 26157) exited]
[Thread 0x7fffe4bff700 (LWP 26153) exited]
[Thread 0x7fffe6fff700 (LWP 26152) exited]

Thread 44 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffe35ff700 (LWP 26130)]
0x00007fffef47f813 in (anonymous namespace)::AddressingModeMatcher::matchOperationAddr(llvm::User*, unsigned int, unsigned int, bool*) [clone .part.974] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
(gdb) bt
#0  0x00007fffef47f813 in (anonymous namespace)::AddressingModeMatcher::matchOperationAddr(llvm::User*, unsigned int, unsigned int, bool*) [clone .part.974] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#1  0x00007fffef47fc7a in (anonymous namespace)::AddressingModeMatcher::matchAddr(llvm::Value*, unsigned int) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#2  0x00007fffef483c69 in (anonymous namespace)::CodeGenPrepare::optimizeMemoryInst(llvm::Instruction*, llvm::Value*, llvm::Type*, unsigned int) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#3  0x00007fffef484ff5 in (anonymous namespace)::CodeGenPrepare::optimizeInst(llvm::Instruction*, bool&) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#4  0x00007fffef487bdf in (anonymous namespace)::CodeGenPrepare::runOnFunction(llvm::Function&) [clone .part.1001] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#5  0x00007fffefc6edea in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#6  0x00007fffefc6ee83 in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#7  0x00007fffefc6f7f0 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#8  0x00007fffee6dfc47 in LLVMRustWriteOutputFile () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-c09101ffce2aa427.so
#9  0x00007ffff5fb3276 in rustc_trans::back::write::write_output_file () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#10 0x00007ffff5fe9254 in rustc_trans::back::write::codegen::{{closure}} () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#11 0x00007ffff5fe3339 in rustc::util::common::time () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#12 0x00007ffff5fb5355 in rustc_trans::back::write::codegen () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#13 0x00007ffff5fa7ab3 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#14 0x00007ffff5faa908 in std::panicking::try::do_call () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#15 0x00007ffff76b3bdf in __rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:101
#16 0x00007ffff5fb1b3b in <F as alloc::boxed::FnBox<A>>::call_box () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-a5f63655b1c9a5ed.so
#17 0x00007ffff768de18 in _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h0ba846e0a948eb30 ()
    at /checkout/src/liballoc/boxed.rs:827
#18 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:24
#19 0x00007ffff7692229 in std::sys::unix::thread::Thread::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:90
#20 0x00007ffff16dc08a in start_thread () from /usr/lib/libpthread.so.0
#21 0x00007ffff735c42f in clone () from /usr/lib/libc.so.6

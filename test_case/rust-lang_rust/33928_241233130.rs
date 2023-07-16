
make[2]: Leaving directory '/home/dev/src/rustc-nightly/arm-unknown-linux-gnueabihf/rt/libbacktrace'
make[1]: Leaving directory '/home/dev/src/rustc-nightly/arm-unknown-linux-gnueabihf/rt/libbacktrace'
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liblibc
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librand
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librustc_unicode
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/librustc_bitflags
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liballoc_system
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libunwind
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/liballoc
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcollections
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libpanic_abort
rustc: arm-unknown-linux-gnueabihf/stage0/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libpanic_unwind
error[E0522]: definition of an unknown language item: `eh_personality_catch`.
   --> src/libpanic_unwind/gcc.rs:284:1
    |
284 | pub unsafe extern "C" fn rust_eh_personality_catch(state: uw::_Unwind_State,
    | ^

error: aborting due to previous error

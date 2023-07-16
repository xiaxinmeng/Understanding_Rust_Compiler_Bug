bash

# compiling uutils/coreutils
# rustc 1.52.0-nightly (fe1bf8e05 2021-02-23)
RUSTFLAGS=-Zsanitizer=memory MSAN_OPTIONS=verbosity=2 cargo test --target=x86_64-unknown-linux-gnu
...
 Running `/root/github/meta_coreutils/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f`
==363==MemorySanitizer: failed to intercept '__isoc99_printf'
'==363==MemorySanitizer: failed to intercept '__isoc99_sprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_snprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_fprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_vprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_vsprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_vsnprintf'
'==363==MemorySanitizer: failed to intercept '__isoc99_vfprintf'
'==363==MemorySanitizer: failed to intercept 'crypt'
'==363==MemorySanitizer: failed to intercept 'crypt_r'
'==363==Installed the sigaction for signal 11
==363==Installed the sigaction for signal 7
==363==Installed the sigaction for signal 8
__msan_init 0x5562988b87b0
app-1: 0 - ffffffffff
shadow-2: 10000000000 - fffffffffff
invalid: 100000000000 - 10ffffffffff
origin-2: 110000000000 - 1fffffffffff
shadow-3: 200000000000 - 2fffffffffff
origin-3: 300000000000 - 3fffffffffff
invalid: 400000000000 - 4fffffffffff
shadow-1: 500000000000 - 50ffffffffff
app-2: 510000000000 - 5fffffffffff
origin-1: 600000000000 - 60ffffffffff
invalid: 610000000000 - 6fffffffffff
app-3: 700000000000 - 7fffffffffff
==363==Using llvm-symbolizer found at: /usr/bin/llvm-symbolizer
MemorySanitizer init done
Uninitialized bytes in __interceptor_memchr at offset 0 inside [0x701000000000, 4)
Shadow map of [0x201000000000, 0x201000000004), 4 bytes:
0x201000000000: ffffffff ........ ........ ........

==363==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x55629896c82a in std::sys::unix::memchr::memchr::h25e994819b34ae60 /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/sys/unix/memchr.rs:6:9
    #1 0x55629896c82a in std::memchr::memchr::hbfa59afe241958cf /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/memchr.rs:28:5
    #2 0x55629896c82a in std::ffi::c_str::CString::_new::h1135c0ef219c53d5 /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/ffi/c_str.rs:405:15
    #3 0x55629896c82a in std::ffi::c_str::CString::new::hfad134e936e82551 /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/ffi/c_str.rs:401:9
    #4 0x55629896c82a in std::thread::Thread::new::_$u7b$$u7b$closure$u7d$$u7d$::hd79cbaf3f8b96dcf /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/thread/mod.rs:1068:26
    #5 0x55629896c82a in core::option::Option$LT$T$GT$::map::h6bf62377f786127c /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/core/src/option.rs:487:29
    #6 0x55629896c82a in std::thread::Thread::new::hdef806dddada2f31 /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/thread/mod.rs:1068:13
    #7 0x556298977b45 in std::rt::lang_start_internal::hc92e27a69d75de2a /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/rt.rs:44:22
    #8 0x5562989151cb in std::rt::lang_start::hd7b49f57b17ff86a /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:65:5
    #9 0x556298914b31 in main (/root/github/meta_coreutils/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0x73b31)
    #10 0x7f20421ba069 in __libc_start_main (/lib64/libc.so.6+0x21069)
    #11 0x5562988b81d9 in _start (/root/github/meta_coreutils/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0x171d9)

SUMMARY: MemorySanitizer: use-of-uninitialized-value /rustc/fe1bf8e05c39bdcc73fc09e246b7209444e389bc/library/std/src/sys/unix/memchr.rs:6:9 in std::sys::unix::memchr::memchr::h25e994819b34ae60
Exiting
error: test failed, to rerun pass '--bin coreutils'
-bash-4.2# /usr/bin/llvm-symbolizer --version
LLVM (http://llvm.org/):
  LLVM version 7.0.1
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: skylake

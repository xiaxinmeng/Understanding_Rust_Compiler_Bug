
$ rustc -C target-feature=+crt-static hello_world.rs
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-m64" "-L" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib" "hello_world.0.o" "-o" "hello_world" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/librand-36efc09e6c3aa7b6.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcollections-64ae13b816203712.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_unicode-a6dea33a6a8b7481.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-52493d6bc7a4edfb.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-8ffb79da497eb47f.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc_system-35e8b7ebb64c18bd.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-2b2567c2ae2ad7d0.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-6588f2833738184c.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-5ef8fa488c4b9f32.rlib"
  = note: /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `collections::vec::{{impl}}::into_boxed_slice<u8>':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/raw_vec.rs:517: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `alloc::heap::deallocate':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `core::result::unwrap_failed<std::ffi::c_str::NulError>':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `std::sys::imp::mutex::{{impl}}::unlock':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/mutex.rs:72: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `drop':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o):/home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: more undefined references to `_Unwind_Resume' follow
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `std::sys::imp::backtrace::tracing::imp::write':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42: undefined reference to `_Unwind_Backtrace'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `std::sys::imp::backtrace::tracing::imp::write::trace_fn':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:62: undefined reference to `_Unwind_GetIPInfo'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:83: undefined reference to `_Unwind_FindEnclosingFunction'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:93: undefined reference to `_Unwind_Resume'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:103: undefined reference to `_Unwind_Resume'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:103: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `alloc::heap::deallocate':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o): In function `std::sys::imp::os_str::{{impl}}::fmt':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-86059d53218552f5.rlib(std-86059d53218552f5.0.o):/home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/arc.rs:768: more undefined references to `_Unwind_Resume' follow
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::imp::find_eh_action':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:264: undefined reference to `_Unwind_GetLanguageSpecificData'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:266: undefined reference to `_Unwind_GetIPInfo'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::dwarf::eh::find_eh_action':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/dwarf/eh.rs:69: undefined reference to `_Unwind_GetRegionStart'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::imp::rust_eh_personality':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:172: undefined reference to `_Unwind_SetGR'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:173: undefined reference to `_Unwind_SetGR'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:174: undefined reference to `_Unwind_SetIP'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::imp::find_eh_action::{{closure}}':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:272: undefined reference to `_Unwind_GetTextRelBase'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:273: undefined reference to `_Unwind_GetDataRelBase'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::imp::cleanup':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:100: undefined reference to `_Unwind_DeleteException'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `panic_unwind::imp::panic':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:83: undefined reference to `_Unwind_RaiseException'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `core::ops::FnOnce::call_once':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:272: undefined reference to `_Unwind_GetTextRelBase'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-12770ed0830004c4.rlib(panic_unwind-12770ed0830004c4.0.o): In function `core::ops::FnOnce::call_once':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:273: undefined reference to `_Unwind_GetDataRelBase'
          collect2: error: ld returned 1 exit status

error: aborting due to previous error

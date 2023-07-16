
$ rustc hello_world.rs
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-Wl,-(" "-m64" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o" "-L" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib" "hello_world.0.o" "-o" "hello_world" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/librand-709b996bacb8eb07.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcollections-df1635bc25c3e592.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_unicode-00ee5191e548d906.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-1e0a4dc78495337e.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-a1562f522536bd3b.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc_system-bde4c172625df17f.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-bf68330d3f5e10dd.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-963774836b754896.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-9802cab648d12028.rlib" "/usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crtn.o" "-Wl,-)"
  = note: /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o: In function `_start':
          crt1.c:(.text+0x0): multiple definition of `_start'
          /usr/lib/gcc/x86_64-alpine-linux-musl/6.3.0/../../../../lib/Scrt1.o:Scrt1.c:(.text+0x0): first defined here
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o: In function `_start_c':
          crt1.c:(.text._start_c+0x0): multiple definition of `_start_c'
          /usr/lib/gcc/x86_64-alpine-linux-musl/6.3.0/../../../../lib/Scrt1.o:Scrt1.c:(.text._start_c+0x0): first defined here
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o: In function `_init':
          /home/buildozer/aports/main/musl/src/musl-1.1.16/crt/x86_64/crti.s:4: multiple definition of `_init'
          /usr/lib/gcc/x86_64-alpine-linux-musl/6.3.0/../../../../lib/crti.o:/home/buildozer/aports/main/musl/src/musl-1.1.16/crt/x86_64/crti.s:4: first defined here
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o: In function `_fini':
          /home/buildozer/aports/main/musl/src/musl-1.1.16/crt/x86_64/crti.s:4: multiple definition of `_fini'
          /usr/lib/gcc/x86_64-alpine-linux-musl/6.3.0/../../../../lib/crti.o:/home/buildozer/aports/main/musl/src/musl-1.1.16/crt/x86_64/crti.s:4: first defined here
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `collections::vec::{{impl}}::into_boxed_slice<u8>':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/raw_vec.rs:517: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `alloc::heap::deallocate':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `std::sys::imp::mutex::{{impl}}::unlock':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/mutex.rs:72: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `core::result::unwrap_failed<std::ffi::c_str::NulError>':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `drop':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o):/home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: more undefined references to `_Unwind_Resume' follow
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `std::sys::imp::backtrace::tracing::imp::write':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42: undefined reference to `_Unwind_Backtrace'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `std::sys::imp::backtrace::tracing::imp::write::trace_fn':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:62: undefined reference to `_Unwind_GetIPInfo'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:83: undefined reference to `_Unwind_FindEnclosingFunction'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:93: undefined reference to `_Unwind_Resume'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:103: undefined reference to `_Unwind_Resume'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:103: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `alloc::heap::deallocate':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o): In function `std::sys::imp::os_str::{{impl}}::fmt':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/heap.rs:113: undefined reference to `_Unwind_Resume'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d73d432e81fd8b19.rlib(std-d73d432e81fd8b19.0.o):/home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/liballoc/arc.rs:768: more undefined references to `_Unwind_Resume' follow
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::imp::find_eh_action':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:264: undefined reference to `_Unwind_GetLanguageSpecificData'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:266: undefined reference to `_Unwind_GetIPInfo'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::dwarf::eh::find_eh_action':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/dwarf/eh.rs:69: undefined reference to `_Unwind_GetRegionStart'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::imp::rust_eh_personality':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:172: undefined reference to `_Unwind_SetGR'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:173: undefined reference to `_Unwind_SetGR'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:174: undefined reference to `_Unwind_SetIP'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::imp::find_eh_action::{{closure}}':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:272: undefined reference to `_Unwind_GetTextRelBase'
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:273: undefined reference to `_Unwind_GetDataRelBase'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::imp::cleanup':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:100: undefined reference to `_Unwind_DeleteException'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `panic_unwind::imp::panic':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:83: undefined reference to `_Unwind_RaiseException'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `core::ops::FnOnce::call_once':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:272: undefined reference to `_Unwind_GetTextRelBase'
          /usr/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-85f96f5a1caa812c.rlib(panic_unwind-85f96f5a1caa812c.0.o): In function `core::ops::FnOnce::call_once':
          /home/jirutjak/aports/testing/rust/src/rustc-1.16.0-src/src/libpanic_unwind/gcc.rs:273: undefined reference to `_Unwind_GetDataRelBase'
          collect2: error: ld returned 1 exit status

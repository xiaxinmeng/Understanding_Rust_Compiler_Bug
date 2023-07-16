plain
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-73e4f69c1c131c8b.rustc_main.6j1563t6-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-73e4f69c1c131c8b.rustc_main.6j1563t6-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-73e4f69c1c131c8b.rustc_main.6j1563t6-cgu.2.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-73e4f69c1c131c8b" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-9109e3c35c4ad4aa/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-119f4caf9adeae34/out" "-L" "/usr/lib/llvm-10/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-162ebbb6b58f4d22" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-51387cd2c9c30626" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b047d35ef14db125.rlib" "-Wl,-Bdynamic" "-lLLVM-10" "-lstdc++" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-162ebbb6b58f4d22.so: undefined reference to `std_detect::detect::cache::CACHE'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-162ebbb6b58f4d22.so: undefined reference to `hashbrown::raw::sse2::Group::static_empty'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-162ebbb6b58f4d22.so: undefined reference to `std_detect::detect::cache::detect_and_initialize'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-162ebbb6b58f4d22.so: undefined reference to `hashbrown::raw::Fallibility::alloc_err'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-162ebbb6b58f4d22.so: undefined reference to `hashbrown::raw::Fallibility::capacity_overflow'
          collect2: error: ld returned 1 exit status

error: aborting due to previous error

error: could not compile `rustc-main`

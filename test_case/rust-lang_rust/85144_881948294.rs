plain
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-deddb2dd1fd284f9.rustc_main.04bc62e8-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-deddb2dd1fd284f9.rustc_main.04bc62e8-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-deddb2dd1fd284f9.rustc_main.04bc62e8-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-deddb2dd1fd284f9.rustc_main.04bc62e8-cgu.3.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-fc72508f457ff2af/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-1b17de65bfd28043/out" "-L" "/usr/lib/llvm-10/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-94967fd0a7379a2d" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-acc102086c90e281" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-b8b1093eb15f57a0.rlib" "-Wl,-Bdynamic" "-lLLVM-10" "-lstdc++" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-deddb2dd1fd284f9" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-94967fd0a7379a2d.so: undefined reference to `hashbrown::raw::Fallibility::alloc_err'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-94967fd0a7379a2d.so: undefined reference to `hashbrown::raw::Fallibility::capacity_overflow'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-94967fd0a7379a2d.so: undefined reference to `hashbrown::raw::sse2::Group::static_empty'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-94967fd0a7379a2d.so: undefined reference to `std_detect::detect::cache::detect_and_initialize'
          /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-94967fd0a7379a2d.so: undefined reference to `std_detect::detect::cache::CACHE'
          collect2: error: ld returned 1 exit status
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error

error: could not compile `rustc-main`


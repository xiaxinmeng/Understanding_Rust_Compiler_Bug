
$ cargo build
    Updating crates.io index
  Downloaded cc v1.0.28
   Compiling cc v1.0.28
   Compiling cc-02-broken v0.1.0 (/home/dcreager/git/rust-cc-linking/cc-02-broken)
   Compiling cc-01-works-by-accident v0.1.0 (/home/dcreager/git/rust-cc-linking/cc-01-works-by-accident)
warning: redundant linker flag specified for library `stdc++`

warning: redundant linker flag specified for library `stdc++`

error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.1kohxk4oy5bpc1tz.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.1kp16pt7op8wd113.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.3l6cjvlvoh2c3so3.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.3nhkg8zoprl8ydjh.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.3sj4jtttlglzuhf6.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.4po5c5g0wu2dwvt4.rcgu.o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.50dua0cq8wfcaoc6.rcgu.o" "-o" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d" "/home/dcreager/git/rust-cc-linking/target/debug/deps/cc_02_broken-3cc2faff0ce9900d.h8uijmbpxrzuyns.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/home/dcreager/git/rust-cc-linking/target/debug/deps" "-L" "/home/dcreager/git/rust-cc-linking/target/debug/build/cc-02-broken-b6be909df134df9e/out" "-L" "/home/dcreager/git/rust-cc-linking/target/debug/build/cc-02-broken-b6be909df134df9e/out" "-L" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lsmall" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lstdc++" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lbig" "-Wl,--no-whole-archive" "-Wl,--start-group" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-296aebc3a1e4ecfc.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-8cfe07ca425de5e2.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-8a74789e627c1a40.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-7e3f00e10075cd58.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-d3b3d6193d1b05b1.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-4c47fecb36fc2925.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-c0c586c7cb2de27a.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-f1719f3d136282db.rlib" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-b22b3fc53f39823e.rlib" "-Wl,--end-group" "/home/dcreager/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-ec6cc32e99245114.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
  = note: /usr/bin/ld: /home/dcreager/git/rust-cc-linking/target/debug/build/cc-02-broken-b6be909df134df9e/out/libbig.a(big.o): in function `big_function':
          /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:8: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::basic_string()'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:12: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::operator+=(char const*)'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:14: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::operator+=(char const*)'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:16: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::size() const'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:8: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::~basic_string()'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/cc-02-broken/src/big.cc:8: undefined reference to `std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::~basic_string()'
          /usr/bin/ld: /home/dcreager/git/rust-cc-linking/target/debug/build/cc-02-broken-b6be909df134df9e/out/libbig.a(big.o):(.data.rel.local.DW.ref.__gxx_personality_v0[DW.ref.__gxx_personality_v0]+0x0): undefined reference to `__gxx_personality_v0'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `cc-02-broken`.
warning: build failed, waiting for other jobs to finish...
error: build failed

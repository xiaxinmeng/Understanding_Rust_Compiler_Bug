plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f382c2748aec2ada91eff88840c996644ff0f70d and caefaf599d1921cdb4acbbcd8038a147a53f2ee0
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/tmp/rustc69gUaU/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-17b4e485bdc9b4cc.rustc_main.9152ab56-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-17b4e485bdc9b4cc.rustc_main.9152ab56-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-17b4e485bdc9b4cc.rustc_main.9152ab56-cgu.2.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-0c6505bd522ceff5/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-a91e4a02ed33ed69/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/lib" "-L" "/usr/lib/gcc/x86_64-linux-gnu/11" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-d57528d7763184e8" "-lLLVM-15-rust-1.66.0-nightly" "-ldl" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-ec7210bb6a2d63f7" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-e15157c448d27e55.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_main-17b4e485bdc9b4cc" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-z,origin" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-ec7210bb6a2d63f7.so: undefined reference to `__clzsi2'
          collect2: error: ld returned 1 exit status
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: could not compile `rustc-main` due to previous error
Build completed unsuccessfully in 0:10:48
cat: /tmp/toolstate/toolstates.json: No such file or directory

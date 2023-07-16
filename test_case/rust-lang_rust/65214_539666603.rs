plain
2019-10-08T18:59:22.8233574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T18:59:23.3709845Z ##[command]git config gc.auto 0
2019-10-08T18:59:23.3715205Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T18:59:23.3725521Z ##[command]git config --get-all http.proxy
2019-10-08T18:59:23.3766880Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65214/merge:refs/remotes/pull/65214/merge
---
2019-10-08T19:26:10.3429064Z    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-10-08T19:27:11.3484862Z    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-10-08T19:27:59.1944740Z    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin/deprecated)
2019-10-08T19:30:18.1180853Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2019-10-08T19:30:18.3678809Z error: linking with `cc` failed: exit code: 1
2019-10-08T19:30:18.3679522Z   |
2019-10-08T19:30:18.3682045Z   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-ff7ef2a49a698d0a.rustc_binary.b9c6s0cq-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-ff7ef2a49a698d0a.rustc_binary.b9c6s0cq-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_binary-ff7ef2a49a698d0a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-c5a99509fe2f0f44/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-cde8801c1a0250cb/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps" "-lrustc_driver-dc46f81f35f7db72" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ltest-4441714d48a81082" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-539b46668317e4b1" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-58f6922a5d63f76a.rlib" "-Wl,-Bdynamic" "-lutil" "-lutil" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
2019-10-08T19:30:18.3682846Z   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-539b46668317e4b1.so: undefined reference to `__sync_val_compare_and_swap_16'
2019-10-08T19:30:18.3683092Z           collect2: error: ld returned 1 exit status
2019-10-08T19:30:18.3683360Z 
2019-10-08T19:30:18.3753489Z error: aborting due to previous error
2019-10-08T19:30:18.3753661Z 
2019-10-08T19:30:18.3931868Z error: could not compile `rustc-main`.
---
2019-10-08T19:30:18.4057326Z == clock drift check ==
2019-10-08T19:30:18.4079146Z   local time: Tue Oct  8 19:30:18 UTC 2019
2019-10-08T19:30:18.5013578Z   network time: Tue, 08 Oct 2019 19:30:18 GMT
2019-10-08T19:30:18.5015332Z == end clock drift check ==
2019-10-08T19:30:21.0151165Z ##[error]Bash exited with code '1'.
2019-10-08T19:30:21.0199463Z ##[section]Starting: Checkout
2019-10-08T19:30:21.0201615Z ==============================================================================
2019-10-08T19:30:21.0201666Z Task         : Get sources
2019-10-08T19:30:21.0201728Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

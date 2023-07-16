plain
2020-01-20T05:55:43.3971346Z status: exit code: 2
2020-01-20T05:55:43.3971428Z command: "make"
2020-01-20T05:55:43.3971505Z stdout:
2020-01-20T05:55:43.3971757Z ------------------------------------------
2020-01-20T05:55:43.3971849Z # First up, try some defaults
2020-01-20T05:55:43.3974511Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319  -Clinker=arm-none-eabi-gcc --crate-type rlib foo.rs
2020-01-20T05:55:43.3975915Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319  -Clinker=arm-none-eabi-gcc --crate-type dylib bar.rs -C opt-level=3
2020-01-20T05:55:43.3976885Z Makefile:6: recipe for target 'all' failed
2020-01-20T05:55:43.3977944Z ------------------------------------------
2020-01-20T05:55:43.3978064Z stderr:
2020-01-20T05:55:43.3978618Z ------------------------------------------
2020-01-20T05:55:43.3978618Z ------------------------------------------
2020-01-20T05:55:43.3978940Z error: linking with `arm-none-eabi-gcc` failed: exit code: 1
2020-01-20T05:55:43.3979264Z   |
2020-01-20T05:55:43.3984847Z   = note: "arm-none-eabi-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/bar.bar.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/libbar.so" "-Wl,--version-script=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/bar.13x131manznmlae2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/bar.4hnz5vh12lvh2qha.rcgu.o" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libfoo.rlib" "-Wl,--no-whole-archive" "-Wl,--start-group" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libstd-3d945bdb2761558e.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libpanic_unwind-929c9774c073f5c2.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libhashbrown-e76531ce814b672f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/librustc_std_workspace_alloc-34b4f3bc58507c0c.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libbacktrace-dbcbee45c7f4565b.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libbacktrace_sys-4092353021877b9d.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/librustc_demangle-2d40e1362beeb7ba.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libunwind-23872e719a6b1f48.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libcfg_if-214c5efc98c112bb.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/liblibc-07d5863846890513.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/liballoc-1f6eb6f48fc19c37.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/librustc_std_workspace_core-7178d5693c033e3a.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libcore-785d943c13f61874.rlib" "-Wl,--no-whole-archive" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue64319/issue64319/rustcd7ik0j/libcompiler_builtins-daa64044d606da19.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-shared"
2020-01-20T05:55:43.3987108Z   = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'
2020-01-20T05:55:43.3987399Z 
2020-01-20T05:55:43.3987476Z error: aborting due to previous error
2020-01-20T05:55:43.3987520Z 
2020-01-20T05:55:43.3987520Z 
2020-01-20T05:55:43.3987639Z make: *** [all] Error 1
2020-01-20T05:55:43.3987985Z ------------------------------------------
2020-01-20T05:55:43.3988052Z 
2020-01-20T05:55:43.3988086Z 
2020-01-20T05:55:43.3988086Z 
2020-01-20T05:55:43.3988342Z ---- [run-make] run-make/share-generics-dylib stdout ----
2020-01-20T05:55:43.3988468Z error: make failed
2020-01-20T05:55:43.3988549Z status: exit code: 2
2020-01-20T05:55:43.3988612Z command: "make"
2020-01-20T05:55:43.3988691Z stdout:
2020-01-20T05:55:43.3988691Z stdout:
2020-01-20T05:55:43.3988932Z ------------------------------------------
2020-01-20T05:55:43.3990085Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib  -Clinker=arm-none-eabi-gcc instance_provider_a.rs -Cprefer-dynamic -Zshare-generics=yes -Ccodegen-units=1 -Zsymbol-mangling-version=v0 --crate-type=rlib
2020-01-20T05:55:43.3991462Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib  -Clinker=arm-none-eabi-gcc instance_provider_b.rs -Cprefer-dynamic -Zshare-generics=yes -Ccodegen-units=1 -Zsymbol-mangling-version=v0 --crate-type=rlib
2020-01-20T05:55:43.3992855Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib  -Clinker=arm-none-eabi-gcc instance_user_dylib.rs -Cprefer-dynamic -Zshare-generics=yes -Ccodegen-units=1 -Zsymbol-mangling-version=v0 --crate-type=dylib
2020-01-20T05:55:43.3993369Z Makefile:17: recipe for target 'all' failed
2020-01-20T05:55:43.3993685Z ------------------------------------------
2020-01-20T05:55:43.3993758Z stderr:
2020-01-20T05:55:43.3994013Z ------------------------------------------
2020-01-20T05:55:43.3994013Z ------------------------------------------
2020-01-20T05:55:43.3994286Z error: linking with `arm-none-eabi-gcc` failed: exit code: 1
2020-01-20T05:55:43.3994383Z   |
2020-01-20T05:55:43.3997098Z   = note: "arm-none-eabi-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/instance_user_dylib.instance_user_dylib.3a1fbbbh-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/libinstance_user_dylib.so" "-Wl,--version-script=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/rustc00nfuP/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/instance_user_dylib.quxmn0l10olw9ij.rcgu.o" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/rustc00nfuP/libinstance_provider_b.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/rustc00nfuP/libinstance_provider_a.rlib" "-Wl,--no-whole-archive" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-lstd-3d945bdb2761558e" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/share-generics-dylib/share-generics-dylib/rustc00nfuP/libcompiler_builtins-daa64044d606da19.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-shared"
2020-01-20T05:55:43.3998239Z   = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'
2020-01-20T05:55:43.3998383Z 
2020-01-20T05:55:43.3998442Z error: aborting due to previous error
2020-01-20T05:55:43.3998485Z 
2020-01-20T05:55:43.3998485Z 
2020-01-20T05:55:43.3998559Z make: *** [all] Error 1
2020-01-20T05:55:43.3998857Z ------------------------------------------
2020-01-20T05:55:43.3998905Z 
2020-01-20T05:55:43.3998938Z 
2020-01-20T05:55:43.4006070Z 
2020-01-20T05:55:43.4006070Z 
2020-01-20T05:55:43.4006995Z failures:
2020-01-20T05:55:43.4007668Z     [run-make] run-make/issue64319
2020-01-20T05:55:43.4008348Z     [run-make] run-make/share-generics-dylib
2020-01-20T05:55:43.4010100Z test result: FAILED. 3 passed; 2 failed; 10 ignored; 0 measured; 0 filtered out
2020-01-20T05:55:43.4010545Z 
2020-01-20T05:55:43.4011146Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-20T05:55:43.4011419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-20T05:55:43.4011419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-20T05:55:43.4012039Z 
2020-01-20T05:55:43.4012096Z 
2020-01-20T05:55:43.4014267Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-20T05:55:43.4015074Z 
2020-01-20T05:55:43.4015110Z 
2020-01-20T05:55:43.4070481Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
2020-01-20T05:55:43.4071266Z Build completed unsuccessfully in 0:59:17
2020-01-20T05:55:43.4071266Z Build completed unsuccessfully in 0:59:17
2020-01-20T05:55:43.4083992Z == clock drift check ==
2020-01-20T05:55:43.4100998Z   local time: Mon Jan 20 05:55:43 UTC 2020
2020-01-20T05:55:43.6761740Z   network time: Mon, 20 Jan 2020 05:55:43 GMT
2020-01-20T05:55:43.6767275Z == end clock drift check ==
2020-01-20T05:55:44.8724244Z 
2020-01-20T05:55:44.8820126Z ##[error]Bash exited with code '1'.
2020-01-20T05:55:44.8858168Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-20T05:55:44.8860479Z ==============================================================================
2020-01-20T05:55:44.8860703Z Task         : Get sources
2020-01-20T05:55:44.8860799Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
    Finished release [optimized] target(s) in 20.59s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12043 tests
......................................................FF.F.F......F...F..FFF...F........F........... 100/12043
.................................................................................................... 300/12043
.................................................................................................... 400/12043
.................................................................................................... 500/12043
.................................................................................................... 600/12043
---
---- [ui] ui/allocator/two-allocators.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/two-allocators.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/allocator/no_std-alloc-error-handler-custom.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/no_std-alloc-error-handler-custom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-custom/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-custom/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.
error: aborting due to previous error


------------------------------------------
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] ui/allocator/no_std-alloc-error-handler-default.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/no_std-alloc-error-handler-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/allocator/two-allocators2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/two-allocators2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/allocator/two-allocators3.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/two-allocators3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/two-allocators3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/allocator/custom.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/custom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.custom.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.2tm03jnxo4nqzf4f.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/auxiliary/libhelper.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.2tm03jnxo4nqzf4f.rcgu.o: In function `__rust_alloc':
           2tm03jnxo4nqzf4f:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.custom.7rcbfp3g-cgu.0.rcgu.o:custom.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.2tm03jnxo4nqzf4f.rcgu.o: In function `__rust_alloc_zeroed':
           2tm03jnxo4nqzf4f:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.custom.7rcbfp3g-cgu.0.rcgu.o:custom.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.2tm03jnxo4nqzf4f.rcgu.o: In function `__rust_dealloc':
           2tm03jnxo4nqzf4f:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.custom.7rcbfp3g-cgu.0.rcgu.o:custom.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.2tm03jnxo4nqzf4f.rcgu.o: In function `__rust_realloc':
           2tm03jnxo4nqzf4f:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom/a.custom.7rcbfp3g-cgu.0.rcgu.o:custom.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/allocator/custom-in-block.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/custom-in-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.custom_in_block.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.zycj8n7cjpgar2l.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.zycj8n7cjpgar2l.rcgu.o: In function `__rust_alloc':
           zycj8n7cjpgar2l:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.custom_in_block.7rcbfp3g-cgu.0.rcgu.o:custom_in_block.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.zycj8n7cjpgar2l.rcgu.o: In function `__rust_alloc_zeroed':
           zycj8n7cjpgar2l:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.custom_in_block.7rcbfp3g-cgu.0.rcgu.o:custom_in_block.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.zycj8n7cjpgar2l.rcgu.o: In function `__rust_dealloc':
           zycj8n7cjpgar2l:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.custom_in_block.7rcbfp3g-cgu.0.rcgu.o:custom_in_block.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.zycj8n7cjpgar2l.rcgu.o: In function `__rust_realloc':
           zycj8n7cjpgar2l:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-block/a.custom_in_block.7rcbfp3g-cgu.0.rcgu.o:custom_in_block.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/allocator/custom-in-submodule.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/custom-in-submodule.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.custom_in_submodule.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.n2739pvhhvatbes.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.n2739pvhhvatbes.rcgu.o: In function `__rust_alloc':
           n2739pvhhvatbes:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.custom_in_submodule.7rcbfp3g-cgu.0.rcgu.o:custom_in_submodule.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.n2739pvhhvatbes.rcgu.o: In function `__rust_alloc_zeroed':
           n2739pvhhvatbes:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.custom_in_submodule.7rcbfp3g-cgu.0.rcgu.o:custom_in_submodule.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.n2739pvhhvatbes.rcgu.o: In function `__rust_dealloc':
           n2739pvhhvatbes:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.custom_in_submodule.7rcbfp3g-cgu.0.rcgu.o:custom_in_submodule.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.n2739pvhhvatbes.rcgu.o: In function `__rust_realloc':
           n2739pvhhvatbes:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/custom-in-submodule/a.custom_in_submodule.7rcbfp3g-cgu.0.rcgu.o:custom_in_submodule.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/allocator/hygiene.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.hygiene.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.4t8qizs53erbbqlw.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.4t8qizs53erbbqlw.rcgu.o: In function `__rust_alloc':
           4t8qizs53erbbqlw:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.hygiene.7rcbfp3g-cgu.0.rcgu.o:hygiene.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.4t8qizs53erbbqlw.rcgu.o: In function `__rust_alloc_zeroed':
           4t8qizs53erbbqlw:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.hygiene.7rcbfp3g-cgu.0.rcgu.o:hygiene.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.4t8qizs53erbbqlw.rcgu.o: In function `__rust_dealloc':
           4t8qizs53erbbqlw:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.hygiene.7rcbfp3g-cgu.0.rcgu.o:hygiene.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.4t8qizs53erbbqlw.rcgu.o: In function `__rust_realloc':
           4t8qizs53erbbqlw:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/hygiene/a.hygiene.7rcbfp3g-cgu.0.rcgu.o:hygiene.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/allocator/xcrate-use.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/xcrate-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.xcrate_use.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.1u988njpsw334mme.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.1u988njpsw334mme.rcgu.o: In function `__rust_alloc':
           1u988njpsw334mme:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.xcrate_use.7rcbfp3g-cgu.0.rcgu.o:xcrate_use.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.1u988njpsw334mme.rcgu.o: In function `__rust_alloc_zeroed':
           1u988njpsw334mme:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.xcrate_use.7rcbfp3g-cgu.0.rcgu.o:xcrate_use.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.1u988njpsw334mme.rcgu.o: In function `__rust_dealloc':
           1u988njpsw334mme:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.xcrate_use.7rcbfp3g-cgu.0.rcgu.o:xcrate_use.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.1u988njpsw334mme.rcgu.o: In function `__rust_realloc':
           1u988njpsw334mme:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use/a.xcrate_use.7rcbfp3g-cgu.0.rcgu.o:xcrate_use.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/allocator/xcrate-use2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/xcrate-use2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.xcrate_use2.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.1t1wdgbp6x2lmpqs.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom_as_global.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom_as_global.rlib(custom-as-global.custom_as_global.3a1fbbbh-cgu.0.rcgu.o): In function `__rust_alloc':
           custom_as_global.3a1fbbbh-cgu.0:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.1t1wdgbp6x2lmpqs.rcgu.o:1t1wdgbp6x2lmpqs:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom_as_global.rlib(custom-as-global.custom_as_global.3a1fbbbh-cgu.0.rcgu.o): In function `__rust_alloc_zeroed':
           custom_as_global.3a1fbbbh-cgu.0:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.1t1wdgbp6x2lmpqs.rcgu.o:1t1wdgbp6x2lmpqs:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom_as_global.rlib(custom-as-global.custom_as_global.3a1fbbbh-cgu.0.rcgu.o): In function `__rust_dealloc':
           custom_as_global.3a1fbbbh-cgu.0:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.1t1wdgbp6x2lmpqs.rcgu.o:1t1wdgbp6x2lmpqs:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/auxiliary/libcustom_as_global.rlib(custom-as-global.custom_as_global.3a1fbbbh-cgu.0.rcgu.o): In function `__rust_realloc':
           custom_as_global.3a1fbbbh-cgu.0:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/xcrate-use2/a.1t1wdgbp6x2lmpqs.rcgu.o:1t1wdgbp6x2lmpqs:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error



------------------------------------------


---- [ui] ui/missing/missing-alloc_error_handler.rs stdout ----
diff of stderr:

+ error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.
+ 
1 error: `#[alloc_error_handler]` function required, but not found.
2 
3 note: Use `#![feature(default_alloc_error_handler)]` for a default error handler.
4 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
6 
---
To only update this specific test, also pass `--test-args missing/missing-alloc_error_handler.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-alloc_error_handler.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-alloc_error_handler" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-alloc_error_handler/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.

error: `#[alloc_error_handler]` function required, but not found.

note: Use `#![feature(default_alloc_error_handler)]` for a default error handler.
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/process/process-panic-after-fork.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/process/process-panic-after-fork.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.process_panic_after_fork.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.18qfbd9dxnqptsc3.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-99269b396c53f0b1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-c88be9d206affbae.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-0475f8794bf8373d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-23488ec57e1c1d8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-559d2fc2ef9d7ff0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-e2ee420e8f991ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-aec1c37ea56b90ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-edc5edc5d3777759.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-4cbfd3b0649b13b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-566b48fdfa7d924d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-53e40ea7a846a211.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d73294cb16ccc1b6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-2f6380202d4f0fe8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e7ab0ca2306407cb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-b66c15bf9fb3a547.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-9d605538e141f7f1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-ec9eb50245b31834.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c71d5dd4d244d533.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.18qfbd9dxnqptsc3.rcgu.o: In function `__rust_alloc':
           18qfbd9dxnqptsc3:(.text.__rust_alloc+0x0): multiple definition of `__rust_alloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.process_panic_after_fork.7rcbfp3g-cgu.0.rcgu.o:process_panic_after_fork.7rcbfp3g-cgu.0:(.text.__rust_alloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.18qfbd9dxnqptsc3.rcgu.o: In function `__rust_alloc_zeroed':
           18qfbd9dxnqptsc3:(.text.__rust_alloc_zeroed+0x0): multiple definition of `__rust_alloc_zeroed'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.process_panic_after_fork.7rcbfp3g-cgu.0.rcgu.o:process_panic_after_fork.7rcbfp3g-cgu.0:(.text.__rust_alloc_zeroed+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.18qfbd9dxnqptsc3.rcgu.o: In function `__rust_dealloc':
           18qfbd9dxnqptsc3:(.text.__rust_dealloc+0x0): multiple definition of `__rust_dealloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.process_panic_after_fork.7rcbfp3g-cgu.0.rcgu.o:process_panic_after_fork.7rcbfp3g-cgu.0:(.text.__rust_dealloc+0x0): first defined here
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.18qfbd9dxnqptsc3.rcgu.o: In function `__rust_realloc':
           18qfbd9dxnqptsc3:(.text.__rust_realloc+0x0): multiple definition of `__rust_realloc'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a.process_panic_after_fork.7rcbfp3g-cgu.0.rcgu.o:process_panic_after_fork.7rcbfp3g-cgu.0:(.text.__rust_realloc+0x0): first defined here
           collect2: error: ld returned 1 exit status

error: aborting due to previous error


---
test result: FAILED. 11933 passed; 13 failed; 97 ignored; 0 measured; 0 filtered out; finished in 102.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:21

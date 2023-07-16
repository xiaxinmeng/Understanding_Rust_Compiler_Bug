plain
---- [ui] src/test/ui/attributes/attr-before-view-item.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/attr-before-view-item.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item/attr-before-view-item.attr_before_view_item.b821e2b7-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item/attr-before-view-item" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/attributes/attr-before-view-item2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/attr-before-view-item2.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item2/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item2/attr-before-view-item2.attr_before_view_item2.b4404285-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item2/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/attr-before-view-item2/attr-before-view-item2" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/consts/const-endianess.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-endianess.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-endianess/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-endianess/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-endianess/a.const_endianess.0c5c6c4a-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-endianess/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-endianess/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/consts/const-ptr-nonnull-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-nonnull-rpass.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull-rpass/a.const_ptr_nonnull_rpass.03884877-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull-rpass/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull-rpass/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/consts/const-ptr-unique-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-unique-rpass.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique-rpass/a.const_ptr_unique_rpass.f4a2cc6c-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique-rpass/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique-rpass/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/enum-discriminant/arbitrary_enum_discriminant.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum-discriminant/arbitrary_enum_discriminant.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant/a.arbitrary_enum_discriminant.b362dec3-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/arbitrary_enum_discriminant/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/drop/dropck_legal_cycles.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dropck_legal_cycles.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a.dropck_legal_cycles.1f635f42-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a.dropck_legal_cycles.1f635f42-cgu.0.rcgu.o:dropck_legal_cycles.1f635f42-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::hb18b4e29c6a01942: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a.dropck_legal_cycles.1f635f42-cgu.0.rcgu.o:dropck_legal_cycles.1f635f42-cgu.0:function dropck_legal_cycles::main::h9e150dd02b6e0516: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a.dropck_legal_cycles.1f635f42-cgu.0.rcgu.o:dropck_legal_cycles.1f635f42-cgu.0:function dropck_legal_cycles::main::h9e150dd02b6e0516: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dropck_legal_cycles/a.dropck_legal_cycles.1f635f42-cgu.0.rcgu.o:dropck_legal_cycles.1f635f42-cgu.0:function dropck_legal_cycles::main::h9e150dd02b6e0516: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/a.foreach_external_iterators_hashmap_break_restart.d33fc359-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/a.foreach_external_iterators_hashmap_break_restart.d33fc359-cgu.0.rcgu.o:foreach_external_iterators_hashmap_break_restart.d33fc359-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h37c744fedddef035: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap-break-restart/a.foreach_external_iterators_hashmap_break_restart.d33fc359-cgu.0.rcgu.o:foreach_external_iterators_hashmap_break_restart.d33fc359-cgu.0:function foreach_external_iterators_hashmap_break_restart::main::h8350b5b26235219f: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/for-loop-while/foreach-external-iterators-hashmap.rs stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-linux-androideabi
error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for-loop-while/foreach-external-iterators-hashmap.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/a.foreach_external_iterators_hashmap.f7091c1d-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/a.foreach_external_iterators_hashmap.f7091c1d-cgu.0.rcgu.o:foreach_external_iterators_hashmap.f7091c1d-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h81d06a0d500e3bc4: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for-loop-while/foreach-external-iterators-hashmap/a.foreach_external_iterators_hashmap.f7091c1d-cgu.0.rcgu.o:foreach_external_iterators_hashmap.f7091c1d-cgu.0:function foreach_external_iterators_hashmap::main::ha79ab08d4cf897af: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/smoke.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/smoke.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-Z" "mir-opt-level=0" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/a.smoke.40e0f984-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/smoke.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/smoke.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--cfg" "default" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/a.smoke.40e0f984-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/hashmap/hashmap-capacity-overflow.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hashmap/hashmap-capacity-overflow.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a.hashmap_capacity_overflow.325dabae-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a.hashmap_capacity_overflow.325dabae-cgu.0.rcgu.o:hashmap_capacity_overflow.325dabae-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::hf324ce14390851ef: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a.hashmap_capacity_overflow.325dabae-cgu.0.rcgu.o:hashmap_capacity_overflow.325dabae-cgu.0:function hashmap_capacity_overflow::main::hd85337b317bbaf1e: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/hashmap/hashmap-memory.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hashmap/hashmap-memory.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a.hashmap_memory.e537fa7d-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a.hashmap_memory.e537fa7d-cgu.0.rcgu.o:hashmap_memory.e537fa7d-cgu.0:function std::sys_common::backtrace::__rust_begin_short_backtrace::h9a3739afe711af4f: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a.hashmap_memory.e537fa7d-cgu.0.rcgu.o:hashmap_memory.e537fa7d-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::hcc8a1dd557deacd3: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-memory/a.hashmap_memory.e537fa7d-cgu.0.rcgu.o:hashmap_memory.e537fa7d-cgu.0:function hashmap_memory::main::h06199c5fd5c22f95: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-12860.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12860.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a.issue_12860.784e83c5-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a.issue_12860.784e83c5-cgu.0.rcgu.o:issue_12860.784e83c5-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::ha32f8a3ba2641c39: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a.issue_12860.784e83c5-cgu.0.rcgu.o:issue_12860.784e83c5-cgu.0:function issue_12860::main::h5085ff229a597c4a: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12860/a.issue_12860.784e83c5-cgu.0.rcgu.o:issue_12860.784e83c5-cgu.0:function issue_12860::main::h5085ff229a597c4a: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-12909.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12909.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/a.issue_12909.d2c40afb-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/a.issue_12909.d2c40afb-cgu.0.rcgu.o:issue_12909.d2c40afb-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::hfc570def672766a7: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12909/a.issue_12909.d2c40afb-cgu.0.rcgu.o:issue_12909.d2c40afb-cgu.0:function core::iter::traits::iterator::Iterator::collect::h90367888a8ee933b: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-16597-empty.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16597-empty.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/a.issue_16597_empty.2cc941fd-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-16597.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16597.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/a.issue_16597.ab37a7b7-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-1696.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-1696.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/a.issue_1696.efb94363-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/a.issue_1696.efb94363-cgu.0.rcgu.o:issue_1696.efb94363-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h6d29d9ef2b207bf2: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-1696/a.issue_1696.efb94363-cgu.0.rcgu.o:issue_1696.efb94363-cgu.0:function issue_1696::main::he67a8cd442125e7c: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-23036.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23036.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a.issue_23036.167a9549-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a.issue_23036.167a9549-cgu.0.rcgu.o:issue_23036.167a9549-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h611d4957d13d21b5: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a.issue_23036.167a9549-cgu.0.rcgu.o:issue_23036.167a9549-cgu.0:function issue_23036::main::hbcfdd030730dd7c2: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-23649-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23649-2.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/a.issue_23649_2.abd284b0-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/a.issue_23649_2.abd284b0-cgu.0.rcgu.o:issue_23649_2.abd284b0-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h69a9b5a06f355786: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23649-2/a.issue_23649_2.abd284b0-cgu.0.rcgu.o:issue_23649_2.abd284b0-cgu.0:function issue_23649_2::main::hc4013d0ded8d04cc: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-2631-b.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-2631-b.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/a.issue_2631_b.db52afa4-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/auxiliary" "-lreq" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/a.issue_2631_b.db52afa4-cgu.0.rcgu.o:issue_2631_b.db52afa4-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h74e324532aa67bd9: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2631-b/a.issue_2631_b.db52afa4-cgu.0.rcgu.o:issue_2631_b.db52afa4-cgu.0:function issue_2631_b::main::h5134d3a3e0e86514: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-3026.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3026.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/a.issue_3026.559ca74d-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/a.issue_3026.559ca74d-cgu.0.rcgu.o:issue_3026.559ca74d-cgu.0:function std::thread::local::fast::Key$LT$T$GT$::try_initialize::h49cf5f5fdfb6b851: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3026/a.issue_3026.559ca74d-cgu.0.rcgu.o:issue_3026.559ca74d-cgu.0:function issue_3026::main::h4acd324445efaf76: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-34932.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34932.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `/android/ndk/arm-14/bin/arm-linux-androideabi-clang` failed: exit status: 1
   |
   = note: "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/a.issue_34932.41f55ccb-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-ltest-47f4081efa363316" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-lstd-f1bb957259d6702d" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libcompiler_builtins-25d675b04b05292e.rlib" "-Wl,-Bdynamic" "-ldl" "-llog" "-lgcc" "-ldl" "-lc" "-lm" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-linux-androideabi/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib/libtest-47f4081efa363316.so: error: undefined reference to '__emutls_v._ZN3std11collections4hash3map11RandomState3new4KEYS7__getit5__KEY17hf565897338a8c3c9E'
           clang50: error: linker command failed with exit code 1 (use -v to see invocation)
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------




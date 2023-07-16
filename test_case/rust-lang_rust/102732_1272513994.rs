plain
failures:

---- [ui] src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----

error in revision `fat0`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat0" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/tmp/rustcCdjFTJ/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a.issue_64655_extern_rust_must_allow_unwind.2a84b153-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c9b03ebf09765622.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,--strip-debug" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c9b03ebf09765622.rlib(compiler_builtins-c9b03ebf09765622.compiler_builtins.0478efcd-cgu.43.rcgu.o): in function `core::hint::unreachable_unchecked':
           compiler_builtins.0478efcd-cgu.43:(.text._ZN4core4hint21unreachable_unchecked17h7273d49baf2b983fE+0xf): undefined reference to `core::panicking::panic_str_nounwind'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-44056.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44056.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ctarget-feature=+avx" "-Clto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/tmp/rustc6fdQlJ/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/issue-44056.issue_44056.9f62d9c4-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c9b03ebf09765622.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/issue-44056" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-Wl,--strip-debug" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
   = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-c9b03ebf09765622.rlib(compiler_builtins-c9b03ebf09765622.compiler_builtins.0478efcd-cgu.43.rcgu.o): in function `core::hint::unreachable_unchecked':
           compiler_builtins.0478efcd-cgu.43:(.text._ZN4core4hint21unreachable_unchecked17h7273d49baf2b983fE+0xf): undefined reference to `core::panicking::panic_str_nounwind'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error
------------------------------------------



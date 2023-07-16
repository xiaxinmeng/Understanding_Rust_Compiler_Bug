plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 244 tests
i............................................................................i..........  88/244
...............................................i.....i...F.......ii.i................... 176/244
.......................ii........i....F........................F..F.
failures:

---- [mir-opt] tests/mir-opt/generator_tiny.rs stdout ----


error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/generator_tiny.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
error: linking with `x86_64-linux-gnu-gcc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" VSLANG="1033" "x86_64-linux-gnu-gcc" "-m32" "/tmp/rustcJhCXGp/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny.generator_tiny.62c67f7d-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny.generator_tiny.62c67f7d-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny.generator_tiny.62c67f7d-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny.52z2jrp92sybm65e.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_abort-e00abe61f32ec2a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libobject-3a67fe4386e1cdbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libmemchr-78c5b23cc4240873.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libaddr2line-e82dcf80b2e55bba.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libgimli-c36c94ebfee662db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_demangle-e6c65809a7f494cc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd_detect-b7dd891eb7b8d427.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libhashbrown-54e50a613da975b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libminiz_oxide-4c831172d9f8c4de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libadler-34904bed090b353e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_alloc-033e6b6500b58ed8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-2dffc119da31f255.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcfg_if-96018f8f1394f424.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-09c2b214222256f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-7a3625c53239b76b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_core-151273f1331a1bf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-c922854fce2b6ba9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-f927f870fce2dee1.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib(std-558ef0c8bb7682dc.std.de5f1121-cgu.0.rcgu.o): TLS transition from R_386_TLS_LDM to R_386_TLS_LE_32 against `_ZN3std2io5stdio14OUTPUT_CAPTURE7__getit5__KEY17h12086c72ecd90e72E' at 0x17 in section `.text._ZN3std3sys6common12thread_local10fast_local4fast12Key$LT$T$GT$14try_initialize17h74384c4744ad601aE' failed
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [mir-opt] tests/mir-opt/simplify_cfg.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/simplify_cfg.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/auxiliary" "-Cpanic=abort"
stdout: none
--- stderr -------------------------------
error: linking with `x86_64-linux-gnu-gcc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" VSLANG="1033" "x86_64-linux-gnu-gcc" "-m32" "/tmp/rustcpTAqjm/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/simplify_cfg.simplify_cfg.b31a50d8-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/simplify_cfg.simplify_cfg.b31a50d8-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/simplify_cfg.simplify_cfg.b31a50d8-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/simplify_cfg.59lyn3kkpiywrkc2.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_abort-e00abe61f32ec2a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libobject-3a67fe4386e1cdbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libmemchr-78c5b23cc4240873.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libaddr2line-e82dcf80b2e55bba.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libgimli-c36c94ebfee662db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_demangle-e6c65809a7f494cc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd_detect-b7dd891eb7b8d427.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libhashbrown-54e50a613da975b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libminiz_oxide-4c831172d9f8c4de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libadler-34904bed090b353e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_alloc-033e6b6500b58ed8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-2dffc119da31f255.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcfg_if-96018f8f1394f424.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-09c2b214222256f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-7a3625c53239b76b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_core-151273f1331a1bf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-c922854fce2b6ba9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-f927f870fce2dee1.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/simplify_cfg" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib(std-558ef0c8bb7682dc.std.de5f1121-cgu.0.rcgu.o): TLS transition from R_386_TLS_LDM to R_386_TLS_LE_32 against `_ZN3std2io5stdio14OUTPUT_CAPTURE7__getit5__KEY17h12086c72ecd90e72E' at 0x17 in section `.text._ZN3std3sys6common12thread_local10fast_local4fast12Key$LT$T$GT$14try_initialize17h74384c4744ad601aE' failed
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [mir-opt] tests/mir-opt/sroa/lifetimes.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/sroa/lifetimes.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=0" "-Zmir-enable-passes=+ScalarReplacementOfAggregates" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/auxiliary" "-Cpanic=abort"
stdout: none
--- stderr -------------------------------
error: linking with `x86_64-linux-gnu-gcc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" VSLANG="1033" "x86_64-linux-gnu-gcc" "-m32" "/tmp/rustc8FYyTn/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes.lifetimes.9c2b6592-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes.lifetimes.9c2b6592-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes.lifetimes.9c2b6592-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes.lifetimes.9c2b6592-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes.3gkztul7kj8bw96c.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_abort-e00abe61f32ec2a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libobject-3a67fe4386e1cdbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libmemchr-78c5b23cc4240873.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libaddr2line-e82dcf80b2e55bba.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libgimli-c36c94ebfee662db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_demangle-e6c65809a7f494cc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd_detect-b7dd891eb7b8d427.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libhashbrown-54e50a613da975b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libminiz_oxide-4c831172d9f8c4de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libadler-34904bed090b353e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_alloc-033e6b6500b58ed8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-2dffc119da31f255.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcfg_if-96018f8f1394f424.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-09c2b214222256f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-7a3625c53239b76b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_core-151273f1331a1bf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-c922854fce2b6ba9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-f927f870fce2dee1.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/lifetimes/lifetimes" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib(std-558ef0c8bb7682dc.std.de5f1121-cgu.0.rcgu.o): TLS transition from R_386_TLS_LDM to R_386_TLS_LE_32 against `_ZN3std4sync7remutex25current_thread_unique_ptr1X7__getit3VAL17he06daeea2db790c2E' at 0x20 in section `.text._ZN61_$LT$$RF$std..io..stdio..Stderr$u20$as$u20$std..io..Write$GT$9write_fmt17hab2b45204631a906E' failed
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status

error: aborting due to previous error
------------------------------------------



---- [mir-opt] tests/mir-opt/sroa/structs.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/sroa/structs.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=0" "-Zmir-enable-passes=+ScalarReplacementOfAggregates" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/auxiliary" "-Cpanic=abort"
stdout: none
--- stderr -------------------------------
warning: unused variable: `t`
   |
   |
83 |     let t = y.a;
   |         ^ help: if this is intentional, prefix it with an underscore: `_t`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `u`
  --> /checkout/tests/mir-opt/sroa/structs.rs:84:9
  --> /checkout/tests/mir-opt/sroa/structs.rs:84:9
   |
84 |     let u = y.c;
   |         ^ help: if this is intentional, prefix it with an underscore: `_u`
warning: unused variable: `a`
  --> /checkout/tests/mir-opt/sroa/structs.rs:86:9
   |
86 |     let a = z.b;
86 |     let a = z.b;
   |         ^ help: if this is intentional, prefix it with an underscore: `_a`

warning: unused variable: `t`
  --> /checkout/tests/mir-opt/sroa/structs.rs:91:9
   |
91 |     let t = y.a;
   |         ^ help: if this is intentional, prefix it with an underscore: `_t`
warning: unused variable: `u`
  --> /checkout/tests/mir-opt/sroa/structs.rs:92:9
   |
   |
92 |     let u = y.c;
   |         ^ help: if this is intentional, prefix it with an underscore: `_u`
warning: unused variable: `t`
  --> /checkout/tests/mir-opt/sroa/structs.rs:98:9
   |
   |
98 |     let t = y.0;
   |         ^ help: if this is intentional, prefix it with an underscore: `_t`
warning: unused variable: `u`
  --> /checkout/tests/mir-opt/sroa/structs.rs:99:9
   |
   |
99 |     let u = y.1;
   |         ^ help: if this is intentional, prefix it with an underscore: `_u`

error: linking with `x86_64-linux-gnu-gcc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" VSLANG="1033" "x86_64-linux-gnu-gcc" "-m32" "/tmp/rustcxhzzbf/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/structs.structs.d3786250-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/structs.structs.d3786250-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/structs.structs.d3786250-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/structs.4a3h3plpl4h9caav.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libpanic_abort-e00abe61f32ec2a5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libobject-3a67fe4386e1cdbd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libmemchr-78c5b23cc4240873.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libaddr2line-e82dcf80b2e55bba.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libgimli-c36c94ebfee662db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_demangle-e6c65809a7f494cc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd_detect-b7dd891eb7b8d427.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libhashbrown-54e50a613da975b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libminiz_oxide-4c831172d9f8c4de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libadler-34904bed090b353e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_alloc-033e6b6500b58ed8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libunwind-2dffc119da31f255.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcfg_if-96018f8f1394f424.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liblibc-09c2b214222256f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/liballoc-7a3625c53239b76b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/librustc_std_workspace_core-151273f1331a1bf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcore-c922854fce2b6ba9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libcompiler_builtins-f927f870fce2dee1.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/sroa/structs/structs" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags" "-Wl,-z,origin"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib/libstd-558ef0c8bb7682dc.rlib(std-558ef0c8bb7682dc.std.de5f1121-cgu.0.rcgu.o): TLS transition from R_386_TLS_LDM to R_386_TLS_LE_32 against `_ZN3std4sync7remutex25current_thread_unique_ptr1X7__getit3VAL17he06daeea2db790c2E' at 0x20 in section `.text._ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17he7d45670aff2e707E' failed
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status

error: aborting due to previous error; 7 warnings emitted
------------------------------------------


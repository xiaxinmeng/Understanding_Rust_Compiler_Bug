
> cargo +stage2 run --target=x86_64-unknown-linux-musl -v
   Compiling hello v0.1.0 (/tmp/hello)
...
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2h6xqd3sn2km8459.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2lghhrfuwtr7nzjh.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2u47if6aty8w50qw.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.3xxa7ojbyma30x26.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.4pslqofh5v0drlel.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.4yql2ov1nj568sqm.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.52kbsv0lrgok17ka.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.a0667wcj9jbydoq.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.eupsp68inw7mmh6.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.sq5vqa0suj01eo6.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.29ixghcst8l42l42.rcgu.o" "-Wl,--as-needed" "-L" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps" "-L" "/usr/obj/rust/debug/deps" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d33aa8849ca77db7.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-c61ce5214b6f566e.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-94c2c41de143830a.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-523c6f72fcce4fbe.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-369cfdc3f09ca1d8.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-a01a54ac0945ddff.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-98c962b295e9efef.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-1c7a419a49ad1630.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-5a370cc380b8855c.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-57aba56194934692.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-33d5545291287325.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-7b4587c633c797db.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-0d78a2579e15eaf3.rlib" "/tmp/rustc8kUCIu/liblibc-078b8b0ceafefac4.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-6acd1212f64ca562.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-8db4f733d31c499d.rlib" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-3653a7c23bc163c4.rlib" "-Wl,--end-group" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-62cc6e3c7da28150.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
INFO rustc_codegen_ssa::back::link linker stderr:
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../Scrt1.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crti.o
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtbegin-x86_64.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2h6xqd3sn2km8459.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2lghhrfuwtr7nzjh.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.2u47if6aty8w50qw.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.3xxa7ojbyma30x26.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.4pslqofh5v0drlel.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.4yql2ov1nj568sqm.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.52kbsv0lrgok17ka.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.a0667wcj9jbydoq.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.eupsp68inw7mmh6.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.sq5vqa0suj01eo6.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-1750057c5a8b7204.29ixghcst8l42l42.rcgu.o
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-d33aa8849ca77db7.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-c61ce5214b6f566e.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-94c2c41de143830a.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-523c6f72fcce4fbe.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-369cfdc3f09ca1d8.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-a01a54ac0945ddff.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-98c962b295e9efef.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-1c7a419a49ad1630.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-5a370cc380b8855c.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-57aba56194934692.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-33d5545291287325.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-7b4587c633c797db.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-0d78a2579e15eaf3.rlib
ld.lld: /tmp/rustc8kUCIu/liblibc-078b8b0ceafefac4.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-6acd1212f64ca562.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-8db4f733d31c499d.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-3653a7c23bc163c4.rlib
ld.lld: /home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-62cc6e3c7da28150.rlib
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc_s.so
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc_s.so.1
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libc.so
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtend-x86_64.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crtn.o

INFO rustc_codegen_ssa::back::link linker stdout:

    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `/usr/obj/rust/x86_64-unknown-linux-musl/debug/hello`
Hello, world!
> cargo +stage2 run --target=x86_64-unknown-linux-musl -Zbuild-std -v
...
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.1c9egvhx9rjepc9m.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.1fp4y02kz61hf6ch.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.26h3iko21zs63scu.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.4i2dc40pknq03be6.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.4znfkztdm1xmle87.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.5e15ka3cxqaprjdg.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.5fo0655swo10p3a.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.e5669naovnxfxj0.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.uccsh72nhfkhkfc.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.d47jlbkoktvnq2d.rcgu.o" "-Wl,--as-needed" "-L" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps" "-L" "/usr/obj/rust/debug/deps" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-096b37bce81c005d.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-313a4ff768c43196.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libminiz_oxide-f4f2d0d5fc492fe5.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libadler-46c4c11c2514d168.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libobject-6b703050c80b4bc5.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libaddr2line-f0051036f479ef52.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libgimli-c0329afb9952efba.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e0e489c3f8b370dc.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-7ed447842c1ec9a8.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib" "-Wl,--end-group" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
INFO rustc_codegen_ssa::back::link linker stderr:
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../Scrt1.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crti.o
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtbegin-x86_64.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.1c9egvhx9rjepc9m.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.1fp4y02kz61hf6ch.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.26h3iko21zs63scu.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.4i2dc40pknq03be6.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.4znfkztdm1xmle87.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.5e15ka3cxqaprjdg.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.5fo0655swo10p3a.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.e5669naovnxfxj0.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.uccsh72nhfkhkfc.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-dc75968db1cc77ff.d47jlbkoktvnq2d.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-096b37bce81c005d.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-313a4ff768c43196.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libminiz_oxide-f4f2d0d5fc492fe5.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libadler-46c4c11c2514d168.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libobject-6b703050c80b4bc5.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libaddr2line-f0051036f479ef52.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libgimli-c0329afb9952efba.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e0e489c3f8b370dc.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-7ed447842c1ec9a8.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc_s.so
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc_s.so.1
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/libgcc.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libc.so
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtend-x86_64.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crtn.o

INFO rustc_codegen_ssa::back::link linker stdout:

    Finished dev [unoptimized + debuginfo] target(s) in 26.17s
     Running `/usr/obj/rust/x86_64-unknown-linux-musl/debug/hello`
Hello, world!
> cargo +stage2 run --target=x86_64-unknown-linux-musl -Zbuild-std -Zbuild-std-features=llvm-libunwind  -v
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.17yjesypybb3mepe.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.29qo6epwb5ow5fto.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.2t0t40bclu93ba4y.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.37zmbt4ybutxa15z.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.3emy1wwy08657fxw.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.3f4pv93d6osg02dt.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4s82wvpgf6u9uo8x.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4uhw5e820goczdwz.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4uo1v6uy9033erdu.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.vvbtlj0jg3rle60.rcgu.o" "-Wl,--as-needed" "-L" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps" "-L" "/usr/obj/rust/debug/deps" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-dc8166c28007c38e.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-800a124145d61ed4.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e711e35d975432e1.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-b860cdaceb763b60.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib" "-Wl,--end-group" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib" "-lunwind" "-Wl,-Bdynamic" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
INFO rustc_codegen_ssa::back::link linker stderr:
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../Scrt1.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crti.o
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtbegin-x86_64.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.17yjesypybb3mepe.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.29qo6epwb5ow5fto.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.2t0t40bclu93ba4y.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.37zmbt4ybutxa15z.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.3emy1wwy08657fxw.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.3f4pv93d6osg02dt.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4s82wvpgf6u9uo8x.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4uhw5e820goczdwz.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.4uo1v6uy9033erdu.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-b7bd35d6d10d88b3.vvbtlj0jg3rle60.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-dc8166c28007c38e.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-800a124145d61ed4.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e711e35d975432e1.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-b860cdaceb763b60.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libunwind.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libc.so
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtend-x86_64.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crtn.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libdl.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libpthread.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libdl.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libpthread.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libdl.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libpthread.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libdl.a
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libpthread.a

INFO rustc_codegen_ssa::back::link linker stdout:

    Finished dev [unoptimized + debuginfo] target(s) in 7.00s
     Running `/usr/obj/rust/x86_64-unknown-linux-musl/debug/hello`
Hello, world!
> cargo +stage2 run --target=x86_64-unknown-linux-musl -Zbuild-std -Zbuild-std-features=system-llvm-libunwind  -v
...
INFO rustc_codegen_ssa::back::link "cc" "-m64" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.10jd5rwxei9esjnh.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.19rw70kr9nqv8b1k.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.1ojhp3h9f8t2gc8m.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.1p1q7lnzkcpk7rme.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.24zuzayewu3232ht.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.2k7k87ibtmqpsfx8.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.3u5nlwzan9bmytkx.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.41tebk4o29itd6ge.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.5eqqlu7h0g4640v1.rcgu.o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.36qszmoj57owiiiq.rcgu.o" "-Wl,--as-needed" "-L" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps" "-L" "/usr/obj/rust/debug/deps" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-40e155d1da97b2fc.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-daea58e01250ec2f.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e711e35d975432e1.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-3a41767c714a22b3.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib" "-Wl,--end-group" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib" "-Wl,-Bdynamic" "-lunwind" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/home/han/rust/obj/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-Wl,--verbose"
INFO rustc_codegen_ssa::back::link linker stderr:
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../Scrt1.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crti.o
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtbegin-x86_64.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.10jd5rwxei9esjnh.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.19rw70kr9nqv8b1k.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.1ojhp3h9f8t2gc8m.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.1p1q7lnzkcpk7rme.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.24zuzayewu3232ht.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.2k7k87ibtmqpsfx8.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.3u5nlwzan9bmytkx.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.41tebk4o29itd6ge.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.5eqqlu7h0g4640v1.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/hello-124ae595ed0f5bdb.36qszmoj57owiiiq.rcgu.o
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd-40e155d1da97b2fc.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libpanic_unwind-daea58e01250ec2f.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libstd_detect-e711e35d975432e1.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_demangle-147ff7388a41c098.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libhashbrown-5e8b0d91869a4fe2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_alloc-2279f88b332e0163.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libunwind-3a41767c714a22b3.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcfg_if-f6c66040ba908d02.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liblibc-ec82c3c7c8a9bca2.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/liballoc-e3a55ee441b17939.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/librustc_std_workspace_core-95957d3f24f15bbd.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcore-70c4f994ce8a28f0.rlib
ld.lld: /usr/obj/rust/x86_64-unknown-linux-musl/debug/deps/libcompiler_builtins-9c56f7979e8c521d.rlib
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libunwind.so
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../libc.so
ld.lld: /usr/lib/llvm/12/bin/../../../../lib/clang/12.0.0/lib/linux/clang_rt.crtend-x86_64.o
ld.lld: /usr/lib/gcc/x86_64-gentoo-linux-musl/11.1.0/../../../crtn.o

INFO rustc_codegen_ssa::back::link linker stdout:

    Finished dev [unoptimized + debuginfo] target(s) in 6.87s
     Running `/usr/obj/rust/x86_64-unknown-linux-musl/debug/hello`
Hello, world!

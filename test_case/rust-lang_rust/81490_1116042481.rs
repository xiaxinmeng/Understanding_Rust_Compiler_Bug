
+ docker run -it rustc-linkargs-example:latest ./run.sh
info: using existing install for 'stable-x86_64-unknown-linux-gnu'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.60.0 (7737e0b5c 2022-04-04)

gcc (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0
Copyright (C) 2019 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

rustc 1.60.0 (7737e0b5c 2022-04-04)
LLD 10.0.0 (compatible with GNU linkers)
Creating native library foo
Building rust rlib bar
Building rust bin main
Cleaning up
+ docker run -it rustc-linkargs-example:latest ./run.sh nightly
info: using existing install for 'nightly-2022-05-02-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2022-05-02-x86_64-unknown-linux-gnu'

  nightly-2022-05-02-x86_64-unknown-linux-gnu unchanged - rustc 1.62.0-nightly (4dd8b420c 2022-05-01)

gcc (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0
Copyright (C) 2019 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

rustc 1.62.0-nightly (4dd8b420c 2022-05-01)
LLD 10.0.0 (compatible with GNU linkers)
Creating native library foo
Building rust rlib bar
Building rust bin main
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/tmp/rustcSwvF2s/symbols.o" "main.main.729108ea-cgu.0.rcgu.o" "main.1cg9fo4wc2frqp4k.rcgu.o" "-Wl,--as-needed" "-L" "/example" "-L" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-lfoo" "/example/libbar.rlib" "-Wl,--start-group" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-202fc93d8ccaebf2.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-1333333cbe389678.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-93491bde8b3642ba.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-d338f5690e3fda2f.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-2cd7f06709609788.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-05bd833c6cc845b5.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-5543e955d2b2e107.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-a963b8f78c0365f5.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-626bd4749ba5679b.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1c5c08d77aa4ee1f.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-43c97e136c6f66b3.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-3ad551729ddf5bdf.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-46de9b9399df1cae.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-aa03de290f9594ce.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-020914c5936c5f85.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-89782a6344bc3ddf.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2a6a2797f7a73818.rlib" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0e3656b1fda5fd7b.rlib" "-Wl,--end-group" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-16d69221f10b0282.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/root/.rustup/toolchains/nightly-2022-05-02-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "main" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs" "-fuse-ld=lld" "-Wl,--warn-backrefs" "-Wl,--fatal-warnings"
  = note: ld.lld: error: backward reference detected: f in /example/libbar.rlib(bar.bar.412f8222-cgu.0.rcgu.o) refers to /example/libfoo.a(foo.o)
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

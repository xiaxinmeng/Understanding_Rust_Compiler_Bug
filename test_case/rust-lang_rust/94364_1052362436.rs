log
cross build --target x86_64-unknown-linux-musl --verbose
+ "rustc" "--print" "sysroot"
+ "rustup" "toolchain" "list"
+ "rustup" "target" "list" "--toolchain" "nightly-x86_64-unknown-linux-gnu"
+ "rustup" "component" "list" "--toolchain" "nightly-x86_64-unknown-linux-gnu"
+ "/usr/bin/docker" "run" "--userns" "host" "-e" "RUST_BACKTRACE" "-e" "RUSTC_LOG" "-e" "RUSTFLAFS" "-e" "TRAVIS" "-e" "CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER" "-e" "PKG_CONFIG_ALLOW_CROSS=1" "--rm" "--user" "1000:1000" "-e" "XARGO_HOME=/xargo" "-e" "CARGO_HOME=/cargo" "-e" "CARGO_TARGET_DIR=/target" "-e" "USER=hit" "-e" "CROSS_RUNNER=" "-v" "/home/hit/.xargo:/xargo:Z" "-v" "/home/hit/.cargo:/cargo:Z" "-v" "/cargo/bin" "-v" "/tmp/hello:/project:Z" "-v" "/home/hit/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu:/rust:Z,ro" "-v" "/tmp/hello/target:/target:Z" "-w" "/project" "-i" "-t" "rustembedded/cross:x86_64-unknown-linux-musl-0.2.1" "sh" "-c" "PATH=$PATH:/rust/bin cargo build --target x86_64-unknown-linux-musl --verbose"
   Compiling hello v0.1.0 (/project)
     Running `rustc --crate-name hello --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=829887921b773502 -C extra-filename=-829887921b773502 --out-dir /target/x86_64-unknown-linux-musl/debug/deps --target x86_64-unknown-linux-musl -C linker=/project/./linker.sh -C incremental=/target/x86_64-unknown-linux-musl/debug/incremental -L dependency=/target/x86_64-unknown-linux-musl/debug/deps -L dependency=/target/debug/deps`
INFO rustc_codegen_ssa::back::link preparing Executable to "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502"
INFO rustc_codegen_ssa::back::link "/project/./linker.sh" "-m64" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.1rv0d91piuhiqxh4.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2e5m234xyhhdttby.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2xhw809g3q9mir5i.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3dmb1mmv8ui0prei.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.4u3yezs97b6o5vs6.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.51bhq2yhabgg8rv4.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.557c5ccqr830zw1g.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.bgt79wj3eb54hlp.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3k0isbt8x00k3j1z.rcgu.o" "-Wl,--as-needed" "-L" "/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/target/debug/deps" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-63e39689c2cf3538.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-bb462bc3569a27f5.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-819593aa59c3d95d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-c83b23b18c7ed46d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-dd85e916c6d6e986.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libmemchr-4b2248e24e4aaaa4.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-7b7755d6c6aa1e78.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-370655f1f5fc977e.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-d8c91cec4bdd3371.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-ed18b4e68c4df61b.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-67ccd6d816ae08e5.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-d5c4301e1deb03a3.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-e422a750c87789d6.rlib" "-lunwind" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-ff40ea7fd0a612e3.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-9946a456dc1d9b49.rlib" "-lc" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-20d0ff2e8313b17d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d23d1fdf2f9c2e82.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-4927c240c984c2ab.rlib" "-Wl,--end-group" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-nostartfiles" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502" "-Wl,--gc-sections" "-static-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
INFO rustc_codegen_ssa::back::link linker output: "Using built-in specs.\nCOLLECT_GCC=x86_64-linux-musl-gcc\nCOLLECT_LTO_WRAPPER=/usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/lto-wrapper\nx86_64-linux-musl-gcc: error: unrecognized command line option '-static-pie'; did you mean '-static'?\nTarget: x86_64-linux-musl\nConfigured with: ../src_gcc/configure --enable-languages=c,c++ --disable-werror --target=x86_64-linux-musl --prefix= --libdir=/lib --disable-multilib --with-sysroot=/x86_64-linux-musl --enable-tls --disable-libmudflap --disable-libsanitizer --disable-gnu-indirect-function --disable-libmpx --enable-libstdcxx-time --with-build-sysroot=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_sysroot AR_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/ar AS_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/gas/as-new LD_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/ld/ld-new NM_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/nm-new OBJCOPY_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/objcopy OBJDUMP_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/objdump RANLIB_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/ranlib READELF_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/readelf STRIP_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/strip-new\nThread model: posix\ngcc version 6.4.0 (GCC) \n"
WARN rustc_codegen_ssa::back::link Linker does not support -static-pie command line option. Retrying with -static instead.
INFO rustc_codegen_ssa::back::link "/project/./linker.sh" "-m64" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crt1.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbegin.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.1rv0d91piuhiqxh4.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2e5m234xyhhdttby.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2xhw809g3q9mir5i.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3dmb1mmv8ui0prei.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.4u3yezs97b6o5vs6.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.51bhq2yhabgg8rv4.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.557c5ccqr830zw1g.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.bgt79wj3eb54hlp.rcgu.o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3k0isbt8x00k3j1z.rcgu.o" "-Wl,--as-needed" "-L" "/target/x86_64-unknown-linux-musl/debug/deps" "-L" "/target/debug/deps" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-63e39689c2cf3538.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-bb462bc3569a27f5.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-819593aa59c3d95d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-c83b23b18c7ed46d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-dd85e916c6d6e986.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libmemchr-4b2248e24e4aaaa4.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-7b7755d6c6aa1e78.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-370655f1f5fc977e.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-d8c91cec4bdd3371.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-ed18b4e68c4df61b.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-67ccd6d816ae08e5.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-d5c4301e1deb03a3.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-e422a750c87789d6.rlib" "-lunwind" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-ff40ea7fd0a612e3.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-9946a456dc1d9b49.rlib" "-lc" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-20d0ff2e8313b17d.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d23d1fdf2f9c2e82.rlib" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-4927c240c984c2ab.rlib" "-Wl,--end-group" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-nostartfiles" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib" "-L" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-o" "/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502" "-Wl,--gc-sections" "-static" "-Wl,-zrelro,-znow" "-nodefaultlibs" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtend.o" "/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
INFO rustc_codegen_ssa::back::link linker stderr:
Using built-in specs.
COLLECT_GCC=x86_64-linux-musl-gcc
COLLECT_LTO_WRAPPER=/usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/lto-wrapper
Target: x86_64-linux-musl
Configured with: ../src_gcc/configure --enable-languages=c,c++ --disable-werror --target=x86_64-linux-musl --prefix= --libdir=/lib --disable-multilib --with-sysroot=/x86_64-linux-musl --enable-tls --disable-libmudflap --disable-libsanitizer --disable-gnu-indirect-function --disable-libmpx --enable-libstdcxx-time --with-build-sysroot=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_sysroot AR_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/ar AS_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/gas/as-new LD_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/ld/ld-new NM_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/nm-new OBJCOPY_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/objcopy OBJDUMP_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/objdump RANLIB_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/ranlib READELF_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/readelf STRIP_FOR_TARGET=/tmp/tmp.vJc5MTJtFv/build/local/x86_64-linux-musl/obj_binutils/binutils/strip-new
Thread model: posix
gcc version 6.4.0 (GCC)
COMPILER_PATH=/usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/:/usr/local/bin/../libexec/gcc/:/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.4.0/../../../../x86_64-linux-musl/bin/
LIBRARY_PATH=/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.4.0/:/usr/local/bin/../lib/gcc/:/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.4.0/../../../../x86_64-linux-musl/lib/:/usr/local/bin/../x86_64-linux-musl/lib/
COLLECT_GCC_OPTIONS='-v' '-m64' '-L/target/x86_64-unknown-linux-musl/debug/deps' '-L/target/debug/deps' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib' '-nostartfiles' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained' '-o' '/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502' '-static' '-nodefaultlibs' '-mtune=generic' '-march=x86-64'
 /usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/collect2 -plugin /usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/liblto_plugin.so -plugin-opt=/usr/local/bin/../libexec/gcc/x86_64-linux-musl/6.4.0/lto-wrapper -plugin-opt=-fresolution=/tmp/ccIPxw2L.res --sysroot=/usr/local/bin/../x86_64-linux-musl -m elf_x86_64 -static -o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502 -L/target/x86_64-unknown-linux-musl/debug/deps -L/target/debug/deps -L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib -L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib -L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained -L/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.4.0 -L/usr/local/bin/../lib/gcc -L/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.4.0/../../../../x86_64-linux-musl/lib -L/usr/local/bin/../x86_64-linux-musl/lib --verbose /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crt1.o /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbegin.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.1rv0d91piuhiqxh4.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2e5m234xyhhdttby.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2xhw809g3q9mir5i.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3dmb1mmv8ui0prei.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.4u3yezs97b6o5vs6.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.51bhq2yhabgg8rv4.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.557c5ccqr830zw1g.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.bgt79wj3eb54hlp.rcgu.o /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3k0isbt8x00k3j1z.rcgu.o --as-needed --start-group -Bstatic /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-63e39689c2cf3538.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-bb462bc3569a27f5.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-819593aa59c3d95d.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-c83b23b18c7ed46d.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-dd85e916c6d6e986.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libmemchr-4b2248e24e4aaaa4.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-7b7755d6c6aa1e78.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-370655f1f5fc977e.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-d8c91cec4bdd3371.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-ed18b4e68c4df61b.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-67ccd6d816ae08e5.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-d5c4301e1deb03a3.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-e422a750c87789d6.rlib -lunwind /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-ff40ea7fd0a612e3.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-9946a456dc1d9b49.rlib -lc /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-20d0ff2e8313b17d.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d23d1fdf2f9c2e82.rlib /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-4927c240c984c2ab.rlib --end-group /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib -Bdynamic --eh-frame-hdr -znoexecstack --gc-sections -zrelro -znow /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtend.o /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o
COLLECT_GCC_OPTIONS='-v' '-m64' '-L/target/x86_64-unknown-linux-musl/debug/deps' '-L/target/debug/deps' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib' '-nostartfiles' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib' '-L/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained' '-o' '/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502' '-static' '-nodefaultlibs' '-mtune=generic' '-march=x86-64'

INFO rustc_codegen_ssa::back::link linker stdout:
GNU ld (GNU Binutils) 2.27
  Supported emulations:
   elf_x86_64
   elf32_x86_64
   elf_i386
   elf_iamcu
   i386linux
   elf_l1om
   elf_k1om
using internal linker script:
==================================================
/* Script for -z combreloc -z now -z relro: combine and sort reloc sections */
/* Copyright (C) 2014-2016 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */
OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64",
	      "elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)
SEARCH_DIR("=/x86_64-linux-musl/lib64"); SEARCH_DIR("=/usr/local/lib64"); SEARCH_DIR("=/lib64"); SEARCH_DIR("=/usr/lib64"); SEARCH_DIR("=/x86_64-linux-musl/lib"); SEARCH_DIR("=/usr/local/lib"); SEARCH_DIR("=/lib"); SEARCH_DIR("=/usr/lib");
SECTIONS
{
  /* Read-only sections, merged into text segment: */
  PROVIDE (__executable_start = SEGMENT_START("text-segment", 0x400000)); . = SEGMENT_START("text-segment", 0x400000) + SIZEOF_HEADERS;
  .interp         : { *(.interp) }
  .note.gnu.build-id : { *(.note.gnu.build-id) }
  .hash           : { *(.hash) }
  .gnu.hash       : { *(.gnu.hash) }
  .dynsym         : { *(.dynsym) }
  .dynstr         : { *(.dynstr) }
  .gnu.version    : { *(.gnu.version) }
  .gnu.version_d  : { *(.gnu.version_d) }
  .gnu.version_r  : { *(.gnu.version_r) }
  .rela.dyn       :
    {
      *(.rela.init)
      *(.rela.text .rela.text.* .rela.gnu.linkonce.t.*)
      *(.rela.fini)
      *(.rela.rodata .rela.rodata.* .rela.gnu.linkonce.r.*)
      *(.rela.data .rela.data.* .rela.gnu.linkonce.d.*)
      *(.rela.tdata .rela.tdata.* .rela.gnu.linkonce.td.*)
      *(.rela.tbss .rela.tbss.* .rela.gnu.linkonce.tb.*)
      *(.rela.ctors)
      *(.rela.dtors)
      *(.rela.got)
      *(.rela.bss .rela.bss.* .rela.gnu.linkonce.b.*)
      *(.rela.ldata .rela.ldata.* .rela.gnu.linkonce.l.*)
      *(.rela.lbss .rela.lbss.* .rela.gnu.linkonce.lb.*)
      *(.rela.lrodata .rela.lrodata.* .rela.gnu.linkonce.lr.*)
      *(.rela.ifunc)
    }
  .rela.plt       :
    {
      *(.rela.plt)
      PROVIDE_HIDDEN (__rela_iplt_start = .);
      *(.rela.iplt)
      PROVIDE_HIDDEN (__rela_iplt_end = .);
    }
  .init           :
  {
    KEEP (*(SORT_NONE(.init)))
  }
  .plt            : { *(.plt) *(.iplt) }
.plt.got        : { *(.plt.got) }
.plt.bnd        : { *(.plt.bnd) }
  .text           :
  {
    *(.text.unlikely .text.*_unlikely .text.unlikely.*)
    *(.text.exit .text.exit.*)
    *(.text.startup .text.startup.*)
    *(.text.hot .text.hot.*)
    *(.text .stub .text.* .gnu.linkonce.t.*)
    /* .gnu.warning sections are handled specially by elf32.em.  */
    *(.gnu.warning)
  }
  .fini           :
  {
    KEEP (*(SORT_NONE(.fini)))
  }
  PROVIDE (__etext = .);
  PROVIDE (_etext = .);
  PROVIDE (etext = .);
  .rodata         : { *(.rodata .rodata.* .gnu.linkonce.r.*) }
  .rodata1        : { *(.rodata1) }
  .eh_frame_hdr : { *(.eh_frame_hdr) *(.eh_frame_entry .eh_frame_entry.*) }
  .eh_frame       : ONLY_IF_RO { KEEP (*(.eh_frame)) *(.eh_frame.*) }
  .gcc_except_table   : ONLY_IF_RO { *(.gcc_except_table
  .gcc_except_table.*) }
  .gnu_extab   : ONLY_IF_RO { *(.gnu_extab*) }
  /* These sections are generated by the Sun/Oracle C++ compiler.  */
  .exception_ranges   : ONLY_IF_RO { *(.exception_ranges
  .exception_ranges*) }
  /* Adjust the address for the data segment.  We want to adjust up to
     the same address within the page on the next page up.  */
  . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));
  /* Exception handling  */
  .eh_frame       : ONLY_IF_RW { KEEP (*(.eh_frame)) *(.eh_frame.*) }
  .gnu_extab      : ONLY_IF_RW { *(.gnu_extab) }
  .gcc_except_table   : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) }
  .exception_ranges   : ONLY_IF_RW { *(.exception_ranges .exception_ranges*) }
  /* Thread Local Storage sections  */
  .tdata	  : { *(.tdata .tdata.* .gnu.linkonce.td.*) }
  .tbss		  : { *(.tbss .tbss.* .gnu.linkonce.tb.*) *(.tcommon) }
  .preinit_array     :
  {
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP (*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);
  }
  .init_array     :
  {
    PROVIDE_HIDDEN (__init_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
    PROVIDE_HIDDEN (__init_array_end = .);
  }
  .fini_array     :
  {
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
    PROVIDE_HIDDEN (__fini_array_end = .);
  }
  .ctors          :
  {
    /* gcc uses crtbegin.o to find the start of
       the constructors, so we make sure it is
       first.  Because this is a wildcard, it
       doesn't matter if the user does not
       actually link against crtbegin.o; the
       linker won't look for a file to match a
       wildcard.  The wildcard also means that it
       doesn't matter which directory crtbegin.o
       is in.  */
    KEEP (*crtbegin.o(.ctors))
    KEEP (*crtbegin?.o(.ctors))
    /* We don't want to include the .ctor section from
       the crtend.o file until after the sorted ctors.
       The .ctor section from the crtend file contains the
       end of ctors marker and it must be last */
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
    KEEP (*(SORT(.ctors.*)))
    KEEP (*(.ctors))
  }
  .dtors          :
  {
    KEEP (*crtbegin.o(.dtors))
    KEEP (*crtbegin?.o(.dtors))
    KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
    KEEP (*(SORT(.dtors.*)))
    KEEP (*(.dtors))
  }
  .jcr            : { KEEP (*(.jcr)) }
  .data.rel.ro : { *(.data.rel.ro.local* .gnu.linkonce.d.rel.ro.local.*) *(.data.rel.ro .data.rel.ro.* .gnu.linkonce.d.rel.ro.*) }
  .dynamic        : { *(.dynamic) }
  .got            : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }
  . = DATA_SEGMENT_RELRO_END (0, .);
  .data           :
  {
    *(.data .data.* .gnu.linkonce.d.*)
    SORT(CONSTRUCTORS)
  }
  .data1          : { *(.data1) }
  _edata = .; PROVIDE (edata = .);
  . = .;
  __bss_start = .;
  .bss            :
  {
   *(.dynbss)
   *(.bss .bss.* .gnu.linkonce.b.*)
   *(COMMON)
   /* Align here to ensure that the .bss section occupies space up to
      _end.  Align after .bss to ensure correct alignment even if the
      .bss section disappears because there are no input sections.
      FIXME: Why do we need it? When there is no .bss section, we don't
      pad the .data section.  */
   . = ALIGN(. != 0 ? 64 / 8 : 1);
  }
  .lbss   :
  {
    *(.dynlbss)
    *(.lbss .lbss.* .gnu.linkonce.lb.*)
    *(LARGE_COMMON)
  }
  . = ALIGN(64 / 8);
  . = SEGMENT_START("ldata-segment", .);
  .lrodata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
  {
    *(.lrodata .lrodata.* .gnu.linkonce.lr.*)
  }
  .ldata   ALIGN(CONSTANT (MAXPAGESIZE)) + (. & (CONSTANT (MAXPAGESIZE) - 1)) :
  {
    *(.ldata .ldata.* .gnu.linkonce.l.*)
    . = ALIGN(. != 0 ? 64 / 8 : 1);
  }
  . = ALIGN(64 / 8);
  _end = .; PROVIDE (end = .);
  . = DATA_SEGMENT_END (.);
  /* Stabs debugging sections.  */
  .stab          0 : { *(.stab) }
  .stabstr       0 : { *(.stabstr) }
  .stab.excl     0 : { *(.stab.excl) }
  .stab.exclstr  0 : { *(.stab.exclstr) }
  .stab.index    0 : { *(.stab.index) }
  .stab.indexstr 0 : { *(.stab.indexstr) }
  .comment       0 : { *(.comment) }
  /* DWARF debug sections.
     Symbols in the DWARF debugging sections are relative to the beginning
     of the section so we begin them at 0.  */
  /* DWARF 1 */
  .debug          0 : { *(.debug) }
  .line           0 : { *(.line) }
  /* GNU DWARF 1 extensions */
  .debug_srcinfo  0 : { *(.debug_srcinfo) }
  .debug_sfnames  0 : { *(.debug_sfnames) }
  /* DWARF 1.1 and DWARF 2 */
  .debug_aranges  0 : { *(.debug_aranges) }
  .debug_pubnames 0 : { *(.debug_pubnames) }
  /* DWARF 2 */
  .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
  .debug_abbrev   0 : { *(.debug_abbrev) }
  .debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end ) }
  .debug_frame    0 : { *(.debug_frame) }
  .debug_str      0 : { *(.debug_str) }
  .debug_loc      0 : { *(.debug_loc) }
  .debug_macinfo  0 : { *(.debug_macinfo) }
  /* SGI/MIPS DWARF 2 extensions */
  .debug_weaknames 0 : { *(.debug_weaknames) }
  .debug_funcnames 0 : { *(.debug_funcnames) }
  .debug_typenames 0 : { *(.debug_typenames) }
  .debug_varnames  0 : { *(.debug_varnames) }
  /* DWARF 3 */
  .debug_pubtypes 0 : { *(.debug_pubtypes) }
  .debug_ranges   0 : { *(.debug_ranges) }
  /* DWARF Extension.  */
  .debug_macro    0 : { *(.debug_macro) }
  .gnu.attributes 0 : { KEEP (*(.gnu.attributes)) }
  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}


==================================================
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crt1.o succeeded
/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crt1.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o succeeded
/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbegin.o succeeded
/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbegin.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.1rv0d91piuhiqxh4.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.1rv0d91piuhiqxh4.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2e5m234xyhhdttby.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2e5m234xyhhdttby.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2xhw809g3q9mir5i.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.2xhw809g3q9mir5i.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3dmb1mmv8ui0prei.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3dmb1mmv8ui0prei.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.4u3yezs97b6o5vs6.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.4u3yezs97b6o5vs6.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.51bhq2yhabgg8rv4.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.51bhq2yhabgg8rv4.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.557c5ccqr830zw1g.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.557c5ccqr830zw1g.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.bgt79wj3eb54hlp.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.bgt79wj3eb54hlp.rcgu.o
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3k0isbt8x00k3j1z.rcgu.o succeeded
/target/x86_64-unknown-linux-musl/debug/deps/hello-829887921b773502.3k0isbt8x00k3j1z.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-63e39689c2cf3538.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-63e39689c2cf3538.rlib)std-63e39689c2cf3538.std.cb90f29a-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-bb462bc3569a27f5.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-bb462bc3569a27f5.rlib)panic_unwind-bb462bc3569a27f5.panic_unwind.427bf97e-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-819593aa59c3d95d.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libminiz_oxide-819593aa59c3d95d.rlib)miniz_oxide-819593aa59c3d95d.miniz_oxide.705ce713-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-c83b23b18c7ed46d.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libadler-c83b23b18c7ed46d.rlib)adler-c83b23b18c7ed46d.adler.44c8b7a8-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-dd85e916c6d6e986.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libobject-dd85e916c6d6e986.rlib)object-dd85e916c6d6e986.object.f69f2563-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libmemchr-4b2248e24e4aaaa4.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libmemchr-4b2248e24e4aaaa4.rlib)memchr-4b2248e24e4aaaa4.memchr.e0360402-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-7b7755d6c6aa1e78.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libaddr2line-7b7755d6c6aa1e78.rlib)addr2line-7b7755d6c6aa1e78.addr2line.43dd430f-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-370655f1f5fc977e.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libgimli-370655f1f5fc977e.rlib)gimli-370655f1f5fc977e.gimli.1b405565-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd_detect-d8c91cec4bdd3371.rlib succeeded
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-ed18b4e68c4df61b.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-ed18b4e68c4df61b.rlib)rustc_demangle-ed18b4e68c4df61b.rustc_demangle.3041a10e-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libhashbrown-67ccd6d816ae08e5.rlib succeeded
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_alloc-d5c4301e1deb03a3.rlib succeeded
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-e422a750c87789d6.rlib succeeded
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/libunwind.a failed
attempt to open /target/debug/deps/libunwind.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a)UnwindLevel1-gcc-ext.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a)UnwindLevel1.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a)UnwindRegistersSave.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a)libunwind.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libunwind.a)UnwindRegistersRestore.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcfg_if-ff40ea7fd0a612e3.rlib succeeded
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-9946a456dc1d9b49.rlib succeeded
attempt to open /target/x86_64-unknown-linux-musl/debug/deps/libc.a failed
attempt to open /target/debug/deps/libc.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libc.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libc.a failed
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sysconf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)closedir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dirfd.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)opendir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)readdir_r.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__environ.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__libc_start_main.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getenv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setenv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)unsetenv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__errno_location.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)abort.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)assert.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)exit.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fcntl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)open.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)defsysinfo.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)libc.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)syscall_ret.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dl_iterate_phdr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dladdr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)chroot.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)prctl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sendfile.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setgroups.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)splice.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sysinfo.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lite_malloc.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)malloc.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_memalign.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getrlimit.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)ioctl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)realpath.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)syscall.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)madvise.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mmap.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mprotect.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mremap.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)munmap.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)accept4.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)bind.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)connect.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)freeaddrinfo.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)gai_strerror.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getaddrinfo.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getpeername.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getsockname.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getsockopt.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)htons.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)listen.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lookup_name.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lookup_serv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)recv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)recvfrom.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)recvmsg.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)res_mkquery.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)res_msend.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)resolvconf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)send.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sendmsg.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sendto.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setsockopt.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)shutdown.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)socket.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)socketpair.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getpw_r.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)execvp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fork.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawn_file_actions_adddup2.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawn_file_actions_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawn_file_actions_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnattr_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnattr_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnattr_setflags.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnattr_setsigdefault.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnattr_setsigmask.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawnp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)waitpid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)affinity.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sched_yield.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)poll.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)block.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)kill.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)raise.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sigaction.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sigaddset.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sigaltstack.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sigemptyset.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)signal.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)restore.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)chmod.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fchmod.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fstat.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fstatat.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lstat.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mkdir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)stat.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__fclose_ca.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__fopen_rb_ca.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__stdio_close.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__stdio_read.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__stdio_seek.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fflush.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fgets.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fprintf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fwrite.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getc.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)ofl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)rename.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)snprintf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)stderr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)vfprintf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)vsnprintf.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)qsort.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strtol.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)bcmp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memchr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memcmp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memrchr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strchr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strchrnul.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strcpy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strdup.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strerror_r.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strlen.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strncmp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strnlen.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strstr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memcpy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memmove.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memset.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__lock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__syscall_cp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__tls_get_addr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__wait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_attr_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_attr_get.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_attr_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_attr_setstacksize.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cleanup_push.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_broadcast.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_signal.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_timedwait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_cond_wait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_condattr_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_condattr_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_condattr_setclock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_create.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_detach.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_getattr_np.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_getspecific.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_join.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_key_create.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_lock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_timedlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_trylock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutex_unlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutexattr_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutexattr_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_mutexattr_settype.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_rdlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_timedrdlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_tryrdlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_unlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_wrlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_self.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_setcancelstate.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_setspecific.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_sigmask.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_testcancel.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)vmlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__unmapself.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)clone.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)clock_gettime.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)nanosleep.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)_exit.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)chdir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)chown.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)close.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dup2.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fchown.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fdatasync.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fsync.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)ftruncate.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getcwd.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getpid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getppid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getuid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lchown.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)linkat.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lseek.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pipe2.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pread.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pwrite.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)read.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)readlink.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)readv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)rmdir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setgid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setuid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)setxid.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)symlink.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)unlink.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)write.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)writev.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)isalnum.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)readdir.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__init_tls.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)putenv.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strerror.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)_Exit.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)intscan.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)procfdname.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)shgetc.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)vdso.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__lctrans.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)expand_heap.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)memalign.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__fpclassifyl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__signbitl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)frexpl.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mbstowcs.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)wctomb.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)accept.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dn_expand.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)dns_parse.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)lookup_ipliteral.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getpw_a.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getpwent_a.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)nscd_query.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)execve.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)posix_spawn.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sigismember.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__fdopen.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__lockfile.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__stdio_write.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__towrite.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__uflow.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fclose.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)ferror.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fopen.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)fread.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getline.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)ofl_add.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)stpcpy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strcmp.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__timedwait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)default_attr.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_timedwrlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pthread_rwlock_trywrlock.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)synccall.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__set_thread_area.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)clock_nanosleep.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)pipe.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)mbsrtowcs.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)wcrtomb.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)if_nametoindex.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)inet_aton.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)inet_pton.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__fmodeflags.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__stdio_exit.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)__toread.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)getdelim.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)strncpy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_destroy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_init.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_post.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_wait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)internal.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)stpncpy.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_timedwait.lo
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/libc.a)sem_trywait.lo
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-20d0ff2e8313b17d.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-20d0ff2e8313b17d.rlib)alloc-20d0ff2e8313b17d.alloc.4253fd37-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-d23d1fdf2f9c2e82.rlib succeeded
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-4927c240c984c2ab.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-4927c240c984c2ab.rlib)core-4927c240c984c2ab.core.73711de5-cgu.0.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib succeeded
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib)compiler_builtins-0b55627163d3fb03.compiler_builtins.1284b472-cgu.101.rcgu.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib)compiler_builtins-0b55627163d3fb03.compiler_builtins.1284b472-cgu.118.rcgu.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib)compiler_builtins-0b55627163d3fb03.compiler_builtins.1284b472-cgu.2.rcgu.o
(/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-0b55627163d3fb03.rlib)compiler_builtins-0b55627163d3fb03.compiler_builtins.1284b472-cgu.43.rcgu.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtend.o succeeded
/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtend.o
attempt to open /rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o succeeded
/rust/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o

    Finished dev [unoptimized + debuginfo] target(s) in 0.34s

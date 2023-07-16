
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-m64" "-nostdlib" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o" "-L" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib" "/home/johannes/hello/target/x86_64-unknown-linux-musl/release/deps/hello-88b5f1c27ffd5796.hello.81h291ix-cgu.0.rcgu.o" "-o" "/home/johannes/hello/target/x86_64-unknown-linux-musl/release/deps/hello-88b5f1c27ffd5796" "-Wl,--gc-sections" "-no-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/home/johannes/hello/target/x86_64-unknown-linux-musl/release/deps" "-L" "/home/johannes/hello/target/release/deps" "-L" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-lc" "-Wl,--no-whole-archive" "/home/johannes/hello/target/x86_64-unknown-linux-musl/release/deps/liblibc-9b25b07157410626.rlib" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-58a61c6fb52da4c6.rlib" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-8f8f2fe3f7b7398f.rlib" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-6db28108a6c10236.rlib" "-static" "-Wl,-Bdynamic" "/home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/crtn.o"
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(rcmd.o): in function `__validuser2_sa':
          (.text+0x5a9): warning: Using 'getaddrinfo' in statically linked applications requires at runtime the shared libraries from the glibc version used for linking
          /usr/bin/ld: /home/johannes/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o: in function `_start':
          crt1.c:(.text+0x9): undefined reference to `_DYNAMIC'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(printf_fp.o): in function `__printf_fp_l':
          (.text+0x52a): undefined reference to `__unordtf2'
          /usr/bin/ld: (.text+0x562): undefined reference to `__unordtf2'
          /usr/bin/ld: (.text+0x588): undefined reference to `__letf2'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(printf_fphex.o): in function `__printf_fphex':
          (.text+0xac): undefined reference to `__unordtf2'
          /usr/bin/ld: (.text+0xe2): undefined reference to `__unordtf2'
          /usr/bin/ld: (.text+0xfa): undefined reference to `__letf2'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iofclose.o): in function `_IO_new_fclose.cold':
          (.text.unlikely+0x4c): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iofclose.o):(.data.rel.local.DW.ref.__gcc_personality_v0[DW.ref.__gcc_personality_v0]+0x0): undefined reference to `__gcc_personality_v0'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iofflush.o): in function `_IO_fflush.cold':
          (.text.unlikely+0x4b): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iofputs.o): in function `_IO_fputs.cold':
          (.text.unlikely+0x4b): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iofwrite.o): in function `_IO_fwrite.cold':
          (.text.unlikely+0x4b): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(iogetdelim.o): in function `_IO_getdelim.cold':
          (.text.unlikely+0x4b): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(ioputs.o): in function `_IO_puts.cold':
          (.text.unlikely+0x4c): undefined reference to `_Unwind_Resume'
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(wfileops.o):(.text.unlikely+0x4c): more undefined references to `_Unwind_Resume' follow
          /usr/bin/ld: /usr/lib/gcc/x86_64-pc-linux-gnu/9.1.0/../../../../lib/libc.a(dl-reloc-static-pie.o): in function `_dl_relocate_static_pie':
          (.text+0x1a): undefined reference to `_DYNAMIC'
          /usr/bin/ld: (.text+0x39): undefined reference to `_DYNAMIC'
          /usr/bin/ld: /home/johannes/hello/target/x86_64-unknown-linux-musl/release/deps/hello-88b5f1c27ffd5796: hidden symbol `_DYNAMIC' isn't defined
          /usr/bin/ld: final link failed: bad value
          collect2: error: ld returned 1 exit status

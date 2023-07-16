
Compiling rustscan v2.1.1 (/home/$USER/Projets/XXX/RustScan)
error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/home/$USER/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/$USER/.vscode-server/bin/441438abd1ac652551dbe4d408dfcec8a499b8bf/bin/remote-cli:/home/$USER/.local/bin:/home/$USER/.cargo/bin:/home/$USER/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" VSLANG="1033" "cc" "-m32" "/tmp/rustcxtfFc3/symbols.o" "/home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps/rustscan-8069d2dff506ca4d.rustscan.a822f4a7-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps" "-L" "/home/$USER/Projets/XXX/RustScan/target/release/deps" "-L" "/home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/build/ring-b1a3c9ae3ccb6cc9/out" "-L" "/home/$USER/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/tmp/rustcxtfFc3/libring-a85cb11ce5dd843d.rlib" "/home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps/libcompiler_builtins-e8354e5232d2c05a.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/home/$USER/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib" "-o" "/home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps/rustscan-8069d2dff506ca4d" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: /usr/bin/ld: /home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps/libcompiler_builtins-e8354e5232d2c05a.rlib(compiler_builtins-e8354e5232d2c05a.compiler_builtins.e77cb671-cgu.0.rcgu.o): in function `<u32 as compiler_builtins::int::Int>::logical_shr':
          compiler_builtins.e77cb671-cgu.0:(.text._RNvXs4_NtCsg8UtXzRGVrx_17compiler_builtins3intmNtB5_3Int11logical_shr+0x1e): undefined reference to `<u32 as core::convert::TryInto<u32>>::try_into'
          /usr/bin/ld: /home/$USER/Projets/XXX/RustScan/target/i686-unknown-linux-gnu/release/deps/libcompiler_builtins-e8354e5232d2c05a.rlib(compiler_builtins-e8354e5232d2c05a.compiler_builtins.e77cb671-cgu.0.rcgu.o): in function `<u32 as compiler_builtins::int::Int>::wrapping_shl':
          compiler_builtins.e77cb671-cgu.0:(.text._RNvXs4_NtCsg8UtXzRGVrx_17compiler_builtins3intmNtB5_3Int12wrapping_shl+0x1e): undefined reference to `<u32 as core::convert::TryInto<u32>>::try_into'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `rustscan` due to previous error

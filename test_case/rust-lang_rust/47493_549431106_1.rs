
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/andy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "foo.foo.7rcbfp3g-cgu.0.rcgu.o" "foo.foo.7rcbfp3g-cgu.1.rcgu.o" "foo.foo.7rcbfp3g-cgu.2.rcgu.o" "foo.foo.7rcbfp3g-cgu.3.rcgu.o" "-o" "foo" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/home/andy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/andy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-837ca740df32db0a.rlib" "/home/andy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-db27c965e824589f.rlib" "/home/andy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-68a4f8466685ed76.rlib" "-nostartfiles" "-Wl,-Bdynamic"
  = note: foo.foo.7rcbfp3g-cgu.3.rcgu.o: In function `core::result::Result<T,E>::unwrap':
          foo.7rcbfp3g-cgu.3:(.text._ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+0x20): undefined reference to `_Unwind_Resume'
          foo.foo.7rcbfp3g-cgu.3.rcgu.o:(.data.DW.ref.rust_eh_personality[DW.ref.rust_eh_personality]+0x0): undefined reference to `rust_eh_personality'
          collect2: error: ld returned 1 exit status

plain
= note: riscv64-linux-gnu-ld: /home/light4/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libstd-76c3ec9d745aded6.rlib(s
td-76c3ec9d745aded6.std.2i3x0diz-cgu.0.rcgu.o): in function `std::sys::unix::os::home_dir::fallback':
/rustc/a2f8f6281817d430e20726128b739d3c6708561c//library/std/src/sys/unix/os.rs:615: warning: Using 'getpwuid_r' in statically linked applications requires at ru
ntime the shared libraries from the glibc version used for linking
riscv64-linux-gnu-ld: /home/light4/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/riscv64gc-unknown-linux-gnu/lib/libstd-76c3ec9d745aded6.rlib(s
td-76c3ec9d745aded6.std.2i3x0diz-cgu.0.rcgu.o): in function `<std::sys_common::net::LookupHost as core::convert::TryFrom<(&str,u16)>>::try_from':
/rustc/a2f8f6281817d430e20726128b739d3c6708561c//library/std/src/sys_common/net.rs:197: warning: Using 'getaddrinfo' in statically linked applications requires a
t runtime the shared libraries from the glibc version used for linking
riscv64-linux-gnu-ld: cannot find -lgcc_eh
riscv64-linux-gnu-ld: cannot find -lgcc

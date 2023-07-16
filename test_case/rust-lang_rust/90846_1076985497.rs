
  = note: /nix/store/3dvmvdf1a4p2ra4myhzdw3i2gr6xhy9b-aarch64-unknown-linux-musl-binutils-2.35.2/bin/aarch64-unknown-linux-musl-ld: /home/petwal01/.rustup/toolchains/nightly-2021-11-29-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/libstd-f57035255a9d9f2c.rlib(std-f57035255a9d9f2c.std.c43b9eb8-cgu.0.rcgu.o): in function `std::sys::unix::os::glibc_version':
          /rustc/db9d361a4731ca0bb48533fab6297a8fea75696f//library/std/src/sys/unix/os.rs:644: undefined reference to `gnu_get_libc_version'
          collect2: error: ld returned 1 exit status

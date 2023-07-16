plain
[RUSTC-TIMING] gimli test:false 5.960
[RUSTC-TIMING] object test:false 11.414
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: type does not implement `Debug`; consider adding `#[derive(Debug)]` or a manual implementation
  --> library/std/src/os/./fortanix_sgx/arch.rs:12:1
   |
12 | pub struct Align16<T>(pub T);
   |
note: the lint level is defined here
note: the lint level is defined here
  --> library/std/src/os/./fortanix_sgx/mod.rs:6:23
   |
6  | #![deny(missing_docs, missing_debug_implementations)]


error: type does not implement `Debug`; consider adding `#[derive(Debug)]` or a manual implementation
  --> library/std/src/os/./fortanix_sgx/arch.rs:17:1
   |
17 | pub struct Align128<T>(pub T);


error: type does not implement `Debug`; consider adding `#[derive(Debug)]` or a manual implementation
  --> library/std/src/os/./fortanix_sgx/arch.rs:22:1
   |
22 | pub struct Align512<T>(pub T);

error: aborting due to 3 previous errors; 1 warning emitted

[RUSTC-TIMING] std test:false 3.722
[RUSTC-TIMING] std test:false 3.722
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fortanix-unknown-sgx" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:26:09

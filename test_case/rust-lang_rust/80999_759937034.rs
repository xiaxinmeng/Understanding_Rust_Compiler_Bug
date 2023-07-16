plain
[RUSTC-TIMING] core test:false 36.465
[RUSTC-TIMING] addr2line test:false 0.437
[RUSTC-TIMING] gimli test:false 5.984
[RUSTC-TIMING] object test:false 10.820
error: use of module `core::i64` that will be deprecated in a future Rust version: all constants in this module replaced by associated constants on `i64`
 --> library/std/src/sys/unix/process/zircon.rs:4:5
4 | use crate::i64;
  |     ^^^^^^^^^^
  |
  |
  = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of constant `core::i64::MAX` that will be deprecated in a future Rust version: replaced by the `MAX` associated constant on this type
  --> library/std/src/sys/unix/process/zircon.rs:19:41
   |
19 | pub const ZX_TIME_INFINITE: zx_time_t = i64::MAX;

error: aborting due to 2 previous errors

[RUSTC-TIMING] std test:false 2.547
[RUSTC-TIMING] std test:false 2.547
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:16:54

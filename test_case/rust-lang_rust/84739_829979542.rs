plain
[RUSTC-TIMING] gimli test:false 5.402
[RUSTC-TIMING] object test:false 10.260
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: invalid register `rbx`: rbx is used internally by LLVM and cannot be used as an operand for inline asm
  --> library/std/src/sys/sgx/ext/arch.rs:37:13
   |
37 |             in("rbx") request,


error: invalid register `rbx`: rbx is used internally by LLVM and cannot be used as an operand for inline asm
  --> library/std/src/sys/sgx/ext/arch.rs:65:13
   |
65 |             in("rbx") targetinfo,

error: aborting due to 2 previous errors; 1 warning emitted

[RUSTC-TIMING] std test:false 1.859
[RUSTC-TIMING] std test:false 1.859
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fortanix-unknown-sgx" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:24:28

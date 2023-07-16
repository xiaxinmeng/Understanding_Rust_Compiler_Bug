plain
[RUSTC-TIMING] gimli test:false 6.171
[RUSTC-TIMING] object test:false 12.404
warning: dropping unsupported crate type `dylib` for target `x86_64-fortanix-unknown-sgx`

error: call to unsafe function is unsafe and requires unsafe block (error E0133)
  --> library/std/src/sys/sgx/mod.rs:45:5
45 |     args::init(argc, argv);
   |     ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
note: the lint level is defined here
note: the lint level is defined here
  --> library/std/src/sys/sgx/mod.rs:5:9
   |
5  | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error; 1 warning emitted

[RUSTC-TIMING] std test:false 3.877
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fortanix-unknown-sgx" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:28:35

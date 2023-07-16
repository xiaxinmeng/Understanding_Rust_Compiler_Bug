plain

error[E0412]: cannot find type `PhantomData` in this scope
  --> library/std/src/sys/wasi/os.rs:89:30
   |
89 | pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);
   |
help: consider importing one of these items
   |
3  | use core::marker::PhantomData;
3  | use core::marker::PhantomData;
   |
3  | use crate::marker::PhantomData;
   |

error[E0392]: parameter `'a` is never used
    |
    |
378 | pub struct SplitPaths<'a> {
    |                       ^^ unused parameter
    |
    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0392, E0412.
For more information about an error, try `rustc --explain E0392`.
For more information about an error, try `rustc --explain E0392`.
[RUSTC-TIMING] std test:false 1.888
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-wasi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:18:11

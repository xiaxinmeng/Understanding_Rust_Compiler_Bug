plain

error[E0308]: mismatched types
    --> library/std/src/fs.rs:2235:5
     |
2234 | pub fn try_exists<P: AsRef<Path>>(path: P) -> io::Result<bool> {
     |                                               ---------------- expected `core::result::Result<bool, io::error::Error>` because of return type
2235 |     fs_imp::try_exists(path.as_ref())
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
     = note: expected enum `core::result::Result<bool, _>`
                found enum `core::result::Result<(), _>`

error: aborting due to previous error; 1 warning emitted
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] std test:false 1.572
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:15:51

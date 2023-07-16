plain

error[E0412]: cannot find type `Args` in this scope
   --> library/std/src/sys/wasi/os.rs:133:16
    |
133 | impl !Send for Args {}
    |
help: consider importing one of these items
    |
3   | use crate::env::Args;
3   | use crate::env::Args;
    |
3   | use crate::sys::args::Args;
    |

error[E0412]: cannot find type `Args` in this scope
   --> library/std/src/sys/wasi/os.rs:134:16
    |
134 | impl !Sync for Args {}
    |
help: consider importing one of these items
    |
3   | use crate::env::Args;
3   | use crate::env::Args;
    |
3   | use crate::sys::args::Args;
    |

error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented for a struct/enum type, not `[type error]`
   --> library/std/src/sys/wasi/os.rs:133:1
    |
133 | impl !Send for Args {}
    | ^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type

error[E0321]: cross-crate traits with a default impl, like `Sync`, can only be implemented for a struct/enum type, not `[type error]`
   --> library/std/src/sys/wasi/os.rs:134:1
    |
134 | impl !Sync for Args {}
    | ^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type
error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0321, E0412.
For more information about an error, try `rustc --explain E0321`.
For more information about an error, try `rustc --explain E0321`.
[RUSTC-TIMING] std test:false 1.823
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-wasi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-pc-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:18:28

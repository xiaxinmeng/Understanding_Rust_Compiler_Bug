plain

error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:148:9
    |
147 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
148 |         libc::STDIN_FILENO
    |         ^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
148 |         libc::STDIN_FILENO.try_into().unwrap()
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:154:9
    |
153 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
154 |         libc::STDOUT_FILENO
    |         ^^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
154 |         libc::STDOUT_FILENO.try_into().unwrap()
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:160:9
    |
159 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
160 |         libc::STDERR_FILENO
    |         ^^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
160 |         libc::STDERR_FILENO.try_into().unwrap()
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:166:9
    |
165 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
166 |         libc::STDIN_FILENO
    |         ^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
166 |         libc::STDIN_FILENO.try_into().unwrap()
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:172:9
    |
171 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
172 |         libc::STDOUT_FILENO
    |         ^^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
172 |         libc::STDOUT_FILENO.try_into().unwrap()
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/std/src/sys/wasi/ext/io.rs:178:9
    |
177 |     fn as_raw_fd(&self) -> RawFd {
    |                            ----- expected `u32` because of return type
178 |         libc::STDERR_FILENO
    |         ^^^^^^^^^^^^^^^^^^^ expected `u32`, found `i32`
    |
help: you can convert an `i32` to a `u32` and panic if the converted value doesn't fit
178 |         libc::STDERR_FILENO.try_into().unwrap()
    |

error: aborting due to 6 previous errors; 1 warning emitted
error: aborting due to 6 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] std test:false 1.959
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-wasi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:19:00

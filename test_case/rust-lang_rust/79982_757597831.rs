plain
[RUSTC-TIMING] addr2line test:false 0.465
[RUSTC-TIMING] core test:false 32.197
[RUSTC-TIMING] gimli test:false 5.017
[RUSTC-TIMING] object test:false 9.358
error[E0599]: no method named `core_dumped` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:227:25
    |
227 |         self.as_inner().core_dumped()
    |                         ^^^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `core_dumped`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:178:1
    |
178 | pub trait ExitStatusExt: private::Sealed {


error[E0599]: no method named `stopped_signal` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:231:25
    |
231 |         self.as_inner().stopped_signal()
    |                         ^^^^^^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `stopped_signal`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:178:1
    |
178 | pub trait ExitStatusExt: private::Sealed {


error[E0599]: no method named `continued` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:235:25
    |
235 |         self.as_inner().continued()
    |                         ^^^^^^^^^ method not found in `&process_inner::ExitStatus`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `continued`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:178:1
    |
178 | pub trait ExitStatusExt: private::Sealed {


error[E0599]: no method named `into_raw` found for reference `&process_inner::ExitStatus` in the current scope
   --> library/std/src/sys/unix/ext/process.rs:239:25
    |
239 |         self.as_inner().into_raw().into()
    |                         ^^^^^^^^ method not found in `&process_inner::ExitStatus`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `ExitStatusExt` defines an item `into_raw`, perhaps you need to implement it
   --> library/std/src/sys/unix/ext/process.rs:178:1
    |
178 | pub trait ExitStatusExt: private::Sealed {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 2.217
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi,i686-unknown-freebsd
Build completed unsuccessfully in 0:14:51

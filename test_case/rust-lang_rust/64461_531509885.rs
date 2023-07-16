plain
2019-09-14T20:10:05.8916789Z [RUSTC-TIMING] backtrace test:false 0.368
2019-09-14T20:10:06.5521263Z [RUSTC-TIMING] hashbrown test:false 0.858
2019-09-14T20:10:07.4785886Z warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`
2019-09-14T20:10:07.4791372Z 
2019-09-14T20:10:11.5515776Z thread 'rustc' panicked at 'Tried to access field 1 of union with 0 fields', src/librustc_mir/interpret/place.rs:386:17
2019-09-14T20:10:11.6092847Z 
2019-09-14T20:10:11.6093099Z error: internal compiler error: unexpected panic
2019-09-14T20:10:11.6093147Z 
2019-09-14T20:10:11.6093206Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-14T20:10:11.6093206Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-14T20:10:11.6093270Z 
2019-09-14T20:10:11.6094659Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-09-14T20:10:11.6094762Z 
2019-09-14T20:10:11.6095130Z note: rustc 1.39.0-nightly (3a7c257b5 2019-09-14) running on x86_64-unknown-linux-gnu
2019-09-14T20:10:11.6095210Z 
2019-09-14T20:10:11.6095771Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C debuginfo=1 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 --crate-type dylib --crate-type rlib
2019-09-14T20:10:11.6095983Z note: some of the compiler flags provided by cargo are hidden
2019-09-14T20:10:11.6096034Z 
2019-09-14T20:10:11.6121728Z [RUSTC-TIMING] std test:false 5.056
2019-09-14T20:10:11.6178912Z error: Could not compile `std`.
2019-09-14T20:10:11.6178912Z error: Could not compile `std`.
2019-09-14T20:10:11.6179009Z 
2019-09-14T20:10:11.6179276Z To learn more, run the command again with --verbose.
2019-09-14T20:10:11.6209763Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-09-14T20:10:11.6223839Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi
2019-09-14T20:10:11.6224116Z Build completed unsuccessfully in 1:03:34
2019-09-14T20:10:11.6286106Z == clock drift check ==
2019-09-14T20:10:11.6304294Z   local time: Sat Sep 14 20:10:11 UTC 2019
2019-09-14T20:10:11.6304294Z   local time: Sat Sep 14 20:10:11 UTC 2019
2019-09-14T20:10:11.6538957Z   network time: Sat, 14 Sep 2019 20:10:11 GMT
2019-09-14T20:10:11.6543938Z == end clock drift check ==
2019-09-14T20:10:13.1921979Z ##[error]Bash exited with code '1'.
2019-09-14T20:10:13.1961136Z ##[section]Starting: Upload CPU usage statistics
2019-09-14T20:10:13.1971835Z ==============================================================================
2019-09-14T20:10:13.1971944Z Task         : Bash
2019-09-14T20:10:13.1972007Z Description  : Run a Bash script on macOS, Linux, or Windows

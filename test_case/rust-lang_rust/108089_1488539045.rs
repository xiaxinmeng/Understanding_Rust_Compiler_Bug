plain
failures:

---- [ui] tests/ui/thread-local/tls-dylib-access.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/thread-local/auxiliary/tls-rlib.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/thread-local/auxiliary/tls-rlib.rs" "-Zthreads=1" "--target=arm-linux-androideabi" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local/tls-dylib-access/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "-Clinker=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi19-clang" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local/tls-dylib-access/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `cfg(target_thread_local)` is experimental and subject to change
  --> fake-test-src-base/thread-local/auxiliary/tls-rlib.rs:6:8
LL | #![cfg(target_thread_local)]
   |        ^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #29594 <https://github.com/rust-lang/rust/issues/29594> for more information
   = note: see issue #29594 <https://github.com/rust-lang/rust/issues/29594> for more information
   = help: add `#![feature(cfg_target_thread_local)]` to the crate attributes to enable

error: the generated executable for the input file "/checkout/tests/ui/thread-local/auxiliary/tls-rlib.rs" conflicts with the existing directory "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local/tls-dylib-access/auxiliary/tls-rlib"
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
------------------------------------------

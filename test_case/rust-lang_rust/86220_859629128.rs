plain
............i.....................i.....................i.....................i..................... 2800/2860
............................................................
failures:

---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::write (line 416) stdout ----
error[E0599]: no method named `to_to_vec` found for reference `&&'static [u8; 13]` in the current scope
   |
   |
10 |     let hello = x.write((&b"Hello, world!").to_to_vec());
   |                                             ^^^^^^^^^ help: there is an associated function with a similar name: `to_vec`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:01

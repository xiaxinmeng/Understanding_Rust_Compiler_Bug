plain
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::write (line 416) stdout ----
error[E0308]: mismatched types
  --> src/mem/maybe_uninit.rs:423:25
   |
10 |     let hello = x.write(b"Hello, world!".to_owned());
   |                         |
   |                         |
   |                         expected struct `Vec`, found array `[u8; 13]`
   |                         help: try using a conversion method: `b"Hello, world!".to_owned().to_vec()`
   = note: expected struct `Vec<u8>`
   = note: expected struct `Vec<u8>`
               found array `[u8; 13]`
error[E0308]: mismatched types
  --> src/mem/maybe_uninit.rs:425:13
   |
   |
12 |     hello = b"Hello".to_owned();
   |             ^^^^^^^^^^^^^^^^^^^ expected `&mut Vec<u8>`, found array `[u8; 5]`
   = note: expected mutable reference `&mut Vec<u8>`
   = note: expected mutable reference `&mut Vec<u8>`
                          found array `[u8; 5]`

error[E0277]: can't compare `&[u8; 5]` with `Vec<u8>`
   |
   |
17 | assert_eq!(b"hello", s);
   | ^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&[u8; 5] == Vec<u8>`
   |
   = help: the trait `PartialEq<Vec<u8>>` is not implemented for `&[u8; 5]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:44

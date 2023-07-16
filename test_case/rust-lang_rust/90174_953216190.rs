plain
...............................iiiiii.................................i............................. 1200/1202
..
failures:

---- src/error.rs - error::Report (line 819) stdout ----
error[E0726]: implicit elided lifetime not allowed here
   |
   |
23 | impl !Send for SuperError {}
   |                ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...

error[E0726]: implicit elided lifetime not allowed here
   |
   |
24 | impl !Sync for SuperError {}
   |                ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...
error: aborting due to 2 previous errors

Couldn't compile the test.

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:24:55

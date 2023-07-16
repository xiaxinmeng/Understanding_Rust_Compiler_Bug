plain
---- src/num/nonzero.rs - num::nonzero::NonZero (line 33) stdout ----
error[E0599]: no function or associated item named `new` found for type `_` in the current scope
  --> src/num/nonzero.rs:47:18
   |
17 |         NonZero::new(ch).map(|ch| Separator { ch })
   |                  ^^^ function or associated item not found in `_`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
5  | use gimli::read::lookup::PubStuffEntry;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:18:30

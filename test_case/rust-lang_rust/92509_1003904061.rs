plain

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::partition (line 1763) stdout ----
warning: `-Z symbol-mangling-version` is deprecated; use `-C symbol-mangling-version`

error[E0369]: cannot mod `&&{integer}` by `{integer}`
  |
  |
8 |     .partition::<Vec<_>, _>(|n| n % 2 == 0);
  |                                 - ^ - {integer}
  |                                 &&{integer}
  |
  |
help: `%` can be used on `{integer}`, you can dereference `n`
  |
8 |     .partition::<Vec<_>, _>(|n| *n % 2 == 0);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:21:40

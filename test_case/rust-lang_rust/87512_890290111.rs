plain
     |
1744 |                         id: id,
     |                         ^^^^^^ help: replace it with: `id`
     |
     = note: `-D redundant-field-initializers` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `alloc`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:16:35

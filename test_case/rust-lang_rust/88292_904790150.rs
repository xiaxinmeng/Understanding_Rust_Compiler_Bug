plain
..................................
test result: ok. 434 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s

   Doc-tests alloc
error: the `-Z unstable-options` flag must also be passed to enable the flag `generate-link-to-definition`
error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:16:35

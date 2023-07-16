plain
    Checking object v0.22.0
error[E0283]: type annotations needed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/miniz_oxide-0.4.0/src/deflate/core.rs:1871:43
     |
1871 |         let far_and_small = cur_match_len == MIN_MATCH_LEN.into() && cur_match_dist >= 8 * 1024;
     |                                           ^^ -------------------- this method call resolves to `T`
     |                                           cannot infer type
     |
     |
     = note: cannot satisfy `u32: PartialEq<_>`
error[E0283]: type annotations needed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/miniz_oxide-0.4.0/src/deflate/core.rs:2037:43
     |
     |
2037 |                         || (cur_match_len == MIN_MATCH_LEN.into() && cur_match_dist >= 8 * 1024)
     |                                           ^^ -------------------- this method call resolves to `T`
     |                                           cannot infer type
     |
     |
     = note: cannot satisfy `u32: PartialEq<_>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
error: could not compile `miniz_oxide`
error: could not compile `miniz_oxide`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:23

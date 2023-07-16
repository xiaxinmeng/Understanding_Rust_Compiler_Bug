plain
   Compiling once_cell v1.7.2
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:63:9
   |
63 |         write_code(&mut html, src, Edition::Edition2018);
   |         |
   |         expected 4 arguments
   |
note: function defined here
---
87 |     src: &str,
   |     ---------
88 |     edition: Edition,
   |     ----------------
89 |     context_info: Option<ContextInfo<'_, '_, '_>>,

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:24:13

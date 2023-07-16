plain
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:25:13
   |
25 |             write_code(&mut out, src, Edition::Edition2018, None);
   |             |
   |             expected 5 arguments
   |
note: function defined here
---
92 |     src: &str,
   |     ---------
93 |     edition: Edition,
   |     ----------------
94 |     context_info: Option<ContextInfo<'_, '_, '_>>,
95 |     decoration_info: Option<DecorationInfo>,
   |     ---------------------------------------

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:39:9
   |
39 |         write_code(&mut html, src, Edition::Edition2018, None);
   |         |
   |         expected 5 arguments
   |
note: function defined here
---
92 |     src: &str,
   |     ---------
93 |     edition: Edition,
   |     ----------------
94 |     context_info: Option<ContextInfo<'_, '_, '_>>,
95 |     decoration_info: Option<DecorationInfo>,
   |     ---------------------------------------

error[E0061]: this function takes 5 arguments but 4 arguments were supplied
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:53:9
   |
53 |         write_code(&mut html, src, Edition::Edition2018, None);
   |         |
   |         expected 5 arguments
   |
note: function defined here
---
92 |     src: &str,
   |     ---------
93 |     edition: Edition,
   |     ----------------
94 |     context_info: Option<ContextInfo<'_, '_, '_>>,
95 |     decoration_info: Option<DecorationInfo>,
   |     ---------------------------------------

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 3 previous errors


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:24:18

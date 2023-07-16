plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this struct takes 7 arguments but 6 arguments were supplied
   --> src/librustdoc/html/markdown/tests.rs:151:13
    |
151 |             Markdown(input, &[], &mut map, ErrorCodes::Yes, DEFAULT_EDITION, &None).into_string();
    |             |
    |             expected 7 arguments
    |
note: tuple struct defined here
note: tuple struct defined here
   --> src/librustdoc/html/markdown.rs:72:12
    |
72  | pub struct Markdown<'a>(

error[E0061]: this struct takes 7 arguments but 6 arguments were supplied
   --> src/librustdoc/html/markdown/tests.rs:185:13
    |
    |
185 |             Markdown(input, &[], map, ErrorCodes::Yes, DEFAULT_EDITION, &None).into_string();
    |             |
    |             expected 7 arguments
    |
note: tuple struct defined here
note: tuple struct defined here
   --> src/librustdoc/html/markdown.rs:72:12
    |
72  | pub struct Markdown<'a>(

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 2 previous errors



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:24:24

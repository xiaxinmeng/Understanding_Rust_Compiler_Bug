plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> src/librustdoc/html/markdown/tests.rs:224:22
     |
224  |         let output = short_markdown_summary(input);
     |                      |
     |                      expected 2 arguments
     |
note: function defined here
note: function defined here
    --> src/librustdoc/html/markdown.rs:1141:10
     |
1141 | crate fn short_markdown_summary(markdown: &str, link_names: &[RenderedLink]) -> String {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:01

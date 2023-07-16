plain
   Compiling difference v2.0.0
   Compiling once_cell v1.7.2
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: couldn't read src/librustdoc/theme/../html/static/themes/dark.css: No such file or directory (os error 2)
    --> src/librustdoc/theme/tests.rs:108:16
     |
108  |       let text = include_str!("../html/static/themes/dark.css");
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
    ::: /checkout/library/core/src/macros/mod.rs:1105:5
     |
1105 | /     macro_rules! include_str {
1105 | /     macro_rules! include_str {
1106 | |         ($file:expr $(,)?) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `include_str!`

error: aborting due to previous error


error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:24:35

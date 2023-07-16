plain
   Compiling difference v2.0.0
   Compiling once_cell v1.7.2
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: `&str` is not an iterator
   --> src/librustdoc/html/highlight/tests.rs:59:9
    |
59  |         write_code(&mut html, src, Edition::Edition2018, None);
    |         ^^^^^^^^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
   ::: src/librustdoc/html/highlight.rs:111:4
    |
111 | fn write_code(
    |    ---------- required by a bound in this
    |    ---------- required by a bound in this
112 |     out: &mut Buffer,
113 |     src: impl Iterator<Item = Line<'a>>,
    |               ------------------------- required by this bound in `write_code`
    |
    = help: the trait `Iterator` is not implemented for `&str`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:24:45

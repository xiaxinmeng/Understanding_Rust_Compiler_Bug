plain
running 63 tests
............................................F..................
failures:

---- html::markdown::tests::test_short_markdown_summary stdout ----
error: test failed, to rerun pass '-p rustdoc --lib'
thread 'html::markdown::tests::test_short_markdown_summary' panicked at 'assertion failed: `(left == right)`
  left: `"&amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; …"`,
 right: `"&amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; &amp; …"`: original: & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & & &', src/librustdoc/html/markdown/tests.rs:225:9


failures:
    html::markdown::tests::test_short_markdown_summary
    html::markdown::tests::test_short_markdown_summary

test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:23:53

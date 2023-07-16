plain
    Finished release [optimized] target(s) in 27.90s
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-169b4e3f884f31b4)

running 76 tests
.............................................F.....F........................

---- html::markdown::tests::test_header_ids_multiple_blocks stdout ----
thread 'html::markdown::tests::test_header_ids_multiple_blocks' panicked at 'assertion failed: `(left == right)`
thread 'html::markdown::tests::test_header_ids_multiple_blocks' panicked at 'assertion failed: `(left == right)`
  left: `"<h2 id=\"main\" class=\"section-header\"><a href=\"#main\">Main</a></h2>"`,
 right: `"<h2 id=\"main-1\" class=\"section-header\"><a href=\"#main-1\">Main</a></h2>"`: original: # Main', src/librustdoc/html/markdown/tests.rs:202:9

---- html::markdown::tests::test_unique_id stdout ----
---- html::markdown::tests::test_unique_id stdout ----
thread 'html::markdown::tests::test_unique_id' panicked at 'assertion failed: `(left == right)`
  left: `["foo", "examples", "examples-1", "method.into_iter", "examples-2", "method.into_iter-1", "foo-1", "main", "search-1", "methods", "examples-3", "method.into_iter-2", "assoc_type.Item", "assoc_type.Item-1"]`,
 right: `["foo", "examples", "examples-1", "method.into_iter", "examples-2", "method.into_iter-1", "foo-1", "main-1", "search-1", "methods", "examples-3", "method.into_iter-2", "assoc_type.Item", "assoc_type.Item-1"]`', src/librustdoc/html/markdown/tests.rs:42:5

failures:
    html::markdown::tests::test_header_ids_multiple_blocks
    html::markdown::tests::test_unique_id
    html::markdown::tests::test_unique_id

test result: FAILED. 74 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:28:55

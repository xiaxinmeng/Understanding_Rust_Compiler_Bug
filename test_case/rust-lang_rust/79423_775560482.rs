plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.074 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.37s

 finished in 2.453 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 32.25s
     Running build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-c6465bc84f83bbc5

running 51 tests
............................F.F....F...............

---- html::markdown::tests::test_markdown_html_escape stdout ----
---- html::markdown::tests::test_markdown_html_escape stdout ----
thread 'html::markdown::tests::test_markdown_html_escape' panicked at 'assertion failed: `(left == right)`
  left: `"<p>Struct&lt;’a, T&gt;</p>\n"`,
 right: `"<p>Struct&lt;\'a, T&gt;</p>\n"`: original: Struct<'a, T>', src/librustdoc/html/markdown/tests.rs:249:9

---- html::markdown::tests::test_short_markdown_summary stdout ----
---- html::markdown::tests::test_short_markdown_summary stdout ----
thread 'html::markdown::tests::test_short_markdown_summary' panicked at 'assertion failed: `(left == right)`
  left: `"code <code>let x = i32;</code> …"`,
 right: `"code <code>let x = i32;</code> ..."`: original: code `let x = i32;` ...', src/librustdoc/html/markdown/tests.rs:194:9
---- html::markdown::tests::test_plain_text_summary stdout ----
---- html::markdown::tests::test_plain_text_summary stdout ----
thread 'html::markdown::tests::test_plain_text_summary' panicked at 'assertion failed: `(left == right)`
  left: `"code `let x = i32;` …"`,
 right: `"code `let x = i32;` ..."`: original: code `let x = i32;` ...', src/librustdoc/html/markdown/tests.rs:221:9

failures:
    html::markdown::tests::test_markdown_html_escape
    html::markdown::tests::test_plain_text_summary
    html::markdown::tests::test_plain_text_summary
    html::markdown::tests::test_short_markdown_summary

test result: FAILED. 48 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:32:19

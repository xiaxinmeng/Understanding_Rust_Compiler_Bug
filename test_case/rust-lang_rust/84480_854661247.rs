plain
Set({"src/test/rustdoc-js"}) not skipped for "bootstrap::test::RustdocJSNotStd" -- not in ["src/tools/tidy"]
Set({"src/tools/rustdoc-themes"}) not skipped for "bootstrap::test::RustdocTheme" -- not in ["src/tools/tidy"]
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Suite("src/test/rustdoc-json") not skipped for "bootstrap::test::RustdocJson" -- not in ["src/tools/tidy"]
Set({"src/tools/html-checker"}) not skipped for "bootstrap::test::HtmlCheck" -- not in ["src/tools/tidy"]
not running HTML-check tool because `tidy` is missing
Note that `tidy` is not the in-tree `src/tools/tidy` but needs to be installed
thread 'main' panicked at 'tidy needs to be run on CI!', src/bootstrap/test.rs:179:13
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00

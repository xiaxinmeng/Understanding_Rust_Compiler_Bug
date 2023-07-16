plain
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-js"}) not skipped for "bootstrap::test::RustdocJSNotStd" -- not in ["src/tools/tidy"]
Set({"src/test/rustdoc-gui"}) not skipped for "bootstrap::test::RustdocGUI" -- not in ["src/tools/tidy"]
thread 'main' panicked at 'npm should be present', src/bootstrap/test.rs:788:47
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:00:00

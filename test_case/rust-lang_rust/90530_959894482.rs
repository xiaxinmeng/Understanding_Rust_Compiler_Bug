plain
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
 finished in 1.234 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
/checkout/src/tools/rustdoc-js/tester.js:426
        if (Object.hasOwn(correspondences, args[i])) {


TypeError: Object.hasOwn is not a function
    at parseOptions (/checkout/src/tools/rustdoc-js/tester.js:426:20)
    at main (/checkout/src/tools/rustdoc-js/tester.js:462:16)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:487:14)
    at Module._compile (internal/modules/cjs/loader.js:778:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:789:10)


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.58.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"
    at Module.load (internal/modules/cjs/loader.js:653:32)
    at tryModuleLoad (internal/modules/cjs/loader.js:593:12)
    at Function.Module._load (internal/modules/cjs/loader.js:585:3)
    at Function.Module.runMain (internal/modules/cjs/loader.js:831:12)
expected success, got: exit status: 1
    at startup (internal/bootstrap/node.js:283:19)

Build completed unsuccessfully in 0:34:01

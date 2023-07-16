plain
Testing keyword.js ... OK
Testing macro-check.js ... OK
Testing macro-print.js ... OK
Testing never.js ... OK
/checkout/src/tools/rustdoc-js/tester.js:143
                result_v.forEach((value, index) => {


TypeError: result_v.forEach is not a function
    at valueCheck (/checkout/src/tools/rustdoc-js/tester.js:143:26)
    at runParser (/checkout/src/tools/rustdoc-js/tester.js:170:9)
    at /checkout/src/tools/rustdoc-js/tester.js:306:20
    at runCheck (/checkout/src/tools/rustdoc-js/tester.js:256:30)
    at runChecks (/checkout/src/tools/rustdoc-js/tester.js:305:16)
    at /checkout/src/tools/rustdoc-js/tester.js:426:23
    at Array.forEach (<anonymous>)
    at main (/checkout/src/tools/rustdoc-js/tester.js:421:45)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:433:14)
    at Module._compile (internal/modules/cjs/loader.js:999:30)

plain
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
 finished in 1.221 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
Suite("src/test/rustdoc-js-std") not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
tmp.js:2
function hasOwnPropertyRustdoc(obj,property){return Object.prototype.hasOwnProperty.call(obj,property)}exports.hasOwnPropertyRustdoc = hasOwnPropertyRustdoc;function onEach(arr,func,reversed){if(arr&&arr.length>0&&func){var length=arr.length;var i;if(reversed){for(i=length-1;i>=0;--i){if(func(arr[i])){return true}}}else{for(i=0;i<length;++i){if(func(arr[i])){return true}}}}return false}exports.onEach = onEach;var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias",];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var NO_TYPE_FILTER=-1;exports.NO_TYPE_FILTER = NO_TYPE_FILTER;var GENERICS_DATA=2;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_D

ReferenceError: tokenizeQuery is not defined
    at Object.execSearch (tmp.js:2:11499)
    at runSearch (/checkout/src/tools/rustdoc-js/tester.js:294:26)
    at runChecks (/checkout/src/tools/rustdoc-js/tester.js:386:22)
    at checkFile (/checkout/src/tools/rustdoc-js/tester.js:463:12)
    at /checkout/src/tools/rustdoc-js/tester.js:487:23
    at Array.forEach (<anonymous>)
    at main (/checkout/src/tools/rustdoc-js/tester.js:483:45)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:493:14)
    at Module._compile (internal/modules/cjs/loader.js:778:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:789:10)


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.59.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


Build completed unsuccessfully in 0:34:30

plain
.................................................................................................... 9300/11545
.................................................................................................... 9400/11545
..........................................................................i......i.................. 9500/11545
.................................................................................................... 9600/11545
....................iiiiiii..iiiiii.i............................................................... 9700/11545
.................................................................................................... 9900/11545
.................................................................................................... 10000/11545
.................................................................................................... 10100/11545
.................................................................................................... 10200/11545
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.062 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.51s

 finished in 2.571 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
 finished in 0.888 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
tmp.js:2
function hasOwnProperty(obj,property){return Object.prototype.hasOwnProperty.call(obj,property)}exports.hasOwnProperty = hasOwnProperty;function onEach(arr,func,reversed){if(arr&&arr.length>0&&func){var length=arr.length;var i;if(reversed!==true){for(i=0;i<length;++i){if(func(arr[i])===true){return true}}}else{for(i=length-1;i>=0;--i){if(func(arr[i])===true){return true}}}}return false}exports.onEach = onEach;var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var NO_TYPE_FILTER=-1;exports.NO_TYPE_FILTER = NO_TYPE_FILTER;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DA

ReferenceError: generateId is not defined
    at Object.buildIndex (tmp.js:2:5453)
    at loadMainJsAndIndex (/checkout/src/tools/rustdoc-js/tester.js:277:24)
    at load_files (/checkout/src/tools/rustdoc-js/tester.js:389:12)
    at main (/checkout/src/tools/rustdoc-js/tester.js:464:27)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:484:14)
    at Module._compile (module.js:652:30)
    at Object.Module._extensions..js (module.js:663:10)
    at Module.load (module.js:565:32)
    at tryModuleLoad (module.js:505:12)
    at Function.Module._load (module.js:497:3)


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.52.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:31:53

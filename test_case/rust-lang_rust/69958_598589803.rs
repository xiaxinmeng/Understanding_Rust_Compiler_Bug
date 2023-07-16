plain
2020-03-13T06:48:35.4242120Z test [ui] ui/hygiene/legacy_interaction.rs ... ok
2020-03-13T06:48:35.5164710Z test [ui] ui/hygiene/local_inner_macros.rs ... ok
2020-03-13T06:48:35.5474160Z test [ui] ui/hygiene/nested_macro_privacy.rs ... ok
2020-03-13T06:48:35.5812950Z test [ui] ui/hygiene/no_implicit_prelude-2018.rs ... ok
2020-03-13T06:48:35.6246440Z test [ui] ui/hygiene/macro-metavars-legacy.rs ... ok
2020-03-13T06:48:35.6556780Z test [ui] ui/hygiene/pattern-macro.rs ... ok
2020-03-13T06:48:35.6556780Z test [ui] ui/hygiene/pattern-macro.rs ... ok
2020-03-13T06:48:35.7112420Z test [ui] ui/hygiene/macro-metavars-transparent.rs ... ok
2020-03-13T06:48:35.7929870Z test [ui] ui/hygiene/privacy.rs ... ok
2020-03-13T06:48:35.8042430Z test [ui] ui/hygiene/rustc-macro-transparency.rs ... ok
2020-03-13T06:48:35.8892160Z test [ui] ui/hygiene/prelude-import-hygiene.rs#rust2015 ... ok
2020-03-13T06:48:35.9066980Z test [ui] ui/hygiene/prelude-import-hygiene.rs#rust2018 ... ok
---
2020-03-13T07:32:21.8176090Z ---- [js-doc-test] rustdoc-js/basic.rs stdout ----
2020-03-13T07:32:21.8176620Z 
2020-03-13T07:32:21.8177450Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8180870Z status: exit code: 1
2020-03-13T07:32:21.8183020Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "basic"
2020-03-13T07:32:21.8201110Z ------------------------------------------
2020-03-13T07:32:21.8201790Z Checking "basic" ... 
2020-03-13T07:32:21.8202210Z 
2020-03-13T07:32:21.8203080Z ------------------------------------------
2020-03-13T07:32:21.8203080Z ------------------------------------------
2020-03-13T07:32:21.8203650Z stderr:
2020-03-13T07:32:21.8204490Z ------------------------------------------
2020-03-13T07:32:21.8205060Z tmp.js:5
2020-03-13T07:32:21.8207570Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8210470Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8210470Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8211070Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8211630Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8212190Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8213320Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8214630Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8215440Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8216150Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8216830Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8217480Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8218210Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8220400Z ------------------------------------------
2020-03-13T07:32:21.8220700Z 
2020-03-13T07:32:21.8220870Z 
2020-03-13T07:32:21.8223950Z ---- [js-doc-test] rustdoc-js/module-substring.rs stdout ----
2020-03-13T07:32:21.8223950Z ---- [js-doc-test] rustdoc-js/module-substring.rs stdout ----
2020-03-13T07:32:21.8224470Z 
2020-03-13T07:32:21.8225780Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8227420Z status: exit code: 1
2020-03-13T07:32:21.8230480Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "module-substring"
2020-03-13T07:32:21.8232440Z ------------------------------------------
2020-03-13T07:32:21.8235600Z Checking "module-substring" ... 
2020-03-13T07:32:21.8236250Z 
2020-03-13T07:32:21.8251700Z ------------------------------------------
2020-03-13T07:32:21.8251700Z ------------------------------------------
2020-03-13T07:32:21.8251980Z stderr:
2020-03-13T07:32:21.8252970Z ------------------------------------------
2020-03-13T07:32:21.8253250Z tmp.js:5
2020-03-13T07:32:21.8258850Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8263100Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8263100Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8263460Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8263790Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8264120Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8265100Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8266070Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8267410Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8267870Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8268310Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8268730Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8269200Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8270170Z ------------------------------------------
2020-03-13T07:32:21.8270380Z 
2020-03-13T07:32:21.8270490Z 
2020-03-13T07:32:21.8271160Z ---- [js-doc-test] rustdoc-js/exact-match.rs stdout ----
2020-03-13T07:32:21.8271160Z ---- [js-doc-test] rustdoc-js/exact-match.rs stdout ----
2020-03-13T07:32:21.8271410Z 
2020-03-13T07:32:21.8271960Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8272230Z status: exit code: 1
2020-03-13T07:32:21.8273300Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "exact-match"
2020-03-13T07:32:21.8274460Z ------------------------------------------
2020-03-13T07:32:21.8275060Z Checking "exact-match" ... 
2020-03-13T07:32:21.8275240Z 
2020-03-13T07:32:21.8275780Z ------------------------------------------
2020-03-13T07:32:21.8275780Z ------------------------------------------
2020-03-13T07:32:21.8276040Z stderr:
2020-03-13T07:32:21.8277200Z ------------------------------------------
2020-03-13T07:32:21.8277800Z tmp.js:5
2020-03-13T07:32:21.8295520Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8304600Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8304600Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8304970Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8306110Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8348300Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8349680Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8363040Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8363600Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8364040Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8364480Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8364900Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8365390Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8366300Z ------------------------------------------
2020-03-13T07:32:21.8366510Z 
2020-03-13T07:32:21.8366640Z 
2020-03-13T07:32:21.8367250Z ---- [js-doc-test] rustdoc-js/search-short-types.rs stdout ----
2020-03-13T07:32:21.8367250Z ---- [js-doc-test] rustdoc-js/search-short-types.rs stdout ----
2020-03-13T07:32:21.8367520Z 
2020-03-13T07:32:21.8384960Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8385290Z status: exit code: 1
2020-03-13T07:32:21.8387040Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "search-short-types"
2020-03-13T07:32:21.8388450Z ------------------------------------------
2020-03-13T07:32:21.8389100Z Checking "search-short-types" ... 
2020-03-13T07:32:21.8389300Z 
2020-03-13T07:32:21.8390090Z ------------------------------------------
2020-03-13T07:32:21.8390090Z ------------------------------------------
2020-03-13T07:32:21.8390360Z stderr:
2020-03-13T07:32:21.8390980Z ------------------------------------------
2020-03-13T07:32:21.8391250Z tmp.js:5
2020-03-13T07:32:21.8393570Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8396410Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8396410Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8396740Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8397060Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8397390Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8398280Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8399410Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8400010Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8400460Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8400900Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8401320Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8401800Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8402710Z ------------------------------------------
2020-03-13T07:32:21.8402920Z 
2020-03-13T07:32:21.8403030Z 
2020-03-13T07:32:21.8404390Z ---- [js-doc-test] rustdoc-js/struct-like-variant.rs stdout ----
2020-03-13T07:32:21.8404390Z ---- [js-doc-test] rustdoc-js/struct-like-variant.rs stdout ----
2020-03-13T07:32:21.8404690Z 
2020-03-13T07:32:21.8405320Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8405590Z status: exit code: 1
2020-03-13T07:32:21.8406680Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "struct-like-variant"
2020-03-13T07:32:21.8407850Z ------------------------------------------
2020-03-13T07:32:21.8408490Z Checking "struct-like-variant" ... 
2020-03-13T07:32:21.8408690Z 
2020-03-13T07:32:21.8409250Z ------------------------------------------
2020-03-13T07:32:21.8409250Z ------------------------------------------
2020-03-13T07:32:21.8409500Z stderr:
2020-03-13T07:32:21.8410070Z ------------------------------------------
2020-03-13T07:32:21.8410320Z tmp.js:5
2020-03-13T07:32:21.8412550Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8414960Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8414960Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8415290Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8415610Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8415920Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8416750Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8417700Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8418210Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8418660Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8419330Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8419760Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8420220Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8421140Z ------------------------------------------
2020-03-13T07:32:21.8421370Z 
2020-03-13T07:32:21.8421480Z 
2020-03-13T07:32:21.8422080Z ---- [js-doc-test] rustdoc-js/substring.rs stdout ----
2020-03-13T07:32:21.8422080Z ---- [js-doc-test] rustdoc-js/substring.rs stdout ----
2020-03-13T07:32:21.8422330Z 
2020-03-13T07:32:21.8422840Z error: rustdoc-js test failed!
2020-03-13T07:32:21.8423120Z status: exit code: 1
2020-03-13T07:32:21.8424170Z command: "/usr/local/bin/node" "/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "substring"
2020-03-13T07:32:21.8425330Z ------------------------------------------
2020-03-13T07:32:21.8425620Z Checking "substring" ... 
2020-03-13T07:32:21.8425800Z 
2020-03-13T07:32:21.8426360Z ------------------------------------------
2020-03-13T07:32:21.8426360Z ------------------------------------------
2020-03-13T07:32:21.8426620Z stderr:
2020-03-13T07:32:21.8427180Z ------------------------------------------
2020-03-13T07:32:21.8427450Z tmp.js:5
2020-03-13T07:32:21.8429980Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-13T07:32:21.8432510Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8432510Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-13T07:32:21.8432850Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-13T07:32:21.8433170Z     at execQuery (tmp.js:5:4447)
2020-03-13T07:32:21.8433490Z     at Object.execSearch (tmp.js:5:16992)
2020-03-13T07:32:21.8434370Z     at main (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:290:30)
2020-03-13T07:32:21.8435330Z     at Object.<anonymous> (/Users/runner/runners/2.165.1/work/1/s/src/tools/rustdoc-js/tester.js:334:14)
2020-03-13T07:32:21.8435850Z     at Module._compile (internal/modules/cjs/loader.js:1158:30)
2020-03-13T07:32:21.8436280Z     at Object.Module._extensions..js (internal/modules/cjs/loader.js:1178:10)
2020-03-13T07:32:21.8436730Z     at Module.load (internal/modules/cjs/loader.js:1002:32)
2020-03-13T07:32:21.8499340Z     at Function.Module._load (internal/modules/cjs/loader.js:901:14)
2020-03-13T07:32:21.8499910Z     at Function.executeUserEntryPoint [as runMain] (internal/modules/run_main.js:74:12)
2020-03-13T07:32:21.8501240Z ------------------------------------------
2020-03-13T07:32:21.8501450Z 
2020-03-13T07:32:21.8501570Z 
2020-03-13T07:32:21.8501690Z 
---
2020-03-13T07:32:21.8508260Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T07:32:21.8508770Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T07:32:21.8509050Z 
2020-03-13T07:32:21.8509170Z 
2020-03-13T07:32:21.8515260Z command did not execute successfully: "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/runner/runners/2.165.1/work/1/s/src/test/rustdoc-js" "--build-base" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-js" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "js-doc-test" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1100.0.30.12\nApple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15)\n" "--lldb-python-dir" "/Applications/Xcode_11.3.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T07:32:21.8519540Z 
2020-03-13T07:32:21.8519660Z 
2020-03-13T07:32:21.8519990Z failed to run: /Users/runner/runners/2.165.1/work/1/s/build/bootstrap/debug/bootstrap test
2020-03-13T07:32:21.8520430Z Build completed unsuccessfully in 1:29:47
2020-03-13T07:32:21.8520430Z Build completed unsuccessfully in 1:29:47
2020-03-13T07:32:21.8520720Z == clock drift check ==
2020-03-13T07:32:21.8521010Z   local time: Fri Mar 13 07:32:21 UTC 2020
2020-03-13T07:32:21.8760850Z   network time: Fri, 13 Mar 2020 07:32:21 GMT
2020-03-13T07:32:21.8862150Z == end clock drift check ==
2020-03-13T07:32:21.8936170Z 
2020-03-13T07:32:21.9094640Z ##[error]Bash exited with code '1'.
2020-03-13T07:32:21.9231250Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-13T07:32:21.9238220Z ==============================================================================
2020-03-13T07:32:21.9238630Z Task         : Get sources
2020-03-13T07:32:21.9239040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

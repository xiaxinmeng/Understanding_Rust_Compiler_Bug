plain
2020-03-16T17:41:41.9727720Z ========================== Starting Command Output ===========================
2020-03-16T17:41:41.9730187Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e452c3f-7b23-4b77-834b-1e8e5ccc811f.sh
2020-03-16T17:41:41.9730477Z 
2020-03-16T17:41:41.9735139Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T17:41:41.9753358Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69402/merge to s
2020-03-16T17:41:41.9756427Z Task         : Get sources
2020-03-16T17:41:41.9756717Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T17:41:41.9757014Z Version      : 1.0.0
2020-03-16T17:41:41.9757203Z Author       : Microsoft
---
2020-03-16T17:41:43.2431116Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T17:41:43.2436892Z ##[command]git config gc.auto 0
2020-03-16T17:41:43.2440368Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T17:41:43.2443661Z ##[command]git config --get-all http.proxy
2020-03-16T17:41:43.2450631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69402/merge:refs/remotes/pull/69402/merge
---
2020-03-16T18:37:38.5574688Z .................................................................................................... 1700/9774
2020-03-16T18:37:42.7545655Z .................................................................................................... 1800/9774
2020-03-16T18:37:53.4751479Z ......................................................................i............................. 1900/9774
2020-03-16T18:37:59.4311150Z .................................................................................................... 2000/9774
2020-03-16T18:38:13.1026128Z ............................................................iiiii................................... 2100/9774
2020-03-16T18:38:23.1541450Z .................................................................................................... 2300/9774
2020-03-16T18:38:25.1966190Z .................................................................................................... 2400/9774
2020-03-16T18:38:27.9754004Z .................................................................................................... 2500/9774
2020-03-16T18:38:46.9968437Z .................................................................................................... 2600/9774
---
2020-03-16T18:41:17.0144458Z ................................i...............i................................................... 5000/9774
2020-03-16T18:41:25.8534253Z .................................................................................................... 5100/9774
2020-03-16T18:41:32.0113713Z ...........................................................................i........................ 5200/9774
2020-03-16T18:41:37.5337398Z .................................................................................................... 5300/9774
2020-03-16T18:41:47.2646973Z ........................................................ii.ii........i...i.......................... 5400/9774
2020-03-16T18:41:55.3804662Z .................................................................................................... 5600/9774
2020-03-16T18:42:04.5513876Z .................................................................................................... 5700/9774
2020-03-16T18:42:10.6045107Z ................................................i................................................... 5800/9774
2020-03-16T18:42:17.2664299Z .................................................................................................... 5900/9774
2020-03-16T18:42:17.2664299Z .................................................................................................... 5900/9774
2020-03-16T18:42:26.7488824Z .................................................................................................... 6000/9774
2020-03-16T18:42:32.8636795Z ..........................................ii...i..ii...........i.................................... 6100/9774
2020-03-16T18:42:51.0785656Z .................................................................................................... 6300/9774
2020-03-16T18:42:54.4646787Z .................................................................................................... 6400/9774
2020-03-16T18:42:54.4646787Z .................................................................................................... 6400/9774
2020-03-16T18:42:59.0021478Z ........................................................................i..ii....................... 6500/9774
2020-03-16T18:43:19.6738963Z .................................................................................................... 6700/9774
2020-03-16T18:43:28.3458922Z ......................................................................i............................. 6800/9774
2020-03-16T18:43:30.2613525Z .................................................................................................... 6900/9774
2020-03-16T18:43:32.3611354Z .................................................................................................... 7000/9774
---
2020-03-16T18:45:08.2601874Z .................................................................................................... 7800/9774
2020-03-16T18:45:13.8403917Z .................................................................................................... 7900/9774
2020-03-16T18:45:19.1968116Z ......................................................i............................................. 8000/9774
2020-03-16T18:45:28.8515479Z .................................................................................................... 8100/9774
2020-03-16T18:45:33.9286438Z ...iiiiiiiiii.i..................................................................................... 8200/9774
2020-03-16T18:45:46.7360742Z .................................................................................................... 8400/9774
2020-03-16T18:45:56.5572217Z .................................................................................................... 8500/9774
2020-03-16T18:46:09.3207575Z .................................................................................................... 8600/9774
2020-03-16T18:46:14.2989309Z .................................................................................................... 8700/9774
---
2020-03-16T18:48:22.7474250Z  finished in 6.995
2020-03-16T18:48:22.7658380Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:48:22.9200518Z 
2020-03-16T18:48:22.9201753Z running 183 tests
2020-03-16T18:48:25.5468486Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-16T18:48:27.9803443Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-16T18:48:27.9807461Z 
2020-03-16T18:48:27.9813397Z  finished in 5.215
2020-03-16T18:48:28.0008100Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:48:28.1525120Z 
---
2020-03-16T18:48:29.9555359Z  finished in 1.955
2020-03-16T18:48:29.9748087Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:48:30.1237497Z 
2020-03-16T18:48:30.1237989Z running 9 tests
2020-03-16T18:48:30.1244560Z iiiiiiiii
2020-03-16T18:48:30.1245626Z 
2020-03-16T18:48:30.1248349Z  finished in 0.150
2020-03-16T18:48:30.1438936Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:48:30.2992089Z 
---
2020-03-16T18:48:49.0580578Z  finished in 18.913
2020-03-16T18:48:49.0771960Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:48:49.2292182Z 
2020-03-16T18:48:49.2293286Z running 115 tests
2020-03-16T18:49:01.6907407Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-16T18:49:03.2281476Z ...iiii.....ii.
2020-03-16T18:49:03.2284031Z 
2020-03-16T18:49:03.2284336Z  finished in 14.151
2020-03-16T18:49:03.2290160Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T18:49:03.2291041Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-03-16T19:00:46.8569719Z 
2020-03-16T19:00:46.8571586Z    Doc-tests core
2020-03-16T19:00:51.0280597Z 
2020-03-16T19:00:51.0281626Z running 2481 tests
2020-03-16T19:00:59.3126421Z ......iiiii......................................................................................... 100/2481
2020-03-16T19:01:07.4150924Z .....................................................................................ii............. 200/2481
2020-03-16T19:01:25.9725769Z ....................i............................................................................... 400/2481
2020-03-16T19:01:25.9725769Z ....................i............................................................................... 400/2481
2020-03-16T19:01:34.8004329Z .........................................................................i..i..................iiii. 500/2481
2020-03-16T19:01:49.7551175Z .................................................................................................... 700/2481
2020-03-16T19:01:57.5349868Z .................................................................................................... 800/2481
2020-03-16T19:02:05.3205661Z .................................................................................................... 900/2481
2020-03-16T19:02:13.0796981Z .................................................................................................... 1000/2481
---
2020-03-16T19:05:27.6216551Z 
2020-03-16T19:05:27.6216748Z running 1010 tests
2020-03-16T19:05:44.0537954Z i................................................................................................... 100/1010
2020-03-16T19:05:53.5559340Z .................................................................................................... 200/1010
2020-03-16T19:06:00.2472116Z ..................iii......i......i...i......i...................................................... 300/1010
2020-03-16T19:06:05.0530202Z .................................................................................................... 400/1010
2020-03-16T19:06:11.5219687Z ............................................i..i......................................ii............ 500/1010
2020-03-16T19:06:23.3818986Z .................................................................................................... 700/1010
2020-03-16T19:06:23.3818986Z .................................................................................................... 700/1010
2020-03-16T19:06:29.8023481Z ....................................iiii............................................................ 800/1010
2020-03-16T19:06:42.9281131Z .................................................................................................... 900/1010
2020-03-16T19:06:49.2667422Z ..........................................................iiii...................................... 1000/1010
2020-03-16T19:06:49.6918411Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-16T19:06:49.6919027Z 
2020-03-16T19:06:49.7033050Z  finished in 154.240
2020-03-16T19:06:49.7045220Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-16T19:22:43.0733660Z  finished in 35.728
2020-03-16T19:22:43.1004901Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T19:22:43.2526879Z 
2020-03-16T19:22:43.2527144Z running 210 tests
2020-03-16T19:23:10.8871184Z ......................i...ii.......................................................................i 100/210
2020-03-16T19:23:39.9048326Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-16T19:23:44.1437148Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-03-16T19:23:44.1437402Z 
2020-03-16T19:23:44.1442758Z  finished in 61.043
2020-03-16T19:23:44.1450689Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-03-16T19:24:02.9013715Z ---- [js-doc-test] rustdoc-js/basic.rs stdout ----
2020-03-16T19:24:02.9013897Z 
2020-03-16T19:24:02.9014235Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9014443Z status: exit code: 1
2020-03-16T19:24:02.9015174Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/basic.js"
2020-03-16T19:24:02.9015965Z ------------------------------------------
2020-03-16T19:24:02.9016188Z Checking "basic" ... 
2020-03-16T19:24:02.9016313Z 
2020-03-16T19:24:02.9016649Z ------------------------------------------
2020-03-16T19:24:02.9016649Z ------------------------------------------
2020-03-16T19:24:02.9016854Z stderr:
2020-03-16T19:24:02.9017201Z ------------------------------------------
2020-03-16T19:24:02.9017394Z tmp.js:5
2020-03-16T19:24:02.9019137Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9021023Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9021023Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9022861Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9023225Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9023484Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9024096Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9024654Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9024976Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9025279Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9025750Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9026083Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9026363Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9026914Z ------------------------------------------
2020-03-16T19:24:02.9027072Z 
2020-03-16T19:24:02.9027163Z 
2020-03-16T19:24:02.9027548Z ---- [js-doc-test] rustdoc-js/exact-match.rs stdout ----
2020-03-16T19:24:02.9027548Z ---- [js-doc-test] rustdoc-js/exact-match.rs stdout ----
2020-03-16T19:24:02.9027729Z 
2020-03-16T19:24:02.9028037Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9028242Z status: exit code: 1
2020-03-16T19:24:02.9028966Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/exact-match.js"
2020-03-16T19:24:02.9029729Z ------------------------------------------
2020-03-16T19:24:02.9030092Z Checking "exact-match" ... 
2020-03-16T19:24:02.9030228Z 
2020-03-16T19:24:02.9030560Z ------------------------------------------
2020-03-16T19:24:02.9030560Z ------------------------------------------
2020-03-16T19:24:02.9030765Z stderr:
2020-03-16T19:24:02.9031101Z ------------------------------------------
2020-03-16T19:24:02.9031293Z tmp.js:5
2020-03-16T19:24:02.9033222Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9035112Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9035112Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9035378Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9035646Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9035901Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9036399Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9036943Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9037264Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9037568Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9037879Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9038143Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9038427Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9038964Z ------------------------------------------
2020-03-16T19:24:02.9039122Z 
2020-03-16T19:24:02.9039214Z 
2020-03-16T19:24:02.9039603Z ---- [js-doc-test] rustdoc-js/module-substring.rs stdout ----
2020-03-16T19:24:02.9039603Z ---- [js-doc-test] rustdoc-js/module-substring.rs stdout ----
2020-03-16T19:24:02.9039794Z 
2020-03-16T19:24:02.9040097Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9040301Z status: exit code: 1
2020-03-16T19:24:02.9041046Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/module-substring.js"
2020-03-16T19:24:02.9041817Z ------------------------------------------
2020-03-16T19:24:02.9042191Z Checking "module-substring" ... 
2020-03-16T19:24:02.9042338Z 
2020-03-16T19:24:02.9042666Z ------------------------------------------
2020-03-16T19:24:02.9042666Z ------------------------------------------
2020-03-16T19:24:02.9042869Z stderr:
2020-03-16T19:24:02.9043208Z ------------------------------------------
2020-03-16T19:24:02.9043460Z tmp.js:5
2020-03-16T19:24:02.9045229Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9047111Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9047111Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9047376Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9047642Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9047898Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9048389Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9048931Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9049250Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9049555Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9049864Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9050131Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9050412Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9050948Z ------------------------------------------
2020-03-16T19:24:02.9051108Z 
2020-03-16T19:24:02.9051203Z 
2020-03-16T19:24:02.9051599Z ---- [js-doc-test] rustdoc-js/search-short-types.rs stdout ----
2020-03-16T19:24:02.9051599Z ---- [js-doc-test] rustdoc-js/search-short-types.rs stdout ----
2020-03-16T19:24:02.9051793Z 
2020-03-16T19:24:02.9052096Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9052300Z status: exit code: 1
2020-03-16T19:24:02.9053257Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/search-short-types.js"
2020-03-16T19:24:02.9054081Z ------------------------------------------
2020-03-16T19:24:02.9054508Z Checking "search-short-types" ... 
2020-03-16T19:24:02.9054669Z 
2020-03-16T19:24:02.9055021Z ------------------------------------------
2020-03-16T19:24:02.9055021Z ------------------------------------------
2020-03-16T19:24:02.9055237Z stderr:
2020-03-16T19:24:02.9055602Z ------------------------------------------
2020-03-16T19:24:02.9055812Z tmp.js:5
2020-03-16T19:24:02.9057672Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9059744Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9059744Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9060073Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9060361Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9060640Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9061172Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9061761Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9062110Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9062438Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9062884Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9063151Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9063431Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9063959Z ------------------------------------------
2020-03-16T19:24:02.9064118Z 
2020-03-16T19:24:02.9064208Z 
2020-03-16T19:24:02.9064594Z ---- [js-doc-test] rustdoc-js/struct-like-variant.rs stdout ----
2020-03-16T19:24:02.9064594Z ---- [js-doc-test] rustdoc-js/struct-like-variant.rs stdout ----
2020-03-16T19:24:02.9064808Z 
2020-03-16T19:24:02.9065113Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9065317Z status: exit code: 1
2020-03-16T19:24:02.9066065Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/struct-like-variant.js"
2020-03-16T19:24:02.9066825Z ------------------------------------------
2020-03-16T19:24:02.9067218Z Checking "struct-like-variant" ... 
2020-03-16T19:24:02.9067367Z 
2020-03-16T19:24:02.9067694Z ------------------------------------------
2020-03-16T19:24:02.9067694Z ------------------------------------------
2020-03-16T19:24:02.9067896Z stderr:
2020-03-16T19:24:02.9068235Z ------------------------------------------
2020-03-16T19:24:02.9068431Z tmp.js:5
2020-03-16T19:24:02.9070157Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9072153Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9072153Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9072421Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9072691Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9072953Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9073444Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9073991Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9074312Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9074618Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9074926Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9075189Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9075471Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9076004Z ------------------------------------------
2020-03-16T19:24:02.9076164Z 
2020-03-16T19:24:02.9076255Z 
2020-03-16T19:24:02.9076614Z ---- [js-doc-test] rustdoc-js/substring.rs stdout ----
2020-03-16T19:24:02.9076614Z ---- [js-doc-test] rustdoc-js/substring.rs stdout ----
2020-03-16T19:24:02.9076810Z 
2020-03-16T19:24:02.9077351Z error: rustdoc-js test failed!
2020-03-16T19:24:02.9077634Z status: exit code: 1
2020-03-16T19:24:02.9078430Z command: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "/checkout/src/test/rustdoc-js/substring.js"
2020-03-16T19:24:02.9079178Z ------------------------------------------
2020-03-16T19:24:02.9079547Z Checking "substring" ... 
2020-03-16T19:24:02.9079681Z 
2020-03-16T19:24:02.9080022Z ------------------------------------------
2020-03-16T19:24:02.9080022Z ------------------------------------------
2020-03-16T19:24:02.9080223Z stderr:
2020-03-16T19:24:02.9080563Z ------------------------------------------
2020-03-16T19:24:02.9080756Z tmp.js:5
2020-03-16T19:24:02.9082491Z var itemTypes=["mod","externcrate","import","struct","enum","fn","type","static","trait","impl","tymethod","method","structfield","variant","macro","primitive","associatedtype","constant","associatedconstant","union","foreigntype","keyword","existential","attr","derive","traitalias"];exports.itemTypes = itemTypes;var MAX_LEV_DISTANCE=3;exports.MAX_LEV_DISTANCE = MAX_LEV_DISTANCE;var MAX_RESULTS=200;exports.MAX_RESULTS = MAX_RESULTS;var GENERICS_DATA=1;exports.GENERICS_DATA = GENERICS_DATA;var NAME=0;exports.NAME = NAME;var INPUTS_DATA=0;exports.INPUTS_DATA = INPUTS_DATA;var OUTPUT_DATA=1;exports.OUTPUT_DATA = OUTPUT_DATA;var TY_PRIMITIVE=itemTypes.indexOf("primitive");exports.TY_PRIMITIVE = TY_PRIMITIVE;var TY_KEYWORD=itemTypes.indexOf("keyword");exports.TY_KEYWORD = TY_KEYWORD;var levenshtein_row2=[];exports.levenshtein_row2 = levenshtein_row2;function buildHrefAndPath(item){var displayPath;var href;var type=itemTypes[item.ty];var name=item.name;var path=item.path;if(type==="mod"){displayPath=path
2020-03-16T19:24:02.9084370Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9084370Z ReferenceError: NO_TYPE_FILTER is not defined
2020-03-16T19:24:02.9084636Z     at itemTypeFromName (tmp.js:5:4362)
2020-03-16T19:24:02.9084903Z     at execQuery (tmp.js:5:4447)
2020-03-16T19:24:02.9085158Z     at Object.execSearch (tmp.js:5:16992)
2020-03-16T19:24:02.9085641Z     at main (/checkout/src/tools/rustdoc-js/tester.js:290:30)
2020-03-16T19:24:02.9086330Z     at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:334:14)
2020-03-16T19:24:02.9086655Z     at Module._compile (module.js:652:30)
2020-03-16T19:24:02.9086960Z     at Object.Module._extensions..js (module.js:663:10)
2020-03-16T19:24:02.9087272Z     at Module.load (module.js:565:32)
2020-03-16T19:24:02.9087536Z     at tryModuleLoad (module.js:505:12)
2020-03-16T19:24:02.9087933Z     at Function.Module._load (module.js:497:3)
2020-03-16T19:24:02.9088479Z ------------------------------------------
2020-03-16T19:24:02.9088638Z 
2020-03-16T19:24:02.9088729Z 
2020-03-16T19:24:02.9088819Z 
---
2020-03-16T19:24:02.9093475Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-16T19:24:02.9093888Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-16T19:24:02.9094122Z 
2020-03-16T19:24:02.9094219Z 
2020-03-16T19:24:02.9098370Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-js" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-js" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "js-doc-test" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-16T19:24:02.9101327Z 
2020-03-16T19:24:02.9101419Z 
2020-03-16T19:24:02.9102739Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-16T19:24:02.9103215Z Build completed unsuccessfully in 1:36:48
2020-03-16T19:24:02.9103215Z Build completed unsuccessfully in 1:36:48
2020-03-16T19:24:02.9122604Z == clock drift check ==
2020-03-16T19:24:02.9140549Z   local time: Mon Mar 16 19:24:02 UTC 2020
2020-03-16T19:24:03.2202440Z   network time: Mon, 16 Mar 2020 19:24:03 GMT
2020-03-16T19:24:03.2208660Z == end clock drift check ==
2020-03-16T19:24:05.6410675Z 
2020-03-16T19:24:05.6486684Z ##[error]Bash exited with code '1'.
2020-03-16T19:24:05.6499837Z ##[section]Finishing: Run build
2020-03-16T19:24:05.6555914Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69402/merge to s
2020-03-16T19:24:05.6560616Z Task         : Get sources
2020-03-16T19:24:05.6560953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T19:24:05.6561257Z Version      : 1.0.0
2020-03-16T19:24:05.6561465Z Author       : Microsoft
2020-03-16T19:24:05.6561465Z Author       : Microsoft
2020-03-16T19:24:05.6561804Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T19:24:05.6562182Z ==============================================================================
2020-03-16T19:24:05.9655869Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T19:24:05.9745357Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69402/merge to s
2020-03-16T19:24:05.9835008Z Cleaning up task key
2020-03-16T19:24:05.9836162Z Start cleaning up orphan processes.
2020-03-16T19:24:06.0004965Z Terminate orphan process: pid (4088) (python)
2020-03-16T19:24:06.0303079Z ##[section]Finishing: Finalize Job

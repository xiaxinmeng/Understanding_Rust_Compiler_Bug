plain
2019-10-04T19:47:30.4820778Z test [ui] ui/extern/extern-wrong-value-type.rs ... ok
2019-10-04T19:47:31.4115712Z test [ui] ui/extern/extern-vectorcall.rs ... ok
2019-10-04T19:47:31.4602186Z test [ui] ui/extern/external-doc-error.rs ... ok
2019-10-04T19:47:32.3380692Z test [ui] ui/extern/extern_fat_drop.rs ... ok
2019-10-04T19:47:33.0851134Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat ... FAILED
2019-10-04T19:47:34.7089839Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no ... FAILED
2019-10-04T19:47:35.7456337Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin ... FAILED
2019-10-04T19:47:36.4293487Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 ... FAILED
2019-10-04T19:47:37.3737263Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 ... FAILED
2019-10-04T19:47:38.9992495Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2 ... FAILED
2019-10-04T19:47:40.2021128Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3 ... FAILED
2019-10-04T19:47:40.9772023Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no0 ... FAILED
2019-10-04T19:47:42.0792252Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no1 ... FAILED
2019-10-04T19:47:43.6085415Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no2 ... FAILED
2019-10-04T19:47:45.2657755Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no3 ... FAILED
2019-10-04T19:47:46.8363524Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0 ... FAILED
2019-10-04T19:47:48.1511122Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1 ... FAILED
2019-10-04T19:47:50.3897845Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2 ... FAILED
2019-10-04T19:47:50.4219221Z test [ui] ui/extoption_env-no-args.rs ... ok
2019-10-04T19:47:51.8781309Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin3 ... FAILED
2019-10-04T19:47:51.9423535Z test [ui] ui/extoption_env-too-many-args.rs ... ok
2019-10-04T19:47:52.2103954Z test [ui] ui/extoption_env-not-defined.rs ... ok
2019-10-04T19:47:52.2610919Z test [ui] ui/fail-no-dead-code-core.rs ... ok
2019-10-04T19:47:52.3338168Z test [ui] ui/fail-no-dead-code.rs ... ok
---
2019-10-04T20:31:01.6925806Z test [ui] ui/zero-sized/zero-sized-vec-push.rs ... ok
2019-10-04T20:31:01.6925933Z 
2019-10-04T20:31:01.6926041Z failures:
2019-10-04T20:31:01.6971436Z 
2019-10-04T20:31:01.6972065Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat stdout ----
2019-10-04T20:31:01.6972136Z 
2019-10-04T20:31:01.6972218Z error in revision `fat`: test run failed!
2019-10-04T20:31:01.6973285Z status: exit code: 101
2019-10-04T20:31:01.6975212Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.fat/a.js"
2019-10-04T20:31:01.6975975Z ------------------------------------------
2019-10-04T20:31:01.6976028Z 
2019-10-04T20:31:01.6976310Z ------------------------------------------
2019-10-04T20:31:01.6976389Z stderr:
2019-10-04T20:31:01.6976389Z stderr:
2019-10-04T20:31:01.6976652Z ------------------------------------------
2019-10-04T20:31:01.6976704Z 
2019-10-04T20:31:01.6976957Z ------------------------------------------
2019-10-04T20:31:01.6977007Z 
2019-10-04T20:31:01.6977070Z 
2019-10-04T20:31:01.6977373Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no stdout ----
2019-10-04T20:31:01.6977439Z 
2019-10-04T20:31:01.6977521Z error in revision `no`: test run failed!
2019-10-04T20:31:01.6977594Z status: exit code: 101
2019-10-04T20:31:01.6978662Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.no/a.js"
2019-10-04T20:31:01.6979080Z ------------------------------------------
2019-10-04T20:31:01.6979148Z 
2019-10-04T20:31:01.6979389Z ------------------------------------------
2019-10-04T20:31:01.6979479Z stderr:
2019-10-04T20:31:01.6979479Z stderr:
2019-10-04T20:31:01.6979939Z ------------------------------------------
2019-10-04T20:31:01.6980027Z 
2019-10-04T20:31:01.6980311Z ------------------------------------------
2019-10-04T20:31:01.6980362Z 
2019-10-04T20:31:01.6980415Z 
2019-10-04T20:31:01.6980717Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin stdout ----
2019-10-04T20:31:01.6980927Z 
2019-10-04T20:31:01.6980994Z error in revision `thin`: test run failed!
2019-10-04T20:31:01.6981084Z status: exit code: 101
2019-10-04T20:31:01.6981633Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.thin/a.js"
2019-10-04T20:31:01.6981934Z ------------------------------------------
2019-10-04T20:31:01.6981989Z 
2019-10-04T20:31:01.6982177Z ------------------------------------------
2019-10-04T20:31:01.6982250Z stderr:
2019-10-04T20:31:01.6982250Z stderr:
2019-10-04T20:31:01.6982437Z ------------------------------------------
2019-10-04T20:31:01.6982665Z 
2019-10-04T20:31:01.6982862Z ------------------------------------------
2019-10-04T20:31:01.6982918Z 
2019-10-04T20:31:01.6982947Z 
2019-10-04T20:31:01.6983188Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----
2019-10-04T20:31:01.6983262Z 
2019-10-04T20:31:01.6983315Z error in revision `fat0`: test run failed!
2019-10-04T20:31:01.6983393Z status: exit code: 101
2019-10-04T20:31:01.6983723Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a.js"
2019-10-04T20:31:01.6984046Z ------------------------------------------
2019-10-04T20:31:01.6984088Z 
2019-10-04T20:31:01.6984282Z ------------------------------------------
2019-10-04T20:31:01.6984359Z stderr:
2019-10-04T20:31:01.6984359Z stderr:
2019-10-04T20:31:01.6984567Z ------------------------------------------
2019-10-04T20:31:01.6984610Z 
2019-10-04T20:31:01.6984810Z ------------------------------------------
2019-10-04T20:31:01.6984868Z 
2019-10-04T20:31:01.6984897Z 
2019-10-04T20:31:01.6985147Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 stdout ----
2019-10-04T20:31:01.6985199Z 
2019-10-04T20:31:01.6985252Z error in revision `fat1`: test run failed!
2019-10-04T20:31:01.6985335Z status: exit code: 101
2019-10-04T20:31:01.6985678Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/a.js"
2019-10-04T20:31:01.6985985Z ------------------------------------------
2019-10-04T20:31:01.6986178Z 
2019-10-04T20:31:01.6986380Z ------------------------------------------
2019-10-04T20:31:01.6986439Z stderr:
2019-10-04T20:31:01.6986439Z stderr:
2019-10-04T20:31:01.6986642Z ------------------------------------------
2019-10-04T20:31:01.6986682Z 
2019-10-04T20:31:01.6986882Z ------------------------------------------
2019-10-04T20:31:01.6986922Z 
2019-10-04T20:31:01.6986951Z 
2019-10-04T20:31:01.6987200Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2 stdout ----
2019-10-04T20:31:01.6987251Z 
2019-10-04T20:31:01.6987318Z error in revision `fat2`: test run failed!
2019-10-04T20:31:01.6987375Z status: exit code: 101
2019-10-04T20:31:01.6987721Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat2/a.js"
2019-10-04T20:31:01.6988194Z ------------------------------------------
2019-10-04T20:31:01.6988235Z 
2019-10-04T20:31:01.6988840Z ------------------------------------------
2019-10-04T20:31:01.6988921Z stderr:
2019-10-04T20:31:01.6988921Z stderr:
2019-10-04T20:31:01.6989175Z ------------------------------------------
2019-10-04T20:31:01.6989225Z 
2019-10-04T20:31:01.6989469Z ------------------------------------------
2019-10-04T20:31:01.6989521Z 
2019-10-04T20:31:01.6989557Z 
2019-10-04T20:31:01.6989974Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3 stdout ----
2019-10-04T20:31:01.6990046Z 
2019-10-04T20:31:01.6990128Z error in revision `fat3`: test run failed!
2019-10-04T20:31:01.6990201Z status: exit code: 101
2019-10-04T20:31:01.6990644Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat3/a.js"
2019-10-04T20:31:01.6991113Z ------------------------------------------
2019-10-04T20:31:01.6991163Z 
2019-10-04T20:31:01.6991411Z ------------------------------------------
2019-10-04T20:31:01.6991485Z stderr:
2019-10-04T20:31:01.6991485Z stderr:
2019-10-04T20:31:01.6991734Z ------------------------------------------
2019-10-04T20:31:01.6991784Z 
2019-10-04T20:31:01.6992156Z ------------------------------------------
2019-10-04T20:31:01.6992196Z 
2019-10-04T20:31:01.6992224Z 
2019-10-04T20:31:01.6992465Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no0 stdout ----
2019-10-04T20:31:01.6992516Z 
2019-10-04T20:31:01.6992589Z error in revision `no0`: test run failed!
2019-10-04T20:31:01.6992647Z status: exit code: 101
2019-10-04T20:31:01.6992981Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no0/a.js"
2019-10-04T20:31:01.6993284Z ------------------------------------------
2019-10-04T20:31:01.6993324Z 
2019-10-04T20:31:01.6993522Z ------------------------------------------
2019-10-04T20:31:01.6993596Z stderr:
2019-10-04T20:31:01.6993596Z stderr:
2019-10-04T20:31:01.6993779Z ------------------------------------------
2019-10-04T20:31:01.6993819Z 
2019-10-04T20:31:01.6994016Z ------------------------------------------
2019-10-04T20:31:01.6994055Z 
2019-10-04T20:31:01.6994098Z 
2019-10-04T20:31:01.6994322Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no1 stdout ----
2019-10-04T20:31:01.6994373Z 
2019-10-04T20:31:01.6994440Z error in revision `no1`: test run failed!
2019-10-04T20:31:01.6994519Z status: exit code: 101
2019-10-04T20:31:01.6994833Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no1/a.js"
2019-10-04T20:31:01.6995126Z ------------------------------------------
2019-10-04T20:31:01.6995188Z 
2019-10-04T20:31:01.6995375Z ------------------------------------------
2019-10-04T20:31:01.6995449Z stderr:
2019-10-04T20:31:01.6995449Z stderr:
2019-10-04T20:31:01.6995632Z ------------------------------------------
2019-10-04T20:31:01.6995686Z 
2019-10-04T20:31:01.6995868Z ------------------------------------------
2019-10-04T20:31:01.6995908Z 
2019-10-04T20:31:01.6995955Z 
2019-10-04T20:31:01.6996177Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no2 stdout ----
2019-10-04T20:31:01.6996241Z 
2019-10-04T20:31:01.6996292Z error in revision `no2`: test run failed!
2019-10-04T20:31:01.6996366Z status: exit code: 101
2019-10-04T20:31:01.6996685Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no2/a.js"
2019-10-04T20:31:01.6996980Z ------------------------------------------
2019-10-04T20:31:01.6997036Z 
2019-10-04T20:31:01.6997226Z ------------------------------------------
2019-10-04T20:31:01.6997300Z stderr:
2019-10-04T20:31:01.6997300Z stderr:
2019-10-04T20:31:01.6997484Z ------------------------------------------
2019-10-04T20:31:01.6997539Z 
2019-10-04T20:31:01.6997720Z ------------------------------------------
2019-10-04T20:31:01.6997774Z 
2019-10-04T20:31:01.6997802Z 
2019-10-04T20:31:01.6998201Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no3 stdout ----
2019-10-04T20:31:01.6998267Z 
2019-10-04T20:31:01.6998320Z error in revision `no3`: test run failed!
2019-10-04T20:31:01.6998755Z status: exit code: 101
2019-10-04T20:31:01.6999303Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no3/a.js"
2019-10-04T20:31:01.6999728Z ------------------------------------------
2019-10-04T20:31:01.6999779Z 
2019-10-04T20:31:01.7000009Z ------------------------------------------
2019-10-04T20:31:01.7000098Z stderr:
2019-10-04T20:31:01.7000098Z stderr:
2019-10-04T20:31:01.7000449Z ------------------------------------------
2019-10-04T20:31:01.7000500Z 
2019-10-04T20:31:01.7000733Z ------------------------------------------
2019-10-04T20:31:01.7000798Z 
2019-10-04T20:31:01.7000835Z 
2019-10-04T20:31:01.7001135Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0 stdout ----
2019-10-04T20:31:01.7001200Z 
2019-10-04T20:31:01.7001266Z error in revision `thin0`: test run failed!
2019-10-04T20:31:01.7001358Z status: exit code: 101
2019-10-04T20:31:01.7001774Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin0/a.js"
2019-10-04T20:31:01.7002151Z ------------------------------------------
2019-10-04T20:31:01.7002358Z 
2019-10-04T20:31:01.7002598Z ------------------------------------------
2019-10-04T20:31:01.7002670Z stderr:
2019-10-04T20:31:01.7002670Z stderr:
2019-10-04T20:31:01.7002909Z ------------------------------------------
2019-10-04T20:31:01.7002967Z 
2019-10-04T20:31:01.7003205Z ------------------------------------------
2019-10-04T20:31:01.7003254Z 
2019-10-04T20:31:01.7003289Z 
2019-10-04T20:31:01.7003580Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1 stdout ----
2019-10-04T20:31:01.7003641Z 
2019-10-04T20:31:01.7003720Z error in revision `thin1`: test run failed!
2019-10-04T20:31:01.7003790Z status: exit code: 101
2019-10-04T20:31:01.7004192Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin1/a.js"
2019-10-04T20:31:01.7005204Z ------------------------------------------
2019-10-04T20:31:01.7005322Z 
2019-10-04T20:31:01.7005696Z ------------------------------------------
2019-10-04T20:31:01.7005773Z stderr:
2019-10-04T20:31:01.7005773Z stderr:
2019-10-04T20:31:01.7006032Z ------------------------------------------
2019-10-04T20:31:01.7006082Z 
2019-10-04T20:31:01.7006551Z ------------------------------------------
2019-10-04T20:31:01.7006609Z 
2019-10-04T20:31:01.7006644Z 
2019-10-04T20:31:01.7006938Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2 stdout ----
2019-10-04T20:31:01.7006999Z 
2019-10-04T20:31:01.7007078Z error in revision `thin2`: test run failed!
2019-10-04T20:31:01.7007148Z status: exit code: 101
2019-10-04T20:31:01.7007549Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin2/a.js"
2019-10-04T20:31:01.7008093Z ------------------------------------------
2019-10-04T20:31:01.7008141Z 
2019-10-04T20:31:01.7008977Z ------------------------------------------
2019-10-04T20:31:01.7009057Z stderr:
2019-10-04T20:31:01.7009057Z stderr:
2019-10-04T20:31:01.7009313Z ------------------------------------------
2019-10-04T20:31:01.7009365Z 
2019-10-04T20:31:01.7009615Z ------------------------------------------
2019-10-04T20:31:01.7009665Z 
2019-10-04T20:31:01.7009710Z 
2019-10-04T20:31:01.7010013Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin3 stdout ----
2019-10-04T20:31:01.7010076Z 
2019-10-04T20:31:01.7010158Z error in revision `thin3`: test run failed!
2019-10-04T20:31:01.7010232Z status: exit code: 101
2019-10-04T20:31:01.7010649Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin3/a.js"
2019-10-04T20:31:01.7011014Z ------------------------------------------
2019-10-04T20:31:01.7011065Z 
2019-10-04T20:31:01.7011315Z ------------------------------------------
2019-10-04T20:31:01.7011405Z stderr:
2019-10-04T20:31:01.7011405Z stderr:
2019-10-04T20:31:01.7014767Z ------------------------------------------
2019-10-04T20:31:01.7014882Z 
2019-10-04T20:31:01.7016195Z ------------------------------------------
2019-10-04T20:31:01.7016251Z 
2019-10-04T20:31:01.7016305Z 
2019-10-04T20:31:01.7016335Z 
2019-10-04T20:31:01.7016388Z failures:
2019-10-04T20:31:01.7016812Z     [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat
2019-10-04T20:31:01.7019373Z     [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no
2019-10-04T20:31:01.7019713Z     [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin
2019-10-04T20:31:01.7020024Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0
2019-10-04T20:31:01.7020314Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1
2019-10-04T20:31:01.7020620Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2
2019-10-04T20:31:01.7020905Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3
2019-10-04T20:31:01.7021227Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no0
2019-10-04T20:31:01.7021578Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no1
2019-10-04T20:31:01.7021893Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no2
2019-10-04T20:31:01.7022354Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no3
2019-10-04T20:31:01.7022600Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0
2019-10-04T20:31:01.7022864Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1
2019-10-04T20:31:01.7023129Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2
2019-10-04T20:31:01.7023378Z     [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin3
2019-10-04T20:31:01.7023705Z test result: FAILED. 8859 passed; 15 failed; 246 ignored; 0 measured; 0 filtered out
2019-10-04T20:31:01.7023763Z 
2019-10-04T20:31:01.7024084Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-04T20:31:01.7024190Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-04T20:31:01.7024190Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-04T20:31:01.7037246Z 
2019-10-04T20:31:01.7037541Z 
2019-10-04T20:31:01.7040365Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-04T20:31:01.7041127Z 
2019-10-04T20:31:01.7041169Z 
2019-10-04T20:31:01.7048908Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/test/run-fail src/libstd src/liballoc src/libcore
2019-10-04T20:31:01.7049094Z Build completed unsuccessfully in 2:06:19
2019-10-04T20:31:01.7049094Z Build completed unsuccessfully in 2:06:19
2019-10-04T20:31:01.7103975Z == clock drift check ==
2019-10-04T20:31:01.7121523Z   local time: Fri Oct  4 20:31:01 UTC 2019
2019-10-04T20:31:01.9859986Z   network time: Fri, 04 Oct 2019 20:31:01 GMT
2019-10-04T20:31:01.9860658Z == end clock drift check ==
2019-10-04T20:31:02.8152807Z ##[error]Bash exited with code '1'.
2019-10-04T20:31:02.8191596Z ##[section]Starting: Upload CPU usage statistics
2019-10-04T20:31:02.8206591Z ==============================================================================
2019-10-04T20:31:02.8206694Z Task         : Bash
2019-10-04T20:31:02.8206757Z Description  : Run a Bash script on macOS, Linux, or Windows

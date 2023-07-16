plain
2019-11-05T05:41:17.1390597Z ---- [ui] ui/derives/deriving-meta-unknown-trait.rs stdout ----
2019-11-05T05:41:17.1390940Z diff of stderr:
2019-11-05T05:41:17.1391115Z 
2019-11-05T05:41:17.1391338Z 3    |
2019-11-05T05:41:17.1391531Z 4 LL | #[derive(Eqr)]
2019-11-05T05:41:17.1391809Z 5    |          ^^^ help: a derive macro with a similar name exists: `Eq`
2019-11-05T05:41:17.1392316Z -    | 
2019-11-05T05:41:17.1393045Z -   ::: $SRC_DIR/libcore/cmp.rs:LL:COL
2019-11-05T05:41:17.1393770Z -    |
2019-11-05T05:41:17.1394402Z - LL | pub macro Eq($item:item) { /* compiler built-in */ }
2019-11-05T05:41:17.1395106Z -    | ------------------------ similarly named derive macro `Eq` defined here
2019-11-05T05:41:17.1395584Z 12 error: aborting due to previous error
2019-11-05T05:41:17.1395921Z 13 
2019-11-05T05:41:17.1396072Z 
2019-11-05T05:41:17.1396227Z 
2019-11-05T05:41:17.1396227Z 
2019-11-05T05:41:17.1396445Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1397130Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/deriving-meta-unknown-trait.stderr
2019-11-05T05:41:17.1397703Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1398318Z To only update this specific test, also pass `--test-args derives/deriving-meta-unknown-trait.rs`
2019-11-05T05:41:17.1399025Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1399310Z status: exit code: 1
2019-11-05T05:41:17.1399310Z status: exit code: 1
2019-11-05T05:41:17.1400390Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1401524Z ------------------------------------------
2019-11-05T05:41:17.1401749Z 
2019-11-05T05:41:17.1402672Z ------------------------------------------
2019-11-05T05:41:17.1402952Z stderr:
2019-11-05T05:41:17.1402952Z stderr:
2019-11-05T05:41:17.1403375Z ------------------------------------------
2019-11-05T05:41:17.1403627Z error: cannot find derive macro `Eqr` in this scope
2019-11-05T05:41:17.1404150Z   --> /checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs:1:10
2019-11-05T05:41:17.1404585Z    |
2019-11-05T05:41:17.1404778Z LL | #[derive(Eqr)]
2019-11-05T05:41:17.1404971Z    |          ^^^ help: a derive macro with a similar name exists: `Eq`
2019-11-05T05:41:17.1405325Z error: aborting due to previous error
2019-11-05T05:41:17.1405490Z 
2019-11-05T05:41:17.1405634Z 
2019-11-05T05:41:17.1406079Z ------------------------------------------
---
2019-11-05T05:41:17.1407261Z 
2019-11-05T05:41:17.1407736Z 1 error[E0573]: expected type, found variant `NoResult`
2019-11-05T05:41:17.1408242Z 2   --> $DIR/issue-17546.rs:12:17
2019-11-05T05:41:17.1408483Z 3    |
2019-11-05T05:41:17.1408904Z - LL |     fn new() -> NoResult<MyEnum, String> {
2019-11-05T05:41:17.1409993Z -    | 
2019-11-05T05:41:17.1409993Z -    | 
2019-11-05T05:41:17.1410526Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2019-11-05T05:41:17.1411032Z + LL |       fn new() -> NoResult<MyEnum, String> {
2019-11-05T05:41:17.1411496Z 8    |
2019-11-05T05:41:17.1411496Z 8    |
2019-11-05T05:41:17.1411891Z - LL | pub enum Result<T, E> {
2019-11-05T05:41:17.1412430Z -    | --------------------- similarly named enum `Result` defined here
2019-11-05T05:41:17.1413316Z 12 help: try using the variant's enum
2019-11-05T05:41:17.1413574Z 13    |
2019-11-05T05:41:17.1413574Z 13    |
2019-11-05T05:41:17.1413999Z 14 LL |     fn new() -> foo::MyEnum {
2019-11-05T05:41:17.1420293Z 57 error[E0573]: expected type, found variant `NoResult`
2019-11-05T05:41:17.1420909Z 58   --> $DIR/issue-17546.rs:33:15
2019-11-05T05:41:17.1421160Z 59    |
2019-11-05T05:41:17.1421160Z 59    |
2019-11-05T05:41:17.1421607Z - LL | fn newer() -> NoResult<foo::MyEnum, String> {
2019-11-05T05:41:17.1429572Z -    | 
2019-11-05T05:41:17.1429572Z -    | 
2019-11-05T05:41:17.1430083Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2019-11-05T05:41:17.1430525Z -    |
2019-11-05T05:41:17.1430994Z - LL | pub enum Result<T, E> {
2019-11-05T05:41:17.1431577Z -    | --------------------- similarly named enum `Result` defined here
2019-11-05T05:41:17.1432127Z + LL |   fn newer() -> NoResult<foo::MyEnum, String> {
2019-11-05T05:41:17.1470392Z 67    |
2019-11-05T05:41:17.1471018Z 68 help: try using the variant's enum
2019-11-05T05:41:17.1471451Z 69    |
2019-11-05T05:41:17.1471664Z 
2019-11-05T05:41:17.1471664Z 
2019-11-05T05:41:17.1471800Z 
2019-11-05T05:41:17.1471899Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1472466Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
2019-11-05T05:41:17.1473132Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1473523Z To only update this specific test, also pass `--test-args issues/issue-17546.rs`
2019-11-05T05:41:17.1473685Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1473778Z status: exit code: 1
2019-11-05T05:41:17.1473778Z status: exit code: 1
2019-11-05T05:41:17.1474660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17546.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1475243Z ------------------------------------------
2019-11-05T05:41:17.1475299Z 
2019-11-05T05:41:17.1475938Z ------------------------------------------
2019-11-05T05:41:17.1476023Z stderr:
2019-11-05T05:41:17.1476023Z stderr:
2019-11-05T05:41:17.1476310Z ------------------------------------------
2019-11-05T05:41:17.1476400Z error[E0573]: expected type, found variant `NoResult`
2019-11-05T05:41:17.1476720Z   --> /checkout/src/test/ui/issues/issue-17546.rs:12:17
2019-11-05T05:41:17.1476806Z    |
2019-11-05T05:41:17.1477106Z LL |       fn new() -> NoResult<MyEnum, String> {
2019-11-05T05:41:17.1477289Z    |
2019-11-05T05:41:17.1477561Z help: try using the variant's enum
2019-11-05T05:41:17.1477636Z    |
2019-11-05T05:41:17.1477636Z    |
2019-11-05T05:41:17.1477907Z LL |     fn new() -> foo::MyEnum {
2019-11-05T05:41:17.1478240Z help: an enum with a similar name exists
2019-11-05T05:41:17.1478311Z    |
2019-11-05T05:41:17.1478311Z    |
2019-11-05T05:41:17.1478639Z LL |     fn new() -> Result<MyEnum, String> {
2019-11-05T05:41:17.1478785Z 
2019-11-05T05:41:17.1478940Z error[E0573]: expected type, found variant `Result`
2019-11-05T05:41:17.1479304Z   --> /checkout/src/test/ui/issues/issue-17546.rs:22:17
2019-11-05T05:41:17.1479404Z    |
2019-11-05T05:41:17.1479404Z    |
2019-11-05T05:41:17.1479679Z LL |     fn new() -> Result<foo::MyEnum, String> {
2019-11-05T05:41:17.1479865Z    |
2019-11-05T05:41:17.1479966Z help: possible better candidates are found in other modules, you can import them into scope
2019-11-05T05:41:17.1480181Z    |
2019-11-05T05:41:17.1480266Z LL |     use std::fmt::Result;
---
2019-11-05T05:41:17.1481064Z 
2019-11-05T05:41:17.1481134Z error[E0573]: expected type, found variant `Result`
2019-11-05T05:41:17.1481486Z   --> /checkout/src/test/ui/issues/issue-17546.rs:28:13
2019-11-05T05:41:17.1481571Z    |
2019-11-05T05:41:17.1481858Z LL | fn new() -> Result<foo::MyEnum, String> {
2019-11-05T05:41:17.1482042Z    |
2019-11-05T05:41:17.1482145Z help: possible better candidates are found in other modules, you can import them into scope
2019-11-05T05:41:17.1482235Z    |
2019-11-05T05:41:17.1482317Z LL | use std::fmt::Result;
---
2019-11-05T05:41:17.1482979Z 
2019-11-05T05:41:17.1483078Z error[E0573]: expected type, found variant `NoResult`
2019-11-05T05:41:17.1483623Z   --> /checkout/src/test/ui/issues/issue-17546.rs:33:15
2019-11-05T05:41:17.1483725Z    |
2019-11-05T05:41:17.1483982Z LL |   fn newer() -> NoResult<foo::MyEnum, String> {
2019-11-05T05:41:17.1484173Z    |
2019-11-05T05:41:17.1484409Z help: try using the variant's enum
2019-11-05T05:41:17.1484499Z    |
2019-11-05T05:41:17.1484499Z    |
2019-11-05T05:41:17.1484730Z LL | fn newer() -> foo::MyEnum {
2019-11-05T05:41:17.1484891Z help: an enum with a similar name exists
2019-11-05T05:41:17.1484989Z    |
2019-11-05T05:41:17.1484989Z    |
2019-11-05T05:41:17.1485242Z LL | fn newer() -> Result<foo::MyEnum, String> {
2019-11-05T05:41:17.1485380Z 
2019-11-05T05:41:17.1485462Z error: aborting due to 4 previous errors
2019-11-05T05:41:17.1485613Z 
2019-11-05T05:41:17.1485908Z For more information about this error, try `rustc --explain E0573`.
---
2019-11-05T05:41:17.1487059Z 3    |
2019-11-05T05:41:17.1487281Z - LL | impl Fo {
2019-11-05T05:41:17.1487533Z -    |      ^^ help: a trait with a similar name exists: `Fn`
2019-11-05T05:41:17.1487892Z -    | 
2019-11-05T05:41:17.1488126Z -   ::: $SRC_DIR/libcore/ops/function.rs:LL:COL
2019-11-05T05:41:17.1488353Z -    |
2019-11-05T05:41:17.1488597Z - LL | pub trait Fn<Args> : FnMut<Args> {
2019-11-05T05:41:17.1489011Z -    | -------------------------------- similarly named trait `Fn` defined here
2019-11-05T05:41:17.1489193Z + LL |   impl Fo {
2019-11-05T05:41:17.1489281Z +    |        ^^ help: a trait with a similar name exists: `Fn`
2019-11-05T05:41:17.1489436Z 12 error: aborting due to previous error
2019-11-05T05:41:17.1489521Z 13 
2019-11-05T05:41:17.1489556Z 
2019-11-05T05:41:17.1489589Z 
2019-11-05T05:41:17.1489589Z 
2019-11-05T05:41:17.1489675Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1490072Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/issue-7607-1.stderr
2019-11-05T05:41:17.1490382Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1490706Z To only update this specific test, also pass `--test-args issues/issue-7607-1.rs`
2019-11-05T05:41:17.1490860Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1490930Z status: exit code: 1
2019-11-05T05:41:17.1490930Z status: exit code: 1
2019-11-05T05:41:17.1491921Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7607-1.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1492437Z ------------------------------------------
2019-11-05T05:41:17.1492506Z 
2019-11-05T05:41:17.1492753Z ------------------------------------------
2019-11-05T05:41:17.1492853Z stderr:
2019-11-05T05:41:17.1492853Z stderr:
2019-11-05T05:41:17.1493099Z ------------------------------------------
2019-11-05T05:41:17.1493201Z error[E0412]: cannot find type `Fo` in this scope
2019-11-05T05:41:17.1493492Z   --> /checkout/src/test/ui/issues/issue-7607-1.rs:5:6
2019-11-05T05:41:17.1493570Z    |
2019-11-05T05:41:17.1493663Z LL |   impl Fo { //~ ERROR cannot find type `Fo` in this scope
2019-11-05T05:41:17.1493750Z    |        ^^ help: a trait with a similar name exists: `Fn`
2019-11-05T05:41:17.1494005Z error: aborting due to previous error
2019-11-05T05:41:17.1494047Z 
2019-11-05T05:41:17.1494479Z For more information about this error, try `rustc --explain E0412`.
2019-11-05T05:41:17.1494540Z 
---
2019-11-05T05:41:17.1495321Z 
2019-11-05T05:41:17.1495401Z 1 error: cannot find macro `printlx` in this scope
2019-11-05T05:41:17.1495686Z 2   --> $DIR/macro-name-typo.rs:2:5
2019-11-05T05:41:17.1495780Z 3    |
2019-11-05T05:41:17.1496025Z - LL |     printlx!("oh noes!");
2019-11-05T05:41:17.1496588Z -    | 
2019-11-05T05:41:17.1496588Z -    | 
2019-11-05T05:41:17.1496853Z -   ::: $SRC_DIR/libstd/macros.rs:LL:COL
2019-11-05T05:41:17.1497083Z -    |
2019-11-05T05:41:17.1497475Z - LL | macro_rules! println {
2019-11-05T05:41:17.1497864Z -    | -------------------- similarly named macro `println` defined here
2019-11-05T05:41:17.1497962Z + LL |       printlx!("oh noes!");
2019-11-05T05:41:17.1498120Z 11 
2019-11-05T05:41:17.1498192Z 12 error: aborting due to previous error
2019-11-05T05:41:17.1498252Z 13 
2019-11-05T05:41:17.1498284Z 
2019-11-05T05:41:17.1498284Z 
2019-11-05T05:41:17.1498333Z 
2019-11-05T05:41:17.1498479Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1498858Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/macro-name-typo.stderr
2019-11-05T05:41:17.1499164Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1499526Z To only update this specific test, also pass `--test-args macros/macro-name-typo.rs`
2019-11-05T05:41:17.1499675Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1499759Z status: exit code: 1
2019-11-05T05:41:17.1499759Z status: exit code: 1
2019-11-05T05:41:17.1500582Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-name-typo.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1501106Z ------------------------------------------
2019-11-05T05:41:17.1501157Z 
2019-11-05T05:41:17.1501414Z ------------------------------------------
2019-11-05T05:41:17.1501485Z stderr:
2019-11-05T05:41:17.1501485Z stderr:
2019-11-05T05:41:17.1501731Z ------------------------------------------
2019-11-05T05:41:17.1501808Z error: cannot find macro `printlx` in this scope
2019-11-05T05:41:17.1502081Z   --> /checkout/src/test/ui/macros/macro-name-typo.rs:2:5
2019-11-05T05:41:17.1502172Z    |
2019-11-05T05:41:17.1502240Z LL |       printlx!("oh noes!"); //~ ERROR cannot find
2019-11-05T05:41:17.1502388Z 
2019-11-05T05:41:17.1502461Z error: aborting due to previous error
2019-11-05T05:41:17.1502503Z 
2019-11-05T05:41:17.1502534Z 
---
2019-11-05T05:41:17.1503331Z 3    |
2019-11-05T05:41:17.1503413Z 4 LL |     inline!();
2019-11-05T05:41:17.1503485Z 5    |     ^^^^^^ help: a macro with a similar name exists: `line`
2019-11-05T05:41:17.1503726Z -    | 
2019-11-05T05:41:17.1503954Z -   ::: $SRC_DIR/libcore/macros.rs:LL:COL
2019-11-05T05:41:17.1504184Z -    |
2019-11-05T05:41:17.1504453Z - LL |     macro_rules! line { () => { /* compiler built-in */ } }
2019-11-05T05:41:17.1504728Z -    |     ----------------- similarly named macro `line` defined here
2019-11-05T05:41:17.1504880Z 12 error: aborting due to previous error
2019-11-05T05:41:17.1504961Z 13 
2019-11-05T05:41:17.1504995Z 
2019-11-05T05:41:17.1505026Z 
2019-11-05T05:41:17.1505026Z 
2019-11-05T05:41:17.1505117Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1505488Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/macro-path-prelude-fail-3.stderr
2019-11-05T05:41:17.1505787Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1506119Z To only update this specific test, also pass `--test-args macros/macro-path-prelude-fail-3.rs`
2019-11-05T05:41:17.1506257Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1506321Z status: exit code: 1
2019-11-05T05:41:17.1506321Z status: exit code: 1
2019-11-05T05:41:17.1507168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-path-prelude-fail-3.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1507769Z ------------------------------------------
2019-11-05T05:41:17.1507903Z 
2019-11-05T05:41:17.1508169Z ------------------------------------------
2019-11-05T05:41:17.1508260Z stderr:
2019-11-05T05:41:17.1508260Z stderr:
2019-11-05T05:41:17.1508486Z ------------------------------------------
2019-11-05T05:41:17.1508580Z error: cannot find macro `inline` in this scope
2019-11-05T05:41:17.1508860Z   --> /checkout/src/test/ui/macros/macro-path-prelude-fail-3.rs:2:5
2019-11-05T05:41:17.1508940Z    |
2019-11-05T05:41:17.1509026Z LL |     inline!(); //~ ERROR cannot find macro `inline` in this scope
2019-11-05T05:41:17.1509108Z    |     ^^^^^^ help: a macro with a similar name exists: `line`
2019-11-05T05:41:17.1509231Z error: aborting due to previous error
2019-11-05T05:41:17.1509282Z 
2019-11-05T05:41:17.1509332Z 
2019-11-05T05:41:17.1509568Z ------------------------------------------
2019-11-05T05:41:17.1509568Z ------------------------------------------
2019-11-05T05:41:17.1509634Z 
2019-11-05T05:41:17.1509666Z 
2019-11-05T05:41:17.1509909Z ---- [ui] ui/proc-macro/parent-source-spans.rs stdout ----
2019-11-05T05:41:17.1510010Z diff of stderr:
2019-11-05T05:41:17.1510048Z 
2019-11-05T05:41:17.1510120Z 132 ...
2019-11-05T05:41:17.1510175Z 133 LL |     one!("hello", "world");
2019-11-05T05:41:17.1510687Z -    | 
2019-11-05T05:41:17.1510687Z -    | 
2019-11-05T05:41:17.1510915Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2019-11-05T05:41:17.1511138Z -    |
2019-11-05T05:41:17.1511392Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2019-11-05T05:41:17.1511718Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2019-11-05T05:41:17.1511894Z 141 error[E0425]: cannot find value `ok` in this scope
2019-11-05T05:41:17.1512171Z 142   --> $DIR/parent-source-spans.rs:28:5
2019-11-05T05:41:17.1512219Z 
2019-11-05T05:41:17.1512273Z 146 ...
2019-11-05T05:41:17.1512273Z 146 ...
2019-11-05T05:41:17.1512348Z 147 LL |     two!("yay", "rust");
2019-11-05T05:41:17.1520025Z -    | 
2019-11-05T05:41:17.1520025Z -    | 
2019-11-05T05:41:17.1520286Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2019-11-05T05:41:17.1520499Z -    |
2019-11-05T05:41:17.1520774Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2019-11-05T05:41:17.1521089Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2019-11-05T05:41:17.1521260Z 155 error[E0425]: cannot find value `ok` in this scope
2019-11-05T05:41:17.1521526Z 156   --> $DIR/parent-source-spans.rs:28:5
2019-11-05T05:41:17.1521576Z 
2019-11-05T05:41:17.1521650Z 160 ...
2019-11-05T05:41:17.1521650Z 160 ...
2019-11-05T05:41:17.1521705Z 161 LL |     three!("hip", "hop");
2019-11-05T05:41:17.1522227Z -    | 
2019-11-05T05:41:17.1522227Z -    | 
2019-11-05T05:41:17.1522459Z -   ::: $SRC_DIR/libcore/result.rs:LL:COL
2019-11-05T05:41:17.1522681Z -    |
2019-11-05T05:41:17.1522937Z - LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
2019-11-05T05:41:17.1523275Z -    |     --------------------------------------------------- similarly named tuple variant `Ok` defined here
2019-11-05T05:41:17.1523445Z 169 error: aborting due to 21 previous errors
2019-11-05T05:41:17.1523508Z 170 
2019-11-05T05:41:17.1523560Z 
2019-11-05T05:41:17.1523592Z 
2019-11-05T05:41:17.1523592Z 
2019-11-05T05:41:17.1523672Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1524020Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/parent-source-spans.stderr
2019-11-05T05:41:17.1524325Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1524824Z To only update this specific test, also pass `--test-args proc-macro/parent-source-spans.rs`
2019-11-05T05:41:17.1524966Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1525049Z status: exit code: 1
2019-11-05T05:41:17.1525049Z status: exit code: 1
2019-11-05T05:41:17.1525956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/parent-source-spans.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/parent-source-spans/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1526514Z ------------------------------------------
2019-11-05T05:41:17.1526564Z 
2019-11-05T05:41:17.1526831Z ------------------------------------------
2019-11-05T05:41:17.1526900Z stderr:
2019-11-05T05:41:17.1526900Z stderr:
2019-11-05T05:41:17.1527143Z ------------------------------------------
2019-11-05T05:41:17.1527231Z error: first final: "hello"
2019-11-05T05:41:17.1527496Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:15:12
2019-11-05T05:41:17.1527601Z    |
2019-11-05T05:41:17.1527663Z LL |     three!($a, $b);
2019-11-05T05:41:17.1527803Z ...
2019-11-05T05:41:17.1527803Z ...
2019-11-05T05:41:17.1527878Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1528208Z 
2019-11-05T05:41:17.1528265Z error: second final: "world"
2019-11-05T05:41:17.1528545Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:15:16
2019-11-05T05:41:17.1528619Z    |
2019-11-05T05:41:17.1528619Z    |
2019-11-05T05:41:17.1528692Z LL |     three!($a, $b);
2019-11-05T05:41:17.1528832Z ...
2019-11-05T05:41:17.1528832Z ...
2019-11-05T05:41:17.1528916Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1529226Z 
2019-11-05T05:41:17.1529299Z error: first parent: "hello"
2019-11-05T05:41:17.1529572Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:9:5
2019-11-05T05:41:17.1529657Z    |
2019-11-05T05:41:17.1529657Z    |
2019-11-05T05:41:17.1529733Z LL |     two!($a, $b);
2019-11-05T05:41:17.1529872Z ...
2019-11-05T05:41:17.1529872Z ...
2019-11-05T05:41:17.1529929Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1530251Z 
2019-11-05T05:41:17.1530322Z error: second parent: "world"
2019-11-05T05:41:17.1530581Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:9:5
2019-11-05T05:41:17.1530675Z    |
2019-11-05T05:41:17.1530675Z    |
2019-11-05T05:41:17.1530729Z LL |     two!($a, $b);
2019-11-05T05:41:17.1530866Z ...
2019-11-05T05:41:17.1530866Z ...
2019-11-05T05:41:17.1530939Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1531272Z 
2019-11-05T05:41:17.1531272Z 
2019-11-05T05:41:17.1531328Z error: first grandparent: "hello"
2019-11-05T05:41:17.1531708Z    |
2019-11-05T05:41:17.1531708Z    |
2019-11-05T05:41:17.1531767Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1531889Z 
2019-11-05T05:41:17.1531889Z 
2019-11-05T05:41:17.1531943Z error: second grandparent: "world"
2019-11-05T05:41:17.1532324Z    |
2019-11-05T05:41:17.1532324Z    |
2019-11-05T05:41:17.1532380Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1532501Z 
2019-11-05T05:41:17.1532574Z error: first source: "hello"
2019-11-05T05:41:17.1532836Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:35:5
2019-11-05T05:41:17.1532927Z    |
2019-11-05T05:41:17.1532927Z    |
2019-11-05T05:41:17.1533071Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1533191Z 
2019-11-05T05:41:17.1533264Z error: second source: "world"
2019-11-05T05:41:17.1533557Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:35:5
2019-11-05T05:41:17.1533716Z    |
2019-11-05T05:41:17.1533716Z    |
2019-11-05T05:41:17.1533782Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1533902Z 
2019-11-05T05:41:17.1533902Z 
2019-11-05T05:41:17.1533975Z error: first final: "yay"
2019-11-05T05:41:17.1534359Z    |
2019-11-05T05:41:17.1534359Z    |
2019-11-05T05:41:17.1534413Z LL |     three!($a, $b);
2019-11-05T05:41:17.1534555Z ...
2019-11-05T05:41:17.1534555Z ...
2019-11-05T05:41:17.1534629Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1534947Z 
2019-11-05T05:41:17.1535002Z error: second final: "rust"
2019-11-05T05:41:17.1535294Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:15:16
2019-11-05T05:41:17.1535385Z    |
2019-11-05T05:41:17.1535385Z    |
2019-11-05T05:41:17.1535442Z LL |     three!($a, $b);
2019-11-05T05:41:17.1535579Z ...
2019-11-05T05:41:17.1535579Z ...
2019-11-05T05:41:17.1535662Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1535988Z 
2019-11-05T05:41:17.1535988Z 
2019-11-05T05:41:17.1536045Z error: first parent: "yay"
2019-11-05T05:41:17.1536396Z    |
2019-11-05T05:41:17.1536396Z    |
2019-11-05T05:41:17.1536470Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1536592Z 
2019-11-05T05:41:17.1536647Z error: second parent: "rust"
2019-11-05T05:41:17.1536928Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:41:5
2019-11-05T05:41:17.1537004Z    |
2019-11-05T05:41:17.1537004Z    |
2019-11-05T05:41:17.1537078Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1537207Z 
2019-11-05T05:41:17.1537262Z error: first source: "yay"
2019-11-05T05:41:17.1537544Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:41:5
2019-11-05T05:41:17.1537619Z    |
2019-11-05T05:41:17.1537619Z    |
2019-11-05T05:41:17.1537700Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1537820Z 
2019-11-05T05:41:17.1537875Z error: second source: "rust"
2019-11-05T05:41:17.1538160Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:41:5
2019-11-05T05:41:17.1538234Z    |
2019-11-05T05:41:17.1538234Z    |
2019-11-05T05:41:17.1538306Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1538427Z 
2019-11-05T05:41:17.1538427Z 
2019-11-05T05:41:17.1538481Z error: first final: "hip"
2019-11-05T05:41:17.1538840Z    |
2019-11-05T05:41:17.1538840Z    |
2019-11-05T05:41:17.1538912Z LL |     three!("hip", "hop");
2019-11-05T05:41:17.1539043Z 
2019-11-05T05:41:17.1539043Z 
2019-11-05T05:41:17.1539097Z error: second final: "hop"
2019-11-05T05:41:17.1539474Z    |
2019-11-05T05:41:17.1539474Z    |
2019-11-05T05:41:17.1539539Z LL |     three!("hip", "hop");
2019-11-05T05:41:17.1539661Z 
2019-11-05T05:41:17.1539714Z error: first source: "hip"
2019-11-05T05:41:17.1540000Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:47:12
2019-11-05T05:41:17.1540092Z    |
2019-11-05T05:41:17.1540092Z    |
2019-11-05T05:41:17.1540148Z LL |     three!("hip", "hop");
2019-11-05T05:41:17.1540266Z 
2019-11-05T05:41:17.1540339Z error: second source: "hop"
2019-11-05T05:41:17.1540604Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:47:19
2019-11-05T05:41:17.1540695Z    |
2019-11-05T05:41:17.1540695Z    |
2019-11-05T05:41:17.1540750Z LL |     three!("hip", "hop");
2019-11-05T05:41:17.1540958Z 
2019-11-05T05:41:17.1541036Z error[E0425]: cannot find value `ok` in this scope
2019-11-05T05:41:17.1541332Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:28:5
2019-11-05T05:41:17.1541425Z    |
2019-11-05T05:41:17.1541425Z    |
2019-11-05T05:41:17.1541549Z LL |     parent_source_spans!($($tokens)*);
2019-11-05T05:41:17.1541661Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
2019-11-05T05:41:17.1541743Z ...
2019-11-05T05:41:17.1541826Z LL |     one!("hello", "world");
2019-11-05T05:41:17.1542180Z 
2019-11-05T05:41:17.1542242Z error[E0425]: cannot find value `ok` in this scope
2019-11-05T05:41:17.1542527Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:28:5
2019-11-05T05:41:17.1542617Z    |
2019-11-05T05:41:17.1542617Z    |
2019-11-05T05:41:17.1542676Z LL |     parent_source_spans!($($tokens)*);
2019-11-05T05:41:17.1542776Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
2019-11-05T05:41:17.1542865Z ...
2019-11-05T05:41:17.1542940Z LL |     two!("yay", "rust");
2019-11-05T05:41:17.1543265Z 
2019-11-05T05:41:17.1543337Z error[E0425]: cannot find value `ok` in this scope
2019-11-05T05:41:17.1543625Z   --> /checkout/src/test/ui/proc-macro/parent-source-spans.rs:28:5
2019-11-05T05:41:17.1543700Z    |
2019-11-05T05:41:17.1543700Z    |
2019-11-05T05:41:17.1543777Z LL |     parent_source_spans!($($tokens)*);
2019-11-05T05:41:17.1543859Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `Ok`
2019-11-05T05:41:17.1543957Z ...
2019-11-05T05:41:17.1544029Z LL |     three!("hip", "hop");
2019-11-05T05:41:17.1544335Z 
2019-11-05T05:41:17.1551136Z error: aborting due to 21 previous errors
2019-11-05T05:41:17.1551184Z 
2019-11-05T05:41:17.1551652Z For more information about this error, try `rustc --explain E0425`.
---
2019-11-05T05:41:17.1557151Z ---- [ui] ui/proc-macro/resolve-error.rs stdout ----
2019-11-05T05:41:17.1557262Z diff of stderr:
2019-11-05T05:41:17.1557335Z 
2019-11-05T05:41:17.1557396Z 59    |
2019-11-05T05:41:17.1557479Z 60 LL | #[derive(Dlone)]
2019-11-05T05:41:17.1557566Z 61    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
2019-11-05T05:41:17.1558458Z -    | 
2019-11-05T05:41:17.1572653Z -   ::: $SRC_DIR/libcore/clone.rs:LL:COL
2019-11-05T05:41:17.1572894Z -    |
2019-11-05T05:41:17.1573168Z - LL | pub macro Clone($item:item) { /* compiler built-in */ }
2019-11-05T05:41:17.1573458Z -    | --------------------------- similarly named derive macro `Clone` defined here
2019-11-05T05:41:17.1573562Z 67 
2019-11-05T05:41:17.1573628Z 68 error: cannot find attribute `FooWithLongNan` in this scope
2019-11-05T05:41:17.1573965Z 
2019-11-05T05:41:17.1574013Z 
2019-11-05T05:41:17.1574198Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1574849Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
2019-11-05T05:41:17.1574849Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
2019-11-05T05:41:17.1575188Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1575540Z To only update this specific test, also pass `--test-args proc-macro/resolve-error.rs`
2019-11-05T05:41:17.1575694Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1575770Z status: exit code: 1
2019-11-05T05:41:17.1575770Z status: exit code: 1
2019-11-05T05:41:17.1578648Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/resolve-error.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1579374Z ------------------------------------------
2019-11-05T05:41:17.1579451Z 
2019-11-05T05:41:17.1579705Z ------------------------------------------
2019-11-05T05:41:17.1579799Z stderr:
2019-11-05T05:41:17.1579799Z stderr:
2019-11-05T05:41:17.1580040Z ------------------------------------------
2019-11-05T05:41:17.1580143Z error: cannot find macro `bang_proc_macrp` in this scope
2019-11-05T05:41:17.1580530Z    |
2019-11-05T05:41:17.1580530Z    |
2019-11-05T05:41:17.1580608Z LL |     bang_proc_macrp!();
2019-11-05T05:41:17.1580691Z    |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `bang_proc_macro`
2019-11-05T05:41:17.1581657Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:15:1
2019-11-05T05:41:17.1581785Z    |
2019-11-05T05:41:17.1581785Z    |
2019-11-05T05:41:17.1582129Z LL | pub fn empty(_: TokenStream) -> TokenStream {
2019-11-05T05:41:17.1584458Z    | ------------------------------------------- similarly named macro `bang_proc_macro` defined here
2019-11-05T05:41:17.1584577Z 
2019-11-05T05:41:17.1584651Z error: cannot find macro `Dlona` in this scope
2019-11-05T05:41:17.1585077Z    |
2019-11-05T05:41:17.1585077Z    |
2019-11-05T05:41:17.1585159Z LL |     Dlona!();
2019-11-05T05:41:17.1585289Z 
2019-11-05T05:41:17.1585289Z 
2019-11-05T05:41:17.1585362Z error: cannot find macro `attr_proc_macra` in this scope
2019-11-05T05:41:17.1585875Z    |
2019-11-05T05:41:17.1585875Z    |
2019-11-05T05:41:17.1585954Z LL | / macro_rules! attr_proc_mac {
2019-11-05T05:41:17.1586113Z LL | | }
2019-11-05T05:41:17.1586113Z LL | | }
2019-11-05T05:41:17.1586394Z    | |_- similarly named macro `attr_proc_mac` defined here
2019-11-05T05:41:17.1586495Z ...
2019-11-05T05:41:17.1586576Z LL |       attr_proc_macra!();
2019-11-05T05:41:17.1586673Z    |       ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `attr_proc_mac`
2019-11-05T05:41:17.1586735Z 
2019-11-05T05:41:17.1586823Z error: cannot find macro `FooWithLongNama` in this scope
2019-11-05T05:41:17.1587218Z    |
2019-11-05T05:41:17.1587218Z    |
2019-11-05T05:41:17.1587298Z LL | / macro_rules! FooWithLongNam {
2019-11-05T05:41:17.1587455Z LL | | }
2019-11-05T05:41:17.1587455Z LL | | }
2019-11-05T05:41:17.1587850Z    | |_- similarly named macro `FooWithLongNam` defined here
2019-11-05T05:41:17.1587952Z ...
2019-11-05T05:41:17.1588012Z LL |       FooWithLongNama!();
2019-11-05T05:41:17.1588114Z    |       ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `FooWithLongNam`
2019-11-05T05:41:17.1588186Z 
2019-11-05T05:41:17.1588275Z error: cannot find derive macro `attr_proc_macra` in this scope
2019-11-05T05:41:17.1588692Z    |
2019-11-05T05:41:17.1588692Z    |
2019-11-05T05:41:17.1588784Z LL | #[derive(attr_proc_macra)]
2019-11-05T05:41:17.1589620Z 
2019-11-05T05:41:17.1589620Z 
2019-11-05T05:41:17.1589792Z error: cannot find derive macro `Dlona` in this scope
2019-11-05T05:41:17.1590316Z    |
2019-11-05T05:41:17.1590316Z    |
2019-11-05T05:41:17.1590398Z LL | #[derive(Dlona)]
2019-11-05T05:41:17.1590484Z    |          ^^^^^ help: a derive macro with a similar name exists: `Clona`
2019-11-05T05:41:17.1590585Z    | 
2019-11-05T05:41:17.1590888Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-clona.rs:11:1
2019-11-05T05:41:17.1590994Z    |
2019-11-05T05:41:17.1591276Z LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
2019-11-05T05:41:17.1591960Z    | ------------------------------------------------------- similarly named derive macro `Clona` defined here
2019-11-05T05:41:17.1592135Z error: cannot find derive macro `Dlone` in this scope
2019-11-05T05:41:17.1592769Z   --> /checkout/src/test/ui/proc-macro/resolve-error.rs:34:10
2019-11-05T05:41:17.1592898Z    |
2019-11-05T05:41:17.1592898Z    |
2019-11-05T05:41:17.1592978Z LL | #[derive(Dlone)]
2019-11-05T05:41:17.1593188Z    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
2019-11-05T05:41:17.1593267Z 
2019-11-05T05:41:17.1593338Z error: cannot find attribute `FooWithLongNan` in this scope
2019-11-05T05:41:17.1593938Z    |
2019-11-05T05:41:17.1593938Z    |
2019-11-05T05:41:17.1594031Z LL | #[FooWithLongNan] //~ ERROR cannot find attribute `FooWithLongNan` in this scope
2019-11-05T05:41:17.1594188Z 
2019-11-05T05:41:17.1594188Z 
2019-11-05T05:41:17.1594258Z error: cannot find attribute `attr_proc_macra` in this scope
2019-11-05T05:41:17.1594695Z    |
2019-11-05T05:41:17.1594695Z    |
2019-11-05T05:41:17.1594805Z LL | #[attr_proc_macra] //~ ERROR cannot find attribute `attr_proc_macra` in this scope
2019-11-05T05:41:17.1594914Z    |   ^^^^^^^^^^^^^^^ help: an attribute macro with a similar name exists: `attr_proc_macro`
2019-11-05T05:41:17.1595354Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:20:1
2019-11-05T05:41:17.1595443Z    |
2019-11-05T05:41:17.1595443Z    |
2019-11-05T05:41:17.1595765Z LL | pub fn empty_attr(_: TokenStream, _: TokenStream) -> TokenStream {
2019-11-05T05:41:17.1596284Z    | ---------------------------------------------------------------- similarly named attribute macro `attr_proc_macro` defined here
2019-11-05T05:41:17.1596379Z 
2019-11-05T05:41:17.1596450Z error: cannot find derive macro `FooWithLongNan` in this scope
2019-11-05T05:41:17.1596846Z    |
2019-11-05T05:41:17.1596846Z    |
2019-11-05T05:41:17.1596923Z LL | #[derive(FooWithLongNan)]
2019-11-05T05:41:17.1597010Z    |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
2019-11-05T05:41:17.1597419Z   ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-foo.rs:11:1
2019-11-05T05:41:17.1597624Z    |
2019-11-05T05:41:17.1597624Z    |
2019-11-05T05:41:17.1598043Z LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
2019-11-05T05:41:17.1598409Z    | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
2019-11-05T05:41:17.1598575Z error: aborting due to 10 previous errors
2019-11-05T05:41:17.1598622Z 
2019-11-05T05:41:17.1598679Z 
2019-11-05T05:41:17.1598940Z ------------------------------------------
2019-11-05T05:41:17.1598940Z ------------------------------------------
2019-11-05T05:41:17.1599013Z 
2019-11-05T05:41:17.1599050Z 
2019-11-05T05:41:17.1599316Z ---- [ui] ui/resolve/levenshtein.rs stdout ----
2019-11-05T05:41:17.1599428Z diff of stderr:
2019-11-05T05:41:17.1599471Z 
2019-11-05T05:41:17.1599562Z 16 error[E0412]: cannot find type `Opiton` in this scope
2019-11-05T05:41:17.1599840Z 17   --> $DIR/levenshtein.rs:12:10
2019-11-05T05:41:17.1599938Z 18    |
2019-11-05T05:41:17.1600257Z - LL | type B = Opiton<u8>; // Misspelled type name from the prelude.
2019-11-05T05:41:17.1600821Z -    |          ^^^^^^ help: an enum with a similar name exists: `Option`
2019-11-05T05:41:17.1601070Z -    | 
2019-11-05T05:41:17.1601314Z -   ::: $SRC_DIR/libcore/option.rs:LL:COL
2019-11-05T05:41:17.1601777Z - LL | pub enum Option<T> {
2019-11-05T05:41:17.1601777Z - LL | pub enum Option<T> {
2019-11-05T05:41:17.1602210Z -    | ------------------ similarly named enum `Option` defined here
2019-11-05T05:41:17.1602307Z + LL |   type B = Opiton<u8>; // Misspelled type name from the prelude.
2019-11-05T05:41:17.1602418Z +    |            ^^^^^^ help: an enum with a similar name exists: `Option`
2019-11-05T05:41:17.1602700Z 27 error[E0412]: cannot find type `Baz` in this scope
2019-11-05T05:41:17.1603020Z 28   --> $DIR/levenshtein.rs:16:14
2019-11-05T05:41:17.1603073Z 
2019-11-05T05:41:17.1603107Z 
2019-11-05T05:41:17.1603107Z 
2019-11-05T05:41:17.1603195Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1603623Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/levenshtein.stderr
2019-11-05T05:41:17.1603996Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1604329Z To only update this specific test, also pass `--test-args resolve/levenshtein.rs`
2019-11-05T05:41:17.1604477Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1604549Z status: exit code: 1
2019-11-05T05:41:17.1604549Z status: exit code: 1
2019-11-05T05:41:17.1605574Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/levenshtein.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1606257Z ------------------------------------------
2019-11-05T05:41:17.1606334Z 
2019-11-05T05:41:17.1606610Z ------------------------------------------
2019-11-05T05:41:17.1606709Z stderr:
2019-11-05T05:41:17.1606709Z stderr:
2019-11-05T05:41:17.1606977Z ------------------------------------------
2019-11-05T05:41:17.1607087Z error[E0412]: cannot find type `esize` in this scope
2019-11-05T05:41:17.1607390Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:5:11
2019-11-05T05:41:17.1607495Z    |
2019-11-05T05:41:17.1607591Z LL | fn foo(c: esize) {} // Misspelled primitive type name.
2019-11-05T05:41:17.1607688Z    |           ^^^^^ help: a builtin type with a similar name exists: `isize`
2019-11-05T05:41:17.1607856Z error[E0412]: cannot find type `Baz` in this scope
2019-11-05T05:41:17.1608189Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:10:10
2019-11-05T05:41:17.1608274Z    |
2019-11-05T05:41:17.1608365Z LL | enum Bar { }
2019-11-05T05:41:17.1608365Z LL | enum Bar { }
2019-11-05T05:41:17.1608668Z    | ------------ similarly named enum `Bar` defined here
2019-11-05T05:41:17.1608898Z LL | 
2019-11-05T05:41:17.1609088Z LL | type A = Baz; // Misspelled type name.
2019-11-05T05:41:17.1609194Z    |          ^^^ help: an enum with a similar name exists: `Bar`
2019-11-05T05:41:17.1609249Z 
2019-11-05T05:41:17.1609336Z error[E0412]: cannot find type `Opiton` in this scope
2019-11-05T05:41:17.1609748Z    |
2019-11-05T05:41:17.1609748Z    |
2019-11-05T05:41:17.1609823Z LL |   type B = Opiton<u8>; // Misspelled type name from the prelude.
2019-11-05T05:41:17.1609937Z    |            ^^^^^^ help: an enum with a similar name exists: `Option`
2019-11-05T05:41:17.1610094Z error[E0412]: cannot find type `Baz` in this scope
2019-11-05T05:41:17.1610410Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:16:14
2019-11-05T05:41:17.1610495Z    |
2019-11-05T05:41:17.1610495Z    |
2019-11-05T05:41:17.1610596Z LL |     type A = Baz; // No suggestion here, Bar is not visible
2019-11-05T05:41:17.1610753Z 
2019-11-05T05:41:17.1610753Z 
2019-11-05T05:41:17.1610825Z error[E0425]: cannot find value `MAXITEM` in this scope
2019-11-05T05:41:17.1611221Z    |
2019-11-05T05:41:17.1611304Z LL | const MAX_ITEM: usize = 10;
2019-11-05T05:41:17.1611612Z    | --------------------------- similarly named constant `MAX_ITEM` defined here
2019-11-05T05:41:17.1611723Z ...
2019-11-05T05:41:17.1611723Z ...
2019-11-05T05:41:17.1611798Z LL |     let v = [0u32; MAXITEM]; // Misspelled constant name.
2019-11-05T05:41:17.1611916Z    |                    ^^^^^^^ help: a constant with a similar name exists: `MAX_ITEM`
2019-11-05T05:41:17.1612166Z error[E0425]: cannot find function `foobar` in this scope
2019-11-05T05:41:17.1612492Z   --> /checkout/src/test/ui/resolve/levenshtein.rs:26:5
2019-11-05T05:41:17.1612594Z    |
2019-11-05T05:41:17.1612594Z    |
2019-11-05T05:41:17.1612753Z LL | fn foo_bar() {}
2019-11-05T05:41:17.1613085Z    | --------------- similarly named function `foo_bar` defined here
2019-11-05T05:41:17.1613263Z LL |     foobar(); // Misspelled function name.
2019-11-05T05:41:17.1613263Z LL |     foobar(); // Misspelled function name.
2019-11-05T05:41:17.1613370Z    |     ^^^^^^ help: a function with a similar name exists: `foo_bar`
2019-11-05T05:41:17.1613428Z 
2019-11-05T05:41:17.1613518Z error[E0412]: cannot find type `first` in module `m`
2019-11-05T05:41:17.1613917Z    |
2019-11-05T05:41:17.1613981Z LL |     pub struct First;
2019-11-05T05:41:17.1613981Z LL |     pub struct First;
2019-11-05T05:41:17.1614297Z    |     ----------------- similarly named struct `First` defined here
2019-11-05T05:41:17.1614396Z ...
2019-11-05T05:41:17.1614489Z LL |     let b: m::first = m::second; // Misspelled item in module.
2019-11-05T05:41:17.1614614Z    |               ^^^^^ help: a struct with a similar name exists (notice the capitalization): `First`
2019-11-05T05:41:17.1614692Z 
2019-11-05T05:41:17.1614788Z error[E0425]: cannot find value `second` in module `m`
2019-11-05T05:41:17.1615194Z    |
2019-11-05T05:41:17.1615259Z LL |     pub struct Second;
2019-11-05T05:41:17.1615259Z LL |     pub struct Second;
2019-11-05T05:41:17.1615583Z    |     ------------------ similarly named unit struct `Second` defined here
2019-11-05T05:41:17.1615672Z ...
2019-11-05T05:41:17.1615765Z LL |     let b: m::first = m::second; // Misspelled item in module.
2019-11-05T05:41:17.1615875Z    |                          ^^^^^^ help: a unit struct with a similar name exists (notice the capitalization): `Second`
2019-11-05T05:41:17.1616043Z error: aborting due to 8 previous errors
2019-11-05T05:41:17.1616108Z 
2019-11-05T05:41:17.1616179Z Some errors have detailed explanations: E0412, E0425.
2019-11-05T05:41:17.1616508Z For more information about an error, try `rustc --explain E0412`.
---
2019-11-05T05:41:17.1617447Z 18    |
2019-11-05T05:41:17.1617529Z 19 LL | #[tests]
2019-11-05T05:41:17.1617611Z 20    |   ^^^^^ help: an attribute macro with a similar name exists: `test`
2019-11-05T05:41:17.1617875Z -    | 
2019-11-05T05:41:17.1618134Z -   ::: $SRC_DIR/libcore/macros.rs:LL:COL
2019-11-05T05:41:17.1618390Z -    |
2019-11-05T05:41:17.1618688Z - LL |     pub macro test($item:item) { /* compiler built-in */ }
2019-11-05T05:41:17.1619033Z -    |     -------------------------- similarly named attribute macro `test` defined here
2019-11-05T05:41:17.1619141Z 26 
2019-11-05T05:41:17.1619214Z 27 error: cannot find attribute `deprcated` in this scope
2019-11-05T05:41:17.1619557Z 
2019-11-05T05:41:17.1619603Z 
2019-11-05T05:41:17.1619696Z The actual stderr differed from the expected stderr.
2019-11-05T05:41:17.1620093Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/attribute-typos.stderr
2019-11-05T05:41:17.1620093Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/attribute-typos.stderr
2019-11-05T05:41:17.1620423Z To update references, rerun the tests and pass the `--bless` flag
2019-11-05T05:41:17.1620768Z To only update this specific test, also pass `--test-args suggestions/attribute-typos.rs`
2019-11-05T05:41:17.1620923Z error: 1 errors occurred comparing output.
2019-11-05T05:41:17.1620997Z status: exit code: 1
2019-11-05T05:41:17.1620997Z status: exit code: 1
2019-11-05T05:41:17.1622019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/attribute-typos.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/auxiliary" "-A" "unused"
2019-11-05T05:41:17.1622670Z ------------------------------------------
2019-11-05T05:41:17.1622744Z 
2019-11-05T05:41:17.1623004Z ------------------------------------------
2019-11-05T05:41:17.1623105Z stderr:
---
2019-11-05T05:41:17.1625633Z 
2019-11-05T05:41:17.1625724Z error: cannot find attribute `tests` in this scope
2019-11-05T05:41:17.1626021Z   --> /checkout/src/test/ui/suggestions/attribute-typos.rs:4:3
2019-11-05T05:41:17.1626136Z    |
2019-11-05T05:41:17.1626231Z LL | #[tests] //~ ERROR cannot find attribute `tests` in this scope
2019-11-05T05:41:17.1626328Z    |   ^^^^^ help: an attribute macro with a similar name exists: `test`
2019-11-05T05:41:17.1626403Z 
2019-11-05T05:41:17.1626481Z error: cannot find attribute `deprcated` in this scope
2019-11-05T05:41:17.1627002Z    |
2019-11-05T05:41:17.1627002Z    |
2019-11-05T05:41:17.1627096Z LL | #[deprcated] //~ ERROR cannot find attribute `deprcated` in this scope
2019-11-05T05:41:17.1627413Z    |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `deprecated`
2019-11-05T05:41:17.1627561Z error: aborting due to 4 previous errors
2019-11-05T05:41:17.1627625Z 
2019-11-05T05:41:17.1627904Z For more information about this error, try `rustc --explain E0658`.
2019-11-05T05:41:17.1627963Z 
---
2019-11-05T05:41:17.1631948Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-05T05:41:17.1632176Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-05T05:41:17.1632236Z 
2019-11-05T05:41:17.1632272Z 
2019-11-05T05:41:17.1634730Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-05T05:41:17.1635773Z 
2019-11-05T05:41:17.1635812Z 
2019-11-05T05:41:17.1636235Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-11-05T05:41:17.1636365Z Build completed unsuccessfully in 1:13:17
2019-11-05T05:41:17.1636365Z Build completed unsuccessfully in 1:13:17
2019-11-05T05:41:17.1636496Z == clock drift check ==
2019-11-05T05:41:17.1636590Z   local time: Tue Nov  5 05:41:17 UTC 2019
2019-11-05T05:41:17.6880200Z   network time: Tue, 05 Nov 2019 05:41:17 GMT
2019-11-05T05:41:17.6885539Z == end clock drift check ==
2019-11-05T05:41:19.0433474Z 
2019-11-05T05:41:19.0544425Z ##[error]Bash exited with code '1'.
2019-11-05T05:41:19.0606842Z ##[section]Starting: Checkout
2019-11-05T05:41:19.0617052Z ==============================================================================
2019-11-05T05:41:19.0622276Z Task         : Get sources
2019-11-05T05:41:19.0622394Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

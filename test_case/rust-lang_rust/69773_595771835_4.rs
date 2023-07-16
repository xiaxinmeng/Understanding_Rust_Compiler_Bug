
2020-03-06T13:39:14.1314528Z     let f: extern "C" fn(*mut i32) = transmute(foo as extern "C" fn(_));
2020-03-06T13:39:14.1314884Z     let f: extern "C" fn(*mut i32) = transmute(foo as usize); // works too
2020-03-06T13:39:14.1315244Z 
2020-03-06T13:39:14.1315244Z 
2020-03-06T13:39:14.1315479Z The same applies to transmutes to `*mut fn()`, which were observed in practice.
2020-03-06T13:39:14.1316146Z The intention is typically to describe a function pointer, but just `fn()`
2020-03-06T13:39:14.1316478Z alone suffices for that. `*mut fn()` is a pointer to a fn pointer.
2020-03-06T13:39:14.1316478Z alone suffices for that. `*mut fn()` is a pointer to a fn pointer.
2020-03-06T13:39:14.1316830Z (Since these values are typically just passed to C code, however, this rarely
2020-03-06T13:39:14.1317255Z 
2020-03-06T13:39:14.1317255Z 
2020-03-06T13:39:14.1317708Z [rfc401]: https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md
2020-03-06T13:39:14.1318290Z ------------------------------------------
2020-03-06T13:39:14.1318478Z stderr:
2020-03-06T13:39:14.1318835Z ------------------------------------------
2020-03-06T13:39:14.1319173Z 
---
2020-03-06T13:39:14.1320480Z diff of stderr:
2020-03-06T13:39:14.1320604Z 
2020-03-06T13:39:14.1321022Z 292   --> $DIR/attr-stmt-expr-attr-bad.rs:80:32
2020-03-06T13:39:14.1321256Z 293    |
2020-03-06T13:39:14.1321501Z 294 LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
2020-03-06T13:39:14.1322105Z -    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1322748Z +    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1323374Z 297    |                        previous outer attribute
2020-03-06T13:39:14.1323600Z 298    |
2020-03-06T13:39:14.1323714Z 
2020-03-06T13:39:14.1324111Z 302   --> $DIR/attr-stmt-expr-attr-bad.rs:82:32
2020-03-06T13:39:14.1324111Z 302   --> $DIR/attr-stmt-expr-attr-bad.rs:82:32
2020-03-06T13:39:14.1324355Z 303    |
2020-03-06T13:39:14.1324586Z 304 LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
2020-03-06T13:39:14.1325161Z -    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1325817Z +    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1326418Z 307    |                        previous outer attribute
2020-03-06T13:39:14.1326657Z 308    |
2020-03-06T13:39:14.1326770Z 
2020-03-06T13:39:14.1327164Z 312   --> $DIR/attr-stmt-expr-attr-bad.rs:84:32
2020-03-06T13:39:14.1327164Z 312   --> $DIR/attr-stmt-expr-attr-bad.rs:84:32
2020-03-06T13:39:14.1327408Z 313    |
2020-03-06T13:39:14.1327647Z 314 LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
2020-03-06T13:39:14.1328237Z -    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1328887Z +    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1329488Z 317    |                        previous outer attribute
2020-03-06T13:39:14.1329726Z 318    |
2020-03-06T13:39:14.1329838Z 
2020-03-06T13:39:14.1330233Z 322   --> $DIR/attr-stmt-expr-attr-bad.rs:86:32
2020-03-06T13:39:14.1330233Z 322   --> $DIR/attr-stmt-expr-attr-bad.rs:86:32
2020-03-06T13:39:14.1330674Z 323    |
2020-03-06T13:39:14.1330999Z 324 LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
2020-03-06T13:39:14.1331612Z -    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1332249Z +    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1332866Z 327    |                        previous outer attribute
2020-03-06T13:39:14.1333158Z 328    |
2020-03-06T13:39:14.1333283Z 
2020-03-06T13:39:14.1333685Z 332   --> $DIR/attr-stmt-expr-attr-bad.rs:88:32
2020-03-06T13:39:14.1333685Z 332   --> $DIR/attr-stmt-expr-attr-bad.rs:88:32
2020-03-06T13:39:14.1333914Z 333    |
2020-03-06T13:39:14.1334156Z 334 LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
2020-03-06T13:39:14.1334848Z -    |                        ------- ^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1335435Z +    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1336113Z 337    |                        previous outer attribute
2020-03-06T13:39:14.1336295Z 338    |
2020-03-06T13:39:14.1336385Z 
2020-03-06T13:39:14.1336466Z 
2020-03-06T13:39:14.1336466Z 
2020-03-06T13:39:14.1336648Z The actual stderr differed from the expected stderr.
2020-03-06T13:39:14.1337420Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/attr-stmt-expr-attr-bad.stderr
2020-03-06T13:39:14.1338053Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T13:39:14.1338618Z To only update this specific test, also pass `--test-args parser/attr-stmt-expr-attr-bad.rs`
2020-03-06T13:39:14.1339040Z error: 1 errors occurred comparing output.
2020-03-06T13:39:14.1339388Z status: exit code: 1
2020-03-06T13:39:14.1339388Z status: exit code: 1
2020-03-06T13:39:14.1340999Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr-stmt-expr-attr-bad/auxiliary"
2020-03-06T13:39:14.1342325Z ------------------------------------------
2020-03-06T13:39:14.1342481Z 
2020-03-06T13:39:14.1342775Z ------------------------------------------
2020-03-06T13:39:14.1342939Z stderr:
2020-03-06T13:39:14.1342939Z stderr:
2020-03-06T13:39:14.1343237Z ------------------------------------------
2020-03-06T13:39:14.1343491Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1344157Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:5:36
2020-03-06T13:39:14.1344410Z    |
2020-03-06T13:39:14.1344619Z LL | #[cfg(FALSE)] fn e() { let _ = box #![attr] 0; }
2020-03-06T13:39:14.1345081Z    |
2020-03-06T13:39:14.1345081Z    |
2020-03-06T13:39:14.1345548Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1346151Z error: expected expression, found `]`
2020-03-06T13:39:14.1346642Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:7:40
2020-03-06T13:39:14.1346878Z    |
2020-03-06T13:39:14.1346878Z    |
2020-03-06T13:39:14.1347074Z LL | #[cfg(FALSE)] fn e() { let _ = [#[attr]]; }
2020-03-06T13:39:14.1347578Z 
2020-03-06T13:39:14.1347578Z 
2020-03-06T13:39:14.1347834Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
2020-03-06T13:39:14.1348697Z    |
2020-03-06T13:39:14.1348697Z    |
2020-03-06T13:39:14.1348898Z LL | #[cfg(FALSE)] fn e() { let _ = foo#[attr](); }
2020-03-06T13:39:14.1349221Z    |                                   ^ expected one of 7 possible tokens
2020-03-06T13:39:14.1349632Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1350150Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:36
2020-03-06T13:39:14.1350453Z    |
2020-03-06T13:39:14.1350453Z    |
2020-03-06T13:39:14.1350656Z LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
2020-03-06T13:39:14.1351132Z    |
2020-03-06T13:39:14.1351132Z    |
2020-03-06T13:39:14.1351587Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1352193Z error: expected expression, found `)`
2020-03-06T13:39:14.1352683Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:11:44
2020-03-06T13:39:14.1352922Z    |
2020-03-06T13:39:14.1352922Z    |
2020-03-06T13:39:14.1353136Z LL | #[cfg(FALSE)] fn e() { let _ = foo(#![attr]); }
2020-03-06T13:39:14.1353647Z 
2020-03-06T13:39:14.1353851Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1354381Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:38
2020-03-06T13:39:14.1354621Z    |
2020-03-06T13:39:14.1354621Z    |
2020-03-06T13:39:14.1354827Z LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
2020-03-06T13:39:14.1355309Z    |
2020-03-06T13:39:14.1355309Z    |
2020-03-06T13:39:14.1355762Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1356375Z error: expected expression, found `)`
2020-03-06T13:39:14.1356948Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:14:46
2020-03-06T13:39:14.1357170Z    |
2020-03-06T13:39:14.1357170Z    |
2020-03-06T13:39:14.1357349Z LL | #[cfg(FALSE)] fn e() { let _ = x.foo(#![attr]); }
2020-03-06T13:39:14.1357796Z 
2020-03-06T13:39:14.1357986Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1358437Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:17:36
2020-03-06T13:39:14.1358644Z    |
2020-03-06T13:39:14.1358644Z    |
2020-03-06T13:39:14.1358833Z LL | #[cfg(FALSE)] fn e() { let _ = 0 + #![attr] 0; }
2020-03-06T13:39:14.1359426Z    |
2020-03-06T13:39:14.1359426Z    |
2020-03-06T13:39:14.1359893Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1360524Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1361048Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:19:33
2020-03-06T13:39:14.1361285Z    |
2020-03-06T13:39:14.1361285Z    |
2020-03-06T13:39:14.1361482Z LL | #[cfg(FALSE)] fn e() { let _ = !#![attr] 0; }
2020-03-06T13:39:14.1361948Z    |
2020-03-06T13:39:14.1361948Z    |
2020-03-06T13:39:14.1362398Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1363038Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1363723Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:21:33
2020-03-06T13:39:14.1363979Z    |
2020-03-06T13:39:14.1363979Z    |
2020-03-06T13:39:14.1364439Z LL | #[cfg(FALSE)] fn e() { let _ = -#![attr] 0; }
2020-03-06T13:39:14.1364928Z    |
2020-03-06T13:39:14.1364928Z    |
2020-03-06T13:39:14.1365431Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1365888Z 
2020-03-06T13:39:14.1366162Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `#`
2020-03-06T13:39:14.1373722Z    |
2020-03-06T13:39:14.1373722Z    |
2020-03-06T13:39:14.1373952Z LL | #[cfg(FALSE)] fn e() { let _ = x #![attr] as Y; }
2020-03-06T13:39:14.1374313Z    |                                  ^ expected one of 7 possible tokens
2020-03-06T13:39:14.1374874Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1375451Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:25:35
2020-03-06T13:39:14.1375714Z    |
2020-03-06T13:39:14.1375714Z    |
2020-03-06T13:39:14.1376120Z LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] foo; }
2020-03-06T13:39:14.1377018Z    |
2020-03-06T13:39:14.1377018Z    |
2020-03-06T13:39:14.1385203Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1385924Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1386633Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:27:40
2020-03-06T13:39:14.1386876Z    |
2020-03-06T13:39:14.1386876Z    |
2020-03-06T13:39:14.1387108Z LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] foo; }
2020-03-06T13:39:14.1387751Z    |
2020-03-06T13:39:14.1387751Z    |
2020-03-06T13:39:14.1388231Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1389131Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1390035Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:29:35
2020-03-06T13:39:14.1390305Z    |
2020-03-06T13:39:14.1390305Z    |
2020-03-06T13:39:14.1390763Z LL | #[cfg(FALSE)] fn e() { let _ = || #![attr] {foo}; }
2020-03-06T13:39:14.1391438Z    |
2020-03-06T13:39:14.1391438Z    |
2020-03-06T13:39:14.1391928Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1392620Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1393238Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:31:40
2020-03-06T13:39:14.1393506Z    |
2020-03-06T13:39:14.1393506Z    |
2020-03-06T13:39:14.1393762Z LL | #[cfg(FALSE)] fn e() { let _ = move || #![attr] {foo}; }
2020-03-06T13:39:14.1394286Z    |
2020-03-06T13:39:14.1394286Z    |
2020-03-06T13:39:14.1394788Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1396177Z error: expected expression, found `..`
2020-03-06T13:39:14.1396788Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:33:40
2020-03-06T13:39:14.1397049Z    |
2020-03-06T13:39:14.1397049Z    |
2020-03-06T13:39:14.1397283Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..#[attr] 0; }
2020-03-06T13:39:14.1397849Z 
2020-03-06T13:39:14.1398034Z error: expected expression, found `..`
2020-03-06T13:39:14.1398779Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:35:40
2020-03-06T13:39:14.1399048Z    |
2020-03-06T13:39:14.1399048Z    |
2020-03-06T13:39:14.1399263Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] ..; }
2020-03-06T13:39:14.1409244Z 
2020-03-06T13:39:14.1409517Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1410624Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:37:41
2020-03-06T13:39:14.1411089Z    |
2020-03-06T13:39:14.1411089Z    |
2020-03-06T13:39:14.1411932Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &#![attr] 0; }
2020-03-06T13:39:14.1412482Z    |
2020-03-06T13:39:14.1412482Z    |
2020-03-06T13:39:14.1412974Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1413684Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1420252Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:39:45
2020-03-06T13:39:14.1420551Z    |
2020-03-06T13:39:14.1420551Z    |
2020-03-06T13:39:14.1420790Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] &mut #![attr] 0; }
2020-03-06T13:39:14.1421361Z    |
2020-03-06T13:39:14.1421361Z    |
2020-03-06T13:39:14.1421855Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1422573Z error: attributes are not yet allowed on `if` expressions
2020-03-06T13:39:14.1423627Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:41:32
2020-03-06T13:39:14.1423905Z    |
2020-03-06T13:39:14.1423905Z    |
2020-03-06T13:39:14.1424130Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] if 0 {}; }
2020-03-06T13:39:14.1424613Z 
2020-03-06T13:39:14.1424613Z 
2020-03-06T13:39:14.1424785Z error: expected `{`, found `#`
2020-03-06T13:39:14.1425570Z    |
2020-03-06T13:39:14.1425570Z    |
2020-03-06T13:39:14.1425792Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 #[attr] {}; }
2020-03-06T13:39:14.1426474Z    |                                --   ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1427201Z    |                                |    expected `{`
2020-03-06T13:39:14.1427560Z    |                                this `if` expression has a condition, but no block
2020-03-06T13:39:14.1427808Z 
2020-03-06T13:39:14.1428045Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1428045Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1428598Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:45:38
2020-03-06T13:39:14.1428856Z    |
2020-03-06T13:39:14.1429101Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {#![attr]}; }
2020-03-06T13:39:14.1429606Z    |
2020-03-06T13:39:14.1429606Z    |
2020-03-06T13:39:14.1430112Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1430574Z 
2020-03-06T13:39:14.1430823Z error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
2020-03-06T13:39:14.1431667Z    |
2020-03-06T13:39:14.1431667Z    |
2020-03-06T13:39:14.1431905Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} #[attr] else {}; }
2020-03-06T13:39:14.1432316Z    |                                        ^ expected one of `.`, `;`, `?`, `else`, or an operator
2020-03-06T13:39:14.1432586Z 
2020-03-06T13:39:14.1432760Z error: expected `{`, found `#`
2020-03-06T13:39:14.1433697Z    |
2020-03-06T13:39:14.1433697Z    |
2020-03-06T13:39:14.1433931Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] {}; }
2020-03-06T13:39:14.1434647Z    |                                             ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1435423Z    |                                             expected `{`
2020-03-06T13:39:14.1435691Z 
2020-03-06T13:39:14.1435925Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1436480Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:51:46
2020-03-06T13:39:14.1436480Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:51:46
2020-03-06T13:39:14.1436738Z    |
2020-03-06T13:39:14.1436989Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else {#![attr]}; }
2020-03-06T13:39:14.1437534Z    |
2020-03-06T13:39:14.1437534Z    |
2020-03-06T13:39:14.1438044Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1438724Z error: attributes are not yet allowed on `if` expressions
2020-03-06T13:39:14.1439277Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:53:45
2020-03-06T13:39:14.1439549Z    |
2020-03-06T13:39:14.1439549Z    |
2020-03-06T13:39:14.1439794Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
2020-03-06T13:39:14.1440341Z 
2020-03-06T13:39:14.1440341Z 
2020-03-06T13:39:14.1440514Z error: expected `{`, found `#`
2020-03-06T13:39:14.1441285Z    |
2020-03-06T13:39:14.1441285Z    |
2020-03-06T13:39:14.1441529Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else #[attr] if 0 {}; }
2020-03-06T13:39:14.1442282Z    |                                             ^       -------- help: try placing this code inside a block: `{ if 0 {}; }`
2020-03-06T13:39:14.1443077Z    |                                             expected `{`
2020-03-06T13:39:14.1443279Z 
2020-03-06T13:39:14.1443279Z 
2020-03-06T13:39:14.1443451Z error: expected `{`, found `#`
2020-03-06T13:39:14.1444223Z    |
2020-03-06T13:39:14.1444223Z    |
2020-03-06T13:39:14.1444472Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 #[attr] {}; }
2020-03-06T13:39:14.1445219Z    |                                             --   ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1446021Z    |                                             |    expected `{`
2020-03-06T13:39:14.1446437Z    |                                             this `if` expression has a condition, but no block
2020-03-06T13:39:14.1446709Z 
2020-03-06T13:39:14.1446935Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1446935Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1447500Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:58:51
2020-03-06T13:39:14.1447756Z    |
2020-03-06T13:39:14.1448001Z LL | #[cfg(FALSE)] fn e() { let _ = if 0 {} else if 0 {#![attr]}; }
2020-03-06T13:39:14.1448585Z    |
2020-03-06T13:39:14.1448585Z    |
2020-03-06T13:39:14.1449081Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1449771Z error: attributes are not yet allowed on `if` expressions
2020-03-06T13:39:14.1450449Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:60:32
2020-03-06T13:39:14.1450724Z    |
2020-03-06T13:39:14.1450724Z    |
2020-03-06T13:39:14.1451066Z LL | #[cfg(FALSE)] fn e() { let _ = #[attr] if let _ = 0 {}; }
2020-03-06T13:39:14.1451570Z 
2020-03-06T13:39:14.1451570Z 
2020-03-06T13:39:14.1451742Z error: expected `{`, found `#`
2020-03-06T13:39:14.1452464Z    |
2020-03-06T13:39:14.1452464Z    |
2020-03-06T13:39:14.1452697Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 #[attr] {}; }
2020-03-06T13:39:14.1453354Z    |                                --           ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1454150Z    |                                |            expected `{`
2020-03-06T13:39:14.1454497Z    |                                this `if` expression has a condition, but no block
2020-03-06T13:39:14.1454725Z 
2020-03-06T13:39:14.1454943Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1454943Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1455460Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:64:46
2020-03-06T13:39:14.1455703Z    |
2020-03-06T13:39:14.1455934Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {#![attr]}; }
2020-03-06T13:39:14.1456440Z    |
2020-03-06T13:39:14.1456440Z    |
2020-03-06T13:39:14.1456908Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1457339Z 
2020-03-06T13:39:14.1457568Z error: expected one of `.`, `;`, `?`, `else`, or an operator, found `#`
2020-03-06T13:39:14.1458349Z    |
2020-03-06T13:39:14.1458349Z    |
2020-03-06T13:39:14.1458580Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} #[attr] else {}; }
2020-03-06T13:39:14.1458974Z    |                                                ^ expected one of `.`, `;`, `?`, `else`, or an operator
2020-03-06T13:39:14.1459253Z 
2020-03-06T13:39:14.1459417Z error: expected `{`, found `#`
2020-03-06T13:39:14.1460135Z    |
2020-03-06T13:39:14.1460135Z    |
2020-03-06T13:39:14.1460364Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] {}; }
2020-03-06T13:39:14.1461056Z    |                                                     ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1461836Z    |                                                     expected `{`
2020-03-06T13:39:14.1462037Z 
2020-03-06T13:39:14.1462239Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1462763Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:70:54
2020-03-06T13:39:14.1462763Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:70:54
2020-03-06T13:39:14.1463002Z    |
2020-03-06T13:39:14.1463232Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else {#![attr]}; }
2020-03-06T13:39:14.1463798Z    |
2020-03-06T13:39:14.1463798Z    |
2020-03-06T13:39:14.1464249Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1464888Z error: attributes are not yet allowed on `if` expressions
2020-03-06T13:39:14.1465394Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:72:53
2020-03-06T13:39:14.1465650Z    |
2020-03-06T13:39:14.1465650Z    |
2020-03-06T13:39:14.1465904Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
2020-03-06T13:39:14.1466456Z 
2020-03-06T13:39:14.1466456Z 
2020-03-06T13:39:14.1466616Z error: expected `{`, found `#`
2020-03-06T13:39:14.1467332Z    |
2020-03-06T13:39:14.1467332Z    |
2020-03-06T13:39:14.1467632Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else #[attr] if let _ = 0 {}; }
2020-03-06T13:39:14.1468430Z    |                                                     ^       ---------------- help: try placing this code inside a block: `{ if let _ = 0 {}; }`
2020-03-06T13:39:14.1469248Z    |                                                     expected `{`
2020-03-06T13:39:14.1469497Z 
2020-03-06T13:39:14.1469497Z 
2020-03-06T13:39:14.1469658Z error: expected `{`, found `#`
2020-03-06T13:39:14.1470377Z    |
2020-03-06T13:39:14.1470377Z    |
2020-03-06T13:39:14.1470631Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 #[attr] {}; }
2020-03-06T13:39:14.1471396Z    |                                                     --           ^       --- help: try placing this code inside a block: `{ {}; }`
2020-03-06T13:39:14.1472242Z    |                                                     |            expected `{`
2020-03-06T13:39:14.1472667Z    |                                                     this `if` expression has a condition, but no block
2020-03-06T13:39:14.1472928Z 
2020-03-06T13:39:14.1473131Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1473131Z error: an inner attribute is not permitted in this context
2020-03-06T13:39:14.1473654Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:77:67
2020-03-06T13:39:14.1473897Z    |
2020-03-06T13:39:14.1474151Z LL | #[cfg(FALSE)] fn e() { let _ = if let _ = 0 {} else if let _ = 0 {#![attr]}; }
2020-03-06T13:39:14.1474774Z    |
2020-03-06T13:39:14.1474774Z    |
2020-03-06T13:39:14.1475231Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1475898Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1476427Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:80:32
2020-03-06T13:39:14.1476679Z    |
2020-03-06T13:39:14.1476679Z    |
2020-03-06T13:39:14.1476889Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] let _ = 0; }
2020-03-06T13:39:14.1477425Z    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1477979Z    |                        previous outer attribute
2020-03-06T13:39:14.1478175Z    |
2020-03-06T13:39:14.1478175Z    |
2020-03-06T13:39:14.1478639Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1479283Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1479816Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:82:32
2020-03-06T13:39:14.1480070Z    |
2020-03-06T13:39:14.1480070Z    |
2020-03-06T13:39:14.1480303Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] 0; }
2020-03-06T13:39:14.1480830Z    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1481382Z    |                        previous outer attribute
2020-03-06T13:39:14.1481578Z    |
2020-03-06T13:39:14.1481578Z    |
2020-03-06T13:39:14.1482048Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1482965Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1483509Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:84:32
2020-03-06T13:39:14.1483746Z    |
2020-03-06T13:39:14.1483746Z    |
2020-03-06T13:39:14.1483952Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!(); }
2020-03-06T13:39:14.1484543Z    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1485081Z    |                        previous outer attribute
2020-03-06T13:39:14.1485289Z    |
2020-03-06T13:39:14.1485289Z    |
2020-03-06T13:39:14.1485736Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1486437Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1486972Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:86:32
2020-03-06T13:39:14.1487209Z    |
2020-03-06T13:39:14.1487209Z    |
2020-03-06T13:39:14.1487428Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo![]; }
2020-03-06T13:39:14.1487959Z    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1488509Z    |                        previous outer attribute
2020-03-06T13:39:14.1488707Z    |
2020-03-06T13:39:14.1488707Z    |
2020-03-06T13:39:14.1489155Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1489818Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1490525Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:88:32
2020-03-06T13:39:14.1490763Z    |
2020-03-06T13:39:14.1490763Z    |
2020-03-06T13:39:14.1490982Z LL | #[cfg(FALSE)] fn s() { #[attr] #![attr] foo!{}; }
2020-03-06T13:39:14.1491525Z    |                        ------- ^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1492071Z    |                        previous outer attribute
2020-03-06T13:39:14.1492269Z    |
2020-03-06T13:39:14.1492269Z    |
2020-03-06T13:39:14.1492720Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1493339Z error[E0586]: inclusive range with no end
2020-03-06T13:39:14.1493828Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:94:35
2020-03-06T13:39:14.1494085Z    |
2020-03-06T13:39:14.1494085Z    |
2020-03-06T13:39:14.1494306Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
2020-03-06T13:39:14.1494892Z    |
2020-03-06T13:39:14.1494892Z    |
2020-03-06T13:39:14.1495130Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-03-06T13:39:14.1495345Z 
2020-03-06T13:39:14.1495540Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-03-06T13:39:14.1496517Z    |
2020-03-06T13:39:14.1496517Z    |
2020-03-06T13:39:14.1496738Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] 10 => () } }
2020-03-06T13:39:14.1497087Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-03-06T13:39:14.1497483Z error[E0586]: inclusive range with no end
2020-03-06T13:39:14.1497985Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:97:35
2020-03-06T13:39:14.1498223Z    |
2020-03-06T13:39:14.1498223Z    |
2020-03-06T13:39:14.1498630Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
2020-03-06T13:39:14.1499218Z    |
2020-03-06T13:39:14.1499218Z    |
2020-03-06T13:39:14.1499457Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-03-06T13:39:14.1499672Z 
2020-03-06T13:39:14.1499882Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-03-06T13:39:14.1500618Z    |
2020-03-06T13:39:14.1500618Z    |
2020-03-06T13:39:14.1501089Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] -10 => () } }
2020-03-06T13:39:14.1501434Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-03-06T13:39:14.1501825Z error: unexpected token: `#`
2020-03-06T13:39:14.1502306Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:100:39
2020-03-06T13:39:14.1502547Z    |
2020-03-06T13:39:14.1502547Z    |
2020-03-06T13:39:14.1502944Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=-#[attr] 10 => () } }
2020-03-06T13:39:14.1503460Z 
2020-03-06T13:39:14.1503641Z error[E0586]: inclusive range with no end
2020-03-06T13:39:14.1504151Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:102:35
2020-03-06T13:39:14.1504391Z    |
2020-03-06T13:39:14.1504391Z    |
2020-03-06T13:39:14.1504614Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
2020-03-06T13:39:14.1505200Z    |
2020-03-06T13:39:14.1505200Z    |
2020-03-06T13:39:14.1505445Z    = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
2020-03-06T13:39:14.1505661Z 
2020-03-06T13:39:14.1505871Z error: expected one of `=>`, `if`, or `|`, found `#`
2020-03-06T13:39:14.1506613Z    |
2020-03-06T13:39:14.1506613Z    |
2020-03-06T13:39:14.1506849Z LL | #[cfg(FALSE)] fn e() { match 0 { 0..=#[attr] FOO => () } }
2020-03-06T13:39:14.1507190Z    |                                      ^ expected one of `=>`, `if`, or `|`
2020-03-06T13:39:14.1507583Z error: unexpected token: `#`
2020-03-06T13:39:14.1508056Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:106:34
2020-03-06T13:39:14.1508294Z    |
2020-03-06T13:39:14.1508294Z    |
2020-03-06T13:39:14.1508499Z LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
2020-03-06T13:39:14.1508924Z 
2020-03-06T13:39:14.1508924Z 
2020-03-06T13:39:14.1509142Z error: expected one of `.`, `;`, `?`, or an operator, found `#`
2020-03-06T13:39:14.1509915Z    |
2020-03-06T13:39:14.1509915Z    |
2020-03-06T13:39:14.1510119Z LL | #[cfg(FALSE)] fn e() { let _ = x.#![attr]foo(); }
2020-03-06T13:39:14.1510467Z    |                                  ^ expected one of `.`, `;`, `?`, or an operator
2020-03-06T13:39:14.1510859Z error: unexpected token: `#`
2020-03-06T13:39:14.1511407Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:109:34
2020-03-06T13:39:14.1511628Z    |
2020-03-06T13:39:14.1511628Z    |
2020-03-06T13:39:14.1511804Z LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
2020-03-06T13:39:14.1512174Z 
2020-03-06T13:39:14.1512174Z 
2020-03-06T13:39:14.1512360Z error: expected one of `.`, `;`, `?`, or an operator, found `#`
2020-03-06T13:39:14.1513034Z    |
2020-03-06T13:39:14.1513034Z    |
2020-03-06T13:39:14.1513216Z LL | #[cfg(FALSE)] fn e() { let _ = x.#[attr]foo(); }
2020-03-06T13:39:14.1513505Z    |                                  ^ expected one of `.`, `;`, `?`, or an operator
2020-03-06T13:39:14.1513880Z error: expected statement after outer attribute
2020-03-06T13:39:14.1514310Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:114:37
2020-03-06T13:39:14.1514520Z    |
2020-03-06T13:39:14.1514520Z    |
2020-03-06T13:39:14.1514713Z LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr]; } } }
2020-03-06T13:39:14.1515104Z 
2020-03-06T13:39:14.1515278Z error: expected statement after outer attribute
2020-03-06T13:39:14.1515708Z   --> /checkout/src/test/ui/parser/attr-stmt-expr-attr-bad.rs:116:37
2020-03-06T13:39:14.1515916Z    |
2020-03-06T13:39:14.1515916Z    |
2020-03-06T13:39:14.1516108Z LL | #[cfg(FALSE)] fn e() { { fn foo() { #[attr] } } }
2020-03-06T13:39:14.1516494Z 
2020-03-06T13:39:14.1516689Z error: aborting due to 57 previous errors
2020-03-06T13:39:14.1516847Z 
2020-03-06T13:39:14.1517407Z For more information about this error, try `rustc --explain E0586`.
---
2020-03-06T13:39:14.1518885Z diff of stderr:
2020-03-06T13:39:14.1519016Z 
2020-03-06T13:39:14.1519340Z 7    | |___- previous doc comment
2020-03-06T13:39:14.1519518Z 8 LL | 
2020-03-06T13:39:14.1519693Z 9 LL |   #![recursion_limit="100"]
2020-03-06T13:39:14.1520181Z -    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1520536Z +    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1520777Z 11    |
2020-03-06T13:39:14.1521264Z 12    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1521822Z 
2020-03-06T13:39:14.1521926Z 
2020-03-06T13:39:14.1522120Z The actual stderr differed from the expected stderr.
2020-03-06T13:39:14.1522799Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/inner-attr-after-doc-comment.stderr
2020-03-06T13:39:14.1522799Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/inner-attr-after-doc-comment.stderr
2020-03-06T13:39:14.1523457Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T13:39:14.1524035Z To only update this specific test, also pass `--test-args parser/inner-attr-after-doc-comment.rs`
2020-03-06T13:39:14.1524478Z error: 1 errors occurred comparing output.
2020-03-06T13:39:14.1527363Z status: exit code: 1
2020-03-06T13:39:14.1527363Z status: exit code: 1
2020-03-06T13:39:14.1529414Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/inner-attr-after-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/auxiliary"
2020-03-06T13:39:14.1531624Z ------------------------------------------
2020-03-06T13:39:14.1531791Z 
2020-03-06T13:39:14.1532309Z ------------------------------------------
2020-03-06T13:39:14.1532516Z stderr:
---
2020-03-06T13:39:14.1534416Z LL | |  * My module
2020-03-06T13:39:14.1534585Z LL | |  */
2020-03-06T13:39:14.1534917Z    | |___- previous doc comment
2020-03-06T13:39:14.1535086Z LL | 
2020-03-06T13:39:14.1535270Z LL |   #![recursion_limit="100"]
2020-03-06T13:39:14.1535554Z    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1535792Z    |
2020-03-06T13:39:14.1536259Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1536856Z error: aborting due to previous error
2020-03-06T13:39:14.1537009Z 
2020-03-06T13:39:14.1537114Z 
2020-03-06T13:39:14.1537450Z ------------------------------------------
---
2020-03-06T13:39:14.1539557Z 7 LL | #![recursion_limit="100"]
2020-03-06T13:39:14.1540060Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2020-03-06T13:39:14.1540452Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1540771Z 9    |
2020-03-06T13:39:14.1541273Z 10    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1541879Z 
2020-03-06T13:39:14.1541976Z 
2020-03-06T13:39:14.1542185Z The actual stderr differed from the expected stderr.
2020-03-06T13:39:14.1542852Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr/inner-attr.stderr
2020-03-06T13:39:14.1542852Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr/inner-attr.stderr
2020-03-06T13:39:14.1543470Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T13:39:14.1544051Z To only update this specific test, also pass `--test-args parser/inner-attr.rs`
2020-03-06T13:39:14.1544500Z error: 1 errors occurred comparing output.
2020-03-06T13:39:14.1544739Z status: exit code: 1
2020-03-06T13:39:14.1544739Z status: exit code: 1
2020-03-06T13:39:14.1546738Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/inner-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr/auxiliary"
2020-03-06T13:39:14.1548299Z ------------------------------------------
2020-03-06T13:39:14.1548476Z 
2020-03-06T13:39:14.1548855Z ------------------------------------------
2020-03-06T13:39:14.1549061Z stderr:
2020-03-06T13:39:14.1549061Z stderr:
2020-03-06T13:39:14.1549434Z ------------------------------------------
2020-03-06T13:39:14.1549767Z error: an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1550306Z   --> /checkout/src/test/ui/parser/inner-attr.rs:3:1
2020-03-06T13:39:14.1550537Z    |
2020-03-06T13:39:14.1550726Z LL | #[feature(lang_items)]
2020-03-06T13:39:14.1551167Z    | ---------------------- previous outer attribute
2020-03-06T13:39:14.1551386Z LL | 
2020-03-06T13:39:14.1551703Z LL | #![recursion_limit="100"] //~ ERROR an inner attribute is not permitted following an outer attribute
2020-03-06T13:39:14.1552149Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attribute
2020-03-06T13:39:14.1552400Z    |
2020-03-06T13:39:14.1552916Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-03-06T13:39:14.1553571Z error: aborting due to previous error
2020-03-06T13:39:14.1553739Z 
2020-03-06T13:39:14.1553857Z 
2020-03-06T13:39:14.1554225Z ------------------------------------------
---
2020-03-06T13:39:14.1558324Z 12 LL |     A
2020-03-06T13:39:14.1558448Z 
2020-03-06T13:39:14.1558549Z 
2020-03-06T13:39:14.1558758Z The actual stderr differed from the expected stderr.
2020-03-06T13:39:14.1559424Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad/derive-bad.stderr
2020-03-06T13:39:14.1560108Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T13:39:14.1560699Z To only update this specific test, also pass `--test-args proc-macro/derive-bad.rs`
2020-03-06T13:39:14.1561156Z error: 1 errors occurred comparing output.
2020-03-06T13:39:14.1561396Z status: exit code: 1
2020-03-06T13:39:14.1561396Z status: exit code: 1
2020-03-06T13:39:14.1563331Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad/auxiliary"
2020-03-06T13:39:14.1564917Z ------------------------------------------
2020-03-06T13:39:14.1565092Z 
2020-03-06T13:39:14.1565454Z ------------------------------------------
2020-03-06T13:39:14.1565672Z stderr:
2020-03-06T13:39:14.1565672Z stderr:
2020-03-06T13:39:14.1566043Z ------------------------------------------
2020-03-06T13:39:14.1566297Z error: expected `:`, found `}`
2020-03-06T13:39:14.1567020Z    |
2020-03-06T13:39:14.1567161Z LL |     A
2020-03-06T13:39:14.1567348Z    |     ^ expected `:`
2020-03-06T13:39:14.1567541Z    |
---
2020-03-06T13:39:14.1574794Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-06T13:39:14.1575186Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-06T13:39:14.1575405Z 
2020-03-06T13:39:14.1575495Z 
2020-03-06T13:39:14.1578964Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-06T13:39:14.1582814Z 
2020-03-06T13:39:14.1582939Z 
2020-03-06T13:39:14.1583178Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-06T13:39:14.1583507Z Build completed unsuccessfully in 0:59:35
2020-03-06T13:39:14.1583507Z Build completed unsuccessfully in 0:59:35
2020-03-06T13:39:14.1583770Z == clock drift check ==
2020-03-06T13:39:14.1584019Z   local time: Fri Mar  6 13:39:14 UTC 2020
2020-03-06T13:39:14.2826128Z   network time: Fri, 06 Mar 2020 13:39:14 GMT
2020-03-06T13:39:14.2826593Z == end clock drift check ==
2020-03-06T13:39:15.0451914Z 
2020-03-06T13:39:15.0502278Z ##[error]Bash exited with code '1'.
2020-03-06T13:39:15.0517425Z ##[section]Finishing: Run build
2020-03-06T13:39:15.0573724Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69773/merge to s
2020-03-06T13:39:15.0579431Z Task         : Get sources
2020-03-06T13:39:15.0579780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T13:39:15.0580114Z Version      : 1.0.0
2020-03-06T13:39:15.0580341Z Author       : Microsoft
2020-03-06T13:39:15.0580341Z Author       : Microsoft
2020-03-06T13:39:15.0580695Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-06T13:39:15.0581119Z ==============================================================================
2020-03-06T13:39:15.4899329Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-06T13:39:15.4941751Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69773/merge to s
2020-03-06T13:39:15.5036022Z Cleaning up task key
2020-03-06T13:39:15.5037352Z Start cleaning up orphan processes.
2020-03-06T13:39:15.5217181Z Terminate orphan process: pid (9507) (python)
2020-03-06T13:39:15.5575344Z ##[section]Finishing: Finalize Job

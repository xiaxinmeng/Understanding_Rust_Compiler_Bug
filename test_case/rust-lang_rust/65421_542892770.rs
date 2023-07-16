plain
2019-10-16T19:53:34.7220549Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T19:53:34.7485003Z ##[command]git config gc.auto 0
2019-10-16T19:53:34.7567720Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T19:53:34.7643352Z ##[command]git config --get-all http.proxy
2019-10-16T19:53:34.7784618Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65421/merge:refs/remotes/pull/65421/merge
---
2019-10-16T20:57:34.8794139Z .................................................................................................... 1600/9196
2019-10-16T20:57:40.1132311Z .................................................................................................... 1700/9196
2019-10-16T20:57:53.1564352Z .............................i...............i...................................................... 1800/9196
2019-10-16T20:58:00.7279734Z .................................................................................................... 1900/9196
2019-10-16T20:58:15.1993365Z ...................iiiii............................................................................ 2000/9196
2019-10-16T20:58:23.2217093Z ........................................................F..F........................................ 2100/9196
2019-10-16T20:58:28.1975936Z .................................................................................................... 2300/9196
2019-10-16T20:58:33.5865475Z .................................................................................................... 2400/9196
2019-10-16T20:58:55.8700062Z .................................................................................................... 2500/9196
2019-10-16T20:58:58.5688779Z .................................................................................................... 2600/9196
---
2019-10-16T21:01:57.4223668Z ......................i...............i............................................................. 4800/9196
2019-10-16T21:02:09.6039050Z .................................................................................................... 4900/9196
2019-10-16T21:02:16.0196705Z .................................................................................................... 5000/9196
2019-10-16T21:02:26.7276240Z .................................................................................................... 5100/9196
2019-10-16T21:02:33.1664824Z ......................ii.ii......................................................................... 5200/9196
2019-10-16T21:02:43.7131455Z .................................................................................................... 5400/9196
2019-10-16T21:02:54.1951700Z ........................................................................................i........... 5500/9196
2019-10-16T21:03:02.6214990Z .................................................................................................... 5600/9196
2019-10-16T21:03:07.7538738Z .................................................................................................... 5700/9196
2019-10-16T21:03:07.7538738Z .................................................................................................... 5700/9196
2019-10-16T21:03:18.7252966Z .....................................................................................ii...i..ii..... 5800/9196
2019-10-16T21:03:45.5675084Z .................................................................................................... 6000/9196
2019-10-16T21:03:55.6409491Z .................................................................................................... 6100/9196
2019-10-16T21:04:04.6765879Z .................................................................................................... 6200/9196
2019-10-16T21:04:04.6765879Z .................................................................................................... 6200/9196
2019-10-16T21:04:19.5231787Z .......i..ii........................................................................................ 6300/9196
2019-10-16T21:04:41.5343310Z ...................................................................i................................ 6500/9196
2019-10-16T21:04:43.9036785Z .................................................................................................... 6600/9196
2019-10-16T21:04:46.6262644Z .........................................i.......................................................... 6700/9196
2019-10-16T21:04:50.7480300Z .................................................................................................... 6800/9196
---
2019-10-16T21:09:05.1947497Z 
2019-10-16T21:09:05.1948202Z ---- [ui] ui/empty/empty-struct-tuple-pat.rs stdout ----
2019-10-16T21:09:05.1948412Z diff of stderr:
2019-10-16T21:09:05.1948559Z 
2019-10-16T21:09:05.1948676Z 31 LL |         XE::XEmpty5 => (),
2019-10-16T21:09:05.1949680Z 33    |         |   |
2019-10-16T21:09:05.1949680Z 33    |         |   |
2019-10-16T21:09:05.1950116Z -    |         |   help: a unit variant a similar name exists: `XEmpty4`
2019-10-16T21:09:05.1950312Z +    |         |   help: a unit variant with a similar name exists: `XEmpty4`
2019-10-16T21:09:05.1950493Z 35    |         did you mean `XE::XEmpty5( /* fields */ )`?
2019-10-16T21:09:05.1950651Z 36    | 
2019-10-16T21:09:05.1951032Z 37   ::: $DIR/auxiliary/empty-struct.rs:7:5
2019-10-16T21:09:05.1951316Z 
2019-10-16T21:09:05.1951456Z The actual stderr differed from the expected stderr.
2019-10-16T21:09:05.1951932Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
2019-10-16T21:09:05.1951932Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
2019-10-16T21:09:05.1952362Z To update references, rerun the tests and pass the `--bless` flag
2019-10-16T21:09:05.1953074Z To only update this specific test, also pass `--test-args empty/empty-struct-tuple-pat.rs`
2019-10-16T21:09:05.1953353Z error: 1 errors occurred comparing output.
2019-10-16T21:09:05.1953493Z status: exit code: 1
2019-10-16T21:09:05.1953493Z status: exit code: 1
2019-10-16T21:09:05.1954428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-tuple-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/auxiliary" "-A" "unused"
2019-10-16T21:09:05.1954964Z ------------------------------------------
2019-10-16T21:09:05.1955106Z 
2019-10-16T21:09:05.1955433Z ------------------------------------------
2019-10-16T21:09:05.1955584Z stderr:
2019-10-16T21:09:05.1955584Z stderr:
2019-10-16T21:09:05.1956051Z ------------------------------------------
2019-10-16T21:09:05.1956203Z error[E0530]: match bindings cannot shadow tuple structs
2019-10-16T21:09:05.1956672Z   --> /checkout/src/test/ui/empty/empty-struct-tuple-pat.rs:22:9
2019-10-16T21:09:05.1956842Z    |
2019-10-16T21:09:05.1956956Z LL | struct Empty2();
2019-10-16T21:09:05.1957253Z    | ---------------- the tuple struct `Empty2` is defined here
2019-10-16T21:09:05.1957429Z ...
2019-10-16T21:09:05.1957547Z LL |         Empty2 => () //~ ERROR match bindings cannot shadow tuple structs
2019-10-16T21:09:05.1957666Z    |         ^^^^^^ cannot be named the same as a tuple struct
2019-10-16T21:09:05.1957903Z error[E0530]: match bindings cannot shadow tuple structs
2019-10-16T21:09:05.1958554Z   --> /checkout/src/test/ui/empty/empty-struct-tuple-pat.rs:25:9
2019-10-16T21:09:05.1958784Z    |
2019-10-16T21:09:05.1958924Z LL | use empty_struct::*;
2019-10-16T21:09:05.1958924Z LL | use empty_struct::*;
2019-10-16T21:09:05.1959764Z    |     --------------- the tuple struct `XEmpty6` is imported here
2019-10-16T21:09:05.1960020Z ...
2019-10-16T21:09:05.1960170Z LL |         XEmpty6 => () //~ ERROR match bindings cannot shadow tuple structs
2019-10-16T21:09:05.1960334Z    |         ^^^^^^^ cannot be named the same as a tuple struct
2019-10-16T21:09:05.1960460Z 
2019-10-16T21:09:05.1960604Z error[E0532]: expected unit struct/variant or constant, found tuple variant `E::Empty4`
2019-10-16T21:09:05.1961401Z    |
2019-10-16T21:09:05.1961541Z LL |     Empty4()
2019-10-16T21:09:05.1961541Z LL |     Empty4()
2019-10-16T21:09:05.1961910Z    |     -------- `E::Empty4` defined here
2019-10-16T21:09:05.1962083Z ...
2019-10-16T21:09:05.1962221Z LL |         E::Empty4 => ()
2019-10-16T21:09:05.1962385Z    |         ^^^^^^^^^ did you mean `E::Empty4( /* fields */ )`?
2019-10-16T21:09:05.1962519Z 
2019-10-16T21:09:05.1962823Z error[E0532]: expected unit struct/variant or constant, found tuple variant `XE::XEmpty5`
2019-10-16T21:09:05.1963480Z    |
2019-10-16T21:09:05.1963480Z    |
2019-10-16T21:09:05.1963772Z LL |         XE::XEmpty5 => (),
2019-10-16T21:09:05.1964593Z    |         |   |
2019-10-16T21:09:05.1964593Z    |         |   |
2019-10-16T21:09:05.1964726Z    |         |   help: a unit variant with a similar name exists: `XEmpty4`
2019-10-16T21:09:05.1964886Z    |         did you mean `XE::XEmpty5( /* fields */ )`?
2019-10-16T21:09:05.1965678Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:7:5
2019-10-16T21:09:05.1965831Z    |
2019-10-16T21:09:05.1965948Z LL |     XEmpty4,
2019-10-16T21:09:05.1965948Z LL |     XEmpty4,
2019-10-16T21:09:05.1966281Z    |     ------- similarly named unit variant `XEmpty4` defined here
2019-10-16T21:09:05.1966548Z error: aborting due to 4 previous errors
2019-10-16T21:09:05.1966651Z 
2019-10-16T21:09:05.1966791Z Some errors have detailed explanations: E0530, E0532.
2019-10-16T21:09:05.1967110Z For more information about an error, try `rustc --explain E0530`.
---
2019-10-16T21:09:05.1968459Z ---- [ui] ui/empty/empty-struct-unit-pat.rs stdout ----
2019-10-16T21:09:05.1968603Z diff of stderr:
2019-10-16T21:09:05.1968700Z 
2019-10-16T21:09:05.1968805Z 22 
2019-10-16T21:09:05.1968950Z 23 error[E0532]: expected tuple struct/variant, found unit struct `Empty2`
2019-10-16T21:09:05.1970105Z -    |unit variant
2019-10-16T21:09:05.1970310Z +    |
2019-10-16T21:09:05.1970455Z 26 LL |         Empty2(..) => ()
2019-10-16T21:09:05.1970455Z 26 LL |         Empty2(..) => ()
2019-10-16T21:09:05.1970603Z 27    |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1970910Z 
2019-10-16T21:09:05.1970910Z 
2019-10-16T21:09:05.1971302Z 29   ::: $DIR/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1971484Z 30    |
2019-10-16T21:09:05.1971861Z - LL | pub struct XEmpty6();unit variant
2019-10-16T21:09:05.1972038Z + LL | pub struct XEmpty6();
2019-10-16T21:09:05.1976539Z 32    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1976686Z 33 
2019-10-16T21:09:05.1976748Z 34 error[E0532]: expected tuple struct/variant, found unit struct `XEmpty2`
2019-10-16T21:09:05.1976829Z 36    |
2019-10-16T21:09:05.1976882Z 37 LL |         XEmpty2(..) => ()
2019-10-16T21:09:05.1976882Z 37 LL |         XEmpty2(..) => ()
2019-10-16T21:09:05.1976926Z 38    |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1977196Z -    | unit variant
2019-10-16T21:09:05.1977239Z +    | 
2019-10-16T21:09:05.1977841Z 40   ::: $DIR/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1977889Z 41    |
2019-10-16T21:09:05.1978071Z 42 LL | pub struct XEmpty6();
2019-10-16T21:09:05.1978131Z 
2019-10-16T21:09:05.1978566Z 43    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1978989Z - error[E0532]: expected tuple struct/variant, found unit variantEmpty4`
2019-10-16T21:09:05.1979795Z + error[E0532]: expected tuple struct/variant, found unit variant `E::Empty4`
2019-10-16T21:09:05.1980081Z 46   --> $DIR/empty-struct-unit-pat.rs:34:9
2019-10-16T21:09:05.1980129Z 47    |
2019-10-16T21:09:05.1980129Z 47    |
2019-10-16T21:09:05.1980194Z 48 LL |         E::Empty4() => ()
2019-10-16T21:09:05.1980361Z 
2019-10-16T21:09:05.1981175Z 
2019-10-16T21:09:05.1981236Z The actual stderr differed from the expected stderr.
2019-10-16T21:09:05.1981668Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat/empty-struct-unit-pat.stderr
2019-10-16T21:09:05.1981927Z To update references, rerun the tests and pass the `--bless` flag
2019-10-16T21:09:05.1982245Z To only update this specific test, also pass `--test-args empty/empty-struct-unit-pat.rs`
2019-10-16T21:09:05.1982353Z error: 1 errors occurred comparing output.
2019-10-16T21:09:05.1982406Z status: exit code: 1
2019-10-16T21:09:05.1982406Z status: exit code: 1
2019-10-16T21:09:05.1983952Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-unit-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat/auxiliary" "-A" "unused"
2019-10-16T21:09:05.1984627Z ------------------------------------------
2019-10-16T21:09:05.1984676Z 
2019-10-16T21:09:05.1986278Z ------------------------------------------
2019-10-16T21:09:05.1986364Z stderr:
2019-10-16T21:09:05.1986364Z stderr:
2019-10-16T21:09:05.1986588Z ------------------------------------------
2019-10-16T21:09:05.1986643Z error[E0532]: expected tuple struct/variant, found unit struct `Empty2`
2019-10-16T21:09:05.1986966Z    |
2019-10-16T21:09:05.1986966Z    |
2019-10-16T21:09:05.1987018Z LL |         Empty2() => () //~ ERROR expected tuple struct/variant, found unit struct `Empty2`
2019-10-16T21:09:05.1987094Z    |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1987396Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1987462Z    |
2019-10-16T21:09:05.1987462Z    |
2019-10-16T21:09:05.1987507Z LL | pub struct XEmpty6();
2019-10-16T21:09:05.1987758Z    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1987795Z 
2019-10-16T21:09:05.1987872Z error[E0532]: expected tuple struct/variant, found unit struct `XEmpty2`
2019-10-16T21:09:05.1988164Z    |
2019-10-16T21:09:05.1988164Z    |
2019-10-16T21:09:05.1988233Z LL |         XEmpty2() => () //~ ERROR expected tuple struct/variant, found unit struct `XEmpty2`
2019-10-16T21:09:05.1988291Z    |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1988595Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1988643Z    |
2019-10-16T21:09:05.1988643Z    |
2019-10-16T21:09:05.1988853Z LL | pub struct XEmpty6();
2019-10-16T21:09:05.1989928Z    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1989991Z 
2019-10-16T21:09:05.1990040Z error[E0532]: expected tuple struct/variant, found unit struct `Empty2`
2019-10-16T21:09:05.1990349Z    |
2019-10-16T21:09:05.1990349Z    |
2019-10-16T21:09:05.1990775Z LL |         Empty2(..) => () //~ ERROR expected tuple struct/variant, found unit struct `Empty2`
2019-10-16T21:09:05.1990872Z    |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1995712Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1995775Z    |
2019-10-16T21:09:05.1995775Z    |
2019-10-16T21:09:05.1995810Z LL | pub struct XEmpty6();
2019-10-16T21:09:05.1996050Z    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1996081Z 
2019-10-16T21:09:05.1996121Z error[E0532]: expected tuple struct/variant, found unit struct `XEmpty2`
2019-10-16T21:09:05.1996846Z    |
2019-10-16T21:09:05.1996846Z    |
2019-10-16T21:09:05.1996887Z LL |         XEmpty2(..) => () //~ ERROR expected tuple struct/variant, found unit struct `XEmpty2`
2019-10-16T21:09:05.1996949Z    |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
2019-10-16T21:09:05.1997413Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
2019-10-16T21:09:05.1997454Z    |
2019-10-16T21:09:05.1997454Z    |
2019-10-16T21:09:05.1997765Z LL | pub struct XEmpty6();
2019-10-16T21:09:05.1997979Z    | --------------------- similarly named tuple struct `XEmpty6` defined here
2019-10-16T21:09:05.1998069Z error[E0532]: expected tuple struct/variant, found unit variant `E::Empty4`
2019-10-16T21:09:05.1998270Z   --> /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:34:9
2019-10-16T21:09:05.1998310Z    |
2019-10-16T21:09:05.1998310Z    |
2019-10-16T21:09:05.1998379Z LL |         E::Empty4() => () //~ ERROR expected tuple struct/variant, found unit variant `E::Empty4`
2019-10-16T21:09:05.1998455Z 
2019-10-16T21:09:05.1998455Z 
2019-10-16T21:09:05.1998496Z error[E0532]: expected tuple struct/variant, found unit variant `XE::XEmpty4`
2019-10-16T21:09:05.1998766Z    |
2019-10-16T21:09:05.1998766Z    |
2019-10-16T21:09:05.1998803Z LL |         XE::XEmpty4() => (),
2019-10-16T21:09:05.1999621Z    |             |
2019-10-16T21:09:05.1999621Z    |             |
2019-10-16T21:09:05.1999687Z    |             help: a tuple variant with a similar name exists: `XEmpty5`
2019-10-16T21:09:05.2000035Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:8:5
2019-10-16T21:09:05.2000084Z    |
2019-10-16T21:09:05.2000126Z LL |     XEmpty5(),
2019-10-16T21:09:05.2000126Z LL |     XEmpty5(),
2019-10-16T21:09:05.2000387Z    |     --------- similarly named tuple variant `XEmpty5` defined here
2019-10-16T21:09:05.2000484Z error[E0532]: expected tuple struct/variant, found unit variant `E::Empty4`
2019-10-16T21:09:05.2000745Z   --> /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:42:9
2019-10-16T21:09:05.2000792Z    |
2019-10-16T21:09:05.2000792Z    |
2019-10-16T21:09:05.2000845Z LL |         E::Empty4(..) => () //~ ERROR expected tuple struct/variant, found unit variant `E::Empty4`
2019-10-16T21:09:05.2000959Z 
2019-10-16T21:09:05.2000959Z 
2019-10-16T21:09:05.2001006Z error[E0532]: expected tuple struct/variant, found unit variant `XE::XEmpty4`
2019-10-16T21:09:05.2001700Z    |
2019-10-16T21:09:05.2001700Z    |
2019-10-16T21:09:05.2001754Z LL |         XE::XEmpty4(..) => (),
2019-10-16T21:09:05.2003533Z    |             |
2019-10-16T21:09:05.2003533Z    |             |
2019-10-16T21:09:05.2003576Z    |             help: a tuple variant with a similar name exists: `XEmpty5`
2019-10-16T21:09:05.2003931Z   ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:8:5
2019-10-16T21:09:05.2003974Z    |
2019-10-16T21:09:05.2004008Z LL |     XEmpty5(),
2019-10-16T21:09:05.2004008Z LL |     XEmpty5(),
2019-10-16T21:09:05.2004214Z    |     --------- similarly named tuple variant `XEmpty5` defined here
2019-10-16T21:09:05.2004298Z error: aborting due to 8 previous errors
2019-10-16T21:09:05.2004323Z 
2019-10-16T21:09:05.2004653Z For more information about this error, try `rustc --explain E0532`.
2019-10-16T21:09:05.2004712Z 
---
2019-10-16T21:09:05.2006011Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-16T21:09:05.2006078Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-16T21:09:05.2006157Z 
2019-10-16T21:09:05.2006182Z 
2019-10-16T21:09:05.2007419Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-16T21:09:05.2007628Z 
2019-10-16T21:09:05.2007652Z 
2019-10-16T21:09:05.2007710Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-16T21:09:05.2007752Z Build completed unsuccessfully in 1:07:53
2019-10-16T21:09:05.2007752Z Build completed unsuccessfully in 1:07:53
2019-10-16T21:09:05.2067049Z == clock drift check ==
2019-10-16T21:09:05.2087902Z   local time: Wed Oct 16 21:09:05 UTC 2019
2019-10-16T21:09:05.3715992Z   network time: Wed, 16 Oct 2019 21:09:05 GMT
2019-10-16T21:09:05.3721725Z == end clock drift check ==
2019-10-16T21:09:06.5051150Z ##[error]Bash exited with code '1'.
2019-10-16T21:09:06.5090646Z ##[section]Starting: Checkout
2019-10-16T21:09:06.5095089Z ==============================================================================
2019-10-16T21:09:06.5095146Z Task         : Get sources
2019-10-16T21:09:06.5095209Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

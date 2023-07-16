plain
2019-12-10T09:46:27.2904427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T09:46:27.2916865Z ##[command]git config gc.auto 0
2019-12-10T09:46:27.2920188Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T09:46:27.2921588Z ##[command]git config --get-all http.proxy
2019-12-10T09:46:27.2923443Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67189/merge:refs/remotes/pull/67189/merge
---
2019-12-10T10:30:30.4070298Z .................................................................................................... 1500/9341
2019-12-10T10:30:35.0703312Z .................................................................................................... 1600/9341
2019-12-10T10:30:38.3368346Z .................................................................................................... 1700/9341
2019-12-10T10:30:47.2374465Z ..................................................i................................................. 1800/9341
2019-12-10T10:30:53.1970374Z ........................................F.FFF.........F............................................. 1900/9341
2019-12-10T10:31:03.2926498Z ...................................iiiii............................................................ 2000/9341
2019-12-10T10:31:10.5677948Z .................................................................................................... 2200/9341
2019-12-10T10:31:12.3772310Z .................................................................................................... 2300/9341
2019-12-10T10:31:15.5660233Z .................................................................................................... 2400/9341
2019-12-10T10:31:31.5257522Z .................................................................................................... 2500/9341
---
2019-12-10T10:33:28.2395625Z ...........FF.........................i...............i............................................. 4800/9341
2019-12-10T10:33:35.2725842Z .................................................................................................... 4900/9341
2019-12-10T10:33:40.0863590Z ...............................................................................F..i................. 5000/9341
2019-12-10T10:33:44.6655255Z ............F....................................................................................... 5100/9341
2019-12-10T10:33:52.0469638Z ...............................................ii.ii...........i.................................... 5200/9341
2019-12-10T10:33:58.6740790Z .................................................................................................... 5400/9341
2019-12-10T10:34:05.8973472Z .................................................................................................... 5500/9341
2019-12-10T10:34:11.1384416Z .............................i...................................................................... 5600/9341
2019-12-10T10:34:15.8120839Z .................................................................................................... 5700/9341
2019-12-10T10:34:15.8120839Z .................................................................................................... 5700/9341
2019-12-10T10:34:24.5098536Z .................................................................................................... 5800/9341
2019-12-10T10:34:32.6895941Z ................ii...i..ii...........i.............................................................. 5900/9341
2019-12-10T10:34:45.9909548Z .................................................................................................... 6100/9341
2019-12-10T10:34:51.2252286Z .................................................................................................... 6200/9341
2019-12-10T10:34:51.2252286Z .................................................................................................... 6200/9341
2019-12-10T10:35:00.3212308Z ........................................i..ii....................................................... 6300/9341
2019-12-10T10:35:15.8861640Z .................................................................................................... 6500/9341
2019-12-10T10:35:17.4043750Z ............i....................................................................................... 6600/9341
2019-12-10T10:35:19.0315310Z .................................................................................................... 6700/9341
2019-12-10T10:35:20.8809966Z ...i................................................................................................ 6800/9341
---
2019-12-10T10:36:32.4436101Z .................................................................................................... 7400/9341
2019-12-10T10:36:36.2355712Z .................................................................................................... 7500/9341
2019-12-10T10:36:41.4031348Z .................................................................................................... 7600/9341
2019-12-10T10:36:49.0514165Z .................................................................................................... 7700/9341
2019-12-10T10:36:53.5697495Z ...................iiii............................................................................. 7800/9341
2019-12-10T10:37:03.8385221Z .........................................F.......................................................... 8000/9341
2019-12-10T10:37:11.5828512Z ..............................................................................F..................... 8100/9341
2019-12-10T10:37:21.3835265Z .................................................................................................... 8200/9341
2019-12-10T10:37:26.5489770Z .................................................................................................... 8300/9341
---
2019-12-10T10:38:50.1756651Z 
2019-12-10T10:38:50.1757446Z ---- [ui] ui/autoderef-full-lval.rs stdout ----
2019-12-10T10:38:50.1757887Z diff of stderr:
2019-12-10T10:38:50.1758080Z 
2019-12-10T10:38:50.1758629Z - error[E0369]: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T10:38:50.1759035Z + error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1759984Z 3    |
2019-12-10T10:38:50.1759984Z 3    |
2019-12-10T10:38:50.1760174Z 4 LL |     let z: isize = a.x + b.y;
2019-12-10T10:38:50.1760722Z 8    |
2019-12-10T10:38:50.1760722Z 8    |
2019-12-10T10:38:50.1760933Z 9    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2019-12-10T10:38:50.1761124Z 10 
2019-12-10T10:38:50.1761611Z - error[E0369]: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T10:38:50.1762027Z + error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1762891Z 13    |
2019-12-10T10:38:50.1762891Z 13    |
2019-12-10T10:38:50.1763079Z 14 LL |     let answer: isize = forty.a + two.a;
2019-12-10T10:38:50.1763586Z 
2019-12-10T10:38:50.1763801Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1764545Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/autoderef-full-lval.stderr
2019-12-10T10:38:50.1764545Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/autoderef-full-lval.stderr
2019-12-10T10:38:50.1765217Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1766025Z To only update this specific test, also pass `--test-args autoderef-full-lval.rs`
2019-12-10T10:38:50.1766720Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1766936Z status: exit code: 1
2019-12-10T10:38:50.1766936Z status: exit code: 1
2019-12-10T10:38:50.1767828Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/autoderef-full-lval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1768987Z ------------------------------------------
2019-12-10T10:38:50.1769350Z 
2019-12-10T10:38:50.1769805Z ------------------------------------------
2019-12-10T10:38:50.1770258Z stderr:
2019-12-10T10:38:50.1770258Z stderr:
2019-12-10T10:38:50.1770680Z ------------------------------------------
2019-12-10T10:38:50.1773646Z error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1775700Z    |
2019-12-10T10:38:50.1775700Z    |
2019-12-10T10:38:50.1779065Z LL |     let z: isize = a.x + b.y;
2019-12-10T10:38:50.1779356Z    |                    --- ^ --- std::boxed::Box<isize>
2019-12-10T10:38:50.1779447Z    |                    std::boxed::Box<isize>
2019-12-10T10:38:50.1779481Z    |
2019-12-10T10:38:50.1779481Z    |
2019-12-10T10:38:50.1779535Z    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2019-12-10T10:38:50.1779574Z 
2019-12-10T10:38:50.1779615Z error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1779853Z    |
2019-12-10T10:38:50.1779853Z    |
2019-12-10T10:38:50.1779885Z LL |     let answer: isize = forty.a + two.a;
2019-12-10T10:38:50.1780063Z    |                         ------- ^ ----- std::boxed::Box<isize>
2019-12-10T10:38:50.1780152Z    |                         std::boxed::Box<isize>
2019-12-10T10:38:50.1780184Z    |
2019-12-10T10:38:50.1780184Z    |
2019-12-10T10:38:50.1780238Z    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2019-12-10T10:38:50.1780295Z error: aborting due to 2 previous errors
2019-12-10T10:38:50.1780315Z 
2019-12-10T10:38:50.1780512Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1780537Z 
2019-12-10T10:38:50.1780537Z 
2019-12-10T10:38:50.1780702Z ------------------------------------------
2019-12-10T10:38:50.1780731Z 
2019-12-10T10:38:50.1780766Z 
2019-12-10T10:38:50.1780931Z ---- [ui] ui/binary-op-on-double-ref.rs stdout ----
2019-12-10T10:38:50.1780966Z diff of stderr:
2019-12-10T10:38:50.1780987Z 
2019-12-10T10:38:50.1781184Z - error[E0369]: binary operation `%` cannot be applied to type `&&{integer}`
2019-12-10T10:38:50.1781224Z + error[E0369]: cannot mod `&&{integer}` by `{integer}`
2019-12-10T10:38:50.1781434Z 3    |
2019-12-10T10:38:50.1781465Z 4 LL |         x % 2 == 0
2019-12-10T10:38:50.1781487Z 
2019-12-10T10:38:50.1781506Z 
2019-12-10T10:38:50.1781506Z 
2019-12-10T10:38:50.1781554Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1781788Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref/binary-op-on-double-ref.stderr
2019-12-10T10:38:50.1781970Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1782300Z To only update this specific test, also pass `--test-args binary-op-on-double-ref.rs`
2019-12-10T10:38:50.1782371Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1782403Z status: exit code: 1
2019-12-10T10:38:50.1782403Z status: exit code: 1
2019-12-10T10:38:50.1783016Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binary-op-on-double-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1783262Z ------------------------------------------
2019-12-10T10:38:50.1783287Z 
2019-12-10T10:38:50.1783552Z ------------------------------------------
2019-12-10T10:38:50.1783614Z stderr:
2019-12-10T10:38:50.1783614Z stderr:
2019-12-10T10:38:50.1783774Z ------------------------------------------
2019-12-10T10:38:50.1783811Z error[E0369]: cannot mod `&&{integer}` by `{integer}`
2019-12-10T10:38:50.1784033Z    |
2019-12-10T10:38:50.1784066Z LL |         x % 2 == 0
2019-12-10T10:38:50.1784066Z LL |         x % 2 == 0
2019-12-10T10:38:50.1784234Z    |         - ^ - {integer}
2019-12-10T10:38:50.1784301Z    |         &&{integer}
2019-12-10T10:38:50.1784330Z    |
2019-12-10T10:38:50.1784330Z    |
2019-12-10T10:38:50.1784528Z    = help: `%` can be used on '{integer}', you can dereference `x`: `*x`
2019-12-10T10:38:50.1784585Z error: aborting due to previous error
2019-12-10T10:38:50.1784606Z 
2019-12-10T10:38:50.1784800Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1784825Z 
2019-12-10T10:38:50.1784825Z 
2019-12-10T10:38:50.1784977Z ------------------------------------------
2019-12-10T10:38:50.1785007Z 
2019-12-10T10:38:50.1785040Z 
2019-12-10T10:38:50.1785208Z ---- [ui] ui/binop/binop-bitxor-str.rs stdout ----
2019-12-10T10:38:50.1785243Z diff of stderr:
2019-12-10T10:38:50.1785263Z 
2019-12-10T10:38:50.1785470Z - error[E0369]: binary operation `^` cannot be applied to type `std::string::String`
2019-12-10T10:38:50.1785513Z + error[E0369]: no implementation for `std::string::String ^ std::string::String`
2019-12-10T10:38:50.1785722Z 3    |
2019-12-10T10:38:50.1785722Z 3    |
2019-12-10T10:38:50.1785758Z 4 LL | fn main() { let x = "a".to_string() ^ "b".to_string(); }
2019-12-10T10:38:50.1785799Z 
2019-12-10T10:38:50.1785847Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1786071Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/binop-bitxor-str.stderr
2019-12-10T10:38:50.1786071Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/binop-bitxor-str.stderr
2019-12-10T10:38:50.1786251Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1786479Z To only update this specific test, also pass `--test-args binop/binop-bitxor-str.rs`
2019-12-10T10:38:50.1786537Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1786586Z status: exit code: 1
2019-12-10T10:38:50.1786586Z status: exit code: 1
2019-12-10T10:38:50.1787138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-bitxor-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1787379Z ------------------------------------------
2019-12-10T10:38:50.1787403Z 
2019-12-10T10:38:50.1787584Z ------------------------------------------
2019-12-10T10:38:50.1787689Z stderr:
2019-12-10T10:38:50.1787689Z stderr:
2019-12-10T10:38:50.1787876Z ------------------------------------------
2019-12-10T10:38:50.1787915Z error[E0369]: no implementation for `std::string::String ^ std::string::String`
2019-12-10T10:38:50.1788144Z    |
2019-12-10T10:38:50.1788144Z    |
2019-12-10T10:38:50.1788178Z LL | fn main() { let x = "a".to_string() ^ "b".to_string(); }
2019-12-10T10:38:50.1788380Z    |                     --------------- ^ --------------- std::string::String
2019-12-10T10:38:50.1788451Z    |                     std::string::String
2019-12-10T10:38:50.1788498Z    |
2019-12-10T10:38:50.1788534Z    = note: an implementation of `std::ops::BitXor` might be missing for `std::string::String`
2019-12-10T10:38:50.1788559Z 
---
2019-12-10T10:38:50.1789551Z - error[E0369]: binary operation `*` cannot be applied to type `bool`
2019-12-10T10:38:50.1789589Z + error[E0369]: cannot multiply `bool` to `bool`
2019-12-10T10:38:50.1789755Z 2   --> $DIR/binop-mul-bool.rs:3:26
2019-12-10T10:38:50.1789789Z 3    |
2019-12-10T10:38:50.1789821Z 4 LL | fn main() { let x = true * false; }
2019-12-10T10:38:50.1789879Z 
2019-12-10T10:38:50.1789912Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1790129Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/binop-mul-bool.stderr
2019-12-10T10:38:50.1790129Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/binop-mul-bool.stderr
2019-12-10T10:38:50.1790338Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1790534Z To only update this specific test, also pass `--test-args binop/binop-mul-bool.rs`
2019-12-10T10:38:50.1790591Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1790638Z status: exit code: 1
2019-12-10T10:38:50.1790638Z status: exit code: 1
2019-12-10T10:38:50.1791184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-mul-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1791431Z ------------------------------------------
2019-12-10T10:38:50.1791456Z 
2019-12-10T10:38:50.1791638Z ------------------------------------------
2019-12-10T10:38:50.1791671Z stderr:
2019-12-10T10:38:50.1791671Z stderr:
2019-12-10T10:38:50.1791827Z ------------------------------------------
2019-12-10T10:38:50.1791878Z error[E0369]: cannot multiply `bool` to `bool`
2019-12-10T10:38:50.1792050Z   --> /checkout/src/test/ui/binop/binop-mul-bool.rs:3:26
2019-12-10T10:38:50.1792087Z    |
2019-12-10T10:38:50.1792135Z LL | fn main() { let x = true * false; }
2019-12-10T10:38:50.1792295Z    |                     ---- ^ ----- bool
2019-12-10T10:38:50.1792363Z    |                     bool
2019-12-10T10:38:50.1792409Z    |
2019-12-10T10:38:50.1792444Z    = note: an implementation of `std::ops::Mul` might be missing for `bool`
2019-12-10T10:38:50.1792469Z 
---
2019-12-10T10:38:50.1793486Z - error[E0369]: binary operation `+` cannot be applied to type `bool`
2019-12-10T10:38:50.1793524Z + error[E0369]: cannot add `{integer}` to `bool`
2019-12-10T10:38:50.1793692Z 2   --> $DIR/binop-typeck.rs:6:15
2019-12-10T10:38:50.1793724Z 3    |
2019-12-10T10:38:50.1793755Z 4 LL |     let z = x + y;
2019-12-10T10:38:50.1793809Z 
2019-12-10T10:38:50.1793842Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1794058Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/binop-typeck.stderr
2019-12-10T10:38:50.1794058Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/binop-typeck.stderr
2019-12-10T10:38:50.1794254Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1794540Z To only update this specific test, also pass `--test-args binop/binop-typeck.rs`
2019-12-10T10:38:50.1794616Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1794649Z status: exit code: 1
2019-12-10T10:38:50.1794649Z status: exit code: 1
2019-12-10T10:38:50.1795196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1795439Z ------------------------------------------
2019-12-10T10:38:50.1795471Z 
2019-12-10T10:38:50.1795647Z ------------------------------------------
2019-12-10T10:38:50.1795686Z stderr:
2019-12-10T10:38:50.1795686Z stderr:
2019-12-10T10:38:50.1795839Z ------------------------------------------
2019-12-10T10:38:50.1795891Z error[E0369]: cannot add `{integer}` to `bool`
2019-12-10T10:38:50.1796060Z   --> /checkout/src/test/ui/binop/binop-typeck.rs:6:15
2019-12-10T10:38:50.1796094Z    |
2019-12-10T10:38:50.1796140Z LL |     let z = x + y;
2019-12-10T10:38:50.1796291Z    |             - ^ - {integer}
2019-12-10T10:38:50.1796356Z    |             bool
2019-12-10T10:38:50.1796401Z    |
2019-12-10T10:38:50.1796436Z    = note: an implementation of `std::ops::Add` might be missing for `bool`
2019-12-10T10:38:50.1796462Z 
---
2019-12-10T10:38:50.1797474Z 24    |     [usize; 33]
2019-12-10T10:38:50.1797604Z -    |
2019-12-10T10:38:50.1797798Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2019-12-10T10:38:50.1797850Z 27 
2019-12-10T10:38:50.1797886Z 28 error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-10T10:38:50.1798056Z 29   --> $DIR/core-traits-no-impls-length-33.rs:19:19
2019-12-10T10:38:50.1798081Z 
2019-12-10T10:38:50.1798260Z 32    |     ------------- ^ ------------- [usize; 33]
2019-12-10T10:38:50.1798331Z 34    |     [usize; 33]
2019-12-10T10:38:50.1798479Z -    |
2019-12-10T10:38:50.1798479Z -    |
2019-12-10T10:38:50.1798736Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2019-12-10T10:38:50.1798779Z 37 
2019-12-10T10:38:50.1798832Z 38 error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-10T10:38:50.1799030Z 39   --> $DIR/core-traits-no-impls-length-33.rs:24:14
2019-12-10T10:38:50.1799073Z 
2019-12-10T10:38:50.1799121Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1799121Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1799385Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2019-12-10T10:38:50.1799570Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1799810Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2019-12-10T10:38:50.1799941Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1799991Z status: exit code: 1
2019-12-10T10:38:50.1799991Z status: exit code: 1
2019-12-10T10:38:50.1800646Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1800944Z ------------------------------------------
2019-12-10T10:38:50.1800969Z 
2019-12-10T10:38:50.1801146Z ------------------------------------------
2019-12-10T10:38:50.1801186Z stderr:
2019-12-10T10:38:50.1801186Z stderr:
2019-12-10T10:38:50.1801348Z ------------------------------------------
2019-12-10T10:38:50.1801404Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-10T10:38:50.1801608Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2019-12-10T10:38:50.1801646Z    |
2019-12-10T10:38:50.1801694Z LL |     println!("{:?}", [0_usize; 33]);
2019-12-10T10:38:50.1801735Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-10T10:38:50.1801826Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2019-12-10T10:38:50.1801865Z    = note: required by `std::fmt::Debug::fmt`
2019-12-10T10:38:50.1801886Z 
2019-12-10T10:38:50.1801920Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-10T10:38:50.1801920Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2019-12-10T10:38:50.1802146Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:16
2019-12-10T10:38:50.1802193Z    |
2019-12-10T10:38:50.1802225Z LL |     set.insert([0_usize; 33]);
2019-12-10T10:38:50.1802282Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2019-12-10T10:38:50.1802316Z    |
2019-12-10T10:38:50.1802353Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2019-12-10T10:38:50.1802427Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2019-12-10T10:38:50.1802636Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2019-12-10T10:38:50.1802689Z    |
2019-12-10T10:38:50.1802689Z    |
2019-12-10T10:38:50.1802720Z LL |     [0_usize; 33] == [1_usize; 33]
2019-12-10T10:38:50.1802886Z    |     ------------- ^^ ------------- [usize; 33]
2019-12-10T10:38:50.1802967Z    |     [usize; 33]
2019-12-10T10:38:50.1802995Z 
2019-12-10T10:38:50.1802995Z 
2019-12-10T10:38:50.1803096Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2019-12-10T10:38:50.1803346Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:19:19
2019-12-10T10:38:50.1803382Z    |
2019-12-10T10:38:50.1803413Z LL |     [0_usize; 33] < [1_usize; 33]
2019-12-10T10:38:50.1803594Z    |     ------------- ^ ------------- [usize; 33]
2019-12-10T10:38:50.1803659Z    |     [usize; 33]
2019-12-10T10:38:50.1803680Z 
2019-12-10T10:38:50.1803680Z 
2019-12-10T10:38:50.1803733Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2019-12-10T10:38:50.1803935Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:24:14
2019-12-10T10:38:50.1803971Z    |
2019-12-10T10:38:50.1804018Z LL |     for _ in &[0_usize; 33] {
2019-12-10T10:38:50.1804058Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2019-12-10T10:38:50.1804215Z    = help: the following implementations were found:
2019-12-10T10:38:50.1804215Z    = help: the following implementations were found:
2019-12-10T10:38:50.1804423Z              <&'a [T; _] as std::iter::IntoIterator>
2019-12-10T10:38:50.1804587Z              <&'a [T] as std::iter::IntoIterator>
2019-12-10T10:38:50.1804755Z              <&'a mut [T; _] as std::iter::IntoIterator>
2019-12-10T10:38:50.1804940Z              <&'a mut [T] as std::iter::IntoIterator>
2019-12-10T10:38:50.1805000Z 
2019-12-10T10:38:50.1805047Z error: aborting due to 5 previous errors
2019-12-10T10:38:50.1805068Z 
2019-12-10T10:38:50.1805101Z Some errors have detailed explanations: E0277, E0369.
---
2019-12-10T10:38:50.1807063Z 18 
2019-12-10T10:38:50.1807083Z 
2019-12-10T10:38:50.1807101Z 
2019-12-10T10:38:50.1807149Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1807416Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/derives-span-PartialEq-enum-struct-variant.stderr
2019-12-10T10:38:50.1807599Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1807831Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum-struct-variant.rs`
2019-12-10T10:38:50.1807890Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1807939Z status: exit code: 1
2019-12-10T10:38:50.1807939Z status: exit code: 1
2019-12-10T10:38:50.1808606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1808890Z ------------------------------------------
2019-12-10T10:38:50.1808914Z 
2019-12-10T10:38:50.1809091Z ------------------------------------------
2019-12-10T10:38:50.1809124Z stderr:
2019-12-10T10:38:50.1809124Z stderr:
2019-12-10T10:38:50.1809276Z ------------------------------------------
2019-12-10T10:38:50.1809331Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-12-10T10:38:50.1809527Z   --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs:10:6
2019-12-10T10:38:50.1809630Z    |
2019-12-10T10:38:50.1809687Z LL |      x: Error //~ ERROR
2019-12-10T10:38:50.1809740Z 
2019-12-10T10:38:50.1809773Z error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-12-10T10:38:50.1810008Z   --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs:10:6
2019-12-10T10:38:50.1810045Z    |
2019-12-10T10:38:50.1810045Z    |
2019-12-10T10:38:50.1810075Z LL |      x: Error //~ ERROR
2019-12-10T10:38:50.1810144Z 
2019-12-10T10:38:50.1810175Z error: aborting due to 2 previous errors
2019-12-10T10:38:50.1810196Z 
2019-12-10T10:38:50.1810393Z For more information about this error, try `rustc --explain E0369`.
---
2019-12-10T10:38:50.1812134Z 18 
2019-12-10T10:38:50.1812153Z 
2019-12-10T10:38:50.1812187Z 
2019-12-10T10:38:50.1812226Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1812471Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/derives-span-PartialEq-struct.stderr
2019-12-10T10:38:50.1812671Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1812877Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-struct.rs`
2019-12-10T10:38:50.1812951Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1812983Z status: exit code: 1
2019-12-10T10:38:50.1812983Z status: exit code: 1
2019-12-10T10:38:50.1813633Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1813918Z ------------------------------------------
2019-12-10T10:38:50.1813942Z 
2019-12-10T10:38:50.1814119Z ------------------------------------------
2019-12-10T10:38:50.1814151Z stderr:
2019-12-10T10:38:50.1814151Z stderr:
2019-12-10T10:38:50.1814304Z ------------------------------------------
2019-12-10T10:38:50.1814361Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-12-10T10:38:50.1814546Z   --> /checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs:9:5
2019-12-10T10:38:50.1814582Z    |
2019-12-10T10:38:50.1814630Z LL |     x: Error //~ ERROR
2019-12-10T10:38:50.1814681Z 
2019-12-10T10:38:50.1814781Z error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-12-10T10:38:50.1815013Z   --> /checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs:9:5
2019-12-10T10:38:50.1815049Z    |
2019-12-10T10:38:50.1815049Z    |
2019-12-10T10:38:50.1815078Z LL |     x: Error //~ ERROR
2019-12-10T10:38:50.1815147Z 
2019-12-10T10:38:50.1815178Z error: aborting due to 2 previous errors
2019-12-10T10:38:50.1815199Z 
2019-12-10T10:38:50.1815400Z For more information about this error, try `rustc --explain E0369`.
---
2019-12-10T10:38:50.1840472Z 18 
2019-12-10T10:38:50.1840491Z 
2019-12-10T10:38:50.1840526Z 
2019-12-10T10:38:50.1840560Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1840811Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/derives-span-PartialEq-enum.stderr
2019-12-10T10:38:50.1841026Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1841238Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum.rs`
2019-12-10T10:38:50.1841313Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1841347Z status: exit code: 1
2019-12-10T10:38:50.1841347Z status: exit code: 1
2019-12-10T10:38:50.1841934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1842386Z ------------------------------------------
2019-12-10T10:38:50.1842429Z 
2019-12-10T10:38:50.1842593Z ------------------------------------------
2019-12-10T10:38:50.1842628Z stderr:
---
2019-12-10T10:38:50.1845674Z 18 
2019-12-10T10:38:50.1845710Z 
2019-12-10T10:38:50.1845730Z 
2019-12-10T10:38:50.1845763Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1846020Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/derives-span-PartialEq-tuple-struct.stderr
2019-12-10T10:38:50.1846225Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1846441Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-tuple-struct.rs`
2019-12-10T10:38:50.1846523Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1846555Z status: exit code: 1
2019-12-10T10:38:50.1846555Z status: exit code: 1
2019-12-10T10:38:50.1847161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1847410Z ------------------------------------------
2019-12-10T10:38:50.1847457Z 
2019-12-10T10:38:50.1847684Z ------------------------------------------
2019-12-10T10:38:50.1847727Z stderr:
---
2019-12-10T10:38:50.1849438Z ---- [ui] ui/derives/deriving-no-inner-impl-error-message.rs stdout ----
2019-12-10T10:38:50.1849474Z diff of stderr:
2019-12-10T10:38:50.1849495Z 
2019-12-10T10:38:50.1849538Z 3    |
2019-12-10T10:38:50.1849570Z 4 LL |     x: NoCloneOrEq
2019-12-10T10:38:50.1849754Z -    |
2019-12-10T10:38:50.1849754Z -    |
2019-12-10T10:38:50.1849945Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
2019-12-10T10:38:50.1849982Z 8 
2019-12-10T10:38:50.1850035Z 9 error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
2019-12-10T10:38:50.1850237Z 
2019-12-10T10:38:50.1850272Z 11    |
2019-12-10T10:38:50.1850272Z 11    |
2019-12-10T10:38:50.1850318Z 12 LL |     x: NoCloneOrEq
2019-12-10T10:38:50.1850489Z -    |
2019-12-10T10:38:50.1850489Z -    |
2019-12-10T10:38:50.1850698Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
2019-12-10T10:38:50.1850736Z 16 
2019-12-10T10:38:50.1850776Z 17 error[E0277]: the trait bound `NoCloneOrEq: std::clone::Clone` is not satisfied
2019-12-10T10:38:50.1851024Z 
2019-12-10T10:38:50.1851044Z 
2019-12-10T10:38:50.1851078Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1851365Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/deriving-no-inner-impl-error-message.stderr
2019-12-10T10:38:50.1851365Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/deriving-no-inner-impl-error-message.stderr
2019-12-10T10:38:50.1851566Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1851791Z To only update this specific test, also pass `--test-args derives/deriving-no-inner-impl-error-message.rs`
2019-12-10T10:38:50.1851883Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1851917Z status: exit code: 1
2019-12-10T10:38:50.1851917Z status: exit code: 1
2019-12-10T10:38:50.1852553Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1852812Z ------------------------------------------
2019-12-10T10:38:50.1852839Z 
2019-12-10T10:38:50.1853021Z ------------------------------------------
2019-12-10T10:38:50.1853141Z stderr:
2019-12-10T10:38:50.1853141Z stderr:
2019-12-10T10:38:50.1853348Z ------------------------------------------
2019-12-10T10:38:50.1853392Z error[E0369]: binary operation `==` cannot be applied to type `NoCloneOrEq`
2019-12-10T10:38:50.1853655Z    |
2019-12-10T10:38:50.1853655Z    |
2019-12-10T10:38:50.1853695Z LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
2019-12-10T10:38:50.1853771Z 
2019-12-10T10:38:50.1853771Z 
2019-12-10T10:38:50.1853806Z error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
2019-12-10T10:38:50.1854068Z    |
2019-12-10T10:38:50.1854068Z    |
2019-12-10T10:38:50.1854108Z LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
2019-12-10T10:38:50.1854236Z 
2019-12-10T10:38:50.1854236Z 
2019-12-10T10:38:50.1854301Z error[E0277]: the trait bound `NoCloneOrEq: std::clone::Clone` is not satisfied
2019-12-10T10:38:50.1854575Z    |
2019-12-10T10:38:50.1854575Z    |
2019-12-10T10:38:50.1854623Z LL |     x: NoCloneOrEq
2019-12-10T10:38:50.1854661Z    |     ^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `NoCloneOrEq`
2019-12-10T10:38:50.1854747Z    = note: required by `std::clone::Clone::clone`
2019-12-10T10:38:50.1854770Z 
2019-12-10T10:38:50.1854803Z error: aborting due to 3 previous errors
2019-12-10T10:38:50.1854825Z 
---
2019-12-10T10:38:50.1856337Z 4 LL |     let x = () + ();
2019-12-10T10:38:50.1856360Z 
2019-12-10T10:38:50.1856380Z 
2019-12-10T10:38:50.1856429Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1856671Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error/for-loop-type-error.stderr
2019-12-10T10:38:50.1856870Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1857101Z To only update this specific test, also pass `--test-args for/for-loop-type-error.rs`
2019-12-10T10:38:50.1857178Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1857229Z status: exit code: 1
2019-12-10T10:38:50.1857229Z status: exit code: 1
2019-12-10T10:38:50.1857801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for/for-loop-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1858058Z ------------------------------------------
2019-12-10T10:38:50.1858084Z 
2019-12-10T10:38:50.1858273Z ------------------------------------------
2019-12-10T10:38:50.1858309Z stderr:
2019-12-10T10:38:50.1858309Z stderr:
2019-12-10T10:38:50.1858487Z ------------------------------------------
2019-12-10T10:38:50.1858625Z error[E0369]: cannot add `()` to `()`
2019-12-10T10:38:50.1858850Z   --> /checkout/src/test/ui/for/for-loop-type-error.rs:2:16
2019-12-10T10:38:50.1858891Z    |
2019-12-10T10:38:50.1858948Z LL |     let x = () + (); //~ ERROR binary operation
2019-12-10T10:38:50.1859111Z    |             -- ^ -- ()
2019-12-10T10:38:50.1859182Z    |             ()
2019-12-10T10:38:50.1859229Z    |
2019-12-10T10:38:50.1859266Z    = note: an implementation of `std::ops::Add` might be missing for `()`
2019-12-10T10:38:50.1859290Z 
---
2019-12-10T10:38:50.1859807Z 
2019-12-10T10:38:50.1860085Z ---- [ui] ui/issues/issue-14915.rs stdout ----
2019-12-10T10:38:50.1860128Z diff of stderr:
2019-12-10T10:38:50.1860168Z 
2019-12-10T10:38:50.1860376Z - error[E0369]: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T10:38:50.1860422Z + error[E0369]: cannot add `{integer}` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1860613Z 2   --> $DIR/issue-14915.rs:6:22
2019-12-10T10:38:50.1860682Z 4 LL |     println!("{}", x + 1);
2019-12-10T10:38:50.1860704Z 
2019-12-10T10:38:50.1860739Z 
2019-12-10T10:38:50.1860774Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1860774Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1861009Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915/issue-14915.stderr
2019-12-10T10:38:50.1861222Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1861432Z To only update this specific test, also pass `--test-args issues/issue-14915.rs`
2019-12-10T10:38:50.1861516Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1861557Z status: exit code: 1
2019-12-10T10:38:50.1861557Z status: exit code: 1
2019-12-10T10:38:50.1862123Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14915.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1862382Z ------------------------------------------
2019-12-10T10:38:50.1862409Z 
2019-12-10T10:38:50.1862601Z ------------------------------------------
2019-12-10T10:38:50.1862636Z stderr:
2019-12-10T10:38:50.1862636Z stderr:
2019-12-10T10:38:50.1862805Z ------------------------------------------
2019-12-10T10:38:50.1862873Z error[E0369]: cannot add `{integer}` to `std::boxed::Box<isize>`
2019-12-10T10:38:50.1863062Z   --> /checkout/src/test/ui/issues/issue-14915.rs:6:22
2019-12-10T10:38:50.1863149Z LL |     println!("{}", x + 1);
2019-12-10T10:38:50.1863149Z LL |     println!("{}", x + 1);
2019-12-10T10:38:50.1863321Z    |                    - ^ - {integer}
2019-12-10T10:38:50.1863407Z    |                    std::boxed::Box<isize>
2019-12-10T10:38:50.1863441Z    |
2019-12-10T10:38:50.1863441Z    |
2019-12-10T10:38:50.1863479Z    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2019-12-10T10:38:50.1863554Z error: aborting due to previous error
2019-12-10T10:38:50.1863577Z 
2019-12-10T10:38:50.1863775Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1863802Z 
---
2019-12-10T10:38:50.1865013Z 
2019-12-10T10:38:50.1865047Z 
2019-12-10T10:38:50.1865082Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1865318Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363/issue-24363.stderr
2019-12-10T10:38:50.1865525Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1865822Z To only update this specific test, also pass `--test-args issues/issue-24363.rs`
2019-12-10T10:38:50.1865891Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1865942Z status: exit code: 1
2019-12-10T10:38:50.1865942Z status: exit code: 1
2019-12-10T10:38:50.1866506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1866766Z ------------------------------------------
2019-12-10T10:38:50.1866793Z 
2019-12-10T10:38:50.1866987Z ------------------------------------------
2019-12-10T10:38:50.1867029Z stderr:
2019-12-10T10:38:50.1867029Z stderr:
2019-12-10T10:38:50.1867203Z ------------------------------------------
2019-12-10T10:38:50.1867423Z error[E0610]: `{integer}` is a primitive type and therefore doesn't have fields
2019-12-10T10:38:50.1867610Z   --> /checkout/src/test/ui/issues/issue-24363.rs:2:7
2019-12-10T10:38:50.1867648Z    |
2019-12-10T10:38:50.1867876Z LL |     1.create_a_type_error[ //~ `{integer}` is a primitive type and therefore doesn't have fields
2019-12-10T10:38:50.1867940Z 
2019-12-10T10:38:50.1867972Z error[E0369]: cannot add `()` to `()`
2019-12-10T10:38:50.1868174Z   --> /checkout/src/test/ui/issues/issue-24363.rs:3:11
2019-12-10T10:38:50.1868211Z    |
2019-12-10T10:38:50.1868211Z    |
2019-12-10T10:38:50.1868247Z LL |         ()+() //~ ERROR binary operation `+` cannot be applied
2019-12-10T10:38:50.1868425Z    |         --^-- ()
2019-12-10T10:38:50.1868491Z    |         ()
2019-12-10T10:38:50.1868538Z    |
2019-12-10T10:38:50.1868587Z    = note: an implementation of `std::ops::Add` might be missing for `()`
2019-12-10T10:38:50.1868612Z 
---
2019-12-10T10:38:50.1870423Z 
2019-12-10T10:38:50.1870469Z 8    |
2019-12-10T10:38:50.1870582Z 9    = note: an implementation of `std::ops::Add` might be missing for `A`
2019-12-10T10:38:50.1870625Z 10 
2019-12-10T10:38:50.1870866Z - error[E0369]: binary operation `-` cannot be applied to type `A`
2019-12-10T10:38:50.1870906Z + error[E0369]: cannot substract `A` from `A`
2019-12-10T10:38:50.1871076Z 12   --> $DIR/issue-28837.rs:8:7
2019-12-10T10:38:50.1871279Z 14 LL |     a - a;
2019-12-10T10:38:50.1871304Z 
2019-12-10T10:38:50.1871334Z 18    |
2019-12-10T10:38:50.1871386Z 19    = note: an implementation of `std::ops::Sub` might be missing for `A`
---
2019-12-10T10:38:50.1871870Z 23    |
2019-12-10T10:38:50.1871902Z 24 LL |     a * a;
2019-12-10T10:38:50.1872005Z 
2019-12-10T10:38:50.1872035Z 28    |
2019-12-10T10:38:50.1872078Z 29    = note: an implementation of `std::ops::Mul` might be missing for `A`
2019-12-10T10:38:50.1872129Z 30 
2019-12-10T10:38:50.1872347Z - error[E0369]: binary operation `/` cannot be applied to type `A`
2019-12-10T10:38:50.1872386Z + error[E0369]: cannot divide `A` by `A`
2019-12-10T10:38:50.1872569Z 32   --> $DIR/issue-28837.rs:12:7
2019-12-10T10:38:50.1872636Z 34 LL |     a / a;
2019-12-10T10:38:50.1872710Z 
2019-12-10T10:38:50.1872759Z 38    |
2019-12-10T10:38:50.1872759Z 38    |
2019-12-10T10:38:50.1872796Z 39    = note: an implementation of `std::ops::Div` might be missing for `A`
2019-12-10T10:38:50.1872831Z 40 
2019-12-10T10:38:50.1873045Z - error[E0369]: binary operation `%` cannot be applied to type `A`
2019-12-10T10:38:50.1873084Z + error[E0369]: cannot mod `A` by `A`
2019-12-10T10:38:50.1873252Z 42   --> $DIR/issue-28837.rs:14:7
2019-12-10T10:38:50.1873342Z 44 LL |     a % a;
2019-12-10T10:38:50.1873364Z 
2019-12-10T10:38:50.1873399Z 48    |
2019-12-10T10:38:50.1873399Z 48    |
2019-12-10T10:38:50.1873434Z 49    = note: an implementation of `std::ops::Rem` might be missing for `A`
2019-12-10T10:38:50.1873683Z - error[E0369]: binary operation `&` cannot be applied to type `A`
2019-12-10T10:38:50.1873724Z + error[E0369]: no implementation for `A & A`
2019-12-10T10:38:50.1873912Z 52   --> $DIR/issue-28837.rs:16:7
2019-12-10T10:38:50.1873947Z 53    |
---
2019-12-10T10:38:50.1874645Z 
2019-12-10T10:38:50.1874680Z 68    |
2019-12-10T10:38:50.1874733Z 69    = note: an implementation of `std::ops::BitOr` might be missing for `A`
2019-12-10T10:38:50.1874768Z 70 
2019-12-10T10:38:50.1874966Z - error[E0369]: binary operation `<<` cannot be applied to type `A`
2019-12-10T10:38:50.1875023Z + error[E0369]: no implementation for `A << A
2019-12-10T10:38:50.1875190Z 72   --> $DIR/issue-28837.rs:20:7
2019-12-10T10:38:50.1875225Z 73    |
2019-12-10T10:38:50.1875273Z 74 LL |     a << a;
2019-12-10T10:38:50.1875325Z 78    |
2019-12-10T10:38:50.1875325Z 78    |
2019-12-10T10:38:50.1875361Z 79    = note: an implementation of `std::ops::Shl` might be missing for `A`
2019-12-10T10:38:50.1875413Z 80 
2019-12-10T10:38:50.1875605Z - error[E0369]: binary operation `>>` cannot be applied to type `A`
2019-12-10T10:38:50.1875645Z + error[E0369]: no implementation for `A << A
2019-12-10T10:38:50.1875830Z 82   --> $DIR/issue-28837.rs:22:7
2019-12-10T10:38:50.1875872Z 83    |
2019-12-10T10:38:50.1875903Z 84 LL |     a >> a;
2019-12-10T10:38:50.1876212Z 95    |     - ^^ - A
2019-12-10T10:38:50.1876248Z 96    |     |
2019-12-10T10:38:50.1876280Z 97    |     A
2019-12-10T10:38:50.1876421Z -    |
---
2019-12-10T10:38:50.1877553Z 110 
2019-12-10T10:38:50.1877588Z 111 error[E0369]: binary operation `<` cannot be applied to type `A`
2019-12-10T10:38:50.1877855Z 112   --> $DIR/issue-28837.rs:28:7
2019-12-10T10:38:50.1877886Z 
2019-12-10T10:38:50.1878062Z 115    |     - ^ - A
2019-12-10T10:38:50.1878128Z 117    |     A
2019-12-10T10:38:50.1878284Z -    |
2019-12-10T10:38:50.1878485Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`
2019-12-10T10:38:50.1878523Z 120 
---
2019-12-10T10:38:50.1879397Z 130 
2019-12-10T10:38:50.1879432Z 131 error[E0369]: binary operation `>` cannot be applied to type `A`
2019-12-10T10:38:50.1879611Z 132   --> $DIR/issue-28837.rs:32:7
2019-12-10T10:38:50.1879659Z 
2019-12-10T10:38:50.1879812Z 135    |     - ^ - A
2019-12-10T10:38:50.1879895Z 137    |     A
2019-12-10T10:38:50.1880039Z -    |
2019-12-10T10:38:50.1880238Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`
2019-12-10T10:38:50.1880275Z 140 
2019-12-10T10:38:50.1880275Z 140 
2019-12-10T10:38:50.1880328Z 141 error[E0369]: binary operation `>=` cannot be applied to type `A`
2019-12-10T10:38:50.1880498Z 142   --> $DIR/issue-28837.rs:34:7
2019-12-10T10:38:50.1880689Z 145    |     - ^^ - A
2019-12-10T10:38:50.1880724Z 146    |     |
2019-12-10T10:38:50.1880756Z 147    |     A
2019-12-10T10:38:50.1880897Z -    |
---
2019-12-10T10:38:50.1881258Z 
2019-12-10T10:38:50.1881277Z 
2019-12-10T10:38:50.1881317Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1881573Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28837/issue-28837.stderr
2019-12-10T10:38:50.1881771Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1881979Z To only update this specific test, also pass `--test-args issues/issue-28837.rs`
2019-12-10T10:38:50.1882055Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1882089Z status: exit code: 1
2019-12-10T10:38:50.1882089Z status: exit code: 1
2019-12-10T10:38:50.1882734Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28837.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28837" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28837/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1883034Z ------------------------------------------
2019-12-10T10:38:50.1883061Z 
2019-12-10T10:38:50.1883235Z ------------------------------------------
2019-12-10T10:38:50.1883269Z stderr:
2019-12-10T10:38:50.1883269Z stderr:
2019-12-10T10:38:50.1883453Z ------------------------------------------
2019-12-10T10:38:50.1883490Z error[E0369]: cannot add `A` to `A`
2019-12-10T10:38:50.1883672Z   --> /checkout/src/test/ui/issues/issue-28837.rs:6:7
2019-12-10T10:38:50.1883724Z    |
2019-12-10T10:38:50.1883761Z LL |     a + a; //~ ERROR binary operation `+` cannot be applied to type `A`
2019-12-10T10:38:50.1883917Z    |     - ^ - A
2019-12-10T10:38:50.1884006Z    |     A
2019-12-10T10:38:50.1884037Z    |
2019-12-10T10:38:50.1884142Z    = note: an implementation of `std::ops::Add` might be missing for `A`
2019-12-10T10:38:50.1884190Z 
2019-12-10T10:38:50.1884190Z 
2019-12-10T10:38:50.1884223Z error[E0369]: cannot substract `A` from `A`
2019-12-10T10:38:50.1884436Z   --> /checkout/src/test/ui/issues/issue-28837.rs:8:7
2019-12-10T10:38:50.1884491Z    |
2019-12-10T10:38:50.1884688Z LL |     a - a; //~ ERROR binary operation `-` cannot be applied to type `A`
2019-12-10T10:38:50.1884840Z    |     - ^ - A
2019-12-10T10:38:50.1884921Z    |     A
2019-12-10T10:38:50.1884951Z    |
2019-12-10T10:38:50.1884987Z    = note: an implementation of `std::ops::Sub` might be missing for `A`
2019-12-10T10:38:50.1885028Z 
2019-12-10T10:38:50.1885028Z 
2019-12-10T10:38:50.1885060Z error[E0369]: cannot multiply `A` to `A`
2019-12-10T10:38:50.1885249Z   --> /checkout/src/test/ui/issues/issue-28837.rs:10:7
2019-12-10T10:38:50.1885286Z    |
2019-12-10T10:38:50.1885391Z LL |     a * a; //~ ERROR binary operation `*` cannot be applied to type `A`
2019-12-10T10:38:50.1885575Z    |     - ^ - A
2019-12-10T10:38:50.1885666Z    |     A
2019-12-10T10:38:50.1885702Z    |
2019-12-10T10:38:50.1885702Z    |
2019-12-10T10:38:50.1885738Z    = note: an implementation of `std::ops::Mul` might be missing for `A`
2019-12-10T10:38:50.1885810Z error[E0369]: cannot divide `A` by `A`
2019-12-10T10:38:50.1886005Z   --> /checkout/src/test/ui/issues/issue-28837.rs:12:7
2019-12-10T10:38:50.1886041Z    |
2019-12-10T10:38:50.1886041Z    |
2019-12-10T10:38:50.1886095Z LL |     a / a; //~ ERROR binary operation `/` cannot be applied to type `A`
2019-12-10T10:38:50.1886246Z    |     - ^ - A
2019-12-10T10:38:50.1886328Z    |     A
2019-12-10T10:38:50.1886359Z    |
2019-12-10T10:38:50.1886359Z    |
2019-12-10T10:38:50.1886395Z    = note: an implementation of `std::ops::Div` might be missing for `A`
2019-12-10T10:38:50.1886419Z 
2019-12-10T10:38:50.1886468Z error[E0369]: cannot mod `A` by `A`
2019-12-10T10:38:50.1886653Z   --> /checkout/src/test/ui/issues/issue-28837.rs:14:7
2019-12-10T10:38:50.1886690Z    |
2019-12-10T10:38:50.1886755Z LL |     a % a; //~ ERROR binary operation `%` cannot be applied to type `A`
2019-12-10T10:38:50.1886911Z    |     - ^ - A
2019-12-10T10:38:50.1886976Z    |     A
2019-12-10T10:38:50.1887023Z    |
2019-12-10T10:38:50.1887023Z    |
2019-12-10T10:38:50.1887059Z    = note: an implementation of `std::ops::Rem` might be missing for `A`
2019-12-10T10:38:50.1887137Z error[E0369]: no implementation for `A & A`
2019-12-10T10:38:50.1887323Z   --> /checkout/src/test/ui/issues/issue-28837.rs:16:7
2019-12-10T10:38:50.1887360Z    |
2019-12-10T10:38:50.1887360Z    |
2019-12-10T10:38:50.1887396Z LL |     a & a; //~ ERROR binary operation `&` cannot be applied to type `A`
2019-12-10T10:38:50.1887572Z    |     - ^ - A
2019-12-10T10:38:50.1887638Z    |     A
2019-12-10T10:38:50.1887685Z    |
2019-12-10T10:38:50.1887721Z    = note: an implementation of `std::ops::BitAnd` might be missing for `A`
2019-12-10T10:38:50.1887747Z 
2019-12-10T10:38:50.1887747Z 
2019-12-10T10:38:50.1890675Z error[E0369]: no implementation for `A | A`
2019-12-10T10:38:50.1891255Z   --> /checkout/src/test/ui/issues/issue-28837.rs:18:7
2019-12-10T10:38:50.1891314Z    |
2019-12-10T10:38:50.1891350Z LL |     a | a; //~ ERROR binary operation `|` cannot be applied to type `A`
2019-12-10T10:38:50.1891576Z    |     - ^ - A
2019-12-10T10:38:50.1891640Z    |     A
2019-12-10T10:38:50.1891687Z    |
2019-12-10T10:38:50.1891722Z    = note: an implementation of `std::ops::BitOr` might be missing for `A`
2019-12-10T10:38:50.1891748Z 
2019-12-10T10:38:50.1891748Z 
2019-12-10T10:38:50.1891780Z error[E0369]: no implementation for `A << A
2019-12-10T10:38:50.1891977Z   --> /checkout/src/test/ui/issues/issue-28837.rs:20:7
2019-12-10T10:38:50.1892012Z    |
2019-12-10T10:38:50.1892046Z LL |     a << a; //~ ERROR binary operation `<<` cannot be applied to type `A`
2019-12-10T10:38:50.1892204Z    |     - ^^ - A
2019-12-10T10:38:50.1892268Z    |     A
2019-12-10T10:38:50.1892296Z    |
2019-12-10T10:38:50.1892296Z    |
2019-12-10T10:38:50.1892445Z    = note: an implementation of `std::ops::Shl` might be missing for `A`
2019-12-10T10:38:50.1892501Z error[E0369]: no implementation for `A << A
2019-12-10T10:38:50.1892722Z   --> /checkout/src/test/ui/issues/issue-28837.rs:22:7
2019-12-10T10:38:50.1892757Z    |
2019-12-10T10:38:50.1892757Z    |
2019-12-10T10:38:50.1892793Z LL |     a >> a; //~ ERROR binary operation `>>` cannot be applied to type `A`
2019-12-10T10:38:50.1892955Z    |     - ^^ - A
2019-12-10T10:38:50.1893019Z    |     A
2019-12-10T10:38:50.1893047Z    |
2019-12-10T10:38:50.1893047Z    |
2019-12-10T10:38:50.1893099Z    = note: an implementation of `std::ops::Shr` might be missing for `A`
2019-12-10T10:38:50.1893155Z error[E0369]: binary operation `==` cannot be applied to type `A`
2019-12-10T10:38:50.1893347Z   --> /checkout/src/test/ui/issues/issue-28837.rs:24:7
2019-12-10T10:38:50.1893382Z    |
2019-12-10T10:38:50.1893417Z LL |     a == a; //~ ERROR binary operation `==` cannot be applied to type `A`
2019-12-10T10:38:50.1893417Z LL |     a == a; //~ ERROR binary operation `==` cannot be applied to type `A`
2019-12-10T10:38:50.1893566Z    |     - ^^ - A
2019-12-10T10:38:50.1893622Z    |     |
2019-12-10T10:38:50.1893651Z    |     A
2019-12-10T10:38:50.1893671Z 
2019-12-10T10:38:50.1893704Z error[E0369]: binary operation `!=` cannot be applied to type `A`
2019-12-10T10:38:50.1893899Z   --> /checkout/src/test/ui/issues/issue-28837.rs:26:7
2019-12-10T10:38:50.1893934Z    |
2019-12-10T10:38:50.1893969Z LL |     a != a; //~ ERROR binary operation `!=` cannot be applied to type `A`
2019-12-10T10:38:50.1894127Z    |     - ^^ - A
2019-12-10T10:38:50.1894189Z    |     A
2019-12-10T10:38:50.1894208Z 
2019-12-10T10:38:50.1894258Z error[E0369]: binary operation `<` cannot be applied to type `A`
2019-12-10T10:38:50.1894433Z   --> /checkout/src/test/ui/issues/issue-28837.rs:28:7
2019-12-10T10:38:50.1894433Z   --> /checkout/src/test/ui/issues/issue-28837.rs:28:7
2019-12-10T10:38:50.1894468Z    |
2019-12-10T10:38:50.1894513Z LL |     a < a; //~ ERROR binary operation `<` cannot be applied to type `A`
2019-12-10T10:38:50.1894655Z    |     - ^ - A
2019-12-10T10:38:50.1894741Z    |     A
2019-12-10T10:38:50.1894766Z 
2019-12-10T10:38:50.1894800Z error[E0369]: binary operation `<=` cannot be applied to type `A`
2019-12-10T10:38:50.1895349Z   --> /checkout/src/test/ui/issues/issue-28837.rs:30:7
2019-12-10T10:38:50.1895349Z   --> /checkout/src/test/ui/issues/issue-28837.rs:30:7
2019-12-10T10:38:50.1895387Z    |
2019-12-10T10:38:50.1895424Z LL |     a <= a; //~ ERROR binary operation `<=` cannot be applied to type `A`
2019-12-10T10:38:50.1895597Z    |     - ^^ - A
2019-12-10T10:38:50.1895664Z    |     A
2019-12-10T10:38:50.1895684Z 
2019-12-10T10:38:50.1895906Z error[E0369]: binary operation `>` cannot be applied to type `A`
2019-12-10T10:38:50.1896147Z   --> /checkout/src/test/ui/issues/issue-28837.rs:32:7
2019-12-10T10:38:50.1896147Z   --> /checkout/src/test/ui/issues/issue-28837.rs:32:7
2019-12-10T10:38:50.1896186Z    |
2019-12-10T10:38:50.1896244Z LL |     a > a; //~ ERROR binary operation `>` cannot be applied to type `A`
2019-12-10T10:38:50.1896398Z    |     - ^ - A
2019-12-10T10:38:50.1896483Z    |     A
2019-12-10T10:38:50.1896513Z 
2019-12-10T10:38:50.1896513Z 
2019-12-10T10:38:50.1896708Z error[E0369]: binary operation `>=` cannot be applied to type `A`
2019-12-10T10:38:50.1897627Z   --> /checkout/src/test/ui/issues/issue-28837.rs:34:7
2019-12-10T10:38:50.1897702Z    |
2019-12-10T10:38:50.1897740Z LL |     a >= a; //~ ERROR binary operation `>=` cannot be applied to type `A`
2019-12-10T10:38:50.1897911Z    |     - ^^ - A
2019-12-10T10:38:50.1897995Z    |     A
2019-12-10T10:38:50.1898017Z 
2019-12-10T10:38:50.1898051Z error: aborting due to 15 previous errors
2019-12-10T10:38:50.1898092Z 
---
2019-12-10T10:38:50.1898692Z 
2019-12-10T10:38:50.1898852Z ---- [ui] ui/issues/issue-31076.rs stdout ----
2019-12-10T10:38:50.1898887Z diff of stderr:
2019-12-10T10:38:50.1898926Z 
2019-12-10T10:38:50.1899273Z - error[E0369]: binary operation `+` cannot be applied to type `{integer}`
2019-12-10T10:38:50.1899316Z + error[E0369]: cannot add `{integer}` to `{integer}`
2019-12-10T10:38:50.1899477Z 2   --> $DIR/issue-31076.rs:13:15
2019-12-10T10:38:50.1899562Z 4 LL |     let x = 5 + 6;
2019-12-10T10:38:50.1899584Z 
2019-12-10T10:38:50.1899629Z 8    |
2019-12-10T10:38:50.1899665Z 9    = note: an implementation of `std::ops::Add` might be missing for `{integer}`
---
2019-12-10T10:38:50.1900195Z 
2019-12-10T10:38:50.1900213Z 
2019-12-10T10:38:50.1900246Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1900498Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/issue-31076.stderr
2019-12-10T10:38:50.1900687Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1901058Z To only update this specific test, also pass `--test-args issues/issue-31076.rs`
2019-12-10T10:38:50.1901152Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1901185Z status: exit code: 1
2019-12-10T10:38:50.1901185Z status: exit code: 1
2019-12-10T10:38:50.1901803Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1902254Z ------------------------------------------
2019-12-10T10:38:50.1902289Z 
2019-12-10T10:38:50.1902462Z ------------------------------------------
2019-12-10T10:38:50.1902497Z stderr:
2019-12-10T10:38:50.1902497Z stderr:
2019-12-10T10:38:50.1902671Z ------------------------------------------
2019-12-10T10:38:50.1902710Z error[E0369]: cannot add `{integer}` to `{integer}`
2019-12-10T10:38:50.1907647Z   --> /checkout/src/test/ui/issues/issue-31076.rs:13:15
2019-12-10T10:38:50.1907807Z LL |     let x = 5 + 6;
2019-12-10T10:38:50.1907807Z LL |     let x = 5 + 6;
2019-12-10T10:38:50.1908105Z    |             - ^ - {integer}
2019-12-10T10:38:50.1908196Z    |             {integer}
2019-12-10T10:38:50.1908227Z    |
2019-12-10T10:38:50.1908280Z    = note: an implementation of `std::ops::Add` might be missing for `{integer}`
2019-12-10T10:38:50.1908308Z 
---
2019-12-10T10:38:50.1910038Z 
2019-12-10T10:38:50.1910226Z ---- [ui] ui/issues/issue-35668.rs stdout ----
2019-12-10T10:38:50.1910266Z diff of stderr:
2019-12-10T10:38:50.1910288Z 
2019-12-10T10:38:50.1910589Z - error[E0369]: binary operation `*` cannot be applied to type `&T`
2019-12-10T10:38:50.1912554Z + error[E0369]: cannot multiply `&T` to `&T`
2019-12-10T10:38:50.1913040Z 2   --> $DIR/issue-35668.rs:2:23
2019-12-10T10:38:50.1913086Z 3    |
2019-12-10T10:38:50.1913142Z 4 LL |     a.iter().map(|a| a*a)
2019-12-10T10:38:50.1913187Z 
2019-12-10T10:38:50.1913223Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1913487Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/issue-35668.stderr
2019-12-10T10:38:50.1913487Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/issue-35668.stderr
2019-12-10T10:38:50.1913687Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1913896Z To only update this specific test, also pass `--test-args issues/issue-35668.rs`
2019-12-10T10:38:50.1913978Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1914012Z status: exit code: 1
2019-12-10T10:38:50.1914012Z status: exit code: 1
2019-12-10T10:38:50.1914784Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35668.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1915043Z ------------------------------------------
2019-12-10T10:38:50.1915068Z 
2019-12-10T10:38:50.1915229Z ------------------------------------------
2019-12-10T10:38:50.1915263Z stderr:
2019-12-10T10:38:50.1915263Z stderr:
2019-12-10T10:38:50.1915437Z ------------------------------------------
2019-12-10T10:38:50.1915475Z error[E0369]: cannot multiply `&T` to `&T`
2019-12-10T10:38:50.1915646Z   --> /checkout/src/test/ui/issues/issue-35668.rs:2:23
2019-12-10T10:38:50.1915707Z    |
2019-12-10T10:38:50.1915737Z LL |     a.iter().map(|a| a*a)
2019-12-10T10:38:50.1915896Z    |                      -^- &T
2019-12-10T10:38:50.1915984Z    |                      &T
2019-12-10T10:38:50.1916014Z    |
2019-12-10T10:38:50.1916048Z    = note: an implementation of `std::ops::Mul` might be missing for `&T`
2019-12-10T10:38:50.1916090Z 
---
2019-12-10T10:38:50.1917079Z 
2019-12-10T10:38:50.1917376Z ---- [ui] ui/issues/issue-3820.rs stdout ----
2019-12-10T10:38:50.1917414Z diff of stderr:
2019-12-10T10:38:50.1917435Z 
2019-12-10T10:38:50.1917617Z - error[E0369]: binary operation `*` cannot be applied to type `Thing`
2019-12-10T10:38:50.1917687Z + error[E0369]: cannot multiply `{integer}` to `Thing`
2019-12-10T10:38:50.1917954Z 2   --> $DIR/issue-3820.rs:14:15
2019-12-10T10:38:50.1917999Z 3    |
2019-12-10T10:38:50.1918049Z 4 LL |     let w = u * 3;
2019-12-10T10:38:50.1918088Z 
2019-12-10T10:38:50.1918121Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1918392Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820/issue-3820.stderr
2019-12-10T10:38:50.1918392Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820/issue-3820.stderr
2019-12-10T10:38:50.1918580Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1918777Z To only update this specific test, also pass `--test-args issues/issue-3820.rs`
2019-12-10T10:38:50.1918854Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1919332Z status: exit code: 1
2019-12-10T10:38:50.1919332Z status: exit code: 1
2019-12-10T10:38:50.1920021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3820.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1920412Z ------------------------------------------
2019-12-10T10:38:50.1920438Z 
2019-12-10T10:38:50.1920601Z ------------------------------------------
2019-12-10T10:38:50.1920635Z stderr:
2019-12-10T10:38:50.1920635Z stderr:
2019-12-10T10:38:50.1920810Z ------------------------------------------
2019-12-10T10:38:50.1920848Z error[E0369]: cannot multiply `{integer}` to `Thing`
2019-12-10T10:38:50.1921019Z   --> /checkout/src/test/ui/issues/issue-3820.rs:14:15
2019-12-10T10:38:50.1921073Z    |
2019-12-10T10:38:50.1921110Z LL |     let w = u * 3; //~ ERROR binary operation `*` cannot be applied to type `Thing`
2019-12-10T10:38:50.1921280Z    |             - ^ - {integer}
2019-12-10T10:38:50.1921367Z    |             Thing
2019-12-10T10:38:50.1921397Z    |
2019-12-10T10:38:50.1921397Z    |
2019-12-10T10:38:50.1921449Z    = note: an implementation of `std::ops::Mul` might be missing for `Thing`
2019-12-10T10:38:50.1921503Z error: aborting due to previous error
2019-12-10T10:38:50.1921524Z 
2019-12-10T10:38:50.1921726Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1921751Z 
---
2019-12-10T10:38:50.1922563Z - error[E0369]: binary operation `+` cannot be applied to type `()`
2019-12-10T10:38:50.1922621Z + error[E0369]: cannot add `()` to `()`
2019-12-10T10:38:50.1922790Z 2   --> $DIR/issue-40610.rs:4:8
2019-12-10T10:38:50.1922832Z 3    |
2019-12-10T10:38:50.1922882Z 4 LL |     () + f(&[1.0]);
2019-12-10T10:38:50.1922924Z 
2019-12-10T10:38:50.1922958Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1923209Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/issue-40610.stderr
2019-12-10T10:38:50.1923209Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/issue-40610.stderr
2019-12-10T10:38:50.1923404Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1923608Z To only update this specific test, also pass `--test-args issues/issue-40610.rs`
2019-12-10T10:38:50.1923689Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1923723Z status: exit code: 1
2019-12-10T10:38:50.1923723Z status: exit code: 1
2019-12-10T10:38:50.1924384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40610.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1924683Z ------------------------------------------
2019-12-10T10:38:50.1924710Z 
2019-12-10T10:38:50.1924876Z ------------------------------------------
2019-12-10T10:38:50.1924911Z stderr:
2019-12-10T10:38:50.1924911Z stderr:
2019-12-10T10:38:50.1925259Z ------------------------------------------
2019-12-10T10:38:50.1925295Z error[E0369]: cannot add `()` to `()`
2019-12-10T10:38:50.1925463Z   --> /checkout/src/test/ui/issues/issue-40610.rs:4:8
2019-12-10T10:38:50.1925515Z    |
2019-12-10T10:38:50.1925546Z LL |     () + f(&[1.0]);
2019-12-10T10:38:50.1925691Z    |     -- ^ --------- ()
2019-12-10T10:38:50.1925835Z    |     ()
2019-12-10T10:38:50.1925865Z    |
2019-12-10T10:38:50.1925905Z    = note: an implementation of `std::ops::Add` might be missing for `()`
2019-12-10T10:38:50.1925951Z 
---
2019-12-10T10:38:50.1926458Z 
2019-12-10T10:38:50.1926639Z ---- [ui] ui/issues/issue-41394.rs stdout ----
2019-12-10T10:38:50.1926674Z diff of stderr:
2019-12-10T10:38:50.1926695Z 
2019-12-10T10:38:50.1926869Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.1926924Z + error[E0369]: cannot add `{integer}` to `&str`
2019-12-10T10:38:50.1927076Z 2   --> $DIR/issue-41394.rs:2:12
2019-12-10T10:38:50.1927165Z 4 LL |     A = "" + 1
2019-12-10T10:38:50.1927185Z 
2019-12-10T10:38:50.1927203Z 
2019-12-10T10:38:50.1927241Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1927241Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1927477Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/issue-41394.stderr
2019-12-10T10:38:50.1927660Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1927852Z To only update this specific test, also pass `--test-args issues/issue-41394.rs`
2019-12-10T10:38:50.1927928Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1927960Z status: exit code: 1
2019-12-10T10:38:50.1927960Z status: exit code: 1
2019-12-10T10:38:50.1928531Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1928780Z ------------------------------------------
2019-12-10T10:38:50.1928805Z 
2019-12-10T10:38:50.1928962Z ------------------------------------------
2019-12-10T10:38:50.1928995Z stderr:
2019-12-10T10:38:50.1928995Z stderr:
2019-12-10T10:38:50.1929167Z ------------------------------------------
2019-12-10T10:38:50.1929204Z error[E0369]: cannot add `{integer}` to `&str`
2019-12-10T10:38:50.1929374Z   --> /checkout/src/test/ui/issues/issue-41394.rs:2:12
2019-12-10T10:38:50.1929458Z LL |     A = "" + 1
2019-12-10T10:38:50.1929458Z LL |     A = "" + 1
2019-12-10T10:38:50.1929606Z    |         -- ^ - {integer}
2019-12-10T10:38:50.1929688Z    |         &str
2019-12-10T10:38:50.1929718Z    |
2019-12-10T10:38:50.1929752Z    = note: an implementation of `std::ops::Add` might be missing for `&str`
2019-12-10T10:38:50.1929800Z 
2019-12-10T10:38:50.1929800Z 
2019-12-10T10:38:50.1929895Z error[E0080]: evaluation of constant value failed
2019-12-10T10:38:50.1930102Z   --> /checkout/src/test/ui/issues/issue-41394.rs:7:9
2019-12-10T10:38:50.1930156Z    |
2019-12-10T10:38:50.1930187Z LL |     A = Foo::A as isize
2019-12-10T10:38:50.1930223Z    |         ^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-10T10:38:50.1930295Z error: aborting due to 2 previous errors
2019-12-10T10:38:50.1930316Z 
2019-12-10T10:38:50.1930348Z Some errors have detailed explanations: E0080, E0369.
2019-12-10T10:38:50.1930546Z For more information about an error, try `rustc --explain E0080`.
---
2019-12-10T10:38:50.1931452Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.1931512Z + error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.1931678Z 2   --> $DIR/issue-47377.rs:4:14
2019-12-10T10:38:50.1931713Z 3    |
2019-12-10T10:38:50.1931766Z 4 LL |      let _a = b + ", World!";
2019-12-10T10:38:50.1931808Z 
2019-12-10T10:38:50.1931842Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1932090Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377/issue-47377.stderr
2019-12-10T10:38:50.1932090Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377/issue-47377.stderr
2019-12-10T10:38:50.1932283Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1932484Z To only update this specific test, also pass `--test-args issues/issue-47377.rs`
2019-12-10T10:38:50.1932563Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1932597Z status: exit code: 1
2019-12-10T10:38:50.1932597Z status: exit code: 1
2019-12-10T10:38:50.1933208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47377.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1933465Z ------------------------------------------
2019-12-10T10:38:50.1933492Z 
2019-12-10T10:38:50.1933656Z ------------------------------------------
2019-12-10T10:38:50.1933691Z stderr:
2019-12-10T10:38:50.1933691Z stderr:
2019-12-10T10:38:50.1933871Z ------------------------------------------
2019-12-10T10:38:50.1933909Z error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.1934087Z   --> /checkout/src/test/ui/issues/issue-47377.rs:4:14
2019-12-10T10:38:50.1934147Z    |
2019-12-10T10:38:50.1934186Z LL |      let _a = b + ", World!";
2019-12-10T10:38:50.1934354Z    |               - ^ ---------- &str
2019-12-10T10:38:50.1934580Z    |               | |
2019-12-10T10:38:50.1934616Z    |               | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.1934704Z    |
2019-12-10T10:38:50.1934704Z    |
2019-12-10T10:38:50.1934755Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.1934799Z    |
2019-12-10T10:38:50.1934850Z LL |      let _a = b.to_owned() + ", World!";
2019-12-10T10:38:50.1935079Z 
2019-12-10T10:38:50.1935111Z error: aborting due to previous error
2019-12-10T10:38:50.1935328Z 
2019-12-10T10:38:50.1935530Z For more information about this error, try `rustc --explain E0369`.
---
2019-12-10T10:38:50.1936770Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.1936811Z + error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.1936995Z 2   --> $DIR/issue-47380.rs:3:35
2019-12-10T10:38:50.1937031Z 3    |
2019-12-10T10:38:50.1937233Z 4 LL |     println!("🦀🦀🦀🦀🦀"); let _a = b + ", World!";
2019-12-10T10:38:50.1937298Z 
2019-12-10T10:38:50.1937334Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1937571Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380/issue-47380.stderr
2019-12-10T10:38:50.1937571Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380/issue-47380.stderr
2019-12-10T10:38:50.1937881Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1938117Z To only update this specific test, also pass `--test-args issues/issue-47380.rs`
2019-12-10T10:38:50.1938350Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1938400Z status: exit code: 1
2019-12-10T10:38:50.1938400Z status: exit code: 1
2019-12-10T10:38:50.1938948Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47380.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1939191Z ------------------------------------------
2019-12-10T10:38:50.1939222Z 
2019-12-10T10:38:50.1939405Z ------------------------------------------
2019-12-10T10:38:50.1939440Z stderr:
2019-12-10T10:38:50.1939440Z stderr:
2019-12-10T10:38:50.1939596Z ------------------------------------------
2019-12-10T10:38:50.1939631Z error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.1939820Z   --> /checkout/src/test/ui/issues/issue-47380.rs:3:35
2019-12-10T10:38:50.1939854Z    |
2019-12-10T10:38:50.1940392Z LL |     println!("🦀🦀🦀🦀🦀"); let _a = b + ", World!";
2019-12-10T10:38:50.1940603Z    |                                      - ^ ---------- &str
2019-12-10T10:38:50.1940647Z    |                                      | |
2019-12-10T10:38:50.1940689Z    |                                      | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.1940784Z    |
2019-12-10T10:38:50.1940784Z    |
2019-12-10T10:38:50.1940847Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.1940918Z    |
2019-12-10T10:38:50.1941122Z LL |     println!("🦀🦀🦀🦀🦀"); let _a = b.to_owned() + ", World!";
2019-12-10T10:38:50.1941208Z 
2019-12-10T10:38:50.1941241Z error: aborting due to previous error
2019-12-10T10:38:50.1941265Z 
2019-12-10T10:38:50.1941475Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1941475Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.1941503Z 
2019-12-10T10:38:50.1941669Z ------------------------------------------
2019-12-10T10:38:50.1941694Z 
2019-12-10T10:38:50.1941714Z 
2019-12-10T10:38:50.1941904Z ---- [ui] ui/issues/issue-59488.rs stdout ----
2019-12-10T10:38:50.1941942Z diff of stderr:
2019-12-10T10:38:50.1941964Z 
2019-12-10T10:38:50.1942134Z 58    |     --- ^ --- fn(i64) -> i64 {bar}
2019-12-10T10:38:50.1942361Z 60    |     fn() -> i32 {foo}
2019-12-10T10:38:50.1942503Z -    |
2019-12-10T10:38:50.1942811Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> i32 {foo}`
2019-12-10T10:38:50.1942862Z 63 
2019-12-10T10:38:50.1942862Z 63 
2019-12-10T10:38:50.1942897Z 64 error[E0308]: mismatched types
2019-12-10T10:38:50.1943117Z 65   --> $DIR/issue-59488.rs:25:11
2019-12-10T10:38:50.1943143Z 
2019-12-10T10:38:50.1943475Z 79    |     fn(usize) -> Foo {Foo::Bar}
2019-12-10T10:38:50.1943630Z 80    |     fn(usize) -> Foo {Foo::Bar}
2019-12-10T10:38:50.1943855Z 81    |
2019-12-10T10:38:50.1944072Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1944392Z 84 
2019-12-10T10:38:50.1944392Z 84 
2019-12-10T10:38:50.1944615Z 85 error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2019-12-10T10:38:50.1944739Z 
2019-12-10T10:38:50.1944802Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1945080Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
2019-12-10T10:38:50.1945080Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
2019-12-10T10:38:50.1945289Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1945525Z To only update this specific test, also pass `--test-args issues/issue-59488.rs`
2019-12-10T10:38:50.1945589Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1945643Z status: exit code: 1
2019-12-10T10:38:50.1945643Z status: exit code: 1
2019-12-10T10:38:50.1946244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1946527Z ------------------------------------------
2019-12-10T10:38:50.1946555Z 
2019-12-10T10:38:50.1946905Z ------------------------------------------
2019-12-10T10:38:50.1946959Z stderr:
2019-12-10T10:38:50.1946959Z stderr:
2019-12-10T10:38:50.1947120Z ------------------------------------------
2019-12-10T10:38:50.1947310Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2019-12-10T10:38:50.1947505Z   --> /checkout/src/test/ui/issues/issue-59488.rs:14:9
2019-12-10T10:38:50.1947573Z LL |     foo > 12;
2019-12-10T10:38:50.1947573Z LL |     foo > 12;
2019-12-10T10:38:50.1947741Z    |     --- ^ -- {integer}
2019-12-10T10:38:50.1947922Z    |     fn() -> i32 {foo}
2019-12-10T10:38:50.1947978Z    |     help: you might have forgotten to call this function: `foo()`
2019-12-10T10:38:50.1948011Z 
2019-12-10T10:38:50.1948046Z error[E0308]: mismatched types
---
2019-12-10T10:38:50.1948566Z    |
2019-12-10T10:38:50.1948740Z    = note: expected fn item `fn() -> i32 {foo}`
2019-12-10T10:38:50.1948778Z                  found type `i32`
2019-12-10T10:38:50.1948800Z 
2019-12-10T10:38:50.1949016Z error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
2019-12-10T10:38:50.1949197Z   --> /checkout/src/test/ui/issues/issue-59488.rs:18:9
2019-12-10T10:38:50.1949282Z LL |     bar > 13;
2019-12-10T10:38:50.1949282Z LL |     bar > 13;
2019-12-10T10:38:50.1949436Z    |     --- ^ -- {integer}
2019-12-10T10:38:50.1949472Z    |     |
2019-12-10T10:38:50.1949627Z    |     fn(i64) -> i64 {bar}
2019-12-10T10:38:50.1949785Z    |     help: you might have forgotten to call this function: `bar( /* arguments */ )`
2019-12-10T10:38:50.1949853Z error[E0308]: mismatched types
2019-12-10T10:38:50.1950084Z   --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
2019-12-10T10:38:50.1950121Z    |
2019-12-10T10:38:50.1950153Z LL |     bar > 13;
2019-12-10T10:38:50.1950153Z LL |     bar > 13;
2019-12-10T10:38:50.1950188Z    |           ^^ expected fn item, found integer
2019-12-10T10:38:50.1950238Z    |
2019-12-10T10:38:50.1950414Z    = note: expected fn item `fn(i64) -> i64 {bar}`
2019-12-10T10:38:50.1950491Z 
2019-12-10T10:38:50.1950491Z 
2019-12-10T10:38:50.1950690Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2019-12-10T10:38:50.1950870Z   --> /checkout/src/test/ui/issues/issue-59488.rs:22:9
2019-12-10T10:38:50.1950955Z LL |     foo > foo;
2019-12-10T10:38:50.1950955Z LL |     foo > foo;
2019-12-10T10:38:50.1951121Z    |     --- ^ --- fn() -> i32 {foo}
2019-12-10T10:38:50.1951591Z    |     fn() -> i32 {foo}
2019-12-10T10:38:50.1951630Z    |
2019-12-10T10:38:50.1951664Z help: you might have forgotten to call this function
2019-12-10T10:38:50.1951715Z    |
2019-12-10T10:38:50.1951715Z    |
2019-12-10T10:38:50.1951749Z LL |     foo() > foo;
2019-12-10T10:38:50.1951780Z    |     ^^^^^
2019-12-10T10:38:50.1951993Z help: you might have forgotten to call this function
2019-12-10T10:38:50.1952044Z    |
2019-12-10T10:38:50.1952262Z LL |     foo > foo();
2019-12-10T10:38:50.1952296Z    |           ^^^^^
2019-12-10T10:38:50.1952335Z 
2019-12-10T10:38:50.1952547Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2019-12-10T10:38:50.1952734Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:9
2019-12-10T10:38:50.1952771Z    |
2019-12-10T10:38:50.1952822Z LL |     foo > bar;
2019-12-10T10:38:50.1952994Z    |     --- ^ --- fn(i64) -> i64 {bar}
2019-12-10T10:38:50.1953202Z    |     fn() -> i32 {foo}
2019-12-10T10:38:50.1953234Z 
2019-12-10T10:38:50.1953268Z error[E0308]: mismatched types
2019-12-10T10:38:50.1953459Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
2019-12-10T10:38:50.1953459Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
2019-12-10T10:38:50.1953690Z    |
2019-12-10T10:38:50.1953723Z LL |     foo > bar;
2019-12-10T10:38:50.1953763Z    |           ^^^ expected fn item, found a different fn item
2019-12-10T10:38:50.1953999Z    = note: expected fn item `fn() -> i32 {foo}`
2019-12-10T10:38:50.1953999Z    = note: expected fn item `fn() -> i32 {foo}`
2019-12-10T10:38:50.1954187Z               found fn item `fn(i64) -> i64 {bar}`
2019-12-10T10:38:50.1954214Z 
2019-12-10T10:38:50.1954449Z error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1954813Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2019-12-10T10:38:50.1954851Z    |
2019-12-10T10:38:50.1955166Z LL |     assert_eq!(Foo::Bar, i);
2019-12-10T10:38:50.1955242Z    |     |
2019-12-10T10:38:50.1955242Z    |     |
2019-12-10T10:38:50.1955604Z    |     fn(usize) -> Foo {Foo::Bar}
2019-12-10T10:38:50.1955789Z    |     fn(usize) -> Foo {Foo::Bar}
2019-12-10T10:38:50.1956104Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-10T10:38:50.1956160Z 
2019-12-10T10:38:50.1956160Z 
2019-12-10T10:38:50.1956382Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2019-12-10T10:38:50.1956580Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2019-12-10T10:38:50.1956639Z    |
2019-12-10T10:38:50.1956675Z LL |     assert_eq!(Foo::Bar, i);
2019-12-10T10:38:50.1956945Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2019-12-10T10:38:50.1957008Z    |
2019-12-10T10:38:50.1957383Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1957801Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1958250Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-10T10:38:50.1958285Z 
2019-12-10T10:38:50.1958285Z 
2019-12-10T10:38:50.1958850Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2019-12-10T10:38:50.1959049Z   --> /checkout/src/test/ui/issues/issue-59488.rs:30:5
2019-12-10T10:38:50.1959087Z    |
2019-12-10T10:38:50.1959141Z LL |     assert_eq!(Foo::Bar, i);
2019-12-10T10:38:50.1959409Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2019-12-10T10:38:50.1959454Z    |
2019-12-10T10:38:50.1959689Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1959937Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2019-12-10T10:38:50.1960380Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-10T10:38:50.1960415Z 
2019-12-10T10:38:50.1960453Z error: aborting due to 10 previous errors
2019-12-10T10:38:50.1960477Z 
---
2019-12-10T10:38:50.1961162Z 
2019-12-10T10:38:50.1961340Z ---- [ui] ui/issues/issue-62375.rs stdout ----
2019-12-10T10:38:50.1961378Z diff of stderr:
2019-12-10T10:38:50.1961418Z 
2019-12-10T10:38:50.1961596Z 5    |     - ^^ -------- fn(()) -> A {A::Value}
2019-12-10T10:38:50.1961676Z 7    |     A
2019-12-10T10:38:50.1962005Z -    |
2019-12-10T10:38:50.1962554Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
2019-12-10T10:38:50.1962592Z 10 
2019-12-10T10:38:50.1962592Z 10 
2019-12-10T10:38:50.1962646Z 11 error: aborting due to previous error
2019-12-10T10:38:50.1962679Z 12 
2019-12-10T10:38:50.1962701Z 
2019-12-10T10:38:50.1962721Z 
2019-12-10T10:38:50.1962773Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1963011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/issue-62375.stderr
2019-12-10T10:38:50.1963204Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1963426Z To only update this specific test, also pass `--test-args issues/issue-62375.rs`
2019-12-10T10:38:50.1963488Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1963523Z status: exit code: 1
2019-12-10T10:38:50.1963523Z status: exit code: 1
2019-12-10T10:38:50.1964147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-62375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1964418Z ------------------------------------------
2019-12-10T10:38:50.1964447Z 
2019-12-10T10:38:50.1964620Z ------------------------------------------
2019-12-10T10:38:50.1964676Z stderr:
2019-12-10T10:38:50.1964676Z stderr:
2019-12-10T10:38:50.1964844Z ------------------------------------------
2019-12-10T10:38:50.1964884Z error[E0369]: binary operation `==` cannot be applied to type `A`
2019-12-10T10:38:50.1965086Z   --> /checkout/src/test/ui/issues/issue-62375.rs:7:7
2019-12-10T10:38:50.1965134Z    |
2019-12-10T10:38:50.1965232Z LL |     a == A::Value;
2019-12-10T10:38:50.1965629Z    |     - ^^ -------- fn(()) -> A {A::Value}
2019-12-10T10:38:50.1965699Z    |     A
2019-12-10T10:38:50.1965720Z 
2019-12-10T10:38:50.1965770Z error: aborting due to previous error
2019-12-10T10:38:50.1965792Z 
---
2019-12-10T10:38:50.1966680Z 
2019-12-10T10:38:50.1966713Z 104    |
2019-12-10T10:38:50.1966749Z 105    = note: `#[warn(incomplete_features)]` on by default
2019-12-10T10:38:50.1966802Z 106 
2019-12-10T10:38:50.1966993Z - error[E0369]: binary operation `|` cannot be applied to type `E`
2019-12-10T10:38:50.1967114Z + error[E0369]: no implementation for `E | ()`
2019-12-10T10:38:50.1967337Z 108   --> $DIR/or-patterns-syntactic-fail.rs:24:22
2019-12-10T10:38:50.1967376Z 109    |
2019-12-10T10:38:50.1967410Z 110 LL |     let _ = |A | B: E| ();
2019-12-10T10:38:50.1967454Z 
2019-12-10T10:38:50.1967506Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.1967774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/or-patterns-syntactic-fail.stderr
2019-12-10T10:38:50.1967774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/or-patterns-syntactic-fail.stderr
2019-12-10T10:38:50.1967990Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.1968214Z To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`
2019-12-10T10:38:50.1968281Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.1968334Z status: exit code: 1
2019-12-10T10:38:50.1968334Z status: exit code: 1
2019-12-10T10:38:50.1969156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary" "-A" "unused"
2019-12-10T10:38:50.1969419Z ------------------------------------------
2019-12-10T10:38:50.1969445Z 
2019-12-10T10:38:50.1969631Z ------------------------------------------
2019-12-10T10:38:50.1969666Z stderr:
2019-12-10T10:38:50.1969666Z stderr:
2019-12-10T10:38:50.1969828Z ------------------------------------------
2019-12-10T10:38:50.1970292Z error: an or-pattern parameter must be wrapped in parenthesis
2019-12-10T10:38:50.1970513Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:28:13
2019-12-10T10:38:50.1970554Z    |
2019-12-10T10:38:50.1970783Z LL |     fn fun1(A | B: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
2019-12-10T10:38:50.1970832Z    |             ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`
2019-12-10T10:38:50.1970912Z error: a leading `|` is not allowed in a parameter pattern
2019-12-10T10:38:50.1971117Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:30:13
2019-12-10T10:38:50.1971156Z    |
2019-12-10T10:38:50.1971156Z    |
2019-12-10T10:38:50.1971189Z LL |     fn fun2(| A | B: E) {}
2019-12-10T10:38:50.1971248Z    |             ^ help: remove the `|`
2019-12-10T10:38:50.1971457Z error: an or-pattern parameter must be wrapped in parenthesis
2019-12-10T10:38:50.1971676Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:30:15
2019-12-10T10:38:50.1971715Z    |
2019-12-10T10:38:50.1971715Z    |
2019-12-10T10:38:50.1971755Z LL |     fn fun2(| A | B: E) {}
2019-12-10T10:38:50.1971880Z    |               ^^^^^ help: wrap the pattern in parenthesis: `(A | B)`
2019-12-10T10:38:50.1972125Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1972326Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:41:11
2019-12-10T10:38:50.1972383Z    |
2019-12-10T10:38:50.1972383Z    |
2019-12-10T10:38:50.1972595Z LL |     let ( | A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1972637Z    |           ^ help: remove the `|`
2019-12-10T10:38:50.1973212Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1973413Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:42:11
2019-12-10T10:38:50.1973450Z    |
2019-12-10T10:38:50.1973450Z    |
2019-12-10T10:38:50.1973900Z LL |     let ( | A | B,) = (E::B,); //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1974011Z    |           ^ help: remove the `|`
2019-12-10T10:38:50.1974262Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1974464Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:43:11
2019-12-10T10:38:50.1974501Z    |
2019-12-10T10:38:50.1974501Z    |
2019-12-10T10:38:50.1974722Z LL |     let [ | A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1974763Z    |           ^ help: remove the `|`
2019-12-10T10:38:50.1974964Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1975171Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:44:13
2019-12-10T10:38:50.1997436Z    |
2019-12-10T10:38:50.1997436Z    |
2019-12-10T10:38:50.1997841Z LL |     let TS( | A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1997887Z    |             ^ help: remove the `|`
2019-12-10T10:38:50.1998116Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1998381Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:45:17
2019-12-10T10:38:50.1998421Z    |
2019-12-10T10:38:50.1998421Z    |
2019-12-10T10:38:50.1998653Z LL |     let NS { f: | A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1998695Z    |                 ^ help: remove the `|`
2019-12-10T10:38:50.1998888Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1999084Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:47:11
2019-12-10T10:38:50.1999121Z    |
2019-12-10T10:38:50.1999121Z    |
2019-12-10T10:38:50.1999313Z LL |     let ( || A | B) = E::A; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1999360Z    |           ^^ help: remove the `||`
2019-12-10T10:38:50.1999391Z    |
2019-12-10T10:38:50.1999570Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2019-12-10T10:38:50.1999772Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.1999970Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:48:11
2019-12-10T10:38:50.2000005Z    |
2019-12-10T10:38:50.2000005Z    |
2019-12-10T10:38:50.2000212Z LL |     let [ || A | B ] = [E::A]; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.2000253Z    |           ^^ help: remove the `||`
2019-12-10T10:38:50.2000283Z    |
2019-12-10T10:38:50.2000469Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2019-12-10T10:38:50.2000661Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.2000857Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:49:13
2019-12-10T10:38:50.2000893Z    |
2019-12-10T10:38:50.2000893Z    |
2019-12-10T10:38:50.2001084Z LL |     let TS( || A | B ); //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.2001138Z    |             ^^ help: remove the `||`
2019-12-10T10:38:50.2001169Z    |
2019-12-10T10:38:50.2001355Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2019-12-10T10:38:50.2001734Z error: a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.2001922Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:50:17
2019-12-10T10:38:50.2001957Z    |
2019-12-10T10:38:50.2001957Z    |
2019-12-10T10:38:50.2002153Z LL |     let NS { f: || A | B }; //~ ERROR a leading `|` is only allowed in a top-level pattern
2019-12-10T10:38:50.2002194Z    |                 ^^ help: remove the `||`
2019-12-10T10:38:50.2002225Z    |
2019-12-10T10:38:50.2002406Z    = note: alternatives in or-patterns are separated with `|`, not `||`
2019-12-10T10:38:50.2002463Z error: no rules expected the token `|`
2019-12-10T10:38:50.2002647Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:14:15
2019-12-10T10:38:50.2002687Z    |
2019-12-10T10:38:50.2002687Z    |
2019-12-10T10:38:50.2002718Z LL | macro_rules! accept_pat {
2019-12-10T10:38:50.2002885Z    | ----------------------- when calling this macro
2019-12-10T10:38:50.2003013Z ...
2019-12-10T10:38:50.2003055Z LL | accept_pat!(p | q); //~ ERROR no rules expected the token `|`
2019-12-10T10:38:50.2003092Z    |               ^ no rules expected this token in macro call
2019-12-10T10:38:50.2003161Z error: no rules expected the token `|`
2019-12-10T10:38:50.2003376Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:15:13
2019-12-10T10:38:50.2003412Z    |
2019-12-10T10:38:50.2003412Z    |
2019-12-10T10:38:50.2003459Z LL | macro_rules! accept_pat {
2019-12-10T10:38:50.2003626Z    | ----------------------- when calling this macro
2019-12-10T10:38:50.2003660Z ...
2019-12-10T10:38:50.2003695Z LL | accept_pat!(| p | q); //~ ERROR no rules expected the token `|`
2019-12-10T10:38:50.2003740Z    |             ^ no rules expected this token in macro call
2019-12-10T10:38:50.2003799Z warning: the feature `or_patterns` is incomplete and may cause the compiler to crash
2019-12-10T10:38:50.2004004Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:4:12
2019-12-10T10:38:50.2004046Z    |
2019-12-10T10:38:50.2004076Z LL | #![feature(or_patterns)]
2019-12-10T10:38:50.2004076Z LL | #![feature(or_patterns)]
2019-12-10T10:38:50.2004123Z    |            ^^^^^^^^^^^
2019-12-10T10:38:50.2004154Z    |
2019-12-10T10:38:50.2004186Z    = note: `#[warn(incomplete_features)]` on by default
2019-12-10T10:38:50.2004208Z 
2019-12-10T10:38:50.2004248Z error[E0369]: no implementation for `E | ()`
2019-12-10T10:38:50.2004473Z    |
2019-12-10T10:38:50.2004473Z    |
2019-12-10T10:38:50.2004517Z LL |     let _ = |A | B: E| (); //~ ERROR binary operation `|` cannot be applied to type `E`
2019-12-10T10:38:50.2004673Z    |                  ----^ -- ()
2019-12-10T10:38:50.2004748Z    |                  E
2019-12-10T10:38:50.2004778Z    |
2019-12-10T10:38:50.2004813Z    = note: an implementation of `std::ops::BitOr` might be missing for `E`
2019-12-10T10:38:50.2004842Z 
2019-12-10T10:38:50.2004842Z 
2019-12-10T10:38:50.2004883Z error[E0308]: mismatched types
2019-12-10T10:38:50.2005076Z   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:52:36
2019-12-10T10:38:50.2005111Z    |
2019-12-10T10:38:50.2005157Z LL |     let recovery_witness: String = 0; //~ ERROR mismatched types
2019-12-10T10:38:50.2005317Z    |                           ------   ^
2019-12-10T10:38:50.2005392Z    |                           |        expected struct `std::string::String`, found integer
2019-12-10T10:38:50.2005449Z    |                           |        help: try using a conversion method: `0.to_string()`
2019-12-10T10:38:50.2005487Z    |                           expected due to this
2019-12-10T10:38:50.2005509Z 
---
2019-12-10T10:38:50.2006131Z 
2019-12-10T10:38:50.2006301Z ---- [ui] ui/pattern/pattern-tyvar-2.rs stdout ----
2019-12-10T10:38:50.2006337Z diff of stderr:
2019-12-10T10:38:50.2006358Z 
2019-12-10T10:38:50.2006545Z - error[E0369]: binary operation `*` cannot be applied to type `std::vec::Vec<isize>`
2019-12-10T10:38:50.2006592Z + error[E0369]: cannot multiply `{integer}` to `std::vec::Vec<isize>`
2019-12-10T10:38:50.2006750Z 2   --> $DIR/pattern-tyvar-2.rs:3:71
2019-12-10T10:38:50.2006784Z 3    |
2019-12-10T10:38:50.2006997Z 4 LL | fn foo(t: Bar) -> isize { match t { Bar::T1(_, Some(x)) => { return x * 3; } _ => { panic!(); } } }
2019-12-10T10:38:50.2007044Z 
2019-12-10T10:38:50.2007076Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2007076Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2007312Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2/pattern-tyvar-2.stderr
2019-12-10T10:38:50.2007601Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2007799Z To only update this specific test, also pass `--test-args pattern/pattern-tyvar-2.rs`
2019-12-10T10:38:50.2007877Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2007909Z status: exit code: 1
2019-12-10T10:38:50.2007909Z status: exit code: 1
2019-12-10T10:38:50.2008482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-tyvar-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2008730Z ------------------------------------------
2019-12-10T10:38:50.2008756Z 
2019-12-10T10:38:50.2008913Z ------------------------------------------
2019-12-10T10:38:50.2008954Z stderr:
2019-12-10T10:38:50.2008954Z stderr:
2019-12-10T10:38:50.2009110Z ------------------------------------------
2019-12-10T10:38:50.2009148Z error[E0369]: cannot multiply `{integer}` to `std::vec::Vec<isize>`
2019-12-10T10:38:50.2009337Z   --> /checkout/src/test/ui/pattern/pattern-tyvar-2.rs:3:71
2019-12-10T10:38:50.2009375Z    |
2019-12-10T10:38:50.2009580Z LL | fn foo(t: Bar) -> isize { match t { Bar::T1(_, Some(x)) => { return x * 3; } _ => { panic!(); } } }
2019-12-10T10:38:50.2009789Z    |                                                                     - ^ - {integer}
2019-12-10T10:38:50.2009871Z    |                                                                     std::vec::Vec<isize>
2019-12-10T10:38:50.2009917Z    |
2019-12-10T10:38:50.2009917Z    |
2019-12-10T10:38:50.2009958Z    = note: an implementation of `std::ops::Mul` might be missing for `std::vec::Vec<isize>`
2019-12-10T10:38:50.2010013Z error: aborting due to previous error
2019-12-10T10:38:50.2010041Z 
2019-12-10T10:38:50.2010227Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.2010252Z 
---
2019-12-10T10:38:50.2010864Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2010902Z + error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2011054Z 2   --> $DIR/issue-39018.rs:2:22
2019-12-10T10:38:50.2011110Z 3    |
2019-12-10T10:38:50.2011148Z 4 LL |     let x = "Hello " + "World!";
2019-12-10T10:38:50.2011170Z 
2019-12-10T10:38:50.2011281Z 12 LL |     let x = "Hello ".to_owned() + "World!";
2019-12-10T10:38:50.2011355Z 14 
2019-12-10T10:38:50.2011355Z 14 
2019-12-10T10:38:50.2011565Z - error[E0369]: binary operation `+` cannot be applied to type `World`
2019-12-10T10:38:50.2011614Z + error[E0369]: cannot add `World` to `World`
2019-12-10T10:38:50.2011771Z 16   --> $DIR/issue-39018.rs:8:26
2019-12-10T10:38:50.2011805Z 17    |
2019-12-10T10:38:50.2011843Z 18 LL |     let y = World::Hello + World::Goodbye;
2019-12-10T10:38:50.2011893Z 22    |
2019-12-10T10:38:50.2011893Z 22    |
2019-12-10T10:38:50.2011928Z 23    = note: an implementation of `std::ops::Add` might be missing for `World`
2019-12-10T10:38:50.2012147Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2012187Z + error[E0369]: cannot add `std::string::String` to `&str`
2019-12-10T10:38:50.2012441Z 26   --> $DIR/issue-39018.rs:11:22
2019-12-10T10:38:50.2012483Z 27    |
2019-12-10T10:38:50.2012483Z 27    |
2019-12-10T10:38:50.2012515Z 28 LL |     let x = "Hello " + "World!".to_owned();
2019-12-10T10:38:50.2012549Z 
2019-12-10T10:38:50.2012582Z 36 LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
2019-12-10T10:38:50.2012650Z 38 
2019-12-10T10:38:50.2012861Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2012904Z + error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2013061Z 40   --> $DIR/issue-39018.rs:26:16
2019-12-10T10:38:50.2013061Z 40   --> $DIR/issue-39018.rs:26:16
2019-12-10T10:38:50.2013109Z 41    |
2019-12-10T10:38:50.2013140Z 42 LL |     let _ = &a + &b;
2019-12-10T10:38:50.2013191Z 50 LL |     let _ = a + &b;
2019-12-10T10:38:50.2013231Z 51    |             ^
2019-12-10T10:38:50.2013260Z 52 
2019-12-10T10:38:50.2013452Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2013452Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2013514Z + error[E0369]: cannot add `std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2013674Z 54   --> $DIR/issue-39018.rs:27:16
2019-12-10T10:38:50.2013706Z 55    |
2019-12-10T10:38:50.2013749Z 56 LL |     let _ = &a + b;
2019-12-10T10:38:50.2013804Z 73    |                 expected `&str`, found struct `std::string::String`
2019-12-10T10:38:50.2013841Z 74    |                 help: consider borrowing here: `&b`
2019-12-10T10:38:50.2013881Z 75 
2019-12-10T10:38:50.2014074Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2014074Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2014116Z + error[E0369]: cannot add `std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2014285Z 77   --> $DIR/issue-39018.rs:30:15
2019-12-10T10:38:50.2014319Z 78    |
2019-12-10T10:38:50.2014349Z 79 LL |     let _ = e + b;
2019-12-10T10:38:50.2014437Z 
2019-12-10T10:38:50.2014649Z 87 LL |     let _ = e.to_owned() + &b;
2019-12-10T10:38:50.2014729Z 89 
2019-12-10T10:38:50.2014955Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2015002Z + error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2015168Z 91   --> $DIR/issue-39018.rs:31:15
2019-12-10T10:38:50.2015168Z 91   --> $DIR/issue-39018.rs:31:15
2019-12-10T10:38:50.2015211Z 92    |
2019-12-10T10:38:50.2015243Z 93 LL |     let _ = e + &b;
2019-12-10T10:38:50.2015265Z 
2019-12-10T10:38:50.2015297Z 101 LL |     let _ = e.to_owned() + &b;
2019-12-10T10:38:50.2015370Z 103 
2019-12-10T10:38:50.2015574Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2015626Z + error[E0369]: cannot add `&str` to `&std::string::String`
2019-12-10T10:38:50.2016125Z 105   --> $DIR/issue-39018.rs:32:15
2019-12-10T10:38:50.2016125Z 105   --> $DIR/issue-39018.rs:32:15
2019-12-10T10:38:50.2016169Z 106    |
2019-12-10T10:38:50.2016223Z 107 LL |     let _ = e + d;
2019-12-10T10:38:50.2016368Z 115 LL |     let _ = e.to_owned() + d;
2019-12-10T10:38:50.2016412Z 116    |             ^^^^^^^^^^^^
2019-12-10T10:38:50.2016461Z 117 
2019-12-10T10:38:50.2016699Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2016699Z - error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T10:38:50.2016743Z + error[E0369]: cannot add `&&str` to `&std::string::String`
2019-12-10T10:38:50.2016927Z 119   --> $DIR/issue-39018.rs:33:15
2019-12-10T10:38:50.2016963Z 120    |
2019-12-10T10:38:50.2016995Z 121 LL |     let _ = e + &d;
2019-12-10T10:38:50.2017017Z 
2019-12-10T10:38:50.2017058Z 129 LL |     let _ = e.to_owned() + &d;
2019-12-10T10:38:50.2017123Z 131 
2019-12-10T10:38:50.2017324Z - error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-12-10T10:38:50.2017365Z + error[E0369]: cannot add `&&str` to `&&str`
2019-12-10T10:38:50.2017530Z 133   --> $DIR/issue-39018.rs:34:16
2019-12-10T10:38:50.2017530Z 133   --> $DIR/issue-39018.rs:34:16
2019-12-10T10:38:50.2017662Z 134    |
2019-12-10T10:38:50.2017701Z 135 LL |     let _ = &c + &d;
2019-12-10T10:38:50.2017754Z 139    |
2019-12-10T10:38:50.2017799Z 140    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-12-10T10:38:50.2017834Z 141 
2019-12-10T10:38:50.2018053Z - error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-12-10T10:38:50.2018053Z - error[E0369]: binary operation `+` cannot be applied to type `&&str`
2019-12-10T10:38:50.2018107Z + error[E0369]: cannot add `&str` to `&&str`
2019-12-10T10:38:50.2018274Z 143   --> $DIR/issue-39018.rs:35:16
2019-12-10T10:38:50.2018310Z 144    |
2019-12-10T10:38:50.2018342Z 145 LL |     let _ = &c + d;
2019-12-10T10:38:50.2018404Z 149    |
2019-12-10T10:38:50.2018442Z 150    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-12-10T10:38:50.2018485Z 151 
2019-12-10T10:38:50.2018677Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2018677Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2018718Z + error[E0369]: cannot add `&&str` to `&str`
2019-12-10T10:38:50.2018895Z 153   --> $DIR/issue-39018.rs:36:15
2019-12-10T10:38:50.2018942Z 154    |
2019-12-10T10:38:50.2018974Z 155 LL |     let _ = c + &d;
2019-12-10T10:38:50.2018997Z 
2019-12-10T10:38:50.2019046Z 163 LL |     let _ = c.to_owned() + &d;
2019-12-10T10:38:50.2019113Z 165 
2019-12-10T10:38:50.2019304Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2019361Z + error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2019526Z 167   --> $DIR/issue-39018.rs:37:15
2019-12-10T10:38:50.2019526Z 167   --> $DIR/issue-39018.rs:37:15
2019-12-10T10:38:50.2019562Z 168    |
2019-12-10T10:38:50.2019607Z 169 LL |     let _ = c + d;
2019-12-10T10:38:50.2019629Z 
2019-12-10T10:38:50.2019649Z 
2019-12-10T10:38:50.2019683Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2019921Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
2019-12-10T10:38:50.2020121Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2020325Z To only update this specific test, also pass `--test-args span/issue-39018.rs`
2019-12-10T10:38:50.2020393Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2020427Z status: exit code: 1
2019-12-10T10:38:50.2020427Z status: exit code: 1
2019-12-10T10:38:50.2021006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2021255Z ------------------------------------------
2019-12-10T10:38:50.2021289Z 
2019-12-10T10:38:50.2021527Z ------------------------------------------
2019-12-10T10:38:50.2021573Z stderr:
2019-12-10T10:38:50.2021573Z stderr:
2019-12-10T10:38:50.2021777Z ------------------------------------------
2019-12-10T10:38:50.2021817Z error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2021992Z   --> /checkout/src/test/ui/span/issue-39018.rs:2:22
2019-12-10T10:38:50.2022044Z    |
2019-12-10T10:38:50.2022078Z LL |     let x = "Hello " + "World!";
2019-12-10T10:38:50.2022244Z    |             -------- ^ -------- &str
2019-12-10T10:38:50.2022281Z    |             |        |
2019-12-10T10:38:50.2022331Z    |             |        `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2022406Z    |
2019-12-10T10:38:50.2022406Z    |
2019-12-10T10:38:50.2022468Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2022589Z    |
2019-12-10T10:38:50.2022647Z LL |     let x = "Hello ".to_owned() + "World!";
2019-12-10T10:38:50.2022707Z 
2019-12-10T10:38:50.2022707Z 
2019-12-10T10:38:50.2022740Z error[E0369]: cannot add `World` to `World`
2019-12-10T10:38:50.2022960Z   --> /checkout/src/test/ui/span/issue-39018.rs:8:26
2019-12-10T10:38:50.2022997Z    |
2019-12-10T10:38:50.2023030Z LL |     let y = World::Hello + World::Goodbye;
2019-12-10T10:38:50.2023416Z    |             ------------ ^ -------------- World
2019-12-10T10:38:50.2023501Z    |             World
2019-12-10T10:38:50.2023543Z    |
2019-12-10T10:38:50.2023543Z    |
2019-12-10T10:38:50.2023580Z    = note: an implementation of `std::ops::Add` might be missing for `World`
2019-12-10T10:38:50.2023639Z error[E0369]: cannot add `std::string::String` to `&str`
2019-12-10T10:38:50.2023876Z   --> /checkout/src/test/ui/span/issue-39018.rs:11:22
2019-12-10T10:38:50.2023924Z    |
2019-12-10T10:38:50.2023924Z    |
2019-12-10T10:38:50.2023964Z LL |     let x = "Hello " + "World!".to_owned();
2019-12-10T10:38:50.2024163Z    |             -------- ^ ------------------- std::string::String
2019-12-10T10:38:50.2024241Z    |             |        `+` cannot be used to concatenate a `&str` with a `String`
2019-12-10T10:38:50.2024292Z    |             &str
2019-12-10T10:38:50.2024323Z    |
2019-12-10T10:38:50.2024323Z    |
2019-12-10T10:38:50.2024375Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2024432Z    |
2019-12-10T10:38:50.2024467Z LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
2019-12-10T10:38:50.2024529Z 
2019-12-10T10:38:50.2024574Z error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2024775Z   --> /checkout/src/test/ui/span/issue-39018.rs:26:16
2019-12-10T10:38:50.2024813Z    |
2019-12-10T10:38:50.2024813Z    |
2019-12-10T10:38:50.2024854Z LL |     let _ = &a + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2025025Z    |             -- ^ -- &std::string::String
2019-12-10T10:38:50.2025062Z    |             |  |
2019-12-10T10:38:50.2025105Z    |             |  `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2025352Z    |
2019-12-10T10:38:50.2025352Z    |
2019-12-10T10:38:50.2025407Z help: String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2025449Z    |
2019-12-10T10:38:50.2025485Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2025558Z 
2019-12-10T10:38:50.2025596Z error[E0369]: cannot add `std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2025886Z   --> /checkout/src/test/ui/span/issue-39018.rs:27:16
2019-12-10T10:38:50.2025951Z    |
2019-12-10T10:38:50.2025951Z    |
2019-12-10T10:38:50.2025986Z LL |     let _ = &a + b; //~ ERROR binary operation
2019-12-10T10:38:50.2026192Z    |             -- ^ - std::string::String
2019-12-10T10:38:50.2026286Z    |             |  `+` cannot be used to concatenate a `&str` with a `String`
2019-12-10T10:38:50.2026324Z    |             &std::string::String
2019-12-10T10:38:50.2026357Z    |
2019-12-10T10:38:50.2026357Z    |
2019-12-10T10:38:50.2026419Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2026466Z    |
2019-12-10T10:38:50.2026500Z LL |     let _ = a + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2026637Z 
2019-12-10T10:38:50.2026671Z error[E0308]: mismatched types
2019-12-10T10:38:50.2026908Z   --> /checkout/src/test/ui/span/issue-39018.rs:29:17
2019-12-10T10:38:50.2026946Z    |
2019-12-10T10:38:50.2026946Z    |
2019-12-10T10:38:50.2026981Z LL |     let _ = a + b; //~ ERROR mismatched types
2019-12-10T10:38:50.2027062Z    |                 |
2019-12-10T10:38:50.2027100Z    |                 expected `&str`, found struct `std::string::String`
2019-12-10T10:38:50.2027141Z    |                 help: consider borrowing here: `&b`
2019-12-10T10:38:50.2027171Z 
2019-12-10T10:38:50.2027171Z 
2019-12-10T10:38:50.2027207Z error[E0369]: cannot add `std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2027398Z   --> /checkout/src/test/ui/span/issue-39018.rs:30:15
2019-12-10T10:38:50.2027440Z    |
2019-12-10T10:38:50.2027476Z LL |     let _ = e + b; //~ ERROR binary operation
2019-12-10T10:38:50.2027647Z    |             - ^ - std::string::String
2019-12-10T10:38:50.2027743Z    |             | `+` cannot be used to concatenate a `&str` with a `String`
2019-12-10T10:38:50.2027782Z    |             &std::string::String
2019-12-10T10:38:50.2027815Z    |
2019-12-10T10:38:50.2027815Z    |
2019-12-10T10:38:50.2027882Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2027929Z    |
2019-12-10T10:38:50.2027966Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2028220Z 
2019-12-10T10:38:50.2028416Z error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2019-12-10T10:38:50.2029048Z   --> /checkout/src/test/ui/span/issue-39018.rs:31:15
2019-12-10T10:38:50.2029087Z    |
2019-12-10T10:38:50.2029087Z    |
2019-12-10T10:38:50.2029122Z LL |     let _ = e + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2029303Z    |             - ^ -- &std::string::String
2019-12-10T10:38:50.2029354Z    |             | |
2019-12-10T10:38:50.2029393Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2029470Z    |
2019-12-10T10:38:50.2029470Z    |
2019-12-10T10:38:50.2029523Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2029568Z    |
2019-12-10T10:38:50.2029610Z LL |     let _ = e.to_owned() + &b; //~ ERROR binary operation
2019-12-10T10:38:50.2029670Z 
2019-12-10T10:38:50.2029714Z error[E0369]: cannot add `&str` to `&std::string::String`
2019-12-10T10:38:50.2029905Z   --> /checkout/src/test/ui/span/issue-39018.rs:32:15
2019-12-10T10:38:50.2029942Z    |
2019-12-10T10:38:50.2029942Z    |
2019-12-10T10:38:50.2029977Z LL |     let _ = e + d; //~ ERROR binary operation
2019-12-10T10:38:50.2030229Z    |             - ^ - &str
2019-12-10T10:38:50.2030276Z    |             | |
2019-12-10T10:38:50.2030314Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2030402Z    |
2019-12-10T10:38:50.2030402Z    |
2019-12-10T10:38:50.2030454Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2030515Z    |
2019-12-10T10:38:50.2030551Z LL |     let _ = e.to_owned() + d; //~ ERROR binary operation
2019-12-10T10:38:50.2030619Z 
2019-12-10T10:38:50.2030654Z error[E0369]: cannot add `&&str` to `&std::string::String`
2019-12-10T10:38:50.2030871Z   --> /checkout/src/test/ui/span/issue-39018.rs:33:15
2019-12-10T10:38:50.2030989Z    |
2019-12-10T10:38:50.2030989Z    |
2019-12-10T10:38:50.2031049Z LL |     let _ = e + &d; //~ ERROR binary operation
2019-12-10T10:38:50.2031240Z    |             - ^ -- &&str
2019-12-10T10:38:50.2031277Z    |             | |
2019-12-10T10:38:50.2031325Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2031396Z    |
2019-12-10T10:38:50.2031396Z    |
2019-12-10T10:38:50.2031461Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2031508Z    |
2019-12-10T10:38:50.2031544Z LL |     let _ = e.to_owned() + &d; //~ ERROR binary operation
2019-12-10T10:38:50.2031784Z 
2019-12-10T10:38:50.2031819Z error[E0369]: cannot add `&&str` to `&&str`
2019-12-10T10:38:50.2032172Z   --> /checkout/src/test/ui/span/issue-39018.rs:34:16
2019-12-10T10:38:50.2032219Z    |
2019-12-10T10:38:50.2032219Z    |
2019-12-10T10:38:50.2032256Z LL |     let _ = &c + &d; //~ ERROR binary operation
2019-12-10T10:38:50.2032410Z    |             -- ^ -- &&str
2019-12-10T10:38:50.2032481Z    |             &&str
2019-12-10T10:38:50.2032510Z    |
2019-12-10T10:38:50.2032553Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-12-10T10:38:50.2032577Z 
2019-12-10T10:38:50.2032577Z 
2019-12-10T10:38:50.2032608Z error[E0369]: cannot add `&str` to `&&str`
2019-12-10T10:38:50.2032779Z   --> /checkout/src/test/ui/span/issue-39018.rs:35:16
2019-12-10T10:38:50.2032828Z    |
2019-12-10T10:38:50.2032860Z LL |     let _ = &c + d; //~ ERROR binary operation
2019-12-10T10:38:50.2033008Z    |             -- ^ - &str
2019-12-10T10:38:50.2033086Z    |             &&str
2019-12-10T10:38:50.2033116Z    |
2019-12-10T10:38:50.2033150Z    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2019-12-10T10:38:50.2033190Z 
2019-12-10T10:38:50.2033190Z 
2019-12-10T10:38:50.2033226Z error[E0369]: cannot add `&&str` to `&str`
2019-12-10T10:38:50.2033402Z   --> /checkout/src/test/ui/span/issue-39018.rs:36:15
2019-12-10T10:38:50.2033443Z    |
2019-12-10T10:38:50.2033475Z LL |     let _ = c + &d; //~ ERROR binary operation
2019-12-10T10:38:50.2033626Z    |             - ^ -- &&str
2019-12-10T10:38:50.2033660Z    |             | |
2019-12-10T10:38:50.2033700Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2033764Z    |
2019-12-10T10:38:50.2033764Z    |
2019-12-10T10:38:50.2033820Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2033862Z    |
2019-12-10T10:38:50.2033895Z LL |     let _ = c.to_owned() + &d; //~ ERROR binary operation
2019-12-10T10:38:50.2033965Z 
2019-12-10T10:38:50.2034056Z error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2034279Z   --> /checkout/src/test/ui/span/issue-39018.rs:37:15
2019-12-10T10:38:50.2034313Z    |
2019-12-10T10:38:50.2034313Z    |
2019-12-10T10:38:50.2034346Z LL |     let _ = c + d; //~ ERROR binary operation
2019-12-10T10:38:50.2034492Z    |             - ^ - &str
2019-12-10T10:38:50.2034542Z    |             | |
2019-12-10T10:38:50.2034577Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2034647Z    |
2019-12-10T10:38:50.2034647Z    |
2019-12-10T10:38:50.2034696Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2034738Z    |
2019-12-10T10:38:50.2034780Z LL |     let _ = c.to_owned() + d; //~ ERROR binary operation
2019-12-10T10:38:50.2034907Z 
2019-12-10T10:38:50.2034938Z error: aborting due to 14 previous errors
2019-12-10T10:38:50.2034974Z 
2019-12-10T10:38:50.2035006Z Some errors have detailed explanations: E0308, E0369.
---
2019-12-10T10:38:50.2036157Z 4 LL |     let c = a + b;
2019-12-10T10:38:50.2036184Z 
2019-12-10T10:38:50.2036208Z 
2019-12-10T10:38:50.2036241Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2036479Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-concat-on-double-ref/str-concat-on-double-ref.stderr
2019-12-10T10:38:50.2036674Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2036866Z To only update this specific test, also pass `--test-args str/str-concat-on-double-ref.rs`
2019-12-10T10:38:50.2036934Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2036966Z status: exit code: 1
2019-12-10T10:38:50.2036966Z status: exit code: 1
2019-12-10T10:38:50.2037535Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-concat-on-double-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-concat-on-double-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-concat-on-double-ref/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2037782Z ------------------------------------------
2019-12-10T10:38:50.2037820Z 
2019-12-10T10:38:50.2037983Z ------------------------------------------
2019-12-10T10:38:50.2038017Z stderr:
2019-12-10T10:38:50.2038017Z stderr:
2019-12-10T10:38:50.2038179Z ------------------------------------------
2019-12-10T10:38:50.2038218Z error[E0369]: cannot add `&str` to `&std::string::String`
2019-12-10T10:38:50.2038395Z   --> /checkout/src/test/ui/str/str-concat-on-double-ref.rs:4:15
2019-12-10T10:38:50.2038437Z    |
2019-12-10T10:38:50.2038468Z LL |     let c = a + b;
2019-12-10T10:38:50.2038613Z    |             - ^ - &str
2019-12-10T10:38:50.2038647Z    |             | |
2019-12-10T10:38:50.2038695Z    |             | `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2038830Z    |
2019-12-10T10:38:50.2038830Z    |
2019-12-10T10:38:50.2038885Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2038929Z    |
2019-12-10T10:38:50.2038963Z LL |     let c = a.to_owned() + b;
2019-12-10T10:38:50.2039031Z 
2019-12-10T10:38:50.2039062Z error: aborting due to previous error
2019-12-10T10:38:50.2039084Z 
2019-12-10T10:38:50.2039307Z For more information about this error, try `rustc --explain E0369`.
---
2019-12-10T10:38:50.2040129Z - error[E0369]: binary operation `+` cannot be applied to type `&str`
2019-12-10T10:38:50.2040167Z + error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2040337Z 2   --> $DIR/non-1-width-unicode-multiline-label.rs:5:260
2019-12-10T10:38:50.2040386Z 3    |
2019-12-10T10:38:50.2040701Z 4 LL | ...ཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇...࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun + " really fun!";
2019-12-10T10:38:50.2040756Z 
2019-12-10T10:38:50.2040804Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2041079Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label/non-1-width-unicode-multiline-label.stderr
2019-12-10T10:38:50.2041079Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label/non-1-width-unicode-multiline-label.stderr
2019-12-10T10:38:50.2041277Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2041537Z To only update this specific test, also pass `--test-args terminal-width/non-1-width-unicode-multiline-label.rs`
2019-12-10T10:38:50.2041600Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2041641Z status: exit code: 1
2019-12-10T10:38:50.2041641Z status: exit code: 1
2019-12-10T10:38:50.2042277Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2042546Z ------------------------------------------
2019-12-10T10:38:50.2042573Z 
2019-12-10T10:38:50.2042760Z ------------------------------------------
2019-12-10T10:38:50.2042795Z stderr:
2019-12-10T10:38:50.2042795Z stderr:
2019-12-10T10:38:50.2042964Z ------------------------------------------
2019-12-10T10:38:50.2043002Z error[E0369]: cannot add `&str` to `&str`
2019-12-10T10:38:50.2043225Z   --> /checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs:5:260
2019-12-10T10:38:50.2043265Z    |
2019-12-10T10:38:50.2043561Z LL | ...ཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇...࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun + " really fun!";
2019-12-10T10:38:50.2043784Z    |                                                  -------------- ^ -------------- &str
2019-12-10T10:38:50.2043829Z    |                                                  |              |
2019-12-10T10:38:50.2043879Z    |                                                  |              `+` cannot be used to concatenate two `&str` strings
2019-12-10T10:38:50.2044039Z    |
2019-12-10T10:38:50.2044039Z    |
2019-12-10T10:38:50.2044097Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2019-12-10T10:38:50.2044144Z    |
2019-12-10T10:38:50.2044638Z LL |     let _ = "ༀ༁༂༃༄༅༆༇༈༉༊་༌།༎༏༐༑༒༓༔༕༖༗༘༙༚༛༜༝༞༟༠༡༢༣༤༥༦༧༨༩༪༫༬༭༮༯༰༱༲༳༴༵༶༷༸༹༺༻༼༽༾༿ཀཁགགྷངཅཆཇ཈ཉཊཋཌཌྷཎཏཐདདྷནཔཕབབྷམཙཚཛཛྷཝཞཟའཡརལཤཥསཧཨཀྵཪཫཬ཭཮཯཰ཱཱཱིིུུྲྀཷླྀཹེཻོཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇࿈࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun.to_owned() + " really fun!";
2019-12-10T10:38:50.2044826Z 
2019-12-10T10:38:50.2044878Z error: aborting due to previous error
2019-12-10T10:38:50.2044901Z 
2019-12-10T10:38:50.2045124Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.2045124Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.2045151Z 
2019-12-10T10:38:50.2045327Z ------------------------------------------
2019-12-10T10:38:50.2045352Z 
2019-12-10T10:38:50.2045372Z 
2019-12-10T10:38:50.2045556Z ---- [ui] ui/traits/trait-resolution-in-overloaded-op.rs stdout ----
2019-12-10T10:38:50.2045599Z diff of stderr:
2019-12-10T10:38:50.2045621Z 
2019-12-10T10:38:50.2045809Z - error[E0369]: binary operation `*` cannot be applied to type `&T`
2019-12-10T10:38:50.2045849Z + error[E0369]: cannot multiply `f64` to `&T`
2019-12-10T10:38:50.2046071Z 3    |
2019-12-10T10:38:50.2046103Z 4 LL |     a * b
2019-12-10T10:38:50.2046130Z 
2019-12-10T10:38:50.2046150Z 
2019-12-10T10:38:50.2046150Z 
2019-12-10T10:38:50.2046184Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2046460Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op/trait-resolution-in-overloaded-op.stderr
2019-12-10T10:38:50.2046664Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2046881Z To only update this specific test, also pass `--test-args traits/trait-resolution-in-overloaded-op.rs`
2019-12-10T10:38:50.2046947Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2046982Z status: exit code: 1
2019-12-10T10:38:50.2046982Z status: exit code: 1
2019-12-10T10:38:50.2047592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-resolution-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2047854Z ------------------------------------------
2019-12-10T10:38:50.2047887Z 
2019-12-10T10:38:50.2048062Z ------------------------------------------
2019-12-10T10:38:50.2048097Z stderr:
2019-12-10T10:38:50.2048097Z stderr:
2019-12-10T10:38:50.2048262Z ------------------------------------------
2019-12-10T10:38:50.2048306Z error[E0369]: cannot multiply `f64` to `&T`
2019-12-10T10:38:50.2048542Z    |
2019-12-10T10:38:50.2048542Z    |
2019-12-10T10:38:50.2048587Z LL |     a * b //~ ERROR binary operation `*` cannot be applied to type `&T`
2019-12-10T10:38:50.2048746Z    |     - ^ - f64
2019-12-10T10:38:50.2048819Z    |     &T
2019-12-10T10:38:50.2048849Z    |
2019-12-10T10:38:50.2048959Z    = note: an implementation of `std::ops::Mul` might be missing for `&T`
2019-12-10T10:38:50.2048991Z 
---
2019-12-10T10:38:50.2049951Z - error[E0369]: binary operation `+` cannot be applied to type `T`
2019-12-10T10:38:50.2049990Z + error[E0369]: cannot add `T` to `T`
2019-12-10T10:38:50.2050165Z 2   --> $DIR/missing_trait_impl.rs:5:15
2019-12-10T10:38:50.2050199Z 3    |
2019-12-10T10:38:50.2050231Z 4 LL |     let z = x + y;
2019-12-10T10:38:50.2050347Z 
2019-12-10T10:38:50.2050389Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2050662Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl/missing_trait_impl.stderr
2019-12-10T10:38:50.2050662Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl/missing_trait_impl.stderr
2019-12-10T10:38:50.2050869Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2051087Z To only update this specific test, also pass `--test-args type/type-check/missing_trait_impl.rs`
2019-12-10T10:38:50.2051155Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2051188Z status: exit code: 1
2019-12-10T10:38:50.2051188Z status: exit code: 1
2019-12-10T10:38:50.2051786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/missing_trait_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2052042Z ------------------------------------------
2019-12-10T10:38:50.2052074Z 
2019-12-10T10:38:50.2052246Z ------------------------------------------
2019-12-10T10:38:50.2052281Z stderr:
2019-12-10T10:38:50.2052281Z stderr:
2019-12-10T10:38:50.2052453Z ------------------------------------------
2019-12-10T10:38:50.2052491Z error[E0369]: cannot add `T` to `T`
2019-12-10T10:38:50.2052683Z   --> /checkout/src/test/ui/type/type-check/missing_trait_impl.rs:5:15
2019-12-10T10:38:50.2052722Z    |
2019-12-10T10:38:50.2052768Z LL |     let z = x + y; //~ ERROR binary operation `+` cannot be applied to type `T`
2019-12-10T10:38:50.2052928Z    |             - ^ - T
2019-12-10T10:38:50.2053002Z    |             T
2019-12-10T10:38:50.2053039Z    |
2019-12-10T10:38:50.2053078Z    = note: `T` might need a bound for `std::ops::Add`
2019-12-10T10:38:50.2053107Z 
2019-12-10T10:38:50.2053107Z 
2019-12-10T10:38:50.2053143Z error[E0368]: binary assignment operation `+=` cannot be applied to type `T`
2019-12-10T10:38:50.2053382Z    |
2019-12-10T10:38:50.2053382Z    |
2019-12-10T10:38:50.2053429Z LL |     x += x; //~ ERROR binary assignment operation `+=` cannot be applied to type `T`
2019-12-10T10:38:50.2053583Z    |     -^^^^^
2019-12-10T10:38:50.2053657Z    |     cannot use `+=` on type `T`
2019-12-10T10:38:50.2053689Z    |
2019-12-10T10:38:50.2053725Z    = note: `T` might need a bound for `std::ops::AddAssign`
2019-12-10T10:38:50.2053749Z 
---
2019-12-10T10:38:50.2054391Z 
2019-12-10T10:38:50.2054567Z ---- [ui] ui/vec/vec-res-add.rs stdout ----
2019-12-10T10:38:50.2054604Z diff of stderr:
2019-12-10T10:38:50.2054625Z 
2019-12-10T10:38:50.2054827Z - error[E0369]: binary operation `+` cannot be applied to type `std::vec::Vec<R>`
2019-12-10T10:38:50.2054870Z + error[E0369]: cannot add `std::vec::Vec<R>` to `std::vec::Vec<R>`
2019-12-10T10:38:50.2055076Z 3    |
2019-12-10T10:38:50.2055076Z 3    |
2019-12-10T10:38:50.2055117Z 4 LL |     let k = i + j;
2019-12-10T10:38:50.2055158Z 
2019-12-10T10:38:50.2055193Z The actual stderr differed from the expected stderr.
2019-12-10T10:38:50.2055426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/vec-res-add.stderr
2019-12-10T10:38:50.2055426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/vec-res-add.stderr
2019-12-10T10:38:50.2055910Z To update references, rerun the tests and pass the `--bless` flag
2019-12-10T10:38:50.2056195Z To only update this specific test, also pass `--test-args vec/vec-res-add.rs`
2019-12-10T10:38:50.2056259Z error: 1 errors occurred comparing output.
2019-12-10T10:38:50.2056293Z status: exit code: 1
2019-12-10T10:38:50.2056293Z status: exit code: 1
2019-12-10T10:38:50.2056929Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/vec/vec-res-add.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/auxiliary" "-A" "unused"
2019-12-10T10:38:50.2057180Z ------------------------------------------
2019-12-10T10:38:50.2057214Z 
2019-12-10T10:38:50.2057401Z ------------------------------------------
2019-12-10T10:38:50.2057437Z stderr:
2019-12-10T10:38:50.2057437Z stderr:
2019-12-10T10:38:50.2057608Z ------------------------------------------
2019-12-10T10:38:50.2057647Z error[E0369]: cannot add `std::vec::Vec<R>` to `std::vec::Vec<R>`
2019-12-10T10:38:50.2057879Z    |
2019-12-10T10:38:50.2057879Z    |
2019-12-10T10:38:50.2057911Z LL |     let k = i + j;
2019-12-10T10:38:50.2058087Z    |             - ^ - std::vec::Vec<R>
2019-12-10T10:38:50.2058159Z    |             std::vec::Vec<R>
2019-12-10T10:38:50.2058198Z    |
2019-12-10T10:38:50.2058198Z    |
2019-12-10T10:38:50.2058236Z    = note: an implementation of `std::ops::Add` might be missing for `std::vec::Vec<R>`
2019-12-10T10:38:50.2058302Z error: aborting due to previous error
2019-12-10T10:38:50.2058325Z 
2019-12-10T10:38:50.2058524Z For more information about this error, try `rustc --explain E0369`.
2019-12-10T10:38:50.2058558Z 
---
2019-12-10T10:38:50.2064889Z test result: FAILED. 9262 passed; 32 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-10T10:38:50.2064917Z 
2019-12-10T10:38:50.2064937Z 
2019-12-10T10:38:50.2064957Z 
2019-12-10T10:38:50.2066092Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-10T10:38:50.2066265Z 
2019-12-10T10:38:50.2066293Z 
2019-12-10T10:38:50.2066509Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-10T10:38:50.2066555Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T10:38:50.2066555Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T10:38:50.2066604Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-10T10:38:50.2066641Z Build completed unsuccessfully in 0:47:28
2019-12-10T10:38:50.2066676Z == clock drift check ==
2019-12-10T10:38:50.2066711Z   local time: Tue Dec 10 10:38:50 UTC 2019
2019-12-10T10:38:50.4595172Z   network time: Tue, 10 Dec 2019 10:38:50 GMT
2019-12-10T10:38:50.4595450Z == end clock drift check ==
2019-12-10T10:38:51.3756625Z 
2019-12-10T10:38:51.3831285Z ##[error]Bash exited with code '1'.
2019-12-10T10:38:51.3874644Z ##[section]Starting: Checkout
2019-12-10T10:38:51.3875972Z ==============================================================================
2019-12-10T10:38:51.3876025Z Task         : Get sources
2019-12-10T10:38:51.3876061Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

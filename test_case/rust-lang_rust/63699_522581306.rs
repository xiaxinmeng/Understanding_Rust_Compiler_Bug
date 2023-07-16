plain
2019-08-19T12:31:31.6987409Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T12:31:31.7175401Z ##[command]git config gc.auto 0
2019-08-19T12:31:32.5264787Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T12:31:32.5274425Z ##[command]git config --get-all http.proxy
2019-08-19T12:31:32.5281298Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63699/merge:refs/remotes/pull/63699/merge
---
2019-08-19T12:32:08.2450636Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T12:32:08.2450677Z 
2019-08-19T12:32:08.2450856Z   git checkout -b <new-branch-name>
2019-08-19T12:32:08.2450900Z 
2019-08-19T12:32:08.2450938Z HEAD is now at 7e67a1d5b Merge 9b5b6bf722d14ae5372e276ef2ee911e44f1b80a into cdff9189556bb7de2b9a8a72344c9d8ec6099fcd
2019-08-19T12:32:08.2607112Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T12:32:08.2609311Z ==============================================================================
2019-08-19T12:32:08.2609355Z Task         : Bash
2019-08-19T12:32:08.2609417Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T13:32:19.8443098Z .................................................................................................... 1500/8930
2019-08-19T13:32:25.0036000Z .................................................................................................... 1600/8930
2019-08-19T13:32:37.3375440Z ..................................i...............i................................................. 1700/8930
2019-08-19T13:32:44.6512809Z .................................................................................................... 1800/8930
2019-08-19T13:32:58.2028013Z ..........................iiiii..................................................................... 1900/8930
2019-08-19T13:33:08.2010258Z .................................................................................................... 2100/8930
2019-08-19T13:33:10.5885550Z .................................................................................................... 2200/8930
2019-08-19T13:33:15.1779994Z .................................................................................................... 2300/8930
2019-08-19T13:33:21.7151814Z .................................................................................................... 2400/8930
---
2019-08-19T13:36:06.1416926Z .................................................................................................... 4600/8930
2019-08-19T13:36:12.8478916Z ..........i...............i......................................................................... 4700/8930
2019-08-19T13:36:23.4536023Z .................................................................................................... 4800/8930
2019-08-19T13:36:29.1366628Z .................................................................................................... 4900/8930
2019-08-19T13:36:39.8580967Z ...........................................................................................ii.ii.... 5000/8930
2019-08-19T13:36:49.6965553Z .................................................................................................... 5200/8930
2019-08-19T13:36:59.3102028Z .................................................................................................... 5300/8930
2019-08-19T13:37:06.0484301Z ...............................................i.................................................... 5400/8930
2019-08-19T13:37:12.6189720Z .................................................................................................... 5500/8930
2019-08-19T13:37:12.6189720Z .................................................................................................... 5500/8930
2019-08-19T13:37:23.0519618Z .................................................................................................... 5600/8930
2019-08-19T13:37:31.5603724Z ........................................ii...i..ii...........i...................................... 5700/8930
2019-08-19T13:37:52.1426529Z .................................................................................................... 5900/8930
2019-08-19T13:37:56.9312601Z .................................................................................................... 6000/8930
2019-08-19T13:37:56.9312601Z .................................................................................................... 6000/8930
2019-08-19T13:38:09.6449472Z .........................................i..ii...................................................... 6100/8930
2019-08-19T13:38:29.4377154Z ...................................................................................i................ 6300/8930
2019-08-19T13:38:31.5533908Z .................................................................................................... 6400/8930
2019-08-19T13:38:33.5903885Z .......................................................i............................................ 6500/8930
2019-08-19T13:38:36.4191844Z .................................................................................................... 6600/8930
---
2019-08-19T13:42:25.1838669Z ---- [ui] ui/borrowck/borrowck-escaping-closure-error-1.rs stdout ----
2019-08-19T13:42:25.1838739Z diff of stderr:
2019-08-19T13:42:25.1838766Z 
2019-08-19T13:42:25.1838801Z 13    |     ^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1839031Z 14 help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1839090Z 15    |
2019-08-19T13:42:25.1839318Z - LL |     spawn(move || books.push(4));
2019-08-19T13:42:25.1839496Z -    |           ^^^^^^^
2019-08-19T13:42:25.1839536Z + LL |     spawn(|| books.push(4));
2019-08-19T13:42:25.1839614Z 18 
2019-08-19T13:42:25.1839667Z 19 error: aborting due to previous error
2019-08-19T13:42:25.1839700Z 20 
2019-08-19T13:42:25.1839722Z 
2019-08-19T13:42:25.1839722Z 
2019-08-19T13:42:25.1839742Z 
2019-08-19T13:42:25.1839797Z The actual stderr differed from the expected stderr.
2019-08-19T13:42:25.1840069Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1/borrowck-escaping-closure-error-1.stderr
2019-08-19T13:42:25.1840267Z To update references, rerun the tests and pass the `--bless` flag
2019-08-19T13:42:25.1840550Z To only update this specific test, also pass `--test-args borrowck/borrowck-escaping-closure-error-1.rs`
2019-08-19T13:42:25.1840626Z error: 1 errors occurred comparing output.
2019-08-19T13:42:25.1840684Z status: exit code: 1
2019-08-19T13:42:25.1840684Z status: exit code: 1
2019-08-19T13:42:25.1841314Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1/auxiliary" "-A" "unused"
2019-08-19T13:42:25.1841769Z ------------------------------------------
2019-08-19T13:42:25.1841799Z 
2019-08-19T13:42:25.1842016Z ------------------------------------------
2019-08-19T13:42:25.1842055Z stderr:
2019-08-19T13:42:25.1842055Z stderr:
2019-08-19T13:42:25.1842238Z ------------------------------------------
2019-08-19T13:42:25.1842665Z error[E0373]: closure may outlive the current function, but it borrows `books`, which is owned by the current function
2019-08-19T13:42:25.1842983Z   --> /checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-1.rs:13:11
2019-08-19T13:42:25.1843049Z    |
2019-08-19T13:42:25.1843113Z LL |     spawn(|| books.push(4));
2019-08-19T13:42:25.1843338Z    |           ^^ ----- `books` is borrowed here
2019-08-19T13:42:25.1843388Z    |           |
2019-08-19T13:42:25.1843453Z    |           may outlive borrowed value `books`
2019-08-19T13:42:25.1843726Z note: function requires argument type to outlive `'static`
2019-08-19T13:42:25.1843994Z   --> /checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-1.rs:13:5
2019-08-19T13:42:25.1844043Z    |
2019-08-19T13:42:25.1844043Z    |
2019-08-19T13:42:25.1844095Z LL |     spawn(|| books.push(4));
2019-08-19T13:42:25.1844142Z    |     ^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1844216Z help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1844266Z    |
2019-08-19T13:42:25.1844308Z LL |     spawn(|| books.push(4));
2019-08-19T13:42:25.1844400Z 
2019-08-19T13:42:25.1844451Z error: aborting due to previous error
2019-08-19T13:42:25.1844481Z 
2019-08-19T13:42:25.1844743Z For more information about this error, try `rustc --explain E0373`.
---
2019-08-19T13:42:25.1845298Z ---- [ui] ui/borrowck/borrowck-escaping-closure-error-2.rs stdout ----
2019-08-19T13:42:25.1845347Z diff of stderr:
2019-08-19T13:42:25.1845375Z 
2019-08-19T13:42:25.1845436Z 13    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1845603Z 14 help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1845665Z 15    |
2019-08-19T13:42:25.1846238Z - LL |     Box::new(move || books.push(4))
2019-08-19T13:42:25.1846397Z -    |              ^^^^^^^
2019-08-19T13:42:25.1846434Z + LL |     Box::new(|| books.push(4))
2019-08-19T13:42:25.1846525Z 18 
2019-08-19T13:42:25.1846560Z 19 error: aborting due to previous error
2019-08-19T13:42:25.1846592Z 20 
2019-08-19T13:42:25.1846629Z 
2019-08-19T13:42:25.1846629Z 
2019-08-19T13:42:25.1846649Z 
2019-08-19T13:42:25.1846685Z The actual stderr differed from the expected stderr.
2019-08-19T13:42:25.1846942Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-2/borrowck-escaping-closure-error-2.stderr
2019-08-19T13:42:25.1847151Z To update references, rerun the tests and pass the `--bless` flag
2019-08-19T13:42:25.1847392Z To only update this specific test, also pass `--test-args borrowck/borrowck-escaping-closure-error-2.rs`
2019-08-19T13:42:25.1847476Z error: 1 errors occurred comparing output.
2019-08-19T13:42:25.1847514Z status: exit code: 1
2019-08-19T13:42:25.1847514Z status: exit code: 1
2019-08-19T13:42:25.1848122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-2/auxiliary" "-A" "unused"
2019-08-19T13:42:25.1848506Z ------------------------------------------
2019-08-19T13:42:25.1848561Z 
2019-08-19T13:42:25.1848743Z ------------------------------------------
2019-08-19T13:42:25.1848781Z stderr:
2019-08-19T13:42:25.1848781Z stderr:
2019-08-19T13:42:25.1848975Z ------------------------------------------
2019-08-19T13:42:25.1849021Z error[E0373]: closure may outlive the current function, but it borrows `books`, which is owned by the current function
2019-08-19T13:42:25.1849236Z   --> /checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-2.rs:11:14
2019-08-19T13:42:25.1849303Z    |
2019-08-19T13:42:25.1849338Z LL |     Box::new(|| books.push(4))
2019-08-19T13:42:25.1849525Z    |              ^^ ----- `books` is borrowed here
2019-08-19T13:42:25.1849582Z    |              |
2019-08-19T13:42:25.1849620Z    |              may outlive borrowed value `books`
2019-08-19T13:42:25.1849690Z note: closure is returned here
2019-08-19T13:42:25.1849913Z   --> /checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-2.rs:11:5
2019-08-19T13:42:25.1849953Z    |
2019-08-19T13:42:25.1849953Z    |
2019-08-19T13:42:25.1849994Z LL |     Box::new(|| books.push(4))
2019-08-19T13:42:25.1850048Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1850094Z help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1850134Z    |
2019-08-19T13:42:25.1850187Z LL |     Box::new(|| books.push(4))
2019-08-19T13:42:25.1850251Z 
2019-08-19T13:42:25.1850286Z error: aborting due to previous error
2019-08-19T13:42:25.1850326Z 
2019-08-19T13:42:25.1850530Z For more information about this error, try `rustc --explain E0373`.
---
2019-08-19T13:42:25.1851208Z ---- [ui] ui/issues/issue-4335.rs stdout ----
2019-08-19T13:42:25.1851247Z diff of stderr:
2019-08-19T13:42:25.1851286Z 
2019-08-19T13:42:25.1851321Z 21    |     ^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1851438Z 22 help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1851486Z 23    |
2019-08-19T13:42:25.1851703Z - LL |     id(Box::new(move || *v))
2019-08-19T13:42:25.1851870Z -    |                 ^^^^^^^
2019-08-19T13:42:25.1851907Z + LL |     id(Box::new(|| *v))
2019-08-19T13:42:25.1852000Z 26 
2019-08-19T13:42:25.1852036Z 27 error: aborting due to 2 previous errors
2019-08-19T13:42:25.1852085Z 28 
2019-08-19T13:42:25.1852107Z 
2019-08-19T13:42:25.1852107Z 
2019-08-19T13:42:25.1852128Z 
2019-08-19T13:42:25.1852163Z The actual stderr differed from the expected stderr.
2019-08-19T13:42:25.1852841Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/issue-4335.stderr
2019-08-19T13:42:25.1853658Z To update references, rerun the tests and pass the `--bless` flag
2019-08-19T13:42:25.1853953Z To only update this specific test, also pass `--test-args issues/issue-4335.rs`
2019-08-19T13:42:25.1854057Z error: 1 errors occurred comparing output.
2019-08-19T13:42:25.1854103Z status: exit code: 1
2019-08-19T13:42:25.1854103Z status: exit code: 1
2019-08-19T13:42:25.1854831Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-4335.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/auxiliary" "-A" "unused"
2019-08-19T13:42:25.1855327Z ------------------------------------------
2019-08-19T13:42:25.1855383Z 
2019-08-19T13:42:25.1855616Z ------------------------------------------
2019-08-19T13:42:25.1855664Z stderr:
2019-08-19T13:42:25.1855664Z stderr:
2019-08-19T13:42:25.1855917Z ------------------------------------------
2019-08-19T13:42:25.1856142Z error[E0507]: cannot move out of `*v`, as `v` is a captured variable in an `FnMut` closure
2019-08-19T13:42:25.1856331Z   --> /checkout/src/test/ui/issues/issue-4335.rs:6:20
2019-08-19T13:42:25.1856388Z    |
2019-08-19T13:42:25.1856564Z LL | fn f<'r, T>(v: &'r T) -> Box<dyn FnMut() -> T + 'r> {
2019-08-19T13:42:25.1856731Z    |             - captured outer variable
2019-08-19T13:42:25.1856775Z LL |     id(Box::new(|| *v))
2019-08-19T13:42:25.1856834Z    |                    ^^ move occurs because `*v` has type `T`, which does not implement the `Copy` trait
2019-08-19T13:42:25.1856861Z 
2019-08-19T13:42:25.1856902Z error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
2019-08-19T13:42:25.1857104Z   --> /checkout/src/test/ui/issues/issue-4335.rs:6:17
2019-08-19T13:42:25.1857141Z    |
2019-08-19T13:42:25.1857173Z LL |     id(Box::new(|| *v))
2019-08-19T13:42:25.1857365Z    |                 ^^  - `v` is borrowed here
2019-08-19T13:42:25.1857403Z    |                 |
2019-08-19T13:42:25.1857439Z    |                 may outlive borrowed value `v`
2019-08-19T13:42:25.1857522Z note: closure is returned here
2019-08-19T13:42:25.1857698Z   --> /checkout/src/test/ui/issues/issue-4335.rs:6:5
2019-08-19T13:42:25.1857734Z    |
2019-08-19T13:42:25.1857734Z    |
2019-08-19T13:42:25.1857783Z LL |     id(Box::new(|| *v))
2019-08-19T13:42:25.1857824Z    |     ^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1857866Z help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1857920Z    |
2019-08-19T13:42:25.1857952Z LL |     id(Box::new(|| *v))
2019-08-19T13:42:25.1858007Z 
2019-08-19T13:42:25.1858057Z error: aborting due to 2 previous errors
2019-08-19T13:42:25.1858080Z 
2019-08-19T13:42:25.1858116Z Some errors have detailed explanations: E0373, E0507.
---
2019-08-19T13:42:25.1858872Z ---- [ui] ui/regions/region-borrow-params-issue-29793-big.rs stdout ----
2019-08-19T13:42:25.1858910Z diff of stderr:
2019-08-19T13:42:25.1858932Z 
2019-08-19T13:42:25.1858975Z 13    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1859037Z 14 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1859077Z 15    |
2019-08-19T13:42:25.1859312Z - LL |         WrapB::new().set(move |t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1859485Z -    |                          ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1859529Z + LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1859627Z 18 
2019-08-19T13:42:25.1859627Z 18 
2019-08-19T13:42:25.1859668Z 19 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1859901Z 
2019-08-19T13:42:25.1859937Z 31    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1859937Z 31    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1860055Z 32 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1860112Z 33    |
2019-08-19T13:42:25.1860347Z - LL |         WrapB::new().set(move |t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1860519Z -    |                          ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1860580Z + LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1860659Z 36 
2019-08-19T13:42:25.1860712Z 37 error: aborting due to 2 previous errors
2019-08-19T13:42:25.1860745Z 38 
2019-08-19T13:42:25.1860765Z 
2019-08-19T13:42:25.1860765Z 
2019-08-19T13:42:25.1860784Z 
2019-08-19T13:42:25.1860837Z The actual stderr differed from the expected stderr.
2019-08-19T13:42:25.1861101Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big/region-borrow-params-issue-29793-big.stderr
2019-08-19T13:42:25.1861303Z To update references, rerun the tests and pass the `--bless` flag
2019-08-19T13:42:25.1861539Z To only update this specific test, also pass `--test-args regions/region-borrow-params-issue-29793-big.rs`
2019-08-19T13:42:25.1861601Z error: 1 errors occurred comparing output.
2019-08-19T13:42:25.1861653Z status: exit code: 1
2019-08-19T13:42:25.1861653Z status: exit code: 1
2019-08-19T13:42:25.1862258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-borrow-params-issue-29793-big.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-big/auxiliary" "-A" "unused"
2019-08-19T13:42:25.1862936Z ------------------------------------------
2019-08-19T13:42:25.1862972Z 
2019-08-19T13:42:25.1863205Z ------------------------------------------
2019-08-19T13:42:25.1863250Z stderr:
2019-08-19T13:42:25.1863250Z stderr:
2019-08-19T13:42:25.1866762Z ------------------------------------------
2019-08-19T13:42:25.1866834Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1869634Z    |
2019-08-19T13:42:25.1869634Z    |
2019-08-19T13:42:25.1869701Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1869962Z    |                          ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1870007Z    |                          |
2019-08-19T13:42:25.1870050Z    |                          may outlive borrowed value `x`
2019-08-19T13:42:25.1870150Z note: closure is returned here
2019-08-19T13:42:25.1870367Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-big.rs:67:9
2019-08-19T13:42:25.1871805Z    |
2019-08-19T13:42:25.1871805Z    |
2019-08-19T13:42:25.1873334Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1873415Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1873512Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1873563Z    |
2019-08-19T13:42:25.1873613Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1873713Z 
2019-08-19T13:42:25.1873713Z 
2019-08-19T13:42:25.1873766Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1874394Z    |
2019-08-19T13:42:25.1874394Z    |
2019-08-19T13:42:25.1874444Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1874721Z    |                          ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1874774Z    |                          |
2019-08-19T13:42:25.1874822Z    |                          may outlive borrowed value `y`
2019-08-19T13:42:25.1874940Z note: closure is returned here
2019-08-19T13:42:25.1875195Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-big.rs:67:9
2019-08-19T13:42:25.1875243Z    |
2019-08-19T13:42:25.1875243Z    |
2019-08-19T13:42:25.1875312Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1875365Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T13:42:25.1875430Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1875497Z    |
2019-08-19T13:42:25.1875546Z LL |         WrapB::new().set(|t: bool| if t { x } else { y }) // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1875644Z 
2019-08-19T13:42:25.1875687Z error: aborting due to 2 previous errors
2019-08-19T13:42:25.1875715Z 
2019-08-19T13:42:25.1875968Z For more information about this error, try `rustc --explain E0373`.
---
2019-08-19T13:42:25.1876651Z ---- [ui] ui/regions/region-borrow-params-issue-29793-small.rs stdout ----
2019-08-19T13:42:25.1876689Z diff of stderr:
2019-08-19T13:42:25.1876711Z 
2019-08-19T13:42:25.1876743Z 13    |                ^
2019-08-19T13:42:25.1876808Z 14 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1876847Z 15    |
2019-08-19T13:42:25.1877052Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1877236Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1877279Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1877367Z 18 
2019-08-19T13:42:25.1877367Z 18 
2019-08-19T13:42:25.1877485Z 19 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1877741Z 
2019-08-19T13:42:25.1877791Z 31    |                ^
2019-08-19T13:42:25.1877791Z 31    |                ^
2019-08-19T13:42:25.1877832Z 32 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1877877Z 33    |
2019-08-19T13:42:25.1878099Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1878266Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1878310Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1878397Z 36 
2019-08-19T13:42:25.1878397Z 36 
2019-08-19T13:42:25.1878437Z 37 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1878676Z 
2019-08-19T13:42:25.1878708Z 49    |         ^
2019-08-19T13:42:25.1878708Z 49    |         ^
2019-08-19T13:42:25.1878750Z 50 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1878803Z 51    |
2019-08-19T13:42:25.1879008Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1879272Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1879333Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1879401Z 54 
2019-08-19T13:42:25.1879401Z 54 
2019-08-19T13:42:25.1879457Z 55 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1879680Z 
2019-08-19T13:42:25.1879712Z 67    |         ^
2019-08-19T13:42:25.1879712Z 67    |         ^
2019-08-19T13:42:25.1879769Z 68 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1879805Z 69    |
2019-08-19T13:42:25.1880011Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1880195Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1880245Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1880329Z 72 
2019-08-19T13:42:25.1880329Z 72 
2019-08-19T13:42:25.1880369Z 73 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1906629Z 
2019-08-19T13:42:25.1906704Z 85    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1906704Z 85    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1906767Z 86 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1906823Z 87    |
2019-08-19T13:42:25.1907222Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1907416Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1907480Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1907566Z 90 
2019-08-19T13:42:25.1907566Z 90 
2019-08-19T13:42:25.1907611Z 91 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1907871Z 
2019-08-19T13:42:25.1907906Z 103    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1907906Z 103    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1907966Z 104 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1908299Z 105    |
2019-08-19T13:42:25.1908727Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1908909Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1908953Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1909047Z 108 
2019-08-19T13:42:25.1909047Z 108 
2019-08-19T13:42:25.1909088Z 109 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1909309Z 
2019-08-19T13:42:25.1909351Z 121    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1909351Z 121    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1909394Z 122 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1909432Z 123    |
2019-08-19T13:42:25.1909646Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1909813Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1909856Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1909936Z 126 
2019-08-19T13:42:25.1909936Z 126 
2019-08-19T13:42:25.1909977Z 127 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1910319Z 
2019-08-19T13:42:25.1910352Z 139    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1910352Z 139    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1910394Z 140 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1910445Z 141    |
2019-08-19T13:42:25.1910648Z - LL |         let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1910820Z -    |                 ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1910880Z + LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1910950Z 144 
2019-08-19T13:42:25.1910950Z 144 
2019-08-19T13:42:25.1911005Z 145 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1911229Z 
2019-08-19T13:42:25.1911273Z 157    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1911273Z 157    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1911316Z 158 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1911353Z 159    |
2019-08-19T13:42:25.1911567Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1911738Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1911788Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1911864Z 162 
2019-08-19T13:42:25.1911864Z 162 
2019-08-19T13:42:25.1911905Z 163 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1912132Z 
2019-08-19T13:42:25.1912165Z 175    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1912165Z 175    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1912207Z 176 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1912604Z 177    |
2019-08-19T13:42:25.1912922Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1913141Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1913214Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1913418Z 180 
2019-08-19T13:42:25.1913418Z 180 
2019-08-19T13:42:25.1913488Z 181 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1913808Z 
2019-08-19T13:42:25.1913850Z 193    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1913850Z 193    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1913928Z 194 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1913976Z 195    |
2019-08-19T13:42:25.1914246Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1914474Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1914529Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1914646Z 198 
2019-08-19T13:42:25.1914646Z 198 
2019-08-19T13:42:25.1914699Z 199 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1914992Z 
2019-08-19T13:42:25.1915035Z 211    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1915035Z 211    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1915089Z 212 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1915227Z 213    |
2019-08-19T13:42:25.1915537Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1915756Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1915810Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1915909Z 216 
2019-08-19T13:42:25.1915909Z 216 
2019-08-19T13:42:25.1915969Z 217 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1916555Z 
2019-08-19T13:42:25.1916590Z 229    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1916590Z 229    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1916640Z 230 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1916686Z 231    |
2019-08-19T13:42:25.1917079Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1917278Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1917324Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1917416Z 234 
2019-08-19T13:42:25.1917416Z 234 
2019-08-19T13:42:25.1917467Z 235 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1917704Z 
2019-08-19T13:42:25.1917756Z 247    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1917756Z 247    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1917802Z 248 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1917842Z 249    |
2019-08-19T13:42:25.1918074Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1918266Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1918313Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1918398Z 252 
2019-08-19T13:42:25.1918398Z 252 
2019-08-19T13:42:25.1918442Z 253 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1918769Z 
2019-08-19T13:42:25.1918806Z 265    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1918806Z 265    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1918852Z 266 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1918900Z 267    |
2019-08-19T13:42:25.1919156Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1919354Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1919412Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1919488Z 270 
2019-08-19T13:42:25.1919488Z 270 
2019-08-19T13:42:25.1919547Z 271 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1919786Z 
2019-08-19T13:42:25.1919845Z 283    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1919845Z 283    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1919891Z 284 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1919931Z 285    |
2019-08-19T13:42:25.1920157Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1920351Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1920474Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1920559Z 288 
2019-08-19T13:42:25.1920559Z 288 
2019-08-19T13:42:25.1920603Z 289 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1925677Z 
2019-08-19T13:42:25.1925727Z 301    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1925727Z 301    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1925799Z 302 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1925861Z 303    |
2019-08-19T13:42:25.1926150Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1926525Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1926738Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1926828Z 306 
2019-08-19T13:42:25.1926828Z 306 
2019-08-19T13:42:25.1926880Z 307 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1927285Z 
2019-08-19T13:42:25.1927321Z 319    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1927321Z 319    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1927381Z 320 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1927422Z 321    |
2019-08-19T13:42:25.1927646Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1927841Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1927887Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1927989Z 324 
2019-08-19T13:42:25.1927989Z 324 
2019-08-19T13:42:25.1928033Z 325 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1929719Z 
2019-08-19T13:42:25.1929755Z 337    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1929755Z 337    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1929798Z 338 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1929836Z 339    |
2019-08-19T13:42:25.1930186Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1930413Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1930457Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1930534Z 342 
2019-08-19T13:42:25.1930534Z 342 
2019-08-19T13:42:25.1930581Z 343 error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1930817Z 
2019-08-19T13:42:25.1930850Z 355    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1930850Z 355    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1930908Z 356 help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1930945Z 357    |
2019-08-19T13:42:25.1931159Z - LL |             let f = move |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1931335Z -    |                     ^^^^^^^^^^^^^^
2019-08-19T13:42:25.1931397Z + LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1931468Z 360 
2019-08-19T13:42:25.1931502Z 361 error: aborting due to 20 previous errors
2019-08-19T13:42:25.1931624Z 362 
2019-08-19T13:42:25.1931645Z 
2019-08-19T13:42:25.1931645Z 
2019-08-19T13:42:25.1931665Z 
2019-08-19T13:42:25.1931877Z The actual stderr differed from the expected stderr.
2019-08-19T13:42:25.1932198Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/region-borrow-params-issue-29793-small.stderr
2019-08-19T13:42:25.1932621Z To update references, rerun the tests and pass the `--bless` flag
2019-08-19T13:42:25.1933195Z To only update this specific test, also pass `--test-args regions/region-borrow-params-issue-29793-small.rs`
2019-08-19T13:42:25.1933304Z error: 1 errors occurred comparing output.
2019-08-19T13:42:25.1933349Z status: exit code: 1
2019-08-19T13:42:25.1933349Z status: exit code: 1
2019-08-19T13:42:25.1934201Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/auxiliary" "-A" "unused"
2019-08-19T13:42:25.1934534Z ------------------------------------------
2019-08-19T13:42:25.1934568Z 
2019-08-19T13:42:25.1934780Z ------------------------------------------
2019-08-19T13:42:25.1934842Z stderr:
2019-08-19T13:42:25.1934842Z stderr:
2019-08-19T13:42:25.1935052Z ------------------------------------------
2019-08-19T13:42:25.1935111Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1935431Z    |
2019-08-19T13:42:25.1935431Z    |
2019-08-19T13:42:25.1935489Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1935735Z    |                 ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1935785Z    |                 |
2019-08-19T13:42:25.1935831Z    |                 may outlive borrowed value `x`
2019-08-19T13:42:25.1935923Z note: closure is returned here
2019-08-19T13:42:25.1936311Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:12:16
2019-08-19T13:42:25.1936364Z    |
2019-08-19T13:42:25.1936492Z LL |         return f;
2019-08-19T13:42:25.1936492Z LL |         return f;
2019-08-19T13:42:25.1936534Z    |                ^
2019-08-19T13:42:25.1936578Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1936635Z    |
2019-08-19T13:42:25.1936673Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1936756Z 
2019-08-19T13:42:25.1936756Z 
2019-08-19T13:42:25.1937929Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1938329Z    |
2019-08-19T13:42:25.1938329Z    |
2019-08-19T13:42:25.1938367Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1938705Z    |                 ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1938759Z    |                 |
2019-08-19T13:42:25.1938810Z    |                 may outlive borrowed value `y`
2019-08-19T13:42:25.1938879Z note: closure is returned here
2019-08-19T13:42:25.1939127Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:12:16
2019-08-19T13:42:25.1939167Z    |
2019-08-19T13:42:25.1939199Z LL |         return f;
2019-08-19T13:42:25.1939199Z LL |         return f;
2019-08-19T13:42:25.1939242Z    |                ^
2019-08-19T13:42:25.1939410Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1939450Z    |
2019-08-19T13:42:25.1939505Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1940622Z 
2019-08-19T13:42:25.1940622Z 
2019-08-19T13:42:25.1940663Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1941232Z    |
2019-08-19T13:42:25.1941232Z    |
2019-08-19T13:42:25.1941270Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1941515Z    |                 ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1941554Z    |                 |
2019-08-19T13:42:25.1941590Z    |                 may outlive borrowed value `x`
2019-08-19T13:42:25.1941669Z note: closure is returned here
2019-08-19T13:42:25.1941870Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:27:9
2019-08-19T13:42:25.1941946Z    |
2019-08-19T13:42:25.1941981Z LL |         f
2019-08-19T13:42:25.1941981Z LL |         f
2019-08-19T13:42:25.1942013Z    |         ^
2019-08-19T13:42:25.1942065Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1942282Z    |
2019-08-19T13:42:25.1942330Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1942426Z 
2019-08-19T13:42:25.1942426Z 
2019-08-19T13:42:25.1942478Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1942814Z    |
2019-08-19T13:42:25.1942814Z    |
2019-08-19T13:42:25.1946774Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1947163Z    |                 ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1947396Z    |                 |
2019-08-19T13:42:25.1947436Z    |                 may outlive borrowed value `y`
2019-08-19T13:42:25.1947520Z note: closure is returned here
2019-08-19T13:42:25.1947742Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:27:9
2019-08-19T13:42:25.1947782Z    |
2019-08-19T13:42:25.1947815Z LL |         f
2019-08-19T13:42:25.1947815Z LL |         f
2019-08-19T13:42:25.1947995Z    |         ^
2019-08-19T13:42:25.1948054Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1948095Z    |
2019-08-19T13:42:25.1948151Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1948216Z 
2019-08-19T13:42:25.1948216Z 
2019-08-19T13:42:25.1948267Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1948731Z    |
2019-08-19T13:42:25.1948731Z    |
2019-08-19T13:42:25.1948770Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1948966Z    |                 ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1949005Z    |                 |
2019-08-19T13:42:25.1949048Z    |                 may outlive borrowed value `x`
2019-08-19T13:42:25.1949124Z note: closure is returned here
2019-08-19T13:42:25.1949329Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:58:16
2019-08-19T13:42:25.1949376Z    |
2019-08-19T13:42:25.1949409Z LL |         return Box::new(f);
2019-08-19T13:42:25.1949409Z LL |         return Box::new(f);
2019-08-19T13:42:25.1949445Z    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1949502Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1949622Z    |
2019-08-19T13:42:25.1949661Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1949738Z 
2019-08-19T13:42:25.1949738Z 
2019-08-19T13:42:25.1949780Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1950080Z    |
2019-08-19T13:42:25.1950080Z    |
2019-08-19T13:42:25.1950291Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1950493Z    |                 ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1950545Z    |                 |
2019-08-19T13:42:25.1950582Z    |                 may outlive borrowed value `y`
2019-08-19T13:42:25.1950671Z note: closure is returned here
2019-08-19T13:42:25.1951043Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:58:16
2019-08-19T13:42:25.1951088Z    |
2019-08-19T13:42:25.1951132Z LL |         return Box::new(f);
2019-08-19T13:42:25.1951132Z LL |         return Box::new(f);
2019-08-19T13:42:25.1951168Z    |                ^^^^^^^^^^^
2019-08-19T13:42:25.1951212Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1951251Z    |
2019-08-19T13:42:25.1951306Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1951368Z 
2019-08-19T13:42:25.1951368Z 
2019-08-19T13:42:25.1951416Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1951704Z    |
2019-08-19T13:42:25.1951704Z    |
2019-08-19T13:42:25.1951936Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1952137Z    |                 ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1952178Z    |                 |
2019-08-19T13:42:25.1952253Z    |                 may outlive borrowed value `x`
2019-08-19T13:42:25.1952329Z note: closure is returned here
2019-08-19T13:42:25.1952756Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:69:9
2019-08-19T13:42:25.1952822Z    |
2019-08-19T13:42:25.1952963Z LL |         Box::new(f)
2019-08-19T13:42:25.1952963Z LL |         Box::new(f)
2019-08-19T13:42:25.1953016Z    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1953083Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1953131Z    |
2019-08-19T13:42:25.1953179Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1953277Z 
2019-08-19T13:42:25.1953277Z 
2019-08-19T13:42:25.1953328Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1953954Z    |
2019-08-19T13:42:25.1953954Z    |
2019-08-19T13:42:25.1954002Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1954250Z    |                 ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1954317Z    |                 |
2019-08-19T13:42:25.1954362Z    |                 may outlive borrowed value `y`
2019-08-19T13:42:25.1954454Z note: closure is returned here
2019-08-19T13:42:25.1954713Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:69:9
2019-08-19T13:42:25.1954761Z    |
2019-08-19T13:42:25.1954814Z LL |         Box::new(f)
2019-08-19T13:42:25.1954814Z LL |         Box::new(f)
2019-08-19T13:42:25.1954857Z    |         ^^^^^^^^^^^
2019-08-19T13:42:25.1955010Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1955074Z    |
2019-08-19T13:42:25.1955124Z LL |         let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1955203Z 
2019-08-19T13:42:25.1955203Z 
2019-08-19T13:42:25.1955270Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1955635Z    |
2019-08-19T13:42:25.1955635Z    |
2019-08-19T13:42:25.1955693Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1956116Z    |                     ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1956157Z    |                     |
2019-08-19T13:42:25.1956206Z    |                     may outlive borrowed value `x`
2019-08-19T13:42:25.1956441Z note: closure is returned here
2019-08-19T13:42:25.1956660Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:93:20
2019-08-19T13:42:25.1956699Z    |
2019-08-19T13:42:25.1956733Z LL |             return Box::new(f);
2019-08-19T13:42:25.1956733Z LL |             return Box::new(f);
2019-08-19T13:42:25.1956770Z    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1956823Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1956860Z    |
2019-08-19T13:42:25.1956905Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1956977Z 
2019-08-19T13:42:25.1956977Z 
2019-08-19T13:42:25.1959302Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1959719Z    |
2019-08-19T13:42:25.1959719Z    |
2019-08-19T13:42:25.1959757Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1959963Z    |                     ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1960002Z    |                     |
2019-08-19T13:42:25.1960038Z    |                     may outlive borrowed value `y`
2019-08-19T13:42:25.1960113Z note: closure is returned here
2019-08-19T13:42:25.1960405Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:93:20
2019-08-19T13:42:25.1960453Z    |
2019-08-19T13:42:25.1960501Z LL |             return Box::new(f);
2019-08-19T13:42:25.1960501Z LL |             return Box::new(f);
2019-08-19T13:42:25.1960536Z    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1960579Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1960631Z    |
2019-08-19T13:42:25.1960668Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1960746Z 
2019-08-19T13:42:25.1960746Z 
2019-08-19T13:42:25.1960786Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1961071Z    |
2019-08-19T13:42:25.1961071Z    |
2019-08-19T13:42:25.1961110Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1961303Z    |                     ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1961343Z    |                     |
2019-08-19T13:42:25.1961392Z    |                     may outlive borrowed value `x`
2019-08-19T13:42:25.1961456Z note: closure is returned here
2019-08-19T13:42:25.1961664Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:107:13
2019-08-19T13:42:25.1961772Z    |
2019-08-19T13:42:25.1961805Z LL |             Box::new(f)
2019-08-19T13:42:25.1961805Z LL |             Box::new(f)
2019-08-19T13:42:25.1961856Z    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1961898Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1961935Z    |
2019-08-19T13:42:25.1961989Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1962050Z 
2019-08-19T13:42:25.1962050Z 
2019-08-19T13:42:25.1962097Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1962766Z    |
2019-08-19T13:42:25.1962766Z    |
2019-08-19T13:42:25.1962814Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1963074Z    |                     ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1963135Z    |                     |
2019-08-19T13:42:25.1963182Z    |                     may outlive borrowed value `y`
2019-08-19T13:42:25.1963279Z note: closure is returned here
2019-08-19T13:42:25.1963536Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:107:13
2019-08-19T13:42:25.1963599Z    |
2019-08-19T13:42:25.1963641Z LL |             Box::new(f)
2019-08-19T13:42:25.1963641Z LL |             Box::new(f)
2019-08-19T13:42:25.1963685Z    |             ^^^^^^^^^^^
2019-08-19T13:42:25.1963746Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1963806Z    |
2019-08-19T13:42:25.1963855Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1963938Z 
2019-08-19T13:42:25.1963938Z 
2019-08-19T13:42:25.1963989Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1964323Z    |
2019-08-19T13:42:25.1964323Z    |
2019-08-19T13:42:25.1964371Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1964608Z    |                     ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1964666Z    |                     |
2019-08-19T13:42:25.1964712Z    |                     may outlive borrowed value `x`
2019-08-19T13:42:25.1964885Z note: closure is returned here
2019-08-19T13:42:25.1965182Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:135:20
2019-08-19T13:42:25.1965230Z    |
2019-08-19T13:42:25.1965273Z LL |             return Box::new(f);
2019-08-19T13:42:25.1965273Z LL |             return Box::new(f);
2019-08-19T13:42:25.1965335Z    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1965390Z help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1965447Z    |
2019-08-19T13:42:25.1965513Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1965591Z 
2019-08-19T13:42:25.1965591Z 
2019-08-19T13:42:25.1965657Z error[E0373]: closure may outlive the current function, but it borrows `y`, which is owned by the current function
2019-08-19T13:42:25.1966332Z    |
2019-08-19T13:42:25.1966332Z    |
2019-08-19T13:42:25.1966384Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1966584Z    |                     ^^^^^^^^^                   - `y` is borrowed here
2019-08-19T13:42:25.1966624Z    |                     |
2019-08-19T13:42:25.1966661Z    |                     may outlive borrowed value `y`
2019-08-19T13:42:25.1966736Z note: closure is returned here
2019-08-19T13:42:25.1967033Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:135:20
2019-08-19T13:42:25.1967080Z    |
2019-08-19T13:42:25.1967113Z LL |             return Box::new(f);
2019-08-19T13:42:25.1967113Z LL |             return Box::new(f);
2019-08-19T13:42:25.1967148Z    |                    ^^^^^^^^^^^
2019-08-19T13:42:25.1967202Z help: to force the closure to take ownership of `y` (and any other referenced variables), use the `move` keyword
2019-08-19T13:42:25.1967240Z    |
2019-08-19T13:42:25.1967286Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1967365Z 
2019-08-19T13:42:25.1967365Z 
2019-08-19T13:42:25.1967405Z error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
2019-08-19T13:42:25.1967674Z    |
2019-08-19T13:42:25.1967674Z    |
2019-08-19T13:42:25.1967713Z LL |             let f = |t: bool| if t { x } else { y }; // (separate errors for `x` vs `y`)
2019-08-19T13:42:25.1968092Z    |                     ^^^^^^^^^        - `x` is borrowed here
2019-08-19T13:42:25.1968143Z    |                     |
2019-08-19T13:42:25.1968359Z    |                     may outlive borrowed value `x`
2019-08-19T13:42:25.1968442Z note: closure is returned here
2019-08-19T13:42:25.1968659Z   --> /checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs:150:13
2019-08-19T13:42:25.1968700Z    |
---
2019-08-19T13:42:25.1985109Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-19T13:42:25.1985188Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-19T13:42:25.1985237Z 
2019-08-19T13:42:25.1985266Z 
2019-08-19T13:42:25.1987270Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-19T13:42:25.1987536Z 
2019-08-19T13:42:25.1987567Z 
2019-08-19T13:42:25.1987611Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-19T13:42:25.1987675Z Build completed unsuccessfully in 1:03:47
2019-08-19T13:42:25.1987675Z Build completed unsuccessfully in 1:03:47
2019-08-19T13:42:25.1987720Z == clock drift check ==
2019-08-19T13:42:25.1987764Z   local time: Mon Aug 19 13:42:25 UTC 2019
2019-08-19T13:42:25.3524791Z   network time: Mon, 19 Aug 2019 13:42:25 GMT
2019-08-19T13:42:25.3529136Z == end clock drift check ==
2019-08-19T13:42:26.5390068Z ##[error]Bash exited with code '1'.
2019-08-19T13:42:26.5428666Z ##[section]Starting: Checkout
2019-08-19T13:42:26.5430153Z ==============================================================================
2019-08-19T13:42:26.5430216Z Task         : Get sources
2019-08-19T13:42:26.5430254Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

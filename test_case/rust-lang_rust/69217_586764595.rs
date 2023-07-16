plain
2020-02-16T21:43:54.2943969Z ========================== Starting Command Output ===========================
2020-02-16T21:43:54.3268247Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/143a620d-e5d3-4b40-8fba-e4e606d46d8f.sh
2020-02-16T21:43:54.3268476Z 
2020-02-16T21:43:54.3271894Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T21:43:54.3277371Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-16T21:43:54.3279340Z Task         : Get sources
2020-02-16T21:43:54.3279378Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T21:43:54.3279411Z Version      : 1.0.0
2020-02-16T21:43:54.3279443Z Author       : Microsoft
---
2020-02-16T21:43:55.3306552Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T21:43:55.3425557Z ##[command]git config gc.auto 0
2020-02-16T21:43:55.3493445Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T21:43:55.3547726Z ##[command]git config --get-all http.proxy
2020-02-16T21:43:55.3705566Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69217/merge:refs/remotes/pull/69217/merge
---
2020-02-16T22:45:36.0872112Z .................................................................................................... 1700/9650
2020-02-16T22:45:40.9341428Z .................................................................................................... 1800/9650
2020-02-16T22:45:53.0185459Z ..................................i................................................................. 1900/9650
2020-02-16T22:46:00.9458383Z .................................................................................................... 2000/9650
2020-02-16T22:46:16.2959696Z ........................iiiii....................................................................... 2100/9650
2020-02-16T22:46:26.2704301Z .................................................................................................... 2300/9650
2020-02-16T22:46:28.6814290Z .................................................................................................... 2400/9650
2020-02-16T22:46:33.3825566Z .................................................................................................... 2500/9650
2020-02-16T22:46:55.4444856Z .................................................................................................... 2600/9650
---
2020-02-16T22:50:19.2586839Z .................................................................................................... 5600/9650
2020-02-16T22:50:30.4490956Z .......................................................................................i............ 5700/9650
2020-02-16T22:50:37.8094559Z .................................................................................................... 5800/9650
2020-02-16T22:50:43.1499144Z .....................................................................................i.............. 5900/9650
2020-02-16T22:50:53.0643209Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T22:51:05.7215457Z .i.................................................................................................. 6100/9650
2020-02-16T22:51:22.6459153Z .................................................................................................... 6300/9650
2020-02-16T22:51:30.5250000Z .................................................................................................... 6400/9650
2020-02-16T22:51:30.5250000Z .................................................................................................... 6400/9650
2020-02-16T22:51:46.4687176Z .......i..ii........................................................................................ 6500/9650
2020-02-16T22:52:07.1492100Z ...............................................................................................i.... 6700/9650
2020-02-16T22:52:09.3473435Z .................................................................................................... 6800/9650
2020-02-16T22:52:11.5076656Z .................................................................................................... 6900/9650
2020-02-16T22:52:13.9908361Z .....i.............................................................................................. 7000/9650
---
2020-02-16T22:53:52.6199929Z .................................................................................................... 7600/9650
2020-02-16T22:53:57.3092165Z .................................................................................................... 7700/9650
2020-02-16T22:54:03.4484342Z .................................................................................................... 7800/9650
2020-02-16T22:54:10.2027225Z .................................................................................................... 7900/9650
2020-02-16T22:54:19.9363775Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-16T22:54:36.4904584Z ...........................i.......i................................................................ 8200/9650
2020-02-16T22:54:41.5359219Z ..................F................................................................................. 8300/9650
2020-02-16T22:54:53.3786499Z .................................................................................................... 8400/9650
2020-02-16T22:55:05.3190742Z .................................................................................................... 8500/9650
---
2020-02-16T22:57:07.1769943Z 
2020-02-16T22:57:07.1772181Z ---- [ui] ui/autoderef-full-lval.rs stdout ----
2020-02-16T22:57:07.1772953Z diff of stderr:
2020-02-16T22:57:07.1773186Z 
2020-02-16T22:57:07.1780561Z 5    |                    --- ^ --- std::boxed::Box<isize>
2020-02-16T22:57:07.1781051Z 7    |                    std::boxed::Box<isize>
2020-02-16T22:57:07.1781473Z -    |
2020-02-16T22:57:07.1781473Z -    |
2020-02-16T22:57:07.1781750Z -    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2020-02-16T22:57:07.1781816Z 10 
2020-02-16T22:57:07.1781884Z 11 error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2020-02-16T22:57:07.1782152Z 
2020-02-16T22:57:07.1782152Z 
2020-02-16T22:57:07.1782611Z 15    |                         ------- ^ ----- std::boxed::Box<isize>
2020-02-16T22:57:07.1783296Z 17    |                         std::boxed::Box<isize>
2020-02-16T22:57:07.1783535Z -    |
2020-02-16T22:57:07.1783535Z -    |
2020-02-16T22:57:07.1783848Z -    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2020-02-16T22:57:07.1783945Z 21 error: aborting due to 2 previous errors
2020-02-16T22:57:07.1784011Z 22 
2020-02-16T22:57:07.1784039Z 
2020-02-16T22:57:07.1784065Z 
2020-02-16T22:57:07.1784065Z 
2020-02-16T22:57:07.1784113Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1784474Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/autoderef-full-lval.stderr
2020-02-16T22:57:07.1784745Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1785023Z To only update this specific test, also pass `--test-args autoderef-full-lval.rs`
2020-02-16T22:57:07.1785126Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1785182Z status: exit code: 1
2020-02-16T22:57:07.1785182Z status: exit code: 1
2020-02-16T22:57:07.1786026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/autoderef-full-lval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/auxiliary"
2020-02-16T22:57:07.1786518Z ------------------------------------------
2020-02-16T22:57:07.1786549Z 
2020-02-16T22:57:07.1787239Z ------------------------------------------
2020-02-16T22:57:07.1787302Z stderr:
2020-02-16T22:57:07.1787302Z stderr:
2020-02-16T22:57:07.1787508Z ------------------------------------------
2020-02-16T22:57:07.1787745Z error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2020-02-16T22:57:07.1788083Z    |
2020-02-16T22:57:07.1788083Z    |
2020-02-16T22:57:07.1788123Z LL |     let z: isize = a.x + b.y;
2020-02-16T22:57:07.1788361Z    |                    --- ^ --- std::boxed::Box<isize>
2020-02-16T22:57:07.1788450Z    |                    std::boxed::Box<isize>
2020-02-16T22:57:07.1788479Z 
2020-02-16T22:57:07.1788479Z 
2020-02-16T22:57:07.1788540Z error[E0369]: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>`
2020-02-16T22:57:07.1788903Z    |
2020-02-16T22:57:07.1788903Z    |
2020-02-16T22:57:07.1788963Z LL |     let answer: isize = forty.a + two.a;
2020-02-16T22:57:07.1789216Z    |                         ------- ^ ----- std::boxed::Box<isize>
2020-02-16T22:57:07.1789334Z    |                         std::boxed::Box<isize>
2020-02-16T22:57:07.1789363Z 
2020-02-16T22:57:07.1789403Z error: aborting due to 2 previous errors
2020-02-16T22:57:07.1789429Z 
---
2020-02-16T22:57:07.1791225Z 12 
2020-02-16T22:57:07.1791250Z 
2020-02-16T22:57:07.1791274Z 
2020-02-16T22:57:07.1791316Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1791617Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/binop-bitxor-str.stderr
2020-02-16T22:57:07.1791902Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1792158Z To only update this specific test, also pass `--test-args binop/binop-bitxor-str.rs`
2020-02-16T22:57:07.1792258Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1792299Z status: exit code: 1
2020-02-16T22:57:07.1792299Z status: exit code: 1
2020-02-16T22:57:07.1793750Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-bitxor-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/auxiliary"
2020-02-16T22:57:07.1794116Z ------------------------------------------
2020-02-16T22:57:07.1794151Z 
2020-02-16T22:57:07.1794381Z ------------------------------------------
2020-02-16T22:57:07.1794427Z stderr:
2020-02-16T22:57:07.1794427Z stderr:
2020-02-16T22:57:07.1794679Z ------------------------------------------
2020-02-16T22:57:07.1794735Z error[E0369]: no implementation for `std::string::String ^ std::string::String`
2020-02-16T22:57:07.1795060Z    |
2020-02-16T22:57:07.1795060Z    |
2020-02-16T22:57:07.1795107Z LL | fn main() { let x = "a".to_string() ^ "b".to_string(); }
2020-02-16T22:57:07.1795508Z    |                     --------------- ^ --------------- std::string::String
2020-02-16T22:57:07.1795629Z    |                     std::string::String
2020-02-16T22:57:07.1795660Z 
2020-02-16T22:57:07.1795704Z error: aborting due to previous error
2020-02-16T22:57:07.1795752Z 
---
2020-02-16T22:57:07.1797784Z 12 
2020-02-16T22:57:07.1797808Z 
2020-02-16T22:57:07.1797833Z 
2020-02-16T22:57:07.1797894Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1798178Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/binop-mul-bool.stderr
2020-02-16T22:57:07.1798417Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1798701Z To only update this specific test, also pass `--test-args binop/binop-mul-bool.rs`
2020-02-16T22:57:07.1798778Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1798837Z status: exit code: 1
2020-02-16T22:57:07.1798837Z status: exit code: 1
2020-02-16T22:57:07.1799586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-mul-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/auxiliary"
2020-02-16T22:57:07.1799907Z ------------------------------------------
2020-02-16T22:57:07.1799939Z 
2020-02-16T22:57:07.1800172Z ------------------------------------------
2020-02-16T22:57:07.1800215Z stderr:
2020-02-16T22:57:07.1800215Z stderr:
2020-02-16T22:57:07.1800422Z ------------------------------------------
2020-02-16T22:57:07.1800468Z error[E0369]: cannot multiply `bool` to `bool`
2020-02-16T22:57:07.1800714Z   --> /checkout/src/test/ui/binop/binop-mul-bool.rs:3:26
2020-02-16T22:57:07.1800760Z    |
2020-02-16T22:57:07.1800812Z LL | fn main() { let x = true * false; }
2020-02-16T22:57:07.1801041Z    |                     ---- ^ ----- bool
2020-02-16T22:57:07.1801129Z    |                     bool
2020-02-16T22:57:07.1801174Z 
2020-02-16T22:57:07.1801215Z error: aborting due to previous error
2020-02-16T22:57:07.1801242Z 
---
2020-02-16T22:57:07.1801787Z 
2020-02-16T22:57:07.1802002Z ---- [ui] ui/binop/binop-typeck.rs stdout ----
2020-02-16T22:57:07.1802067Z diff of stderr:
2020-02-16T22:57:07.1802092Z 
2020-02-16T22:57:07.1802295Z 5    |             - ^ - {integer}
2020-02-16T22:57:07.1802399Z 7    |             bool
2020-02-16T22:57:07.1803018Z -    |
2020-02-16T22:57:07.1803406Z -    = note: an implementation of `std::ops::Add` might be missing for `bool`
2020-02-16T22:57:07.1803483Z 10 
2020-02-16T22:57:07.1803483Z 10 
2020-02-16T22:57:07.1803533Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1803577Z 12 
2020-02-16T22:57:07.1803624Z 
2020-02-16T22:57:07.1803651Z 
2020-02-16T22:57:07.1803697Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1804010Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/binop-typeck.stderr
2020-02-16T22:57:07.1804294Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1804639Z To only update this specific test, also pass `--test-args binop/binop-typeck.rs`
2020-02-16T22:57:07.1804749Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1804794Z status: exit code: 1
2020-02-16T22:57:07.1804794Z status: exit code: 1
2020-02-16T22:57:07.1805619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/auxiliary"
2020-02-16T22:57:07.1806126Z ------------------------------------------
2020-02-16T22:57:07.1806177Z 
2020-02-16T22:57:07.1806551Z ------------------------------------------
2020-02-16T22:57:07.1806601Z stderr:
2020-02-16T22:57:07.1806601Z stderr:
2020-02-16T22:57:07.1806818Z ------------------------------------------
2020-02-16T22:57:07.1806864Z error[E0369]: cannot add `{integer}` to `bool`
2020-02-16T22:57:07.1807081Z   --> /checkout/src/test/ui/binop/binop-typeck.rs:6:15
2020-02-16T22:57:07.1807124Z    |
2020-02-16T22:57:07.1807180Z LL |     let z = x + y;
2020-02-16T22:57:07.1807384Z    |             - ^ - {integer}
2020-02-16T22:57:07.1807485Z    |             bool
2020-02-16T22:57:07.1807510Z 
2020-02-16T22:57:07.1807549Z error: aborting due to previous error
2020-02-16T22:57:07.1807574Z 
---
2020-02-16T22:57:07.1808760Z 25    |     [usize; 33]
2020-02-16T22:57:07.1808946Z -    |
2020-02-16T22:57:07.1809193Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `[usize; 33]`
2020-02-16T22:57:07.1809248Z 28 
2020-02-16T22:57:07.1809312Z 29 error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2020-02-16T22:57:07.1809564Z 
2020-02-16T22:57:07.1809564Z 
2020-02-16T22:57:07.1809773Z 33    |     ------------- ^ ------------- [usize; 33]
2020-02-16T22:57:07.1809872Z 35    |     [usize; 33]
2020-02-16T22:57:07.1810040Z -    |
2020-02-16T22:57:07.1810040Z -    |
2020-02-16T22:57:07.1810302Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `[usize; 33]`
2020-02-16T22:57:07.1810349Z 38 
2020-02-16T22:57:07.1810402Z 39 error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2020-02-16T22:57:07.1810669Z 
2020-02-16T22:57:07.1810693Z 
2020-02-16T22:57:07.1810734Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1810734Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1811187Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/core-traits-no-impls-length-33.stderr
2020-02-16T22:57:07.1811427Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1811708Z To only update this specific test, also pass `--test-args const-generics/array-impls/core-traits-no-impls-length-33.rs`
2020-02-16T22:57:07.1811799Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1811840Z status: exit code: 1
2020-02-16T22:57:07.1811840Z status: exit code: 1
2020-02-16T22:57:07.1813147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/core-traits-no-impls-length-33/auxiliary"
2020-02-16T22:57:07.1813563Z ------------------------------------------
2020-02-16T22:57:07.1813599Z 
2020-02-16T22:57:07.1813829Z ------------------------------------------
2020-02-16T22:57:07.1813896Z stderr:
2020-02-16T22:57:07.1813896Z stderr:
2020-02-16T22:57:07.1814118Z ------------------------------------------
2020-02-16T22:57:07.1814181Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-02-16T22:57:07.1814486Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:2:22
2020-02-16T22:57:07.1814541Z    |
2020-02-16T22:57:07.1814586Z LL |     println!("{:?}", [0_usize; 33]);
2020-02-16T22:57:07.1814661Z    |                      ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-02-16T22:57:07.1814775Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[usize; 33]`
2020-02-16T22:57:07.1814845Z    = note: required by `std::fmt::Debug::fmt`
2020-02-16T22:57:07.1815149Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-16T22:57:07.1815188Z 
2020-02-16T22:57:07.1815188Z 
2020-02-16T22:57:07.1815237Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-02-16T22:57:07.1815542Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:9:16
2020-02-16T22:57:07.1815593Z    |
2020-02-16T22:57:07.1815637Z LL |     set.insert([0_usize; 33]);
2020-02-16T22:57:07.1815711Z    |                ^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[usize; 33]`
2020-02-16T22:57:07.1815759Z    |
2020-02-16T22:57:07.1815818Z    = note: required because of the requirements on the impl of `std::cmp::Eq` for `[usize; 33]`
2020-02-16T22:57:07.1815921Z error[E0369]: binary operation `==` cannot be applied to type `[usize; 33]`
2020-02-16T22:57:07.1816521Z   --> /checkout/src/test/ui/const-generics/array-impls/core-traits-no-impls-length-33.rs:14:19
2020-02-16T22:57:07.1816584Z    |
2020-02-16T22:57:07.1816584Z    |
2020-02-16T22:57:07.1816624Z LL |     [0_usize; 33] == [1_usize; 33]
2020-02-16T22:57:07.1816837Z    |     ------------- ^^ ------------- [usize; 33]
2020-02-16T22:57:07.1816936Z    |     [usize; 33]
2020-02-16T22:57:07.1816961Z 
2020-02-16T22:57:07.1816961Z 
2020-02-16T22:57:07.1817011Z error[E0369]: binary operation `<` cannot be applied to type `[usize; 33]`
2020-02-16T22:57:07.1817331Z    |
2020-02-16T22:57:07.1817331Z    |
2020-02-16T22:57:07.1817370Z LL |     [0_usize; 33] < [1_usize; 33]
2020-02-16T22:57:07.1817683Z    |     ------------- ^ ------------- [usize; 33]
2020-02-16T22:57:07.1817782Z    |     [usize; 33]
2020-02-16T22:57:07.1817847Z 
2020-02-16T22:57:07.1817847Z 
2020-02-16T22:57:07.1817910Z error[E0277]: the trait bound `&[usize; 33]: std::iter::IntoIterator` is not satisfied
2020-02-16T22:57:07.1818218Z    |
2020-02-16T22:57:07.1818218Z    |
2020-02-16T22:57:07.1818274Z LL |     for _ in &[0_usize; 33] {
2020-02-16T22:57:07.1818324Z    |              ^^^^^^^^^^^^^^ the trait `std::iter::IntoIterator` is not implemented for `&[usize; 33]`
2020-02-16T22:57:07.1818497Z    = help: the following implementations were found:
2020-02-16T22:57:07.1818497Z    = help: the following implementations were found:
2020-02-16T22:57:07.1818741Z              <&'a [T; _] as std::iter::IntoIterator>
2020-02-16T22:57:07.1818951Z              <&'a [T] as std::iter::IntoIterator>
2020-02-16T22:57:07.1819168Z              <&'a mut [T; _] as std::iter::IntoIterator>
2020-02-16T22:57:07.1819414Z              <&'a mut [T] as std::iter::IntoIterator>
2020-02-16T22:57:07.1819491Z 
2020-02-16T22:57:07.1819764Z error: aborting due to 5 previous errors
2020-02-16T22:57:07.1819793Z 
2020-02-16T22:57:07.1819835Z Some errors have detailed explanations: E0277, E0369.
---
2020-02-16T22:57:07.1820399Z 
2020-02-16T22:57:07.1820634Z ---- [ui] ui/for/for-loop-type-error.rs stdout ----
2020-02-16T22:57:07.1820677Z diff of stderr:
2020-02-16T22:57:07.1820702Z 
2020-02-16T22:57:07.1820894Z 5    |             -- ^ -- ()
2020-02-16T22:57:07.1820992Z 7    |             ()
2020-02-16T22:57:07.1821159Z -    |
2020-02-16T22:57:07.1821405Z -    = note: an implementation of `std::ops::Add` might be missing for `()`
2020-02-16T22:57:07.1821459Z 10 
2020-02-16T22:57:07.1821459Z 10 
2020-02-16T22:57:07.1821499Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1821538Z 12 
2020-02-16T22:57:07.1821581Z 
2020-02-16T22:57:07.1821605Z 
2020-02-16T22:57:07.1821646Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1821935Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error/for-loop-type-error.stderr
2020-02-16T22:57:07.1822364Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1823168Z To only update this specific test, also pass `--test-args for/for-loop-type-error.rs`
2020-02-16T22:57:07.1823274Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1823318Z status: exit code: 1
2020-02-16T22:57:07.1823318Z status: exit code: 1
2020-02-16T22:57:07.1824139Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for/for-loop-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error/auxiliary"
2020-02-16T22:57:07.1824491Z ------------------------------------------
2020-02-16T22:57:07.1824542Z 
2020-02-16T22:57:07.1824775Z ------------------------------------------
2020-02-16T22:57:07.1824829Z stderr:
2020-02-16T22:57:07.1824829Z stderr:
2020-02-16T22:57:07.1825052Z ------------------------------------------
2020-02-16T22:57:07.1825120Z error[E0369]: cannot add `()` to `()`
2020-02-16T22:57:07.1825366Z   --> /checkout/src/test/ui/for/for-loop-type-error.rs:2:16
2020-02-16T22:57:07.1825415Z    |
2020-02-16T22:57:07.1825485Z LL |     let x = () + (); //~ ERROR cannot add `()` to `()`
2020-02-16T22:57:07.1825988Z    |             -- ^ -- ()
2020-02-16T22:57:07.1826256Z    |             ()
2020-02-16T22:57:07.1826281Z 
2020-02-16T22:57:07.1826320Z error: aborting due to previous error
2020-02-16T22:57:07.1826345Z 
---
2020-02-16T22:57:07.1826879Z 
2020-02-16T22:57:07.1827077Z ---- [ui] ui/issues/issue-14915.rs stdout ----
2020-02-16T22:57:07.1827188Z diff of stderr:
2020-02-16T22:57:07.1827218Z 
2020-02-16T22:57:07.1827452Z 5    |                    - ^ - {integer}
2020-02-16T22:57:07.1827535Z 7    |                    std::boxed::Box<isize>
2020-02-16T22:57:07.1827700Z -    |
2020-02-16T22:57:07.1827700Z -    |
2020-02-16T22:57:07.1827959Z -    = note: an implementation of `std::ops::Add` might be missing for `std::boxed::Box<isize>`
2020-02-16T22:57:07.1828055Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1828110Z 12 
2020-02-16T22:57:07.1828134Z 
2020-02-16T22:57:07.1828157Z 
2020-02-16T22:57:07.1828157Z 
2020-02-16T22:57:07.1828197Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1828486Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915/issue-14915.stderr
2020-02-16T22:57:07.1828710Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1828950Z To only update this specific test, also pass `--test-args issues/issue-14915.rs`
2020-02-16T22:57:07.1829039Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1829076Z status: exit code: 1
2020-02-16T22:57:07.1829076Z status: exit code: 1
2020-02-16T22:57:07.1829799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14915.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915/auxiliary"
2020-02-16T22:57:07.1830102Z ------------------------------------------
2020-02-16T22:57:07.1830132Z 
2020-02-16T22:57:07.1830327Z ------------------------------------------
2020-02-16T22:57:07.1830391Z stderr:
2020-02-16T22:57:07.1830391Z stderr:
2020-02-16T22:57:07.1830585Z ------------------------------------------
2020-02-16T22:57:07.1830631Z error[E0369]: cannot add `{integer}` to `std::boxed::Box<isize>`
2020-02-16T22:57:07.1830902Z    |
2020-02-16T22:57:07.1830949Z LL |     println!("{}", x + 1);
2020-02-16T22:57:07.1830949Z LL |     println!("{}", x + 1);
2020-02-16T22:57:07.1831145Z    |                    - ^ - {integer}
2020-02-16T22:57:07.1831245Z    |                    std::boxed::Box<isize>
2020-02-16T22:57:07.1831271Z 
2020-02-16T22:57:07.1831325Z error: aborting due to previous error
2020-02-16T22:57:07.1831350Z 
---
2020-02-16T22:57:07.1831854Z 
2020-02-16T22:57:07.1832059Z ---- [ui] ui/issues/issue-24363.rs stdout ----
2020-02-16T22:57:07.1832100Z diff of stderr:
2020-02-16T22:57:07.1832142Z 
2020-02-16T22:57:07.1832322Z 11    |         --^-- ()
2020-02-16T22:57:07.1832621Z 13    |         ()
2020-02-16T22:57:07.1833260Z -    |
2020-02-16T22:57:07.1833525Z -    = note: an implementation of `std::ops::Add` might be missing for `()`
2020-02-16T22:57:07.1833673Z 16 
2020-02-16T22:57:07.1833673Z 16 
2020-02-16T22:57:07.1833740Z 17 error: aborting due to 2 previous errors
2020-02-16T22:57:07.1833783Z 18 
2020-02-16T22:57:07.1833810Z 
2020-02-16T22:57:07.1833836Z 
2020-02-16T22:57:07.1833900Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1834234Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363/issue-24363.stderr
2020-02-16T22:57:07.1834493Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1834790Z To only update this specific test, also pass `--test-args issues/issue-24363.rs`
2020-02-16T22:57:07.1834971Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1835034Z status: exit code: 1
2020-02-16T22:57:07.1835034Z status: exit code: 1
2020-02-16T22:57:07.1835869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363/auxiliary"
2020-02-16T22:57:07.1836378Z ------------------------------------------
2020-02-16T22:57:07.1836412Z 
2020-02-16T22:57:07.1836798Z ------------------------------------------
2020-02-16T22:57:07.1836840Z stderr:
2020-02-16T22:57:07.1836840Z stderr:
2020-02-16T22:57:07.1837047Z ------------------------------------------
2020-02-16T22:57:07.1837287Z error[E0610]: `{integer}` is a primitive type and therefore doesn't have fields
2020-02-16T22:57:07.1837527Z   --> /checkout/src/test/ui/issues/issue-24363.rs:2:7
2020-02-16T22:57:07.1837573Z    |
2020-02-16T22:57:07.1837825Z LL |     1.create_a_type_error[ //~ `{integer}` is a primitive type and therefore doesn't have fields
2020-02-16T22:57:07.1837926Z 
2020-02-16T22:57:07.1837965Z error[E0369]: cannot add `()` to `()`
2020-02-16T22:57:07.1838204Z   --> /checkout/src/test/ui/issues/issue-24363.rs:3:11
2020-02-16T22:57:07.1838246Z    |
2020-02-16T22:57:07.1838246Z    |
2020-02-16T22:57:07.1838285Z LL |         ()+() //~ ERROR cannot add
2020-02-16T22:57:07.1838482Z    |         --^-- ()
2020-02-16T22:57:07.1838563Z    |         ()
2020-02-16T22:57:07.1838588Z 
2020-02-16T22:57:07.1838644Z error: aborting due to 2 previous errors
2020-02-16T22:57:07.1838678Z 
---
2020-02-16T22:57:07.1839259Z 
2020-02-16T22:57:07.1839485Z ---- [ui] ui/issues/issue-31076.rs stdout ----
2020-02-16T22:57:07.1839528Z diff of stderr:
2020-02-16T22:57:07.1839553Z 
2020-02-16T22:57:07.1839747Z 5    |             - ^ - {integer}
2020-02-16T22:57:07.1839848Z 7    |             {integer}
2020-02-16T22:57:07.1840016Z -    |
2020-02-16T22:57:07.1840251Z -    = note: an implementation of `std::ops::Add` might be missing for `{integer}`
2020-02-16T22:57:07.1840312Z 10 
---
2020-02-16T22:57:07.1841546Z 
2020-02-16T22:57:07.1841570Z 
2020-02-16T22:57:07.1841628Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1842103Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/issue-31076.stderr
2020-02-16T22:57:07.1844858Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1845200Z To only update this specific test, also pass `--test-args issues/issue-31076.rs`
2020-02-16T22:57:07.1845422Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1845496Z status: exit code: 1
2020-02-16T22:57:07.1845496Z status: exit code: 1
2020-02-16T22:57:07.1846341Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary"
2020-02-16T22:57:07.1847174Z ------------------------------------------
2020-02-16T22:57:07.1847205Z 
2020-02-16T22:57:07.1847429Z ------------------------------------------
2020-02-16T22:57:07.1847471Z stderr:
2020-02-16T22:57:07.1847471Z stderr:
2020-02-16T22:57:07.1847671Z ------------------------------------------
2020-02-16T22:57:07.1847725Z error[E0369]: cannot add `{integer}` to `{integer}`
2020-02-16T22:57:07.1848011Z    |
2020-02-16T22:57:07.1848051Z LL |     let x = 5 + 6;
2020-02-16T22:57:07.1848051Z LL |     let x = 5 + 6;
2020-02-16T22:57:07.1848264Z    |             - ^ - {integer}
2020-02-16T22:57:07.1848355Z    |             {integer}
2020-02-16T22:57:07.1848380Z 
2020-02-16T22:57:07.1848437Z error[E0369]: cannot add `i32` to `i32`
2020-02-16T22:57:07.1848706Z   --> /checkout/src/test/ui/issues/issue-31076.rs:15:18
---
2020-02-16T22:57:07.1850917Z 
2020-02-16T22:57:07.1850941Z 
2020-02-16T22:57:07.1850982Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1851285Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/issue-35668.stderr
2020-02-16T22:57:07.1851520Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1851763Z To only update this specific test, also pass `--test-args issues/issue-35668.rs`
2020-02-16T22:57:07.1854930Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1854976Z status: exit code: 1
2020-02-16T22:57:07.1854976Z status: exit code: 1
2020-02-16T22:57:07.1856027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35668.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/auxiliary"
2020-02-16T22:57:07.1856732Z ------------------------------------------
2020-02-16T22:57:07.1856766Z 
2020-02-16T22:57:07.1857329Z ------------------------------------------
2020-02-16T22:57:07.1857372Z stderr:
2020-02-16T22:57:07.1857372Z stderr:
2020-02-16T22:57:07.1857607Z ------------------------------------------
2020-02-16T22:57:07.1857665Z error[E0369]: cannot multiply `&T` to `&T`
2020-02-16T22:57:07.1857901Z   --> /checkout/src/test/ui/issues/issue-35668.rs:2:23
2020-02-16T22:57:07.1857964Z    |
2020-02-16T22:57:07.1858006Z LL |     a.iter().map(|a| a*a)
2020-02-16T22:57:07.1858213Z    |                      -^- &T
2020-02-16T22:57:07.1858499Z    |                      &T
2020-02-16T22:57:07.1858528Z 
2020-02-16T22:57:07.1858571Z error: aborting due to previous error
2020-02-16T22:57:07.1858618Z 
---
2020-02-16T22:57:07.1859220Z 
2020-02-16T22:57:07.1859448Z ---- [ui] ui/issues/issue-40610.rs stdout ----
2020-02-16T22:57:07.1859495Z diff of stderr:
2020-02-16T22:57:07.1859750Z 
2020-02-16T22:57:07.1860030Z 5    |     -- ^ --------- ()
2020-02-16T22:57:07.1860281Z 7    |     ()
2020-02-16T22:57:07.1860682Z -    |
2020-02-16T22:57:07.1860942Z -    = note: an implementation of `std::ops::Add` might be missing for `()`
2020-02-16T22:57:07.1860993Z 10 
2020-02-16T22:57:07.1860993Z 10 
2020-02-16T22:57:07.1861057Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1861100Z 12 
2020-02-16T22:57:07.1861127Z 
2020-02-16T22:57:07.1861154Z 
2020-02-16T22:57:07.1861382Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1861899Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/issue-40610.stderr
2020-02-16T22:57:07.1862880Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1863221Z To only update this specific test, also pass `--test-args issues/issue-40610.rs`
2020-02-16T22:57:07.1863303Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1863348Z status: exit code: 1
2020-02-16T22:57:07.1863348Z status: exit code: 1
2020-02-16T22:57:07.1864193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40610.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/auxiliary"
2020-02-16T22:57:07.1864544Z ------------------------------------------
2020-02-16T22:57:07.1864579Z 
2020-02-16T22:57:07.1864805Z ------------------------------------------
2020-02-16T22:57:07.1864869Z stderr:
2020-02-16T22:57:07.1864869Z stderr:
2020-02-16T22:57:07.1865091Z ------------------------------------------
2020-02-16T22:57:07.1865142Z error[E0369]: cannot add `()` to `()`
2020-02-16T22:57:07.1865558Z   --> /checkout/src/test/ui/issues/issue-40610.rs:4:8
2020-02-16T22:57:07.1865606Z    |
2020-02-16T22:57:07.1865649Z LL |     () + f(&[1.0]);
2020-02-16T22:57:07.1865877Z    |     -- ^ --------- ()
2020-02-16T22:57:07.1866125Z    |     ()
2020-02-16T22:57:07.1866151Z 
2020-02-16T22:57:07.1866210Z error: aborting due to previous error
2020-02-16T22:57:07.1866237Z 
---
2020-02-16T22:57:07.1866878Z 
2020-02-16T22:57:07.1867120Z ---- [ui] ui/issues/issue-41394.rs stdout ----
2020-02-16T22:57:07.1867184Z diff of stderr:
2020-02-16T22:57:07.1867210Z 
2020-02-16T22:57:07.1867411Z 5    |         -- ^ - {integer}
2020-02-16T22:57:07.1867513Z 7    |         &str
2020-02-16T22:57:07.1867698Z -    |
2020-02-16T22:57:07.1867939Z -    = note: an implementation of `std::ops::Add` might be missing for `&str`
2020-02-16T22:57:07.1868004Z 10 
2020-02-16T22:57:07.1868004Z 10 
2020-02-16T22:57:07.1868048Z 11 error[E0080]: evaluation of constant value failed
2020-02-16T22:57:07.1868254Z 12   --> $DIR/issue-41394.rs:7:9
2020-02-16T22:57:07.1868284Z 
2020-02-16T22:57:07.1868326Z 
2020-02-16T22:57:07.1868369Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1868652Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/issue-41394.stderr
2020-02-16T22:57:07.1868911Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1869174Z To only update this specific test, also pass `--test-args issues/issue-41394.rs`
2020-02-16T22:57:07.1869248Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1869308Z status: exit code: 1
2020-02-16T22:57:07.1869308Z status: exit code: 1
2020-02-16T22:57:07.1870055Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary"
2020-02-16T22:57:07.1870380Z ------------------------------------------
2020-02-16T22:57:07.1870418Z 
2020-02-16T22:57:07.1870647Z ------------------------------------------
2020-02-16T22:57:07.1870689Z stderr:
2020-02-16T22:57:07.1870689Z stderr:
2020-02-16T22:57:07.1870894Z ------------------------------------------
2020-02-16T22:57:07.1870958Z error[E0369]: cannot add `{integer}` to `&str`
2020-02-16T22:57:07.1871182Z   --> /checkout/src/test/ui/issues/issue-41394.rs:2:12
2020-02-16T22:57:07.1871235Z    |
2020-02-16T22:57:07.1871292Z LL |     A = "" + 1
2020-02-16T22:57:07.1871492Z    |         -- ^ - {integer}
2020-02-16T22:57:07.1871577Z    |         &str
2020-02-16T22:57:07.1871621Z 
2020-02-16T22:57:07.1871663Z error[E0080]: evaluation of constant value failed
2020-02-16T22:57:07.1871889Z   --> /checkout/src/test/ui/issues/issue-41394.rs:7:9
---
2020-02-16T22:57:07.1873404Z 
2020-02-16T22:57:07.1873682Z ---- [ui] ui/issues/issue-59488.rs stdout ----
2020-02-16T22:57:07.1873732Z diff of stderr:
2020-02-16T22:57:07.1873760Z 
2020-02-16T22:57:07.1873987Z 58    |     --- ^ --- fn(i64) -> i64 {bar}
2020-02-16T22:57:07.1874264Z 60    |     fn() -> i32 {foo}
2020-02-16T22:57:07.1874451Z -    |
2020-02-16T22:57:07.1874750Z -    = note: an implementation of `std::cmp::PartialOrd` might be missing for `fn() -> i32 {foo}`
2020-02-16T22:57:07.1874805Z 63 
2020-02-16T22:57:07.1874805Z 63 
2020-02-16T22:57:07.1874854Z 64 error[E0308]: mismatched types
2020-02-16T22:57:07.1875171Z 65   --> $DIR/issue-59488.rs:25:11
2020-02-16T22:57:07.1875210Z 
2020-02-16T22:57:07.1875453Z 79    |     fn(usize) -> Foo {Foo::Bar}
2020-02-16T22:57:07.1875679Z 80    |     fn(usize) -> Foo {Foo::Bar}
2020-02-16T22:57:07.1875742Z 81    |
2020-02-16T22:57:07.1876032Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1876569Z 84 
2020-02-16T22:57:07.1876569Z 84 
2020-02-16T22:57:07.1876825Z 85 error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-02-16T22:57:07.1876884Z 
2020-02-16T22:57:07.1876946Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1877239Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
2020-02-16T22:57:07.1877239Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
2020-02-16T22:57:07.1877485Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1877767Z To only update this specific test, also pass `--test-args issues/issue-59488.rs`
2020-02-16T22:57:07.1877845Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1877889Z status: exit code: 1
2020-02-16T22:57:07.1877889Z status: exit code: 1
2020-02-16T22:57:07.1878689Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
2020-02-16T22:57:07.1879025Z ------------------------------------------
2020-02-16T22:57:07.1879066Z 
2020-02-16T22:57:07.1879285Z ------------------------------------------
2020-02-16T22:57:07.1879347Z stderr:
2020-02-16T22:57:07.1879347Z stderr:
2020-02-16T22:57:07.1879558Z ------------------------------------------
2020-02-16T22:57:07.1879814Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-02-16T22:57:07.1880132Z    |
2020-02-16T22:57:07.1880173Z LL |     foo > 12;
2020-02-16T22:57:07.1880173Z LL |     foo > 12;
2020-02-16T22:57:07.1880392Z    |     --- ^ -- {integer}
2020-02-16T22:57:07.1880631Z    |     fn() -> i32 {foo}
2020-02-16T22:57:07.1880700Z    |     help: you might have forgotten to call this function: `foo()`
2020-02-16T22:57:07.1880732Z 
2020-02-16T22:57:07.1880773Z error[E0308]: mismatched types
---
2020-02-16T22:57:07.1888390Z    |
2020-02-16T22:57:07.1888632Z    = note: expected fn item `fn() -> i32 {foo}`
2020-02-16T22:57:07.1888683Z                  found type `i32`
2020-02-16T22:57:07.1888714Z 
2020-02-16T22:57:07.1888995Z error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
2020-02-16T22:57:07.1889462Z    |
2020-02-16T22:57:07.1889523Z LL |     bar > 13;
2020-02-16T22:57:07.1889523Z LL |     bar > 13;
2020-02-16T22:57:07.1889726Z    |     --- ^ -- {integer}
2020-02-16T22:57:07.1889772Z    |     |
2020-02-16T22:57:07.1889976Z    |     fn(i64) -> i64 {bar}
2020-02-16T22:57:07.1890060Z    |     help: you might have forgotten to call this function: `bar( /* arguments */ )`
2020-02-16T22:57:07.1890136Z error[E0308]: mismatched types
2020-02-16T22:57:07.1890657Z   --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
2020-02-16T22:57:07.1890705Z    |
2020-02-16T22:57:07.1890827Z LL |     bar > 13;
2020-02-16T22:57:07.1890827Z LL |     bar > 13;
2020-02-16T22:57:07.1890882Z    |           ^^ expected fn item, found integer
2020-02-16T22:57:07.1890948Z    |
2020-02-16T22:57:07.1891211Z    = note: expected fn item `fn(i64) -> i64 {bar}`
2020-02-16T22:57:07.1891310Z 
2020-02-16T22:57:07.1891310Z 
2020-02-16T22:57:07.1891587Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-02-16T22:57:07.1891899Z    |
2020-02-16T22:57:07.1891942Z LL |     foo > foo;
2020-02-16T22:57:07.1891942Z LL |     foo > foo;
2020-02-16T22:57:07.1892164Z    |     --- ^ --- fn() -> i32 {foo}
2020-02-16T22:57:07.1892665Z    |     fn() -> i32 {foo}
2020-02-16T22:57:07.1892714Z    |
2020-02-16T22:57:07.1892760Z help: you might have forgotten to call this function
2020-02-16T22:57:07.1894079Z    |
2020-02-16T22:57:07.1894079Z    |
2020-02-16T22:57:07.1894125Z LL |     foo() > foo;
2020-02-16T22:57:07.1894169Z    |     ^^^^^
2020-02-16T22:57:07.1894230Z help: you might have forgotten to call this function
2020-02-16T22:57:07.1894293Z    |
2020-02-16T22:57:07.1894335Z LL |     foo > foo();
2020-02-16T22:57:07.1894378Z    |           ^^^^^
2020-02-16T22:57:07.1894425Z 
2020-02-16T22:57:07.1894820Z error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
2020-02-16T22:57:07.1895134Z    |
2020-02-16T22:57:07.1895134Z    |
2020-02-16T22:57:07.1895195Z LL |     foo > bar;
2020-02-16T22:57:07.1895424Z    |     --- ^ --- fn(i64) -> i64 {bar}
2020-02-16T22:57:07.1895698Z    |     fn() -> i32 {foo}
2020-02-16T22:57:07.1895730Z 
2020-02-16T22:57:07.1895773Z error[E0308]: mismatched types
2020-02-16T22:57:07.1896013Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
2020-02-16T22:57:07.1896013Z   --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
2020-02-16T22:57:07.1896183Z    |
2020-02-16T22:57:07.1896224Z LL |     foo > bar;
2020-02-16T22:57:07.1896272Z    |           ^^^ expected fn item, found a different fn item
2020-02-16T22:57:07.1896577Z    = note: expected fn item `fn() -> i32 {foo}`
2020-02-16T22:57:07.1896577Z    = note: expected fn item `fn() -> i32 {foo}`
2020-02-16T22:57:07.1897907Z               found fn item `fn(i64) -> i64 {bar}`
2020-02-16T22:57:07.1897956Z 
2020-02-16T22:57:07.1898444Z error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1898760Z    |
2020-02-16T22:57:07.1898760Z    |
2020-02-16T22:57:07.1898821Z LL |     assert_eq!(Foo::Bar, i);
2020-02-16T22:57:07.1898910Z    |     |
2020-02-16T22:57:07.1898910Z    |     |
2020-02-16T22:57:07.1899149Z    |     fn(usize) -> Foo {Foo::Bar}
2020-02-16T22:57:07.1899373Z    |     fn(usize) -> Foo {Foo::Bar}
2020-02-16T22:57:07.1899959Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-16T22:57:07.1900029Z 
2020-02-16T22:57:07.1900029Z 
2020-02-16T22:57:07.1900317Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-02-16T22:57:07.1900631Z    |
2020-02-16T22:57:07.1900631Z    |
2020-02-16T22:57:07.1900674Z LL |     assert_eq!(Foo::Bar, i);
2020-02-16T22:57:07.1901003Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-02-16T22:57:07.1901233Z    |
2020-02-16T22:57:07.1901547Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1901855Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1902221Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-16T22:57:07.1902259Z 
2020-02-16T22:57:07.1902259Z 
2020-02-16T22:57:07.1902954Z error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `std::fmt::Debug`
2020-02-16T22:57:07.1903289Z    |
2020-02-16T22:57:07.1903289Z    |
2020-02-16T22:57:07.1903353Z LL |     assert_eq!(Foo::Bar, i);
2020-02-16T22:57:07.1903684Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2020-02-16T22:57:07.1903747Z    |
2020-02-16T22:57:07.1904039Z    = help: the trait `std::fmt::Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1904346Z    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&fn(usize) -> Foo {Foo::Bar}`
2020-02-16T22:57:07.1904707Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-16T22:57:07.1904746Z 
2020-02-16T22:57:07.1904791Z error: aborting due to 10 previous errors
2020-02-16T22:57:07.1904821Z 
---
2020-02-16T22:57:07.1905481Z 
2020-02-16T22:57:07.1905724Z ---- [ui] ui/pattern/pattern-tyvar-2.rs stdout ----
2020-02-16T22:57:07.1905773Z diff of stderr:
2020-02-16T22:57:07.1905819Z 
2020-02-16T22:57:07.1906251Z 5    |                                                                     - ^ - {integer}
2020-02-16T22:57:07.1906381Z 7    |                                                                     std::vec::Vec<isize>
2020-02-16T22:57:07.1906565Z -    |
2020-02-16T22:57:07.1906829Z -    = note: an implementation of `std::ops::Mul` might be missing for `std::vec::Vec<isize>`
2020-02-16T22:57:07.1906895Z 10 
2020-02-16T22:57:07.1906895Z 10 
2020-02-16T22:57:07.1906948Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1906990Z 12 
2020-02-16T22:57:07.1907016Z 
2020-02-16T22:57:07.1907042Z 
2020-02-16T22:57:07.1907105Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1907408Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2/pattern-tyvar-2.stderr
2020-02-16T22:57:07.1907666Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1907948Z To only update this specific test, also pass `--test-args pattern/pattern-tyvar-2.rs`
2020-02-16T22:57:07.1908026Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1908144Z status: exit code: 1
2020-02-16T22:57:07.1908144Z status: exit code: 1
2020-02-16T22:57:07.1908944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-tyvar-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2/auxiliary"
2020-02-16T22:57:07.1909382Z ------------------------------------------
2020-02-16T22:57:07.1909416Z 
2020-02-16T22:57:07.1909772Z ------------------------------------------
2020-02-16T22:57:07.1909817Z stderr:
2020-02-16T22:57:07.1909817Z stderr:
2020-02-16T22:57:07.1910258Z ------------------------------------------
2020-02-16T22:57:07.1910370Z error[E0369]: cannot multiply `{integer}` to `std::vec::Vec<isize>`
2020-02-16T22:57:07.1910673Z    |
2020-02-16T22:57:07.1910673Z    |
2020-02-16T22:57:07.1911061Z LL | fn foo(t: Bar) -> isize { match t { Bar::T1(_, Some(x)) => { return x * 3; } _ => { panic!(); } } }
2020-02-16T22:57:07.1911379Z    |                                                                     - ^ - {integer}
2020-02-16T22:57:07.1911516Z    |                                                                     std::vec::Vec<isize>
2020-02-16T22:57:07.1911558Z 
2020-02-16T22:57:07.1911603Z error: aborting due to previous error
2020-02-16T22:57:07.1911632Z 
---
2020-02-16T22:57:07.1912220Z 
2020-02-16T22:57:07.1912681Z ---- [ui] ui/span/issue-39018.rs stdout ----
2020-02-16T22:57:07.1912734Z diff of stderr:
2020-02-16T22:57:07.1912763Z 
2020-02-16T22:57:07.1913008Z 136    |             -- ^ -- &&str
2020-02-16T22:57:07.1913111Z 138    |             &&str
2020-02-16T22:57:07.1913303Z -    |
2020-02-16T22:57:07.1913583Z -    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2020-02-16T22:57:07.1913634Z 141 
2020-02-16T22:57:07.1913634Z 141 
2020-02-16T22:57:07.1913678Z 142 error[E0369]: cannot add `&str` to `&&str`
2020-02-16T22:57:07.1913929Z 143   --> $DIR/issue-39018.rs:35:16
2020-02-16T22:57:07.1913962Z 
2020-02-16T22:57:07.1914175Z 146    |             -- ^ - &str
2020-02-16T22:57:07.1914288Z 148    |             &&str
2020-02-16T22:57:07.1914475Z -    |
2020-02-16T22:57:07.1914732Z -    = note: an implementation of `std::ops::Add` might be missing for `&&str`
2020-02-16T22:57:07.1914799Z 151 
2020-02-16T22:57:07.1914799Z 151 
2020-02-16T22:57:07.1914844Z 152 error[E0369]: cannot add `&&str` to `&str`
2020-02-16T22:57:07.1915066Z 153   --> $DIR/issue-39018.rs:36:15
2020-02-16T22:57:07.1915099Z 
2020-02-16T22:57:07.1915143Z 
2020-02-16T22:57:07.1915198Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1915503Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
2020-02-16T22:57:07.1916073Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1916329Z To only update this specific test, also pass `--test-args span/issue-39018.rs`
2020-02-16T22:57:07.1916414Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1916475Z status: exit code: 1
2020-02-16T22:57:07.1916475Z status: exit code: 1
2020-02-16T22:57:07.1917257Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary"
2020-02-16T22:57:07.1917586Z ------------------------------------------
2020-02-16T22:57:07.1917619Z 
2020-02-16T22:57:07.1917855Z ------------------------------------------
2020-02-16T22:57:07.1917996Z stderr:
2020-02-16T22:57:07.1917996Z stderr:
2020-02-16T22:57:07.1918231Z ------------------------------------------
2020-02-16T22:57:07.1918297Z error[E0369]: cannot add `&str` to `&str`
2020-02-16T22:57:07.1918528Z   --> /checkout/src/test/ui/span/issue-39018.rs:2:22
2020-02-16T22:57:07.1918573Z    |
2020-02-16T22:57:07.1918632Z LL |     let x = "Hello " + "World!";
2020-02-16T22:57:07.1918850Z    |             -------- ^ -------- &str
2020-02-16T22:57:07.1918897Z    |             |        |
2020-02-16T22:57:07.1918946Z    |             |        `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1919054Z    |
2020-02-16T22:57:07.1919054Z    |
2020-02-16T22:57:07.1919191Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1919277Z    |
2020-02-16T22:57:07.1919322Z LL |     let x = "Hello ".to_owned() + "World!";
2020-02-16T22:57:07.1919424Z 
2020-02-16T22:57:07.1919424Z 
2020-02-16T22:57:07.1919466Z error[E0369]: cannot add `World` to `World`
2020-02-16T22:57:07.1919794Z    |
2020-02-16T22:57:07.1919794Z    |
2020-02-16T22:57:07.1919836Z LL |     let y = World::Hello + World::Goodbye;
2020-02-16T22:57:07.1920063Z    |             ------------ ^ -------------- World
2020-02-16T22:57:07.1920169Z    |             World
2020-02-16T22:57:07.1920208Z    |
2020-02-16T22:57:07.1920208Z    |
2020-02-16T22:57:07.1920262Z    = note: an implementation of `std::ops::Add` might be missing for `World`
2020-02-16T22:57:07.1920359Z error[E0369]: cannot add `std::string::String` to `&str`
2020-02-16T22:57:07.1920593Z   --> /checkout/src/test/ui/span/issue-39018.rs:11:22
2020-02-16T22:57:07.1920654Z    |
2020-02-16T22:57:07.1920654Z    |
2020-02-16T22:57:07.1920697Z LL |     let x = "Hello " + "World!".to_owned();
2020-02-16T22:57:07.1920949Z    |             -------- ^ ------------------- std::string::String
2020-02-16T22:57:07.1921065Z    |             |        `+` cannot be used to concatenate a `&str` with a `String`
2020-02-16T22:57:07.1921112Z    |             &str
2020-02-16T22:57:07.1921151Z    |
2020-02-16T22:57:07.1921151Z    |
2020-02-16T22:57:07.1921235Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1921293Z    |
2020-02-16T22:57:07.1921362Z LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
2020-02-16T22:57:07.1921442Z 
2020-02-16T22:57:07.1921488Z error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2020-02-16T22:57:07.1921747Z   --> /checkout/src/test/ui/span/issue-39018.rs:26:16
2020-02-16T22:57:07.1921801Z    |
2020-02-16T22:57:07.1921801Z    |
2020-02-16T22:57:07.1921845Z LL |     let _ = &a + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1922086Z    |             -- ^ -- &std::string::String
2020-02-16T22:57:07.1922133Z    |             |  |
2020-02-16T22:57:07.1922180Z    |             |  `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1922286Z    |
2020-02-16T22:57:07.1922286Z    |
2020-02-16T22:57:07.1922343Z help: String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1922420Z    |
2020-02-16T22:57:07.1922463Z LL |     let _ = a + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1922693Z 
2020-02-16T22:57:07.1922997Z error[E0369]: cannot add `std::string::String` to `&std::string::String`
2020-02-16T22:57:07.1923278Z   --> /checkout/src/test/ui/span/issue-39018.rs:27:16
2020-02-16T22:57:07.1923420Z    |
2020-02-16T22:57:07.1923420Z    |
2020-02-16T22:57:07.1923483Z LL |     let _ = &a + b; //~ ERROR cannot add
2020-02-16T22:57:07.1923740Z    |             -- ^ - std::string::String
2020-02-16T22:57:07.1923837Z    |             |  `+` cannot be used to concatenate a `&str` with a `String`
2020-02-16T22:57:07.1923904Z    |             &std::string::String
2020-02-16T22:57:07.1923946Z    |
2020-02-16T22:57:07.1923946Z    |
2020-02-16T22:57:07.1924083Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1924166Z    |
2020-02-16T22:57:07.1924211Z LL |     let _ = a + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1924304Z 
2020-02-16T22:57:07.1924347Z error[E0308]: mismatched types
2020-02-16T22:57:07.1924616Z   --> /checkout/src/test/ui/span/issue-39018.rs:29:17
2020-02-16T22:57:07.1924672Z    |
2020-02-16T22:57:07.1924672Z    |
2020-02-16T22:57:07.1924735Z LL |     let _ = a + b; //~ ERROR mismatched types
2020-02-16T22:57:07.1924824Z    |                 |
2020-02-16T22:57:07.1924890Z    |                 expected `&str`, found struct `std::string::String`
2020-02-16T22:57:07.1924942Z    |                 help: consider borrowing here: `&b`
2020-02-16T22:57:07.1924972Z 
2020-02-16T22:57:07.1924972Z 
2020-02-16T22:57:07.1925036Z error[E0369]: cannot add `std::string::String` to `&std::string::String`
2020-02-16T22:57:07.1925286Z   --> /checkout/src/test/ui/span/issue-39018.rs:30:15
2020-02-16T22:57:07.1925341Z    |
2020-02-16T22:57:07.1925386Z LL |     let _ = e + b; //~ ERROR cannot add
2020-02-16T22:57:07.1925631Z    |             - ^ - std::string::String
2020-02-16T22:57:07.1925730Z    |             | `+` cannot be used to concatenate a `&str` with a `String`
2020-02-16T22:57:07.1925804Z    |             &std::string::String
2020-02-16T22:57:07.1925846Z    |
2020-02-16T22:57:07.1925846Z    |
2020-02-16T22:57:07.1925914Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1925992Z    |
2020-02-16T22:57:07.1926037Z LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1926130Z 
2020-02-16T22:57:07.1926179Z error[E0369]: cannot add `&std::string::String` to `&std::string::String`
2020-02-16T22:57:07.1926603Z   --> /checkout/src/test/ui/span/issue-39018.rs:31:15
2020-02-16T22:57:07.1926648Z    |
2020-02-16T22:57:07.1926648Z    |
2020-02-16T22:57:07.1926707Z LL |     let _ = e + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1926921Z    |             - ^ -- &std::string::String
2020-02-16T22:57:07.1926965Z    |             | |
2020-02-16T22:57:07.1927028Z    |             | `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1927119Z    |
2020-02-16T22:57:07.1927119Z    |
2020-02-16T22:57:07.1927200Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1927255Z    |
2020-02-16T22:57:07.1927297Z LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add
2020-02-16T22:57:07.1927385Z 
2020-02-16T22:57:07.1927433Z error[E0369]: cannot add `&str` to `&std::string::String`
2020-02-16T22:57:07.1927666Z   --> /checkout/src/test/ui/span/issue-39018.rs:32:15
2020-02-16T22:57:07.1927728Z    |
2020-02-16T22:57:07.1927728Z    |
2020-02-16T22:57:07.1927768Z LL |     let _ = e + d; //~ ERROR cannot add
2020-02-16T22:57:07.1927965Z    |             - ^ - &str
2020-02-16T22:57:07.1928026Z    |             | |
2020-02-16T22:57:07.1928150Z    |             | `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1928250Z    |
2020-02-16T22:57:07.1928250Z    |
2020-02-16T22:57:07.1928313Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1928368Z    |
2020-02-16T22:57:07.1928428Z LL |     let _ = e.to_owned() + d; //~ ERROR cannot add
2020-02-16T22:57:07.1928498Z 
2020-02-16T22:57:07.1928590Z error[E0369]: cannot add `&&str` to `&std::string::String`
2020-02-16T22:57:07.1928867Z   --> /checkout/src/test/ui/span/issue-39018.rs:33:15
2020-02-16T22:57:07.1928911Z    |
2020-02-16T22:57:07.1928911Z    |
2020-02-16T22:57:07.1928952Z LL |     let _ = e + &d; //~ ERROR cannot add
2020-02-16T22:57:07.1929169Z    |             - ^ -- &&str
2020-02-16T22:57:07.1929221Z    |             | |
2020-02-16T22:57:07.1929267Z    |             | `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1929367Z    |
2020-02-16T22:57:07.1929367Z    |
2020-02-16T22:57:07.1929431Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1929503Z    |
2020-02-16T22:57:07.1929545Z LL |     let _ = e.to_owned() + &d; //~ ERROR cannot add
2020-02-16T22:57:07.1929621Z 
2020-02-16T22:57:07.1929683Z error[E0369]: cannot add `&&str` to `&&str`
2020-02-16T22:57:07.1929912Z   --> /checkout/src/test/ui/span/issue-39018.rs:34:16
2020-02-16T22:57:07.1929955Z    |
2020-02-16T22:57:07.1929955Z    |
2020-02-16T22:57:07.1930013Z LL |     let _ = &c + &d; //~ ERROR cannot add
2020-02-16T22:57:07.1930220Z    |             -- ^ -- &&str
2020-02-16T22:57:07.1930349Z    |             &&str
2020-02-16T22:57:07.1930374Z 
2020-02-16T22:57:07.1930416Z error[E0369]: cannot add `&str` to `&&str`
2020-02-16T22:57:07.1930638Z   --> /checkout/src/test/ui/span/issue-39018.rs:35:16
2020-02-16T22:57:07.1930638Z   --> /checkout/src/test/ui/span/issue-39018.rs:35:16
2020-02-16T22:57:07.1930697Z    |
2020-02-16T22:57:07.1930737Z LL |     let _ = &c + d; //~ ERROR cannot add
2020-02-16T22:57:07.1930935Z    |             -- ^ - &str
2020-02-16T22:57:07.1931035Z    |             &&str
2020-02-16T22:57:07.1931061Z 
2020-02-16T22:57:07.1931101Z error[E0369]: cannot add `&&str` to `&str`
2020-02-16T22:57:07.1931350Z   --> /checkout/src/test/ui/span/issue-39018.rs:36:15
2020-02-16T22:57:07.1931350Z   --> /checkout/src/test/ui/span/issue-39018.rs:36:15
2020-02-16T22:57:07.1931394Z    |
2020-02-16T22:57:07.1931435Z LL |     let _ = c + &d; //~ ERROR cannot add
2020-02-16T22:57:07.1931647Z    |             - ^ -- &&str
2020-02-16T22:57:07.1931691Z    |             | |
2020-02-16T22:57:07.1931737Z    |             | `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1931846Z    |
2020-02-16T22:57:07.1931846Z    |
2020-02-16T22:57:07.1931909Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1931964Z    |
2020-02-16T22:57:07.1932027Z LL |     let _ = c.to_owned() + &d; //~ ERROR cannot add
2020-02-16T22:57:07.1932096Z 
2020-02-16T22:57:07.1932161Z error[E0369]: cannot add `&str` to `&str`
2020-02-16T22:57:07.1932543Z   --> /checkout/src/test/ui/span/issue-39018.rs:37:15
2020-02-16T22:57:07.1932902Z    |
2020-02-16T22:57:07.1932902Z    |
2020-02-16T22:57:07.1932946Z LL |     let _ = c + d; //~ ERROR cannot add
2020-02-16T22:57:07.1933204Z    |             - ^ - &str
2020-02-16T22:57:07.1933252Z    |             | |
2020-02-16T22:57:07.1933400Z    |             | `+` cannot be used to concatenate two `&str` strings
2020-02-16T22:57:07.1933506Z    |
2020-02-16T22:57:07.1933506Z    |
2020-02-16T22:57:07.1933572Z help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
2020-02-16T22:57:07.1933650Z    |
2020-02-16T22:57:07.1933696Z LL |     let _ = c.to_owned() + d; //~ ERROR cannot add
2020-02-16T22:57:07.1933788Z 
2020-02-16T22:57:07.1933905Z error: aborting due to 14 previous errors
2020-02-16T22:57:07.1933940Z 
2020-02-16T22:57:07.1933986Z Some errors have detailed explanations: E0308, E0369.
---
2020-02-16T22:57:07.1934622Z 
2020-02-16T22:57:07.1934894Z ---- [ui] ui/traits/trait-resolution-in-overloaded-op.rs stdout ----
2020-02-16T22:57:07.1934944Z diff of stderr:
2020-02-16T22:57:07.1934972Z 
2020-02-16T22:57:07.1935191Z 5    |     - ^ - f64
2020-02-16T22:57:07.1935280Z 7    |     &T
2020-02-16T22:57:07.1935466Z -    |
2020-02-16T22:57:07.1935742Z -    = note: an implementation of `std::ops::Mul` might be missing for `&T`
2020-02-16T22:57:07.1935792Z 10 
2020-02-16T22:57:07.1935792Z 10 
2020-02-16T22:57:07.1935838Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1935898Z 12 
2020-02-16T22:57:07.1935925Z 
2020-02-16T22:57:07.1935961Z 
2020-02-16T22:57:07.1936007Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1936530Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op/trait-resolution-in-overloaded-op.stderr
2020-02-16T22:57:07.1936773Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1937051Z To only update this specific test, also pass `--test-args traits/trait-resolution-in-overloaded-op.rs`
2020-02-16T22:57:07.1937146Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1937187Z status: exit code: 1
2020-02-16T22:57:07.1937187Z status: exit code: 1
2020-02-16T22:57:07.1938021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-resolution-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op/auxiliary"
2020-02-16T22:57:07.1938350Z ------------------------------------------
2020-02-16T22:57:07.1938381Z 
2020-02-16T22:57:07.1938593Z ------------------------------------------
2020-02-16T22:57:07.1938636Z stderr:
2020-02-16T22:57:07.1938636Z stderr:
2020-02-16T22:57:07.1938862Z ------------------------------------------
2020-02-16T22:57:07.1938909Z error[E0369]: cannot multiply `f64` to `&T`
2020-02-16T22:57:07.1939215Z    |
2020-02-16T22:57:07.1939215Z    |
2020-02-16T22:57:07.1939260Z LL |     a * b //~ ERROR cannot multiply `f64` to `&T`
2020-02-16T22:57:07.1939451Z    |     - ^ - f64
2020-02-16T22:57:07.1939761Z    |     &T
2020-02-16T22:57:07.1939788Z 
2020-02-16T22:57:07.1939828Z error: aborting due to previous error
2020-02-16T22:57:07.1939873Z 
---
2020-02-16T22:57:07.1940615Z 
2020-02-16T22:57:07.1940825Z ---- [ui] ui/vec/vec-res-add.rs stdout ----
2020-02-16T22:57:07.1940869Z diff of stderr:
2020-02-16T22:57:07.1940912Z 
2020-02-16T22:57:07.1941118Z 5    |             - ^ - std::vec::Vec<R>
2020-02-16T22:57:07.1941204Z 7    |             std::vec::Vec<R>
2020-02-16T22:57:07.1941441Z -    |
2020-02-16T22:57:07.1941694Z -    = note: an implementation of `std::ops::Add` might be missing for `std::vec::Vec<R>`
2020-02-16T22:57:07.1941742Z 10 
2020-02-16T22:57:07.1941742Z 10 
2020-02-16T22:57:07.1941875Z 11 error: aborting due to previous error
2020-02-16T22:57:07.1941924Z 12 
2020-02-16T22:57:07.1941950Z 
2020-02-16T22:57:07.1941974Z 
2020-02-16T22:57:07.1942033Z The actual stderr differed from the expected stderr.
2020-02-16T22:57:07.1942337Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/vec-res-add.stderr
2020-02-16T22:57:07.1943028Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T22:57:07.1943454Z To only update this specific test, also pass `--test-args vec/vec-res-add.rs`
2020-02-16T22:57:07.1943536Z error: 1 errors occurred comparing output.
2020-02-16T22:57:07.1943599Z status: exit code: 1
2020-02-16T22:57:07.1943599Z status: exit code: 1
2020-02-16T22:57:07.1944407Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/vec/vec-res-add.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/auxiliary"
2020-02-16T22:57:07.1944755Z ------------------------------------------
2020-02-16T22:57:07.1944788Z 
2020-02-16T22:57:07.1945014Z ------------------------------------------
2020-02-16T22:57:07.1945080Z stderr:
2020-02-16T22:57:07.1945080Z stderr:
2020-02-16T22:57:07.1945300Z ------------------------------------------
2020-02-16T22:57:07.1945352Z error[E0369]: cannot add `std::vec::Vec<R>` to `std::vec::Vec<R>`
2020-02-16T22:57:07.1945662Z    |
2020-02-16T22:57:07.1945662Z    |
2020-02-16T22:57:07.1945704Z LL |     let k = i + j;
2020-02-16T22:57:07.1945942Z    |             - ^ - std::vec::Vec<R>
2020-02-16T22:57:07.1946042Z    |             std::vec::Vec<R>
2020-02-16T22:57:07.1946072Z 
2020-02-16T22:57:07.1946294Z error: aborting due to previous error
2020-02-16T22:57:07.1946319Z 
---
2020-02-16T22:57:07.1951123Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T22:57:07.1951177Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T22:57:07.1951277Z 
2020-02-16T22:57:07.1951325Z 
2020-02-16T22:57:07.1953070Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T22:57:07.1953324Z 
2020-02-16T22:57:07.1953354Z 
2020-02-16T22:57:07.1953446Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T22:57:07.1953497Z Build completed unsuccessfully in 1:06:47
2020-02-16T22:57:07.1953497Z Build completed unsuccessfully in 1:06:47
2020-02-16T22:57:07.1953562Z == clock drift check ==
2020-02-16T22:57:07.1953607Z   local time: Sun Feb 16 22:57:07 UTC 2020
2020-02-16T22:57:07.4765794Z   network time: Sun, 16 Feb 2020 22:57:07 GMT
2020-02-16T22:57:07.4768787Z == end clock drift check ==
2020-02-16T22:57:07.9429699Z 
2020-02-16T22:57:07.9526193Z ##[error]Bash exited with code '1'.
2020-02-16T22:57:07.9541941Z ##[section]Finishing: Run build
2020-02-16T22:57:07.9587150Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-16T22:57:07.9588872Z Task         : Get sources
2020-02-16T22:57:07.9588935Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T22:57:07.9588995Z Version      : 1.0.0
2020-02-16T22:57:07.9589036Z Author       : Microsoft
2020-02-16T22:57:07.9589036Z Author       : Microsoft
2020-02-16T22:57:07.9589095Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T22:57:07.9589143Z ==============================================================================
2020-02-16T22:57:08.3967187Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T22:57:08.4012448Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-16T22:57:08.4140007Z Cleaning up task key
2020-02-16T22:57:08.4140745Z Start cleaning up orphan processes.
2020-02-16T22:57:08.4246012Z Terminate orphan process: pid (5384) (python)
2020-02-16T22:57:08.4507124Z ##[section]Finishing: Finalize Job

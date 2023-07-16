plain
2020-04-16T22:43:40.1997609Z ========================== Starting Command Output ===========================
2020-04-16T22:43:40.2002676Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6e6a4f6e-e091-431d-b976-429c012f5aaa.sh
2020-04-16T22:43:40.2003266Z 
2020-04-16T22:43:40.2009849Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T22:43:40.2031961Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-16T22:43:40.2036459Z Task         : Get sources
2020-04-16T22:43:40.2036763Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T22:43:40.2037060Z Version      : 1.0.0
2020-04-16T22:43:40.2037266Z Author       : Microsoft
---
2020-04-16T22:43:41.1899302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T22:43:41.1905146Z ##[command]git config gc.auto 0
2020-04-16T22:43:41.1909167Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T22:43:41.1913140Z ##[command]git config --get-all http.proxy
2020-04-16T22:43:41.1920062Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71170/merge:refs/remotes/pull/71170/merge
---
2020-04-16T22:45:49.4059748Z  ---> f58a2bb1e753
2020-04-16T22:45:49.4061041Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-16T22:45:49.4085090Z  ---> Using cache
2020-04-16T22:45:49.4091165Z  ---> d079cc6b6db8
2020-04-16T22:45:49.4095834Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-16T22:45:49.4098127Z  ---> 4183ca46ee56
2020-04-16T22:45:49.4098383Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-16T22:45:49.4098743Z  ---> Using cache
2020-04-16T22:45:49.4099293Z  ---> 69e7f8a2a2fb
---
2020-04-16T22:45:49.4514749Z Looks like docker image is the same as before, not uploading
2020-04-16T22:45:57.0762263Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T22:45:57.1044756Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-16T22:45:57.1076301Z == clock drift check ==
2020-04-16T22:45:57.1086610Z   local time: Thu Apr 16 22:45:57 UTC 2020
2020-04-16T22:45:57.2137627Z   network time: Thu, 16 Apr 2020 22:45:57 GMT
2020-04-16T22:45:57.2165119Z Starting sccache server...
2020-04-16T22:45:57.3093500Z configure: processing command line
2020-04-16T22:45:57.3094724Z configure: 
2020-04-16T22:45:57.3096023Z configure: rust.dist-src        := False
---
2020-04-16T22:51:36.4685856Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T22:51:38.1081003Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T22:51:39.7874094Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T22:51:41.9179355Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T22:51:50.6260840Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T22:51:54.3815872Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T22:51:59.1587705Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T22:52:03.8975005Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T22:52:12.5885936Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T23:16:28.1645139Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-16T23:16:30.0929381Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-16T23:16:32.2804624Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-16T23:16:34.8886472Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-16T23:16:45.8865485Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-16T23:16:49.9253994Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-16T23:16:55.5707019Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-16T23:17:01.3784781Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-16T23:17:12.5591679Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-16T23:44:40.7038072Z .................................................................................................... 1700/9899
2020-04-16T23:44:45.7328024Z .................................................................................................... 1800/9899
2020-04-16T23:44:55.0723054Z .................................................................................................... 1900/9899
2020-04-16T23:45:03.9866548Z .......i............................................................................................ 2000/9899
2020-04-16T23:45:10.9394136Z ................................................................................................iiii 2100/9899
2020-04-16T23:45:26.7679156Z i................................................................................................... 2200/9899
2020-04-16T23:45:36.0332770Z .................................................................................................... 2400/9899
2020-04-16T23:45:37.9267048Z .................................................................................................... 2500/9899
2020-04-16T23:45:43.9893316Z .................................................................................................... 2600/9899
2020-04-16T23:46:04.5721293Z .................................................................................................... 2700/9899
---
2020-04-16T23:48:53.6294138Z .................................................................................................... 5100/9899
2020-04-16T23:49:01.8213298Z .................................................................................................... 5200/9899
2020-04-16T23:49:06.5745492Z ..................i................................................................................. 5300/9899
2020-04-16T23:49:16.9925050Z ........i........................................................................................... 5400/9899
2020-04-16T23:49:22.7100043Z ........ii.ii........i...i.......................................................................... 5500/9899
2020-04-16T23:49:30.4519480Z ......................................................i............................................. 5700/9899
2020-04-16T23:49:40.5730519Z ..........................................................................ii........................ 5800/9899
2020-04-16T23:49:47.6332362Z .............i...................................................................................... 5900/9899
2020-04-16T23:49:53.0675384Z .................................................................................................... 6000/9899
2020-04-16T23:49:53.0675384Z .................................................................................................... 6000/9899
2020-04-16T23:50:04.3377647Z .................................................................................................... 6100/9899
2020-04-16T23:50:15.5690584Z .......ii...i...ii..........i....................................................................... 6200/9899
2020-04-16T23:50:31.5367954Z .................................................................................................... 6400/9899
2020-04-16T23:50:37.9947511Z .................................................................................................... 6500/9899
2020-04-16T23:50:37.9947511Z .................................................................................................... 6500/9899
2020-04-16T23:50:55.8691920Z .....................................i..ii.......................................................... 6600/9899
2020-04-16T23:51:18.7770243Z .................................................................................................... 6800/9899
2020-04-16T23:51:20.8842790Z ......................................i............................................................. 6900/9899
2020-04-16T23:51:23.0814376Z .................................................................................................... 7000/9899
2020-04-16T23:51:25.2213647Z ..............................................................................i..................... 7100/9899
---
2020-04-16T23:53:07.5344167Z .................................................................................................... 7800/9899
2020-04-16T23:53:11.8811977Z .................................................................................................... 7900/9899
2020-04-16T23:53:18.8064066Z .................................................................................................... 8000/9899
2020-04-16T23:53:25.2186285Z ............................................i....................................................... 8100/9899
2020-04-16T23:53:35.4749158Z ............................................................................................iiiiii.i 8200/9899
2020-04-16T23:53:41.9113941Z iiii.i.............................................................................................. 8300/9899
2020-04-16T23:53:56.2098374Z .................................................................................................... 8500/9899
2020-04-16T23:54:06.4537336Z .................................................................................................... 8600/9899
2020-04-16T23:54:21.7142720Z .................................................................................................... 8700/9899
2020-04-16T23:54:29.0591153Z .................................................................................................... 8800/9899
---
2020-04-16T23:56:26.5979229Z ---- [ui] ui/object-safety/object-safety-by-value-self-use.rs stdout ----
2020-04-16T23:56:26.5979776Z 
2020-04-16T23:56:26.5980109Z error: ui test compiled successfully!
2020-04-16T23:56:26.5980461Z status: exit code: 0
2020-04-16T23:56:26.5983502Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-safety/object-safety-by-value-self-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-safety/object-safety-by-value-self-use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-safety/object-safety-by-value-self-use/auxiliary"
2020-04-16T23:56:26.5986509Z ------------------------------------------
2020-04-16T23:56:26.5986699Z 
2020-04-16T23:56:26.5987092Z ------------------------------------------
2020-04-16T23:56:26.5987326Z stderr:
---
2020-04-16T23:56:26.5989448Z 
2020-04-16T23:56:26.5989670Z 45 error[E0382]: borrow of moved value: `x`
2020-04-16T23:56:26.5990168Z 46   --> $DIR/borrow-after-move.rs:39:24
2020-04-16T23:56:26.5990405Z 47    |
2020-04-16T23:56:26.5990868Z + LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.5991825Z +    |             - move occurs because `x` has type `std::boxed::Box<str>`, which does not implement the `Copy` trait
2020-04-16T23:56:26.5992273Z 48 LL |         x.foo();
2020-04-16T23:56:26.5992730Z 49    |         - value moved here
2020-04-16T23:56:26.5992975Z 50 LL |         println!("{}", &x);
2020-04-16T23:56:26.5993623Z -    |                        ^^ value borrowed here after partial move
2020-04-16T23:56:26.5994025Z -    |
2020-04-16T23:56:26.5994025Z -    |
2020-04-16T23:56:26.5994574Z -    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.5995003Z +    |                        ^^ value borrowed here after move
2020-04-16T23:56:26.5995447Z 55 error: aborting due to 5 previous errors
2020-04-16T23:56:26.5995672Z 56 
2020-04-16T23:56:26.5995779Z 
2020-04-16T23:56:26.5995880Z 
2020-04-16T23:56:26.5995880Z 
2020-04-16T23:56:26.5996094Z The actual stderr differed from the expected stderr.
2020-04-16T23:56:26.5996846Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move/borrow-after-move.stderr
2020-04-16T23:56:26.5997519Z To update references, rerun the tests and pass the `--bless` flag
2020-04-16T23:56:26.5998152Z To only update this specific test, also pass `--test-args unsized-locals/borrow-after-move.rs`
2020-04-16T23:56:26.5998634Z error: 1 errors occurred comparing output.
2020-04-16T23:56:26.5998876Z status: exit code: 1
2020-04-16T23:56:26.5998876Z status: exit code: 1
2020-04-16T23:56:26.6000969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/borrow-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move/auxiliary"
2020-04-16T23:56:26.6002622Z ------------------------------------------
2020-04-16T23:56:26.6002801Z 
2020-04-16T23:56:26.6003176Z ------------------------------------------
2020-04-16T23:56:26.6003405Z stderr:
---
2020-04-16T23:56:26.6005917Z LL |         drop_unsized(y);
2020-04-16T23:56:26.6006143Z LL |         println!("{}", &x);
2020-04-16T23:56:26.6006457Z    |                        ^^ value borrowed here after partial move
2020-04-16T23:56:26.6006705Z    |
2020-04-16T23:56:26.6007002Z    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6007502Z error[E0382]: borrow of moved value: `y`
2020-04-16T23:56:26.6008066Z   --> /checkout/src/test/ui/unsized-locals/borrow-after-move.rs:22:24
2020-04-16T23:56:26.6008328Z    |
2020-04-16T23:56:26.6008516Z LL |         let y = *x;
---
2020-04-16T23:56:26.6014700Z LL |         y.foo();
2020-04-16T23:56:26.6015063Z LL |         println!("{}", &x);
2020-04-16T23:56:26.6015365Z    |                        ^^ value borrowed here after partial move
2020-04-16T23:56:26.6015629Z    |
2020-04-16T23:56:26.6016055Z    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6016703Z error[E0382]: borrow of moved value: `y`
2020-04-16T23:56:26.6017473Z   --> /checkout/src/test/ui/unsized-locals/borrow-after-move.rs:32:24
2020-04-16T23:56:26.6017745Z    |
2020-04-16T23:56:26.6017935Z LL |         let y = *x;
---
2020-04-16T23:56:26.6020822Z 
2020-04-16T23:56:26.6021033Z error[E0382]: borrow of moved value: `x`
2020-04-16T23:56:26.6021953Z   --> /checkout/src/test/ui/unsized-locals/borrow-after-move.rs:39:24
2020-04-16T23:56:26.6022242Z    |
2020-04-16T23:56:26.6022653Z LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6023582Z    |             - move occurs because `x` has type `std::boxed::Box<str>`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6024142Z LL |         x.foo();
2020-04-16T23:56:26.6024579Z    |         - value moved here
2020-04-16T23:56:26.6024977Z LL |         println!("{}", &x);
2020-04-16T23:56:26.6025280Z    |                        ^^ value borrowed here after move
2020-04-16T23:56:26.6025823Z error: aborting due to 5 previous errors
2020-04-16T23:56:26.6025997Z 
2020-04-16T23:56:26.6026708Z For more information about this error, try `rustc --explain E0382`.
2020-04-16T23:56:26.6026929Z 
---
2020-04-16T23:56:26.6029102Z 
2020-04-16T23:56:26.6029288Z 38 LL |         y.foo();
2020-04-16T23:56:26.6029697Z 39    |         ^ value used here after move
2020-04-16T23:56:26.6029901Z 40 
2020-04-16T23:56:26.6030569Z - error[E0382]: use of moved value: `*x`
2020-04-16T23:56:26.6031698Z + error[E0382]: use of moved value: `x`
2020-04-16T23:56:26.6032440Z 42   --> $DIR/double-move.rs:45:9
2020-04-16T23:56:26.6032652Z 43    |
2020-04-16T23:56:26.6032857Z 44 LL |         let _y = *x;
2020-04-16T23:56:26.6033410Z 45    |                  -- value moved here
2020-04-16T23:56:26.6033672Z 46 LL |         x.foo();
2020-04-16T23:56:26.6034094Z -    |         ^ value used here after move
2020-04-16T23:56:26.6034094Z -    |         ^ value used here after move
2020-04-16T23:56:26.6034381Z +    |         ^ value used here after partial move
2020-04-16T23:56:26.6034601Z 48    |
2020-04-16T23:56:26.6034928Z 49    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6035331Z 
2020-04-16T23:56:26.6035331Z 
2020-04-16T23:56:26.6035569Z 51 error[E0382]: use of moved value: `*x`
2020-04-16T23:56:26.6036039Z 52   --> $DIR/double-move.rs:51:18
2020-04-16T23:56:26.6036242Z 53    |
2020-04-16T23:56:26.6036494Z + LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6037218Z +    |             - move occurs because `x` has type `std::boxed::Box<str>`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6037627Z 54 LL |         x.foo();
2020-04-16T23:56:26.6038050Z 55    |         - value moved here
2020-04-16T23:56:26.6038408Z 56 LL |         let _y = *x;
2020-04-16T23:56:26.6038784Z 57    |                  ^^ value used here after move
2020-04-16T23:56:26.6039215Z -    |
2020-04-16T23:56:26.6039215Z -    |
2020-04-16T23:56:26.6039762Z -    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6104459Z 61 error: aborting due to 6 previous errors
2020-04-16T23:56:26.6104683Z 62 
2020-04-16T23:56:26.6104996Z 
2020-04-16T23:56:26.6105116Z 
2020-04-16T23:56:26.6105116Z 
2020-04-16T23:56:26.6105354Z The actual stderr differed from the expected stderr.
2020-04-16T23:56:26.6106241Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move/double-move.stderr
2020-04-16T23:56:26.6106903Z To update references, rerun the tests and pass the `--bless` flag
2020-04-16T23:56:26.6107542Z To only update this specific test, also pass `--test-args unsized-locals/double-move.rs`
2020-04-16T23:56:26.6107999Z error: 1 errors occurred comparing output.
2020-04-16T23:56:26.6108261Z status: exit code: 1
2020-04-16T23:56:26.6108261Z status: exit code: 1
2020-04-16T23:56:26.6113612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/double-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move/auxiliary"
2020-04-16T23:56:26.6115463Z ------------------------------------------
2020-04-16T23:56:26.6115646Z 
2020-04-16T23:56:26.6116061Z ------------------------------------------
2020-04-16T23:56:26.6116275Z stderr:
2020-04-16T23:56:26.6116275Z stderr:
2020-04-16T23:56:26.6116679Z ------------------------------------------
2020-04-16T23:56:26.6116980Z error[E0382]: use of moved value: `y`
2020-04-16T23:56:26.6117524Z   --> /checkout/src/test/ui/unsized-locals/double-move.rs:20:22
2020-04-16T23:56:26.6117775Z    |
2020-04-16T23:56:26.6117967Z LL |         let y = *x;
2020-04-16T23:56:26.6118550Z    |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6118897Z LL |         drop_unsized(y);
2020-04-16T23:56:26.6119577Z    |                      - value moved here
2020-04-16T23:56:26.6119881Z LL |         drop_unsized(y); //~ERROR use of moved value
2020-04-16T23:56:26.6120194Z    |                      ^ value used here after move
2020-04-16T23:56:26.6120614Z error[E0382]: use of moved value: `x`
2020-04-16T23:56:26.6121152Z   --> /checkout/src/test/ui/unsized-locals/double-move.rs:26:22
2020-04-16T23:56:26.6121402Z    |
2020-04-16T23:56:26.6121402Z    |
2020-04-16T23:56:26.6121593Z LL |         let _y = *x;
2020-04-16T23:56:26.6122019Z    |                  -- value moved here
2020-04-16T23:56:26.6122314Z LL |         drop_unsized(x); //~ERROR use of moved value
2020-04-16T23:56:26.6122658Z    |                      ^ value used here after partial move
2020-04-16T23:56:26.6122892Z    |
2020-04-16T23:56:26.6123190Z    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6123479Z 
2020-04-16T23:56:26.6123686Z error[E0382]: use of moved value: `*x`
2020-04-16T23:56:26.6124495Z    |
2020-04-16T23:56:26.6124722Z LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6124722Z LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6125439Z    |             - move occurs because `x` has type `std::boxed::Box<str>`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6125866Z LL |         drop_unsized(x);
2020-04-16T23:56:26.6126300Z    |                      - value moved here
2020-04-16T23:56:26.6126686Z LL |         let _y = *x; //~ERROR use of moved value
2020-04-16T23:56:26.6127018Z    |                  ^^ value used here after move
2020-04-16T23:56:26.6127412Z error[E0382]: use of moved value: `y`
2020-04-16T23:56:26.6127979Z   --> /checkout/src/test/ui/unsized-locals/double-move.rs:39:9
2020-04-16T23:56:26.6128245Z    |
2020-04-16T23:56:26.6128417Z LL |         let y = *x;
2020-04-16T23:56:26.6128417Z LL |         let y = *x;
2020-04-16T23:56:26.6128996Z    |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6129354Z LL |         y.foo();
2020-04-16T23:56:26.6129746Z    |         - value moved here
2020-04-16T23:56:26.6130011Z LL |         y.foo(); //~ERROR use of moved value
2020-04-16T23:56:26.6130305Z    |         ^ value used here after move
2020-04-16T23:56:26.6130680Z error[E0382]: use of moved value: `x`
2020-04-16T23:56:26.6131433Z   --> /checkout/src/test/ui/unsized-locals/double-move.rs:45:9
2020-04-16T23:56:26.6131729Z    |
2020-04-16T23:56:26.6131729Z    |
2020-04-16T23:56:26.6131909Z LL |         let _y = *x;
2020-04-16T23:56:26.6132578Z    |                  -- value moved here
2020-04-16T23:56:26.6133052Z LL |         x.foo(); //~ERROR use of moved value
2020-04-16T23:56:26.6133338Z    |         ^ value used here after partial move
2020-04-16T23:56:26.6133539Z    |
2020-04-16T23:56:26.6133980Z    = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6134255Z 
2020-04-16T23:56:26.6134585Z error[E0382]: use of moved value: `*x`
2020-04-16T23:56:26.6135470Z    |
2020-04-16T23:56:26.6135697Z LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6135697Z LL |         let x = "hello".to_owned().into_boxed_str();
2020-04-16T23:56:26.6136635Z    |             - move occurs because `x` has type `std::boxed::Box<str>`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6137235Z LL |         x.foo();
2020-04-16T23:56:26.6137835Z    |         - value moved here
2020-04-16T23:56:26.6138126Z LL |         let _y = *x; //~ERROR use of moved value
2020-04-16T23:56:26.6138447Z    |                  ^^ value used here after move
2020-04-16T23:56:26.6138829Z error: aborting due to 6 previous errors
2020-04-16T23:56:26.6139002Z 
2020-04-16T23:56:26.6145202Z For more information about this error, try `rustc --explain E0382`.
2020-04-16T23:56:26.6145438Z 
2020-04-16T23:56:26.6145438Z 
2020-04-16T23:56:26.6146253Z ------------------------------------------
2020-04-16T23:56:26.6147184Z 
2020-04-16T23:56:26.6147437Z 
2020-04-16T23:56:26.6147962Z ---- [ui] ui/unsized-locals/unsized-exprs2.rs stdout ----
2020-04-16T23:56:26.6148197Z diff of stderr:
2020-04-16T23:56:26.6148320Z 
2020-04-16T23:56:26.6148754Z 1 error[E0508]: cannot move out of type `[u8]`, a non-copy slice
2020-04-16T23:56:26.6149605Z +   --> $DIR/unsized-exprs2.rs:22:5
2020-04-16T23:56:26.6149810Z 3    |
2020-04-16T23:56:26.6149810Z 3    |
2020-04-16T23:56:26.6149992Z 4 LL |     udrop::<[u8]>(foo()[..]);
2020-04-16T23:56:26.6151006Z -    |                   |
2020-04-16T23:56:26.6151413Z -    |                   cannot move out of here
2020-04-16T23:56:26.6151991Z -    |                   move occurs because value has type `[u8]`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6152343Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^
---
2020-04-16T23:56:26.6154232Z 11 
2020-04-16T23:56:26.6154329Z 
2020-04-16T23:56:26.6154419Z 
2020-04-16T23:56:26.6154608Z The actual stderr differed from the expected stderr.
2020-04-16T23:56:26.6155502Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs2/unsized-exprs2.stderr
2020-04-16T23:56:26.6156156Z To update references, rerun the tests and pass the `--bless` flag
2020-04-16T23:56:26.6156720Z To only update this specific test, also pass `--test-args unsized-locals/unsized-exprs2.rs`
2020-04-16T23:56:26.6157327Z error: 1 errors occurred comparing output.
2020-04-16T23:56:26.6157544Z status: exit code: 1
2020-04-16T23:56:26.6157544Z status: exit code: 1
2020-04-16T23:56:26.6159639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/unsized-exprs2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs2/auxiliary"
2020-04-16T23:56:26.6162119Z ------------------------------------------
2020-04-16T23:56:26.6162284Z 
2020-04-16T23:56:26.6162624Z ------------------------------------------
2020-04-16T23:56:26.6162831Z stderr:
2020-04-16T23:56:26.6162831Z stderr:
2020-04-16T23:56:26.6163181Z ------------------------------------------
2020-04-16T23:56:26.6163638Z error[E0508]: cannot move out of type `[u8]`, a non-copy slice
2020-04-16T23:56:26.6164403Z    |
2020-04-16T23:56:26.6164403Z    |
2020-04-16T23:56:26.6164579Z LL |     udrop::<[u8]>(foo()[..]);
2020-04-16T23:56:26.6164991Z    |     |
2020-04-16T23:56:26.6165164Z    |     cannot move out of here
2020-04-16T23:56:26.6165481Z    |     move occurs because value has type `[u8]`, which does not implement the `Copy` trait
2020-04-16T23:56:26.6165715Z 
---
2020-04-16T23:56:26.6171448Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-16T23:56:26.6171854Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-16T23:56:26.6172064Z 
2020-04-16T23:56:26.6172153Z 
2020-04-16T23:56:26.6175681Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-16T23:56:26.6178054Z 
2020-04-16T23:56:26.6178147Z 
2020-04-16T23:56:26.6178668Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-16T23:56:26.6179007Z Build completed unsuccessfully in 1:08:42
2020-04-16T23:56:26.6179007Z Build completed unsuccessfully in 1:08:42
2020-04-16T23:56:26.6179232Z == clock drift check ==
2020-04-16T23:56:26.6179472Z   local time: Thu Apr 16 23:56:26 UTC 2020
2020-04-16T23:56:26.7486369Z   network time: Thu, 16 Apr 2020 23:56:26 GMT
2020-04-16T23:56:27.2081528Z 
2020-04-16T23:56:27.2081528Z 
2020-04-16T23:56:27.2178984Z ##[error]Bash exited with code '1'.
2020-04-16T23:56:27.2197023Z ##[section]Finishing: Run build
2020-04-16T23:56:27.2254501Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-16T23:56:27.2259943Z Task         : Get sources
2020-04-16T23:56:27.2260290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T23:56:27.2260641Z Version      : 1.0.0
2020-04-16T23:56:27.2260865Z Author       : Microsoft
2020-04-16T23:56:27.2260865Z Author       : Microsoft
2020-04-16T23:56:27.2261223Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T23:56:27.2261650Z ==============================================================================
2020-04-16T23:56:27.5869672Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T23:56:27.5920119Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-16T23:56:27.6022971Z Cleaning up task key
2020-04-16T23:56:27.6024265Z Start cleaning up orphan processes.
2020-04-16T23:56:27.6262478Z Terminate orphan process: pid (3439) (python)
2020-04-16T23:56:27.6502866Z ##[section]Finishing: Finalize Job

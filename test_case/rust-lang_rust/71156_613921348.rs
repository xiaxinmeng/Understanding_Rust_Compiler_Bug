plain
2020-04-15T08:16:11.0884592Z ========================== Starting Command Output ===========================
2020-04-15T08:16:11.0888039Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e41522b4-1e15-40ae-897a-e263845ef6fb.sh
2020-04-15T08:16:11.0888427Z 
2020-04-15T08:16:11.0892969Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T08:16:11.0911222Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71156/merge to s
2020-04-15T08:16:11.0914411Z Task         : Get sources
2020-04-15T08:16:11.0914695Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T08:16:11.0914970Z Version      : 1.0.0
2020-04-15T08:16:11.0915178Z Author       : Microsoft
---
2020-04-15T08:16:12.3451401Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T08:16:12.3462232Z ##[command]git config gc.auto 0
2020-04-15T08:16:12.3468806Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T08:16:12.3474877Z ##[command]git config --get-all http.proxy
2020-04-15T08:16:12.3487242Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71156/merge:refs/remotes/pull/71156/merge
---
2020-04-15T08:18:39.4500880Z  ---> f58a2bb1e753
2020-04-15T08:18:39.4501605Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-15T08:18:39.4502452Z  ---> Using cache
2020-04-15T08:18:39.4504098Z  ---> d079cc6b6db8
2020-04-15T08:18:39.4505122Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-15T08:18:39.4506127Z  ---> 4183ca46ee56
2020-04-15T08:18:39.4506344Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-15T08:18:39.4506683Z  ---> Using cache
2020-04-15T08:18:39.4506994Z  ---> 69e7f8a2a2fb
---
2020-04-15T08:18:39.4931074Z Looks like docker image is the same as before, not uploading
2020-04-15T08:18:45.7399461Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T08:18:45.7668152Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T08:18:45.7701925Z == clock drift check ==
2020-04-15T08:18:45.7711486Z   local time: Wed Apr 15 08:18:45 UTC 2020
2020-04-15T08:18:46.0636874Z   network time: Wed, 15 Apr 2020 08:18:46 GMT
2020-04-15T08:18:46.0666704Z Starting sccache server...
2020-04-15T08:18:46.1507309Z configure: processing command line
2020-04-15T08:18:46.1507741Z configure: 
2020-04-15T08:18:46.1513279Z configure: rust.dist-src        := False
---
2020-04-15T08:23:33.6987438Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T08:23:35.0175132Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T08:23:36.4546710Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T08:23:37.2107582Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T08:23:45.4627500Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T08:23:47.2505789Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T08:23:51.2101172Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T08:23:54.9471384Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T08:24:04.0284864Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T08:44:07.1440190Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T08:44:08.7531647Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T08:44:10.6109090Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T08:44:11.9701327Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T08:44:21.8895667Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T08:44:24.3950807Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T08:44:29.1643844Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T08:44:34.0446702Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T08:44:44.1435595Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T09:07:09.3252141Z .................................................................................................... 1800/9908
2020-04-15T09:07:16.2937149Z .................................................................................................... 1900/9908
2020-04-15T09:07:23.2552137Z .....Fi............................................................................................. 2000/9908
2020-04-15T09:07:28.7483709Z .................................................................................................... 2100/9908
2020-04-15T09:07:40.6229662Z .......iiiii........................................................................................ 2200/9908
2020-04-15T09:07:48.5205617Z .................................................................................................... 2400/9908
2020-04-15T09:07:50.4507200Z .................................................................................................... 2500/9908
2020-04-15T09:07:54.7677220Z .................................................................................................... 2600/9908
2020-04-15T09:08:11.7081728Z .................................................................................................... 2700/9908
---
2020-04-15T09:10:29.9023038Z .................................................................................................... 5100/9908
2020-04-15T09:10:37.6241605Z .................................................................................................... 5200/9908
2020-04-15T09:10:41.9968042Z .............................i...................................................................... 5300/9908
2020-04-15T09:10:50.6139807Z ...................i................................................................................ 5400/9908
2020-04-15T09:10:55.9465497Z ...................ii.ii........i...i............................................................... 5500/9908
2020-04-15T09:11:02.7687985Z .................................................................i.................................. 5700/9908
2020-04-15T09:11:11.1069466Z .....................................................................................ii............. 5800/9908
2020-04-15T09:11:17.8294392Z ........................i........................................................................... 5900/9908
2020-04-15T09:11:23.3353659Z .................................................................................................... 6000/9908
2020-04-15T09:11:23.3353659Z .................................................................................................... 6000/9908
2020-04-15T09:11:33.8011086Z .................................................................................................... 6100/9908
2020-04-15T09:11:42.8508854Z ..................ii...i..ii...........i............................................................ 6200/9908
2020-04-15T09:11:56.4114320Z .................................................................................................... 6400/9908
2020-04-15T09:11:59.3467890Z .................................................................................................... 6500/9908
2020-04-15T09:12:08.5690693Z .................................................i.ii............................................... 6600/9908
2020-04-15T09:12:16.1836107Z .................................................................................................... 6700/9908
---
2020-04-15T09:14:04.9180229Z .................................................................................................... 7900/9908
2020-04-15T09:14:10.6348429Z .................................................................................................... 8000/9908
2020-04-15T09:14:15.7695934Z .....................................................i.............................................. 8100/9908
2020-04-15T09:14:25.0057708Z .................................................................................................... 8200/9908
2020-04-15T09:14:29.6185823Z .iiiiii.iiiii.i..................................................................................... 8300/9908
2020-04-15T09:14:40.7629054Z .................................................................................................... 8500/9908
2020-04-15T09:14:47.2936380Z .................................................................................................... 8600/9908
2020-04-15T09:14:59.2872580Z .................................................................................................... 8700/9908
2020-04-15T09:15:04.8969894Z ..........................................................................F......................... 8800/9908
---
2020-04-15T09:16:40.2010342Z +    |         ^
2020-04-15T09:16:40.2010580Z +    | 
2020-04-15T09:16:40.2010844Z +   ::: $DIR/main.rs:5:5
2020-04-15T09:16:40.2011114Z +    |
2020-04-15T09:16:40.2011362Z + LL |     underscore!();
2020-04-15T09:16:40.2012199Z +    |
2020-04-15T09:16:40.2012199Z +    |
2020-04-15T09:16:40.2012960Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2014412Z +    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-15T09:16:40.2014950Z + 
2020-04-15T09:16:40.2015254Z 1 error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2015788Z 2   --> $DIR/underscore.rs:8:9
---
2020-04-15T09:16:40.2019655Z 16 
2020-04-15T09:16:40.2019847Z 
2020-04-15T09:16:40.2020033Z 
2020-04-15T09:16:40.2020333Z The actual stderr differed from the expected stderr.
2020-04-15T09:16:40.2021110Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main/main.stderr
2020-04-15T09:16:40.2021988Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T09:16:40.2022771Z To only update this specific test, also pass `--test-args cross/cross-file-errors/main.rs`
2020-04-15T09:16:40.2023386Z error: 1 errors occurred comparing output.
2020-04-15T09:16:40.2023689Z status: exit code: 1
2020-04-15T09:16:40.2023689Z status: exit code: 1
2020-04-15T09:16:40.2025466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross/cross-file-errors/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main/auxiliary"
2020-04-15T09:16:40.2027049Z ------------------------------------------
2020-04-15T09:16:40.2027307Z 
2020-04-15T09:16:40.2027742Z ------------------------------------------
2020-04-15T09:16:40.2028047Z stderr:
---
2020-04-15T09:16:40.2031354Z    |
2020-04-15T09:16:40.2031601Z LL |     underscore!();
2020-04-15T09:16:40.2032085Z    |     -------------- in this macro invocation
2020-04-15T09:16:40.2032390Z    |
2020-04-15T09:16:40.2033012Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2034212Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-15T09:16:40.2034567Z 
2020-04-15T09:16:40.2034860Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2035462Z   --> /checkout/src/test/ui/cross/cross-file-errors/underscore.rs:8:9
---
2020-04-15T09:16:40.2041603Z 
2020-04-15T09:16:40.2041999Z ---- [ui] ui/issues/issue-34334.rs stdout ----
2020-04-15T09:16:40.2042363Z diff of stderr:
2020-04-15T09:16:40.2042550Z 
2020-04-15T09:16:40.2042796Z 19 LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2043346Z 21 
2020-04-15T09:16:40.2043603Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2044122Z +   --> $DIR/issue-34334.rs:2:23
2020-04-15T09:16:40.2044381Z +    |
2020-04-15T09:16:40.2044381Z +    |
2020-04-15T09:16:40.2044648Z + LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2045163Z +    |
2020-04-15T09:16:40.2045163Z +    |
2020-04-15T09:16:40.2045707Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2049950Z + 
2020-04-15T09:16:40.2050114Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2050605Z +   --> $DIR/issue-34334.rs:2:26
2020-04-15T09:16:40.2050779Z +    |
2020-04-15T09:16:40.2050779Z +    |
2020-04-15T09:16:40.2050937Z + LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2051275Z +    |
2020-04-15T09:16:40.2051275Z +    |
2020-04-15T09:16:40.2051721Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2052240Z + 
2020-04-15T09:16:40.2052407Z 22 error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2052761Z 23   --> $DIR/issue-34334.rs:2:23
2020-04-15T09:16:40.2052920Z 24    |
2020-04-15T09:16:40.2052920Z 24    |
2020-04-15T09:16:40.2053004Z 
2020-04-15T09:16:40.2053225Z 55 LL |     let sr2: Vec<(u32, _, _)> = sr.iter().map(|(faction, th_sender, th_receiver)| {}).collect();
2020-04-15T09:16:40.2053543Z 56    |                                    ^^^^ method not found in `()`
2020-04-15T09:16:40.2054187Z - error: aborting due to 8 previous errors
2020-04-15T09:16:40.2054412Z + error: aborting due to 10 previous errors
2020-04-15T09:16:40.2054557Z 59 
2020-04-15T09:16:40.2054926Z - Some errors have detailed explanations: E0070, E0308, E0423, E0599.
2020-04-15T09:16:40.2054926Z - Some errors have detailed explanations: E0070, E0308, E0423, E0599.
2020-04-15T09:16:40.2055233Z + Some errors have detailed explanations: E0070, E0308, E0423, E0599, E0658.
2020-04-15T09:16:40.2055669Z 61 For more information about an error, try `rustc --explain E0070`.
2020-04-15T09:16:40.2055842Z 62 
2020-04-15T09:16:40.2055919Z 
2020-04-15T09:16:40.2056006Z 
2020-04-15T09:16:40.2056156Z The actual stderr differed from the expected stderr.
2020-04-15T09:16:40.2056644Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334/issue-34334.stderr
2020-04-15T09:16:40.2057112Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T09:16:40.2057541Z To only update this specific test, also pass `--test-args issues/issue-34334.rs`
2020-04-15T09:16:40.2057853Z error: 1 errors occurred comparing output.
2020-04-15T09:16:40.2058047Z status: exit code: 1
2020-04-15T09:16:40.2058047Z status: exit code: 1
2020-04-15T09:16:40.2059442Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34334.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334/auxiliary"
2020-04-15T09:16:40.2060554Z ------------------------------------------
2020-04-15T09:16:40.2060681Z 
2020-04-15T09:16:40.2060972Z ------------------------------------------
2020-04-15T09:16:40.2061311Z stderr:
2020-04-15T09:16:40.2061311Z stderr:
2020-04-15T09:16:40.2061639Z ------------------------------------------
2020-04-15T09:16:40.2062006Z error: expected one of `,` or `>`, found `=`
2020-04-15T09:16:40.2062633Z    |
2020-04-15T09:16:40.2062633Z    |
2020-04-15T09:16:40.2062820Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2063241Z    |         ---                 ^ expected one of `,` or `>`
2020-04-15T09:16:40.2063514Z    |         | |
2020-04-15T09:16:40.2063742Z    |         | help: use `=` if you meant to assign
2020-04-15T09:16:40.2063983Z    |         while parsing the type for `sr`
2020-04-15T09:16:40.2064300Z error[E0423]: expected value, found struct `Vec`
2020-04-15T09:16:40.2064972Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:13
2020-04-15T09:16:40.2069600Z    |
2020-04-15T09:16:40.2069600Z    |
2020-04-15T09:16:40.2069788Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2070058Z    |             ^^^ did you mean `Vec { /* fields */ }`?
2020-04-15T09:16:40.2070404Z error[E0423]: expected value, found builtin type `u32`
2020-04-15T09:16:40.2071008Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:18
2020-04-15T09:16:40.2071199Z    |
2020-04-15T09:16:40.2071199Z    |
2020-04-15T09:16:40.2071373Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2071739Z 
2020-04-15T09:16:40.2071925Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2072364Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:23
2020-04-15T09:16:40.2072567Z    |
2020-04-15T09:16:40.2072567Z    |
2020-04-15T09:16:40.2072739Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2073104Z    |
2020-04-15T09:16:40.2073104Z    |
2020-04-15T09:16:40.2075246Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2075837Z 
2020-04-15T09:16:40.2076020Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2076573Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:26
2020-04-15T09:16:40.2076779Z    |
2020-04-15T09:16:40.2076779Z    |
2020-04-15T09:16:40.2076952Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2077323Z    |
2020-04-15T09:16:40.2077323Z    |
2020-04-15T09:16:40.2077801Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2078364Z 
2020-04-15T09:16:40.2078547Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2078986Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:23
2020-04-15T09:16:40.2079189Z    |
2020-04-15T09:16:40.2079189Z    |
2020-04-15T09:16:40.2079362Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2079745Z 
2020-04-15T09:16:40.2079940Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2080374Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:26
2020-04-15T09:16:40.2080568Z    |
2020-04-15T09:16:40.2080568Z    |
2020-04-15T09:16:40.2080753Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2081143Z 
2020-04-15T09:16:40.2081290Z error[E0308]: mismatched types
2020-04-15T09:16:40.2081715Z   --> /checkout/src/test/ui/issues/issue-34334.rs:2:31
2020-04-15T09:16:40.2081907Z    |
2020-04-15T09:16:40.2081907Z    |
2020-04-15T09:16:40.2082078Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2082412Z    |                               ^^^^^^ expected `bool`, found struct `std::vec::Vec`
2020-04-15T09:16:40.2082826Z    = note: expected type `bool`
2020-04-15T09:16:40.2083064Z             found struct `std::vec::Vec<_>`
2020-04-15T09:16:40.2083707Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-15T09:16:40.2083911Z 
2020-04-15T09:16:40.2083911Z 
2020-04-15T09:16:40.2084226Z error[E0070]: invalid left-hand side of assignment
2020-04-15T09:16:40.2084882Z    |
2020-04-15T09:16:40.2084882Z    |
2020-04-15T09:16:40.2085045Z LL |     let sr: Vec<(u32, _, _) = vec![];
2020-04-15T09:16:40.2085523Z    |             |
2020-04-15T09:16:40.2085742Z    |             cannot assign to this expression
2020-04-15T09:16:40.2085892Z 
2020-04-15T09:16:40.2086086Z error[E0599]: no method named `iter` found for unit type `()` in the current scope
2020-04-15T09:16:40.2086086Z error[E0599]: no method named `iter` found for unit type `()` in the current scope
2020-04-15T09:16:40.2086510Z   --> /checkout/src/test/ui/issues/issue-34334.rs:10:36
2020-04-15T09:16:40.2086691Z    |
2020-04-15T09:16:40.2086911Z LL |     let sr2: Vec<(u32, _, _)> = sr.iter().map(|(faction, th_sender, th_receiver)| {}).collect();
2020-04-15T09:16:40.2087209Z    |                                    ^^^^ method not found in `()`
2020-04-15T09:16:40.2087517Z error: aborting due to 10 previous errors
2020-04-15T09:16:40.2087641Z 
2020-04-15T09:16:40.2087836Z Some errors have detailed explanations: E0070, E0308, E0423, E0599, E0658.
2020-04-15T09:16:40.2088275Z For more information about an error, try `rustc --explain E0070`.
---
2020-04-15T09:16:40.2090332Z +    |
2020-04-15T09:16:40.2090492Z + LL |     let _: usize = foo(_, _);
2020-04-15T09:16:40.2090668Z +    |                        ^
2020-04-15T09:16:40.2090800Z +    |
2020-04-15T09:16:40.2091234Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2091732Z + 
2020-04-15T09:16:40.2091907Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2092312Z +   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:10:27
2020-04-15T09:16:40.2092498Z +    |
2020-04-15T09:16:40.2092498Z +    |
2020-04-15T09:16:40.2092662Z + LL |     let _: usize = foo(_, _);
2020-04-15T09:16:40.2092841Z +    |                           ^
2020-04-15T09:16:40.2092977Z +    |
2020-04-15T09:16:40.2093405Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2093896Z + 
2020-04-15T09:16:40.2094177Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2094592Z +   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:13:18
2020-04-15T09:16:40.2094778Z +    |
2020-04-15T09:16:40.2094778Z +    |
2020-04-15T09:16:40.2094931Z + LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2095220Z +    |
2020-04-15T09:16:40.2095220Z +    |
2020-04-15T09:16:40.2095651Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2096152Z + 
2020-04-15T09:16:40.2096319Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2096722Z +   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:13:21
2020-04-15T09:16:40.2096922Z +    |
2020-04-15T09:16:40.2096922Z +    |
2020-04-15T09:16:40.2097061Z + LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2097353Z +    |
2020-04-15T09:16:40.2097353Z +    |
2020-04-15T09:16:40.2097975Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2098478Z + 
2020-04-15T09:16:40.2098639Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2099115Z +   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:16:27
2020-04-15T09:16:40.2099314Z +    |
2020-04-15T09:16:40.2099314Z +    |
2020-04-15T09:16:40.2099470Z + LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2099804Z +    |
2020-04-15T09:16:40.2099804Z +    |
2020-04-15T09:16:40.2100268Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2100775Z + 
2020-04-15T09:16:40.2100935Z + error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2101347Z +   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:16:30
2020-04-15T09:16:40.2101546Z +    |
2020-04-15T09:16:40.2101546Z +    |
2020-04-15T09:16:40.2101701Z + LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2102044Z +    |
2020-04-15T09:16:40.2102044Z +    |
2020-04-15T09:16:40.2102459Z +    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2102966Z + 
2020-04-15T09:16:40.2103130Z 1 error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2103541Z 2   --> $DIR/fn-or-tuple-struct-with-underscore-args.rs:10:24
2020-04-15T09:16:40.2103744Z 3    |
2020-04-15T09:16:40.2103744Z 3    |
2020-04-15T09:16:40.2103825Z 
2020-04-15T09:16:40.2103976Z 34 LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2104364Z 36 
2020-04-15T09:16:40.2104653Z - error: aborting due to 6 previous errors
2020-04-15T09:16:40.2104852Z + error: aborting due to 12 previous errors
2020-04-15T09:16:40.2105011Z 38 
2020-04-15T09:16:40.2105011Z 38 
2020-04-15T09:16:40.2105344Z + For more information about this error, try `rustc --explain E0658`.
2020-04-15T09:16:40.2105518Z 39 
2020-04-15T09:16:40.2105608Z 
2020-04-15T09:16:40.2105682Z 
2020-04-15T09:16:40.2105834Z The actual stderr differed from the expected stderr.
2020-04-15T09:16:40.2106434Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args/fn-or-tuple-struct-with-underscore-args.stderr
2020-04-15T09:16:40.2106976Z To update references, rerun the tests and pass the `--bless` flag
2020-04-15T09:16:40.2107477Z To only update this specific test, also pass `--test-args suggestions/fn-or-tuple-struct-with-underscore-args.rs`
2020-04-15T09:16:40.2107840Z error: 1 errors occurred comparing output.
2020-04-15T09:16:40.2108012Z status: exit code: 1
2020-04-15T09:16:40.2108012Z status: exit code: 1
2020-04-15T09:16:40.2109604Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args/auxiliary"
2020-04-15T09:16:40.2113074Z ------------------------------------------
2020-04-15T09:16:40.2113234Z 
2020-04-15T09:16:40.2113582Z ------------------------------------------
2020-04-15T09:16:40.2113784Z stderr:
2020-04-15T09:16:40.2113784Z stderr:
2020-04-15T09:16:40.2114141Z ------------------------------------------
2020-04-15T09:16:40.2114398Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2114974Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:10:24
2020-04-15T09:16:40.2115247Z    |
2020-04-15T09:16:40.2115425Z LL |     let _: usize = foo(_, _);
2020-04-15T09:16:40.2115651Z    |                        ^
2020-04-15T09:16:40.2115906Z    |
2020-04-15T09:16:40.2116441Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2117037Z 
2020-04-15T09:16:40.2117229Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2117861Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:10:27
2020-04-15T09:16:40.2118134Z    |
2020-04-15T09:16:40.2118134Z    |
2020-04-15T09:16:40.2118308Z LL |     let _: usize = foo(_, _);
2020-04-15T09:16:40.2118523Z    |                           ^
2020-04-15T09:16:40.2118699Z    |
2020-04-15T09:16:40.2119218Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2119810Z 
2020-04-15T09:16:40.2120006Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2120575Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:13:18
2020-04-15T09:16:40.2120857Z    |
2020-04-15T09:16:40.2120857Z    |
2020-04-15T09:16:40.2121023Z LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2121381Z    |
2020-04-15T09:16:40.2121381Z    |
2020-04-15T09:16:40.2121887Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2122485Z 
2020-04-15T09:16:40.2122678Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2123244Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:13:21
2020-04-15T09:16:40.2123524Z    |
2020-04-15T09:16:40.2123524Z    |
2020-04-15T09:16:40.2123690Z LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2124055Z    |
2020-04-15T09:16:40.2124055Z    |
2020-04-15T09:16:40.2124562Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2127897Z 
2020-04-15T09:16:40.2128093Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2128776Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:16:27
2020-04-15T09:16:40.2129059Z    |
2020-04-15T09:16:40.2129059Z    |
2020-04-15T09:16:40.2129257Z LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2129661Z    |
2020-04-15T09:16:40.2129661Z    |
2020-04-15T09:16:40.2130197Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2130884Z 
2020-04-15T09:16:40.2131063Z error[E0658]: destructuring assignments are unstable
2020-04-15T09:16:40.2131584Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:16:30
2020-04-15T09:16:40.2131830Z    |
2020-04-15T09:16:40.2131830Z    |
2020-04-15T09:16:40.2132023Z LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2132390Z    |
2020-04-15T09:16:40.2132390Z    |
2020-04-15T09:16:40.2132876Z    = note: see issue #71126 <***/issues/71126> for more information
2020-04-15T09:16:40.2133413Z 
2020-04-15T09:16:40.2133610Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2135690Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:10:24
2020-04-15T09:16:40.2135944Z    |
---
2020-04-15T09:16:40.2138443Z 
2020-04-15T09:16:40.2138627Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2139231Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:13:18
2020-04-15T09:16:40.2139483Z    |
2020-04-15T09:16:40.2139638Z LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2140000Z 
2020-04-15T09:16:40.2140182Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2140749Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:13:21
2020-04-15T09:16:40.2141012Z    |
2020-04-15T09:16:40.2141012Z    |
2020-04-15T09:16:40.2141165Z LL |     let _: S = S(_, _);
2020-04-15T09:16:40.2141535Z 
2020-04-15T09:16:40.2141844Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2142393Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:16:27
2020-04-15T09:16:40.2142654Z    |
2020-04-15T09:16:40.2142654Z    |
2020-04-15T09:16:40.2142827Z LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2143225Z 
2020-04-15T09:16:40.2143419Z error: expected expression, found reserved identifier `_`
2020-04-15T09:16:40.2143942Z   --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-with-underscore-args.rs:16:30
2020-04-15T09:16:40.2144189Z    |
2020-04-15T09:16:40.2144189Z    |
2020-04-15T09:16:40.2147589Z LL |     let _: usize = T::baz(_, _);
2020-04-15T09:16:40.2148003Z 
2020-04-15T09:16:40.2148188Z error: aborting due to 12 previous errors
2020-04-15T09:16:40.2148332Z 
2020-04-15T09:16:40.2148848Z For more information about this error, try `rustc --explain E0658`.
---
2020-04-15T09:16:40.2158453Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-15T09:16:40.2164503Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T09:16:40.2164819Z 
2020-04-15T09:16:40.2164917Z 
2020-04-15T09:16:40.2168588Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-15T09:16:40.2171060Z 
2020-04-15T09:16:40.2171168Z 
2020-04-15T09:16:40.2171830Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-15T09:16:40.2172146Z Build completed unsuccessfully in 0:56:26
2020-04-15T09:16:40.2172146Z Build completed unsuccessfully in 0:56:26
2020-04-15T09:16:40.2172371Z == clock drift check ==
2020-04-15T09:16:40.2172581Z   local time: Wed Apr 15 09:16:40 UTC 2020
2020-04-15T09:16:40.4889243Z   network time: Wed, 15 Apr 2020 09:16:40 GMT
2020-04-15T09:16:41.0396624Z 
2020-04-15T09:16:41.0396624Z 
2020-04-15T09:16:41.0462590Z ##[error]Bash exited with code '1'.
2020-04-15T09:16:41.0475383Z ##[section]Finishing: Run build
2020-04-15T09:16:41.0516092Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71156/merge to s
2020-04-15T09:16:41.0520209Z Task         : Get sources
2020-04-15T09:16:41.0520473Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T09:16:41.0520701Z Version      : 1.0.0
2020-04-15T09:16:41.0520870Z Author       : Microsoft
2020-04-15T09:16:41.0520870Z Author       : Microsoft
2020-04-15T09:16:41.0521146Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T09:16:41.0521435Z ==============================================================================
2020-04-15T09:16:41.3381132Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T09:16:41.3479985Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71156/merge to s
2020-04-15T09:16:41.3578856Z Cleaning up task key
2020-04-15T09:16:41.3580068Z Start cleaning up orphan processes.
2020-04-15T09:16:41.3737521Z Terminate orphan process: pid (4435) (python)
2020-04-15T09:16:41.3884343Z ##[section]Finishing: Finalize Job

plain
2020-04-02T00:20:39.0901617Z ========================== Starting Command Output ===========================
2020-04-02T00:20:39.0904688Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/da832d62-7965-4167-bf60-36ceea61befc.sh
2020-04-02T00:20:39.0904970Z 
2020-04-02T00:20:39.0908779Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T00:20:39.0927927Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70643/merge to s
2020-04-02T00:20:39.0931136Z Task         : Get sources
2020-04-02T00:20:39.0931446Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T00:20:39.0931743Z Version      : 1.0.0
2020-04-02T00:20:39.0931942Z Author       : Microsoft
---
2020-04-02T00:20:40.1014891Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T00:20:40.1020082Z ##[command]git config gc.auto 0
2020-04-02T00:20:40.1023509Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T00:20:40.1026716Z ##[command]git config --get-all http.proxy
2020-04-02T00:20:40.1032969Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70643/merge:refs/remotes/pull/70643/merge
---
2020-04-02T00:22:24.7847109Z Looks like docker image is the same as before, not uploading
2020-04-02T00:22:30.7054580Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T00:22:30.7288832Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T00:22:30.7318309Z == clock drift check ==
2020-04-02T00:22:30.7329474Z   local time: Thu Apr  2 00:22:30 UTC 2020
2020-04-02T00:22:30.8988825Z   network time: Thu, 02 Apr 2020 00:22:30 GMT
2020-04-02T00:22:30.9016193Z Starting sccache server...
2020-04-02T00:22:30.9884471Z configure: processing command line
2020-04-02T00:22:30.9884981Z configure: 
2020-04-02T00:22:30.9886115Z configure: rust.dist-src        := False
---
2020-04-02T00:27:31.0241148Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T00:27:32.4036989Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T00:27:33.8912302Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T00:27:34.0171484Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T00:27:43.2009067Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T00:27:44.7100308Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T00:27:48.8409576Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T00:27:52.7534937Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T00:28:02.0394405Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T00:48:54.3608747Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T00:48:56.0342768Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T00:48:57.9086512Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T00:48:58.7458494Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T00:49:09.6512802Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T00:49:11.6933395Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T00:49:16.8291505Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T00:49:22.0923244Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T00:49:33.0216219Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T01:13:47.3979654Z .................................................................................................... 1700/9868
2020-04-02T01:13:51.2868340Z .................................................................................................... 1800/9868
2020-04-02T01:13:59.9157704Z ...............................................................................................i.... 1900/9868
2020-04-02T01:14:07.3989399Z .................................................................................................... 2000/9868
2020-04-02T01:14:13.5496598Z .....................................................................................iiiii.......... 2100/9868
2020-04-02T01:14:33.9421722Z .............................................................................F...................... 2300/9868
2020-04-02T01:14:36.0361271Z .................................................................................................... 2400/9868
2020-04-02T01:14:38.3165444Z .................................................................................................... 2500/9868
2020-04-02T01:14:44.3312691Z .................................................................................................... 2600/9868
---
2020-04-02T01:17:27.8838768Z ...........................................................i...............i........................ 5000/9868
2020-04-02T01:17:34.9664856Z .................................................................................................... 5100/9868
2020-04-02T01:17:43.1798912Z .................................................................................................... 5200/9868
2020-04-02T01:17:47.0508783Z ....i.......................................................F....................................... 5300/9868
2020-04-02T01:17:57.0317394Z ..........................................................................................ii.ii..... 5400/9868
2020-04-02T01:18:01.0388698Z ...i...i............................................................................................ 5500/9868
2020-04-02T01:18:09.4256384Z ...................................i................................................................ 5700/9868
2020-04-02T01:18:18.9050325Z .......................................................ii....................................i...... 5800/9868
2020-04-02T01:18:26.0355384Z .................................................................................................... 5900/9868
2020-04-02T01:18:30.6517323Z .....................................................FFFFF..........F............................... 6000/9868
2020-04-02T01:18:30.6517323Z .....................................................FFFFF..........F............................... 6000/9868
2020-04-02T01:18:39.4151283Z .......................................................................................ii...i..ii... 6100/9868
2020-04-02T01:18:59.1123687Z .................................................................................................... 6300/9868
2020-04-02T01:19:03.5780685Z .................................................................................................... 6400/9868
2020-04-02T01:19:06.3188161Z .................................................................................................... 6500/9868
2020-04-02T01:19:06.3188161Z .................................................................................................... 6500/9868
2020-04-02T01:19:18.1415051Z .................i..ii.............................................................................. 6600/9868
2020-04-02T01:19:37.4970254Z .................................................................................................... 6800/9868
2020-04-02T01:19:39.4301464Z .................i.................................................................................. 6900/9868
2020-04-02T01:19:41.3940497Z .................................................................................................... 7000/9868
2020-04-02T01:19:43.4769591Z ........................................................i........................................... 7100/9868
---
2020-04-02T01:21:16.7068376Z .................................................................................................... 7800/9868
2020-04-02T01:21:21.5635548Z .................................................................................................... 7900/9868
2020-04-02T01:21:27.1500753Z .................................................................................................... 8000/9868
2020-04-02T01:21:35.0514704Z ..................i................................................................................. 8100/9868
2020-04-02T01:21:43.2381824Z ...................................................................iiiiiiiiii.i..................... 8200/9868
2020-04-02T01:21:58.3176195Z ...........i......i................................................................................. 8400/9868
2020-04-02T01:22:03.1179689Z .F.....F............................................................................................ 8500/9868
2020-04-02T01:22:14.0397531Z .................................................................................................... 8600/9868
2020-04-02T01:22:25.5275255Z .................................................................................................... 8700/9868
---
2020-04-02T01:24:16.9009066Z 24    |     ^^^^^^^^^^^
2020-04-02T01:24:16.9009204Z 
2020-04-02T01:24:16.9009303Z 
2020-04-02T01:24:16.9009509Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9012389Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034/E0034.stderr
2020-04-02T01:24:16.9013044Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9013623Z To only update this specific test, also pass `--test-args error-codes/E0034.rs`
2020-04-02T01:24:16.9014088Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9014329Z status: exit code: 1
2020-04-02T01:24:16.9014329Z status: exit code: 1
2020-04-02T01:24:16.9016235Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034/auxiliary"
2020-04-02T01:24:16.9017786Z ------------------------------------------
2020-04-02T01:24:16.9017961Z 
2020-04-02T01:24:16.9018315Z ------------------------------------------
2020-04-02T01:24:16.9018534Z stderr:
---
2020-04-02T01:24:16.9028148Z ---- [ui] ui/inference/inference_unstable_featured.rs stdout ----
2020-04-02T01:24:16.9028405Z diff of stderr:
2020-04-02T01:24:16.9028530Z 
2020-04-02T01:24:16.9028655Z 6    |
2020-04-02T01:24:16.9029053Z 7    = note: candidate #1 is defined in an impl of the trait `inference_unstable_iterator::IpuIterator` for the type `char`
2020-04-02T01:24:16.9029675Z 8    = note: candidate #2 is defined in an impl of the trait `inference_unstable_itertools::IpuItertools` for the type `char`
2020-04-02T01:24:16.9030662Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9030901Z 10    |
2020-04-02T01:24:16.9030901Z 10    |
2020-04-02T01:24:16.9031436Z 11 LL |     assert_eq!(inference_unstable_iterator::IpuIterator::ipu_flatten(&'x'), 0);
2020-04-02T01:24:16.9032112Z 
2020-04-02T01:24:16.9032501Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9032861Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9033101Z 14    |
2020-04-02T01:24:16.9033101Z 14    |
2020-04-02T01:24:16.9033625Z 15 LL |     assert_eq!(inference_unstable_itertools::IpuItertools::ipu_flatten(&'x'), 0);
2020-04-02T01:24:16.9034331Z 
2020-04-02T01:24:16.9034430Z 
2020-04-02T01:24:16.9034639Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9035382Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/inference_unstable_featured.stderr
2020-04-02T01:24:16.9035382Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/inference_unstable_featured.stderr
2020-04-02T01:24:16.9036051Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9036688Z To only update this specific test, also pass `--test-args inference/inference_unstable_featured.rs`
2020-04-02T01:24:16.9037154Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9037492Z status: exit code: 1
2020-04-02T01:24:16.9037492Z status: exit code: 1
2020-04-02T01:24:16.9039547Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable_featured.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/auxiliary"
2020-04-02T01:24:16.9041194Z ------------------------------------------
2020-04-02T01:24:16.9041369Z 
2020-04-02T01:24:16.9041722Z ------------------------------------------
2020-04-02T01:24:16.9041940Z stderr:
2020-04-02T01:24:16.9041940Z stderr:
2020-04-02T01:24:16.9042308Z ------------------------------------------
2020-04-02T01:24:16.9042591Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9043159Z   --> /checkout/src/test/ui/inference/inference_unstable_featured.rs:16:20
2020-04-02T01:24:16.9043430Z    |
2020-04-02T01:24:16.9043846Z LL |     assert_eq!('x'.ipu_flatten(), 0);   //~ ERROR E0034
2020-04-02T01:24:16.9044204Z    |                    ^^^^^^^^^^^ multiple `ipu_flatten` found
2020-04-02T01:24:16.9044435Z    |
2020-04-02T01:24:16.9044816Z    = note: candidate #1 is defined in an impl of the trait `inference_unstable_iterator::IpuIterator` for the type `char`
2020-04-02T01:24:16.9045444Z    = note: candidate #2 is defined in an impl of the trait `inference_unstable_itertools::IpuItertools` for the type `char`
2020-04-02T01:24:16.9046158Z    |
2020-04-02T01:24:16.9046158Z    |
2020-04-02T01:24:16.9046974Z LL |     assert_eq!(inference_unstable_iterator::IpuIterator::ipu_flatten(&'x'), 0);   //~ ERROR E0034
2020-04-02T01:24:16.9047826Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9048052Z    |
2020-04-02T01:24:16.9048052Z    |
2020-04-02T01:24:16.9048621Z LL |     assert_eq!(inference_unstable_itertools::IpuItertools::ipu_flatten(&'x'), 0);   //~ ERROR E0034
2020-04-02T01:24:16.9049341Z 
2020-04-02T01:24:16.9049524Z error: aborting due to previous error
2020-04-02T01:24:16.9049691Z 
2020-04-02T01:24:16.9050133Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9051586Z diff of stderr:
2020-04-02T01:24:16.9051715Z 
2020-04-02T01:24:16.9052022Z 5    |     --^^^--
2020-04-02T01:24:16.9052202Z 6    |     | |
2020-04-02T01:24:16.9052405Z 7    |     | multiple `foo` found
2020-04-02T01:24:16.9052945Z -    |     help: disambiguate the method call for candidate #2: `T::foo(&x)`
2020-04-02T01:24:16.9053379Z +    |     help: disambiguate the associated function for candidate #2: `T::foo(&x)`
2020-04-02T01:24:16.9053930Z 10 note: candidate #1 is defined in an impl for the type `dyn T`
2020-04-02T01:24:16.9054430Z 11   --> $DIR/issue-18446.rs:9:5
2020-04-02T01:24:16.9054596Z 
2020-04-02T01:24:16.9054697Z 
2020-04-02T01:24:16.9054697Z 
2020-04-02T01:24:16.9054923Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9055565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18446/issue-18446.stderr
2020-04-02T01:24:16.9056173Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9056766Z To only update this specific test, also pass `--test-args issues/issue-18446.rs`
2020-04-02T01:24:16.9057328Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9057566Z status: exit code: 1
2020-04-02T01:24:16.9057566Z status: exit code: 1
2020-04-02T01:24:16.9060109Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18446.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18446" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18446/auxiliary"
2020-04-02T01:24:16.9061697Z ------------------------------------------
2020-04-02T01:24:16.9061871Z 
2020-04-02T01:24:16.9062224Z ------------------------------------------
2020-04-02T01:24:16.9062450Z stderr:
2020-04-02T01:24:16.9062450Z stderr:
2020-04-02T01:24:16.9062819Z ------------------------------------------
2020-04-02T01:24:16.9063100Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9063611Z   --> /checkout/src/test/ui/issues/issue-18446.rs:18:7
2020-04-02T01:24:16.9063839Z    |
2020-04-02T01:24:16.9064087Z LL |     x.foo(); //~ ERROR multiple applicable items in scope [E0034]
2020-04-02T01:24:16.9064516Z    |     --^^^--
2020-04-02T01:24:16.9064880Z    |     | multiple `foo` found
2020-04-02T01:24:16.9064880Z    |     | multiple `foo` found
2020-04-02T01:24:16.9065229Z    |     help: disambiguate the associated function for candidate #2: `T::foo(&x)`
2020-04-02T01:24:16.9065746Z note: candidate #1 is defined in an impl for the type `dyn T`
2020-04-02T01:24:16.9067985Z   --> /checkout/src/test/ui/issues/issue-18446.rs:9:5
2020-04-02T01:24:16.9068364Z    |
2020-04-02T01:24:16.9068675Z LL |     fn foo(&self) {}
---
2020-04-02T01:24:16.9074156Z ---- [ui] ui/issues/issue-3702-2.rs stdout ----
2020-04-02T01:24:16.9074405Z diff of stderr:
2020-04-02T01:24:16.9074531Z 
2020-04-02T01:24:16.9074660Z 14    |
2020-04-02T01:24:16.9075041Z 15 LL |     fn to_int(&self) -> isize { *self }
2020-04-02T01:24:16.9075801Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9076131Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9076387Z 18    |
2020-04-02T01:24:16.9076387Z 18    |
2020-04-02T01:24:16.9076639Z 19 LL |         ToPrimitive::to_int(&self) + other.to_int()
2020-04-02T01:24:16.9077137Z 
2020-04-02T01:24:16.9077527Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9077854Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9078092Z 22    |
2020-04-02T01:24:16.9078092Z 22    |
2020-04-02T01:24:16.9078340Z 23 LL |         Add::to_int(&self) + other.to_int()
2020-04-02T01:24:16.9078781Z 
2020-04-02T01:24:16.9078880Z 
2020-04-02T01:24:16.9079103Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9080011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/issue-3702-2.stderr
2020-04-02T01:24:16.9080011Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/issue-3702-2.stderr
2020-04-02T01:24:16.9081600Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9082210Z To only update this specific test, also pass `--test-args issues/issue-3702-2.rs`
2020-04-02T01:24:16.9082650Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9082905Z status: exit code: 1
2020-04-02T01:24:16.9082905Z status: exit code: 1
2020-04-02T01:24:16.9084994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3702-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/auxiliary"
2020-04-02T01:24:16.9086670Z ------------------------------------------
2020-04-02T01:24:16.9086946Z 
2020-04-02T01:24:16.9087329Z ------------------------------------------
2020-04-02T01:24:16.9088199Z stderr:
2020-04-02T01:24:16.9088199Z stderr:
2020-04-02T01:24:16.9088756Z ------------------------------------------
2020-04-02T01:24:16.9089064Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9089743Z   --> /checkout/src/test/ui/issues/issue-3702-2.rs:16:14
2020-04-02T01:24:16.9089986Z    |
2020-04-02T01:24:16.9090751Z LL |         self.to_int() + other.to_int() //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9091121Z    |              ^^^^^^ multiple `to_int` found
2020-04-02T01:24:16.9091454Z    |
2020-04-02T01:24:16.9091878Z note: candidate #1 is defined in an impl of the trait `ToPrimitive` for the type `isize`
2020-04-02T01:24:16.9092721Z    |
2020-04-02T01:24:16.9093245Z LL |     fn to_int(&self) -> isize { 0 }
2020-04-02T01:24:16.9093510Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9093832Z note: candidate #2 is defined in an impl of the trait `Add` for the type `isize`
2020-04-02T01:24:16.9093832Z note: candidate #2 is defined in an impl of the trait `Add` for the type `isize`
2020-04-02T01:24:16.9102238Z   --> /checkout/src/test/ui/issues/issue-3702-2.rs:14:5
2020-04-02T01:24:16.9102496Z    |
2020-04-02T01:24:16.9103322Z LL |     fn to_int(&self) -> isize { *self }
2020-04-02T01:24:16.9103896Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9104251Z    |
2020-04-02T01:24:16.9104251Z    |
2020-04-02T01:24:16.9104581Z LL |         ToPrimitive::to_int(&self) + other.to_int() //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9105242Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9105575Z    |
2020-04-02T01:24:16.9105575Z    |
2020-04-02T01:24:16.9105872Z LL |         Add::to_int(&self) + other.to_int() //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9106398Z 
2020-04-02T01:24:16.9106585Z error: aborting due to previous error
2020-04-02T01:24:16.9106752Z 
2020-04-02T01:24:16.9107200Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9108481Z ---- [ui] ui/issues/issue-65634-raw-ident-suggestion.rs stdout ----
2020-04-02T01:24:16.9108759Z diff of stderr:
2020-04-02T01:24:16.9108883Z 
2020-04-02T01:24:16.9109010Z 14    |
2020-04-02T01:24:16.9109215Z 15 LL |     fn r#struct(&self) {
2020-04-02T01:24:16.9109892Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9110240Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9110479Z 18    |
2020-04-02T01:24:16.9110479Z 18    |
2020-04-02T01:24:16.9110689Z 19 LL |     async::r#struct(&r#fn {});
2020-04-02T01:24:16.9111766Z 
2020-04-02T01:24:16.9112219Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9112776Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9113036Z 22    |
2020-04-02T01:24:16.9113036Z 22    |
2020-04-02T01:24:16.9113438Z 23 LL |     await::r#struct(&r#fn {});
2020-04-02T01:24:16.9114007Z 
2020-04-02T01:24:16.9114105Z 
2020-04-02T01:24:16.9114309Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9115087Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion/issue-65634-raw-ident-suggestion.stderr
2020-04-02T01:24:16.9115087Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion/issue-65634-raw-ident-suggestion.stderr
2020-04-02T01:24:16.9115786Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9116420Z To only update this specific test, also pass `--test-args issues/issue-65634-raw-ident-suggestion.rs`
2020-04-02T01:24:16.9116912Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9117311Z status: exit code: 1
2020-04-02T01:24:16.9117311Z status: exit code: 1
2020-04-02T01:24:16.9119374Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65634-raw-ident-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion/auxiliary"
2020-04-02T01:24:16.9121133Z ------------------------------------------
2020-04-02T01:24:16.9121310Z 
2020-04-02T01:24:16.9121675Z ------------------------------------------
2020-04-02T01:24:16.9121887Z stderr:
2020-04-02T01:24:16.9121887Z stderr:
2020-04-02T01:24:16.9122265Z ------------------------------------------
2020-04-02T01:24:16.9122548Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9123113Z   --> /checkout/src/test/ui/issues/issue-65634-raw-ident-suggestion.rs:21:13
2020-04-02T01:24:16.9123407Z    |
2020-04-02T01:24:16.9123667Z LL |     r#fn {}.r#struct(); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9124005Z    |             ^^^^^^^^ multiple `r#struct` found
2020-04-02T01:24:16.9124230Z    |
2020-04-02T01:24:16.9124498Z note: candidate #1 is defined in an impl of the trait `async` for the type `r#fn`
2020-04-02T01:24:16.9125388Z    |
2020-04-02T01:24:16.9125388Z    |
2020-04-02T01:24:16.9125564Z LL |     fn r#struct(&self) {
2020-04-02T01:24:16.9125779Z    |     ^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9126945Z note: candidate #2 is defined in an impl of the trait `await` for the type `r#fn`
2020-04-02T01:24:16.9127883Z    |
2020-04-02T01:24:16.9127883Z    |
2020-04-02T01:24:16.9128072Z LL |     fn r#struct(&self) {
2020-04-02T01:24:16.9128552Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9128794Z    |
2020-04-02T01:24:16.9128794Z    |
2020-04-02T01:24:16.9129070Z LL |     async::r#struct(&r#fn {}); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9129690Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9129917Z    |
2020-04-02T01:24:16.9129917Z    |
2020-04-02T01:24:16.9130325Z LL |     await::r#struct(&r#fn {}); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9130821Z 
2020-04-02T01:24:16.9131003Z error: aborting due to previous error
2020-04-02T01:24:16.9131168Z 
2020-04-02T01:24:16.9132758Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9137389Z ---- [ui] ui/methods/method-ambig-two-traits-from-bounds.rs stdout ----
2020-04-02T01:24:16.9137835Z diff of stderr:
2020-04-02T01:24:16.9137960Z 
2020-04-02T01:24:16.9138102Z 14    |
2020-04-02T01:24:16.9138291Z 15 LL | trait B { fn foo(&self); }
2020-04-02T01:24:16.9138999Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9139331Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9139569Z 18    |
2020-04-02T01:24:16.9139569Z 18    |
2020-04-02T01:24:16.9139761Z 19 LL |     A::foo(t);
2020-04-02T01:24:16.9140102Z 
2020-04-02T01:24:16.9140632Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9140991Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9141228Z 22    |
2020-04-02T01:24:16.9141228Z 22    |
2020-04-02T01:24:16.9141405Z 23 LL |     B::foo(t);
2020-04-02T01:24:16.9141755Z 
2020-04-02T01:24:16.9141852Z 
2020-04-02T01:24:16.9142057Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9142971Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds/method-ambig-two-traits-from-bounds.stderr
2020-04-02T01:24:16.9142971Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds/method-ambig-two-traits-from-bounds.stderr
2020-04-02T01:24:16.9143671Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9144306Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-from-bounds.rs`
2020-04-02T01:24:16.9144798Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9145035Z status: exit code: 1
2020-04-02T01:24:16.9145035Z status: exit code: 1
2020-04-02T01:24:16.9147134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds/auxiliary"
2020-04-02T01:24:16.9148957Z ------------------------------------------
2020-04-02T01:24:16.9149132Z 
2020-04-02T01:24:16.9149485Z ------------------------------------------
2020-04-02T01:24:16.9149705Z stderr:
2020-04-02T01:24:16.9149705Z stderr:
2020-04-02T01:24:16.9150065Z ------------------------------------------
2020-04-02T01:24:16.9150346Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9150938Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-from-bounds.rs:5:7
2020-04-02T01:24:16.9151220Z    |
2020-04-02T01:24:16.9151406Z LL |     t.foo(); //~ ERROR E0034
2020-04-02T01:24:16.9151657Z    |       ^^^ multiple `foo` found
2020-04-02T01:24:16.9152048Z note: candidate #1 is defined in the trait `A`
2020-04-02T01:24:16.9152626Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-from-bounds.rs:1:11
2020-04-02T01:24:16.9152909Z    |
2020-04-02T01:24:16.9152909Z    |
2020-04-02T01:24:16.9153089Z LL | trait A { fn foo(&self); }
2020-04-02T01:24:16.9153577Z note: candidate #2 is defined in the trait `B`
2020-04-02T01:24:16.9154171Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-from-bounds.rs:2:11
2020-04-02T01:24:16.9154466Z    |
2020-04-02T01:24:16.9154466Z    |
2020-04-02T01:24:16.9154643Z LL | trait B { fn foo(&self); }
2020-04-02T01:24:16.9155137Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9155523Z    |
2020-04-02T01:24:16.9155523Z    |
2020-04-02T01:24:16.9155722Z LL |     A::foo(t); //~ ERROR E0034
2020-04-02T01:24:16.9156210Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9156440Z    |
2020-04-02T01:24:16.9156440Z    |
2020-04-02T01:24:16.9156633Z LL |     B::foo(t); //~ ERROR E0034
2020-04-02T01:24:16.9156996Z 
2020-04-02T01:24:16.9157181Z error: aborting due to previous error
2020-04-02T01:24:16.9157345Z 
2020-04-02T01:24:16.9157804Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9160013Z 16    |     ^^^^^^^^^^^^
2020-04-02T01:24:16.9160459Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9160791Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9161028Z 18    |
2020-04-02T01:24:16.9161225Z 19 LL |     A::foo(AB {});
2020-04-02T01:24:16.9161583Z 
2020-04-02T01:24:16.9161972Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9162313Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9162551Z 22    |
2020-04-02T01:24:16.9162551Z 22    |
2020-04-02T01:24:16.9162735Z 23 LL |     B::foo(AB {});
2020-04-02T01:24:16.9163473Z 
2020-04-02T01:24:16.9163624Z 
2020-04-02T01:24:16.9163828Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9164623Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls/method-ambig-two-traits-from-impls.stderr
2020-04-02T01:24:16.9164623Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls/method-ambig-two-traits-from-impls.stderr
2020-04-02T01:24:16.9165325Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9165959Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-from-impls.rs`
2020-04-02T01:24:16.9166556Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9166795Z status: exit code: 1
2020-04-02T01:24:16.9166795Z status: exit code: 1
2020-04-02T01:24:16.9169043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls/auxiliary"
2020-04-02T01:24:16.9170723Z ------------------------------------------
2020-04-02T01:24:16.9170899Z 
2020-04-02T01:24:16.9171253Z ------------------------------------------
2020-04-02T01:24:16.9171470Z stderr:
2020-04-02T01:24:16.9171470Z stderr:
2020-04-02T01:24:16.9171834Z ------------------------------------------
2020-04-02T01:24:16.9172112Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9172695Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-from-impls.rs:15:11
2020-04-02T01:24:16.9172980Z    |
2020-04-02T01:24:16.9173174Z LL |     AB {}.foo();  //~ ERROR E0034
2020-04-02T01:24:16.9173441Z    |           ^^^ multiple `foo` found
2020-04-02T01:24:16.9173631Z    |
2020-04-02T01:24:16.9173888Z note: candidate #1 is defined in an impl of the trait `A` for the type `AB`
2020-04-02T01:24:16.9174932Z    |
2020-04-02T01:24:16.9175104Z LL |     fn foo(self) {}
2020-04-02T01:24:16.9175315Z    |     ^^^^^^^^^^^^
2020-04-02T01:24:16.9175315Z    |     ^^^^^^^^^^^^
2020-04-02T01:24:16.9175606Z note: candidate #2 is defined in an impl of the trait `B` for the type `AB`
2020-04-02T01:24:16.9176523Z    |
2020-04-02T01:24:16.9176693Z LL |     fn foo(self) {}
2020-04-02T01:24:16.9176889Z    |     ^^^^^^^^^^^^
2020-04-02T01:24:16.9177150Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9177150Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9177390Z    |
2020-04-02T01:24:16.9177593Z LL |     A::foo(AB {});  //~ ERROR E0034
2020-04-02T01:24:16.9178102Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9178328Z    |
2020-04-02T01:24:16.9178328Z    |
2020-04-02T01:24:16.9178525Z LL |     B::foo(AB {});  //~ ERROR E0034
2020-04-02T01:24:16.9178916Z 
2020-04-02T01:24:16.9179104Z error: aborting due to previous error
2020-04-02T01:24:16.9179270Z 
2020-04-02T01:24:16.9179711Z For more information about this error, try `rustc --explain E0034`.
2020-04-02T01:24:16.9179711Z For more information about this error, try `rustc --explain E0034`.
2020-04-02T01:24:16.9179930Z 
2020-04-02T01:24:16.9180285Z ------------------------------------------
2020-04-02T01:24:16.9180459Z 
2020-04-02T01:24:16.9180572Z 
2020-04-02T01:24:16.9181002Z ---- [ui] ui/methods/method-ambig-two-traits-cross-crate.rs stdout ----
2020-04-02T01:24:16.9181272Z diff of stderr:
2020-04-02T01:24:16.9181397Z 
2020-04-02T01:24:16.9181822Z 10 LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
2020-04-02T01:24:16.9182138Z 11    |                      ^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9182560Z 12    = note: candidate #2 is defined in an impl of the trait `ambig_impl_2_lib::Me` for the type `usize`
2020-04-02T01:24:16.9183511Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9183756Z 14    |
2020-04-02T01:24:16.9183756Z 14    |
2020-04-02T01:24:16.9183986Z 15 LL | fn main() { Me2::me(&1_usize); }
2020-04-02T01:24:16.9184415Z 
2020-04-02T01:24:16.9184817Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9185143Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9185379Z 18    |
2020-04-02T01:24:16.9185379Z 18    |
2020-04-02T01:24:16.9185647Z 19 LL | fn main() { ambig_impl_2_lib::Me::me(&1_usize); }
2020-04-02T01:24:16.9186169Z 
2020-04-02T01:24:16.9186266Z 
2020-04-02T01:24:16.9186484Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9187246Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/method-ambig-two-traits-cross-crate.stderr
2020-04-02T01:24:16.9187246Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/method-ambig-two-traits-cross-crate.stderr
2020-04-02T01:24:16.9187940Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9188597Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-cross-crate.rs`
2020-04-02T01:24:16.9189072Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9189326Z status: exit code: 1
2020-04-02T01:24:16.9189326Z status: exit code: 1
2020-04-02T01:24:16.9191396Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/auxiliary"
2020-04-02T01:24:16.9193189Z ------------------------------------------
2020-04-02T01:24:16.9193368Z 
2020-04-02T01:24:16.9193750Z ------------------------------------------
2020-04-02T01:24:16.9193953Z stderr:
2020-04-02T01:24:16.9193953Z stderr:
2020-04-02T01:24:16.9194316Z ------------------------------------------
2020-04-02T01:24:16.9194610Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9195185Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs:11:21
2020-04-02T01:24:16.9195471Z    |
2020-04-02T01:24:16.9195698Z LL | fn main() { 1_usize.me(); } //~ ERROR E0034
2020-04-02T01:24:16.9195982Z    |                     ^^ multiple `me` found
2020-04-02T01:24:16.9196462Z note: candidate #1 is defined in an impl of the trait `Me2` for the type `usize`
2020-04-02T01:24:16.9197082Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs:10:22
2020-04-02T01:24:16.9197364Z    |
2020-04-02T01:24:16.9197364Z    |
2020-04-02T01:24:16.9197787Z LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
2020-04-02T01:24:16.9198102Z    |                      ^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9198510Z    = note: candidate #2 is defined in an impl of the trait `ambig_impl_2_lib::Me` for the type `usize`
2020-04-02T01:24:16.9199182Z    |
2020-04-02T01:24:16.9199182Z    |
2020-04-02T01:24:16.9199417Z LL | fn main() { Me2::me(&1_usize); } //~ ERROR E0034
2020-04-02T01:24:16.9199995Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9200221Z    |
2020-04-02T01:24:16.9200221Z    |
2020-04-02T01:24:16.9200500Z LL | fn main() { ambig_impl_2_lib::Me::me(&1_usize); } //~ ERROR E0034
2020-04-02T01:24:16.9201033Z 
2020-04-02T01:24:16.9201216Z error: aborting due to previous error
2020-04-02T01:24:16.9201395Z 
2020-04-02T01:24:16.9201822Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9203140Z ---- [ui] ui/methods/method-ambig-two-traits-with-default-method.rs stdout ----
2020-04-02T01:24:16.9203411Z diff of stderr:
2020-04-02T01:24:16.9203549Z 
2020-04-02T01:24:16.9203677Z 14    |
2020-04-02T01:24:16.9203883Z 15 LL | trait Bar { fn method(&self) {} }
2020-04-02T01:24:16.9204600Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9204929Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9205171Z 18    |
2020-04-02T01:24:16.9205171Z 18    |
2020-04-02T01:24:16.9205595Z 19 LL |     Foo::method(&1_usize);
2020-04-02T01:24:16.9205997Z 
2020-04-02T01:24:16.9209610Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9209981Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9210228Z 22    |
2020-04-02T01:24:16.9210228Z 22    |
2020-04-02T01:24:16.9210445Z 23 LL |     Bar::method(&1_usize);
2020-04-02T01:24:16.9210852Z 
2020-04-02T01:24:16.9210950Z 
2020-04-02T01:24:16.9211173Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9212013Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/method-ambig-two-traits-with-default-method.stderr
2020-04-02T01:24:16.9212013Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/method-ambig-two-traits-with-default-method.stderr
2020-04-02T01:24:16.9212744Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9213673Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-with-default-method.rs`
2020-04-02T01:24:16.9214165Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9214416Z status: exit code: 1
2020-04-02T01:24:16.9214416Z status: exit code: 1
2020-04-02T01:24:16.9216698Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/auxiliary"
2020-04-02T01:24:16.9218579Z ------------------------------------------
2020-04-02T01:24:16.9218755Z 
2020-04-02T01:24:16.9219126Z ------------------------------------------
2020-04-02T01:24:16.9219328Z stderr:
2020-04-02T01:24:16.9219328Z stderr:
2020-04-02T01:24:16.9219690Z ------------------------------------------
2020-04-02T01:24:16.9219985Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9220593Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs:12:13
2020-04-02T01:24:16.9220898Z    |
2020-04-02T01:24:16.9221117Z LL |     1_usize.method(); //~ ERROR E0034
2020-04-02T01:24:16.9221388Z    |             ^^^^^^ multiple `method` found
2020-04-02T01:24:16.9221871Z note: candidate #1 is defined in an impl of the trait `Foo` for the type `usize`
2020-04-02T01:24:16.9222519Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs:5:13
2020-04-02T01:24:16.9222819Z    |
2020-04-02T01:24:16.9222819Z    |
2020-04-02T01:24:16.9223023Z LL | trait Foo { fn method(&self) {} }
2020-04-02T01:24:16.9223579Z note: candidate #2 is defined in an impl of the trait `Bar` for the type `usize`
2020-04-02T01:24:16.9224233Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs:6:13
2020-04-02T01:24:16.9224536Z    |
2020-04-02T01:24:16.9224536Z    |
2020-04-02T01:24:16.9224731Z LL | trait Bar { fn method(&self) {} }
2020-04-02T01:24:16.9225267Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9225496Z    |
2020-04-02T01:24:16.9225496Z    |
2020-04-02T01:24:16.9225709Z LL |     Foo::method(&1_usize); //~ ERROR E0034
2020-04-02T01:24:16.9226258Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9226485Z    |
2020-04-02T01:24:16.9226485Z    |
2020-04-02T01:24:16.9226714Z LL |     Bar::method(&1_usize); //~ ERROR E0034
2020-04-02T01:24:16.9227122Z 
2020-04-02T01:24:16.9227318Z error: aborting due to previous error
2020-04-02T01:24:16.9227487Z 
2020-04-02T01:24:16.9227913Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9232977Z 24    |     ^^^^^^
2020-04-02T01:24:16.9233107Z 
2020-04-02T01:24:16.9233263Z 
2020-04-02T01:24:16.9233522Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9234306Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls2/method-ambig-two-traits-from-impls2.stderr
2020-04-02T01:24:16.9235000Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9235656Z To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-from-impls2.rs`
2020-04-02T01:24:16.9236129Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9236381Z status: exit code: 1
2020-04-02T01:24:16.9236381Z status: exit code: 1
2020-04-02T01:24:16.9238460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-impls2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls2/auxiliary"
2020-04-02T01:24:16.9240139Z ------------------------------------------
2020-04-02T01:24:16.9240315Z 
2020-04-02T01:24:16.9240681Z ------------------------------------------
2020-04-02T01:24:16.9240883Z stderr:
2020-04-02T01:24:16.9240883Z stderr:
2020-04-02T01:24:16.9241246Z ------------------------------------------
2020-04-02T01:24:16.9241541Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9242114Z   --> /checkout/src/test/ui/methods/method-ambig-two-traits-from-impls2.rs:15:9
2020-04-02T01:24:16.9242398Z    |
2020-04-02T01:24:16.9242609Z LL |     AB::foo();  //~ ERROR E0034
2020-04-02T01:24:16.9242862Z    |         ^^^ multiple `foo` found
2020-04-02T01:24:16.9243052Z    |
2020-04-02T01:24:16.9243327Z note: candidate #1 is defined in an impl of the trait `A` for the type `AB`
2020-04-02T01:24:16.9244216Z    |
2020-04-02T01:24:16.9244391Z LL |     fn foo() {}
2020-04-02T01:24:16.9244574Z    |     ^^^^^^^^
2020-04-02T01:24:16.9244574Z    |     ^^^^^^^^
2020-04-02T01:24:16.9244856Z note: candidate #2 is defined in an impl of the trait `B` for the type `AB`
2020-04-02T01:24:16.9245761Z    |
2020-04-02T01:24:16.9245921Z LL |     fn foo() {}
2020-04-02T01:24:16.9246118Z    |     ^^^^^^^^
2020-04-02T01:24:16.9246481Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9246481Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9246707Z    |
2020-04-02T01:24:16.9246899Z LL |     A::foo();  //~ ERROR E0034
2020-04-02T01:24:16.9247375Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9247612Z    |
2020-04-02T01:24:16.9247612Z    |
2020-04-02T01:24:16.9247819Z LL |     B::foo();  //~ ERROR E0034
2020-04-02T01:24:16.9248157Z 
2020-04-02T01:24:16.9248355Z error: aborting due to previous error
2020-04-02T01:24:16.9248520Z 
2020-04-02T01:24:16.9248953Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9251451Z 37    |     ^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9251898Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9252338Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9252593Z 39    |
2020-04-02T01:24:16.9252812Z 40 LL |     let z = internal::X::foo(x);
2020-04-02T01:24:16.9253287Z 
2020-04-02T01:24:16.9253709Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9254039Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9254275Z 43    |
2020-04-02T01:24:16.9254275Z 43    |
2020-04-02T01:24:16.9254539Z 44 LL |     let z = nuisance_foo::NuisanceFoo::foo(x);
2020-04-02T01:24:16.9255056Z 
2020-04-02T01:24:16.9255460Z - help: disambiguate the method call for candidate #3
2020-04-02T01:24:16.9255787Z + help: disambiguate the associated function for candidate #3
2020-04-02T01:24:16.9256023Z 47    |
2020-04-02T01:24:16.9256023Z 47    |
2020-04-02T01:24:16.9256247Z 48 LL |     let z = FinalFoo::foo(x);
2020-04-02T01:24:16.9256674Z 
2020-04-02T01:24:16.9256771Z 
2020-04-02T01:24:16.9256993Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9257872Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/method-deref-to-same-trait-object-with-separate-params.stderr
2020-04-02T01:24:16.9257872Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/method-deref-to-same-trait-object-with-separate-params.stderr
2020-04-02T01:24:16.9258631Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9259339Z To only update this specific test, also pass `--test-args methods/method-deref-to-same-trait-object-with-separate-params.rs`
2020-04-02T01:24:16.9259846Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9260098Z status: exit code: 1
2020-04-02T01:24:16.9260098Z status: exit code: 1
2020-04-02T01:24:16.9262354Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/auxiliary"
2020-04-02T01:24:16.9264125Z ------------------------------------------
2020-04-02T01:24:16.9264298Z 
2020-04-02T01:24:16.9264666Z ------------------------------------------
2020-04-02T01:24:16.9264867Z stderr:
2020-04-02T01:24:16.9264867Z stderr:
2020-04-02T01:24:16.9265227Z ------------------------------------------
2020-04-02T01:24:16.9265491Z error[E0308]: mismatched types
2020-04-02T01:24:16.9266089Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:85:24
2020-04-02T01:24:16.9266422Z    |
2020-04-02T01:24:16.9266679Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2020-04-02T01:24:16.9267188Z    |                   --   ^ expected `()`, found `u32`
2020-04-02T01:24:16.9267694Z    |                   expected due to this
2020-04-02T01:24:16.9267867Z 
2020-04-02T01:24:16.9268043Z error[E0308]: mismatched types
2020-04-02T01:24:16.9268656Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:102:24
2020-04-02T01:24:16.9268656Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:102:24
2020-04-02T01:24:16.9268982Z    |
2020-04-02T01:24:16.9269217Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2020-04-02T01:24:16.9269720Z    |                   --   ^ expected `()`, found `u64`
2020-04-02T01:24:16.9270223Z    |                   expected due to this
2020-04-02T01:24:16.9270399Z 
2020-04-02T01:24:16.9270619Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9271370Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:120:15
2020-04-02T01:24:16.9271370Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:120:15
2020-04-02T01:24:16.9271701Z    |
2020-04-02T01:24:16.9271964Z LL |     let z = x.foo(); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9272278Z    |               ^^^ multiple `foo` found
2020-04-02T01:24:16.9272472Z    |
2020-04-02T01:24:16.9272783Z note: candidate #1 is defined in an impl of the trait `internal::X` for the type `_`
2020-04-02T01:24:16.9273817Z    |
2020-04-02T01:24:16.9273817Z    |
2020-04-02T01:24:16.9274233Z LL |         fn foo(self: Smaht<Self, u64>) -> u64 {
2020-04-02T01:24:16.9274543Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9274942Z note: candidate #2 is defined in an impl of the trait `nuisance_foo::NuisanceFoo` for the type `_`
2020-04-02T01:24:16.9276010Z    |
2020-04-02T01:24:16.9276187Z LL |         fn foo(self) {}
2020-04-02T01:24:16.9276413Z    |         ^^^^^^^^^^^^
2020-04-02T01:24:16.9276413Z    |         ^^^^^^^^^^^^
2020-04-02T01:24:16.9276669Z note: candidate #3 is defined in the trait `FinalFoo`
2020-04-02T01:24:16.9277628Z    |
2020-04-02T01:24:16.9277962Z LL |     fn foo(&self) -> u8;
2020-04-02T01:24:16.9278181Z    |     ^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9278456Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9278456Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9278698Z    |
2020-04-02T01:24:16.9278981Z LL |     let z = internal::X::foo(x); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9279618Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9279845Z    |
2020-04-02T01:24:16.9279845Z    |
2020-04-02T01:24:16.9280160Z LL |     let z = nuisance_foo::NuisanceFoo::foo(x); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9280885Z help: disambiguate the associated function for candidate #3
2020-04-02T01:24:16.9281112Z    |
2020-04-02T01:24:16.9281112Z    |
2020-04-02T01:24:16.9281400Z LL |     let z = FinalFoo::foo(x); //~ ERROR multiple applicable items in scope
2020-04-02T01:24:16.9281881Z 
2020-04-02T01:24:16.9282069Z error[E0308]: mismatched types
2020-04-02T01:24:16.9282676Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:137:24
2020-04-02T01:24:16.9283005Z    |
2020-04-02T01:24:16.9283005Z    |
2020-04-02T01:24:16.9283253Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2020-04-02T01:24:16.9283755Z    |                   --   ^ expected `()`, found `u8`
2020-04-02T01:24:16.9284247Z    |                   expected due to this
2020-04-02T01:24:16.9284436Z 
2020-04-02T01:24:16.9284617Z error[E0308]: mismatched types
2020-04-02T01:24:16.9285218Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:155:24
2020-04-02T01:24:16.9285218Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:155:24
2020-04-02T01:24:16.9285559Z    |
2020-04-02T01:24:16.9285794Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2020-04-02T01:24:16.9286470Z    |                   --   ^ expected `()`, found `u32`
2020-04-02T01:24:16.9286978Z    |                   expected due to this
2020-04-02T01:24:16.9287152Z 
2020-04-02T01:24:16.9287326Z error[E0308]: mismatched types
2020-04-02T01:24:16.9287952Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:172:24
2020-04-02T01:24:16.9287952Z   --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:172:24
2020-04-02T01:24:16.9288276Z    |
2020-04-02T01:24:16.9288510Z LL |     let _seetype: () = z; //~ ERROR mismatched types
2020-04-02T01:24:16.9289030Z    |                   --   ^ expected `()`, found `u32`
2020-04-02T01:24:16.9289524Z    |                   expected due to this
2020-04-02T01:24:16.9289826Z 
2020-04-02T01:24:16.9290019Z error: aborting due to 6 previous errors
2020-04-02T01:24:16.9290191Z 
---
2020-04-02T01:24:16.9293079Z 16    |     ^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9293532Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9293860Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9294097Z 18    |
2020-04-02T01:24:16.9294291Z 19 LL |     A::foo(&a)
2020-04-02T01:24:16.9294636Z 
2020-04-02T01:24:16.9295024Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9295364Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9295600Z 22    |
2020-04-02T01:24:16.9295600Z 22    |
2020-04-02T01:24:16.9295775Z 23 LL |     B::foo(&a)
2020-04-02T01:24:16.9296133Z 
2020-04-02T01:24:16.9296373Z 39    |
2020-04-02T01:24:16.9296552Z 40 LL |     fn foo(&self) {}
2020-04-02T01:24:16.9296780Z 41    |     ^^^^^^^^^^^^^
---
2020-04-02T01:24:16.9298340Z 
2020-04-02T01:24:16.9298728Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9299064Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9299303Z 47    |
2020-04-02T01:24:16.9299493Z 48 LL |     D::foo(&a)
2020-04-02T01:24:16.9299836Z 
2020-04-02T01:24:16.9299976Z 64    |
2020-04-02T01:24:16.9300153Z 65 LL |     fn foo(self) {}
2020-04-02T01:24:16.9300360Z 66    |     ^^^^^^^^^^^^
---
2020-04-02T01:24:16.9303407Z 
2020-04-02T01:24:16.9303504Z 
2020-04-02T01:24:16.9303713Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9304353Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-37767/issue-37767.stderr
2020-04-02T01:24:16.9304975Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9305542Z To only update this specific test, also pass `--test-args span/issue-37767.rs`
2020-04-02T01:24:16.9305990Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9306228Z status: exit code: 1
2020-04-02T01:24:16.9306228Z status: exit code: 1
2020-04-02T01:24:16.9308156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-37767.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-37767" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-37767/auxiliary"
2020-04-02T01:24:16.9309793Z ------------------------------------------
2020-04-02T01:24:16.9309967Z 
2020-04-02T01:24:16.9310322Z ------------------------------------------
2020-04-02T01:24:16.9310523Z stderr:
2020-04-02T01:24:16.9310523Z stderr:
2020-04-02T01:24:16.9310901Z ------------------------------------------
2020-04-02T01:24:16.9311182Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9311666Z   --> /checkout/src/test/ui/span/issue-37767.rs:10:7
2020-04-02T01:24:16.9311906Z    |
2020-04-02T01:24:16.9312124Z LL |     a.foo() //~ ERROR multiple applicable items
2020-04-02T01:24:16.9312392Z    |       ^^^ multiple `foo` found
2020-04-02T01:24:16.9312806Z note: candidate #1 is defined in the trait `A`
2020-04-02T01:24:16.9313284Z   --> /checkout/src/test/ui/span/issue-37767.rs:2:5
2020-04-02T01:24:16.9313527Z    |
2020-04-02T01:24:16.9313702Z LL |     fn foo(&mut self) {}
---
2020-04-02T01:24:16.9315042Z LL |     fn foo(&mut self) {}
2020-04-02T01:24:16.9315268Z    |     ^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9315535Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9315763Z    |
2020-04-02T01:24:16.9316008Z LL |     A::foo(&a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9316521Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9316765Z    |
2020-04-02T01:24:16.9316765Z    |
2020-04-02T01:24:16.9316994Z LL |     B::foo(&a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9317384Z 
2020-04-02T01:24:16.9317606Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9318097Z   --> /checkout/src/test/ui/span/issue-37767.rs:22:7
2020-04-02T01:24:16.9318323Z    |
2020-04-02T01:24:16.9318323Z    |
2020-04-02T01:24:16.9318556Z LL |     a.foo() //~ ERROR multiple applicable items
2020-04-02T01:24:16.9318825Z    |       ^^^ multiple `foo` found
2020-04-02T01:24:16.9319466Z note: candidate #1 is defined in the trait `C`
2020-04-02T01:24:16.9319960Z   --> /checkout/src/test/ui/span/issue-37767.rs:14:5
2020-04-02T01:24:16.9320183Z    |
2020-04-02T01:24:16.9320367Z LL |     fn foo(&self) {}
---
2020-04-02T01:24:16.9321698Z LL |     fn foo(&self) {}
2020-04-02T01:24:16.9321908Z    |     ^^^^^^^^^^^^^
2020-04-02T01:24:16.9322183Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9322410Z    |
2020-04-02T01:24:16.9322636Z LL |     C::foo(&a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9323160Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9323386Z    |
2020-04-02T01:24:16.9323386Z    |
2020-04-02T01:24:16.9323632Z LL |     D::foo(&a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9324016Z 
2020-04-02T01:24:16.9324223Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9324725Z   --> /checkout/src/test/ui/span/issue-37767.rs:34:7
2020-04-02T01:24:16.9324950Z    |
2020-04-02T01:24:16.9324950Z    |
2020-04-02T01:24:16.9325165Z LL |     a.foo() //~ ERROR multiple applicable items
2020-04-02T01:24:16.9325454Z    |       ^^^ multiple `foo` found
2020-04-02T01:24:16.9325843Z note: candidate #1 is defined in the trait `E`
2020-04-02T01:24:16.9326867Z   --> /checkout/src/test/ui/span/issue-37767.rs:26:5
2020-04-02T01:24:16.9327099Z    |
2020-04-02T01:24:16.9327265Z LL |     fn foo(self) {}
---
2020-04-02T01:24:16.9329125Z LL |     fn foo(self) {}
2020-04-02T01:24:16.9329325Z    |     ^^^^^^^^^^^^
2020-04-02T01:24:16.9329582Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9329821Z    |
2020-04-02T01:24:16.9330045Z LL |     E::foo(a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9330559Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9330783Z    |
2020-04-02T01:24:16.9330783Z    |
2020-04-02T01:24:16.9331005Z LL |     F::foo(a) //~ ERROR multiple applicable items
2020-04-02T01:24:16.9331408Z 
2020-04-02T01:24:16.9331598Z error: aborting due to 3 previous errors
2020-04-02T01:24:16.9331769Z 
2020-04-02T01:24:16.9332236Z For more information about this error, try `rustc --explain E0034`.
2020-04-02T01:24:16.9332236Z For more information about this error, try `rustc --explain E0034`.
2020-04-02T01:24:16.9332453Z 
2020-04-02T01:24:16.9332805Z ------------------------------------------
2020-04-02T01:24:16.9332979Z 
2020-04-02T01:24:16.9333093Z 
2020-04-02T01:24:16.9333453Z ---- [ui] ui/span/issue-7575.rs stdout ----
2020-04-02T01:24:16.9333673Z diff of stderr:
2020-04-02T01:24:16.9333801Z 
2020-04-02T01:24:16.9334003Z 25            candidate #1: `CtxtFn`
2020-04-02T01:24:16.9334262Z 26            candidate #2: `OtherTrait`
2020-04-02T01:24:16.9334526Z 27            candidate #3: `UnusedTrait`
2020-04-02T01:24:16.9335338Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9335578Z 29    |
2020-04-02T01:24:16.9335578Z 29    |
2020-04-02T01:24:16.9335841Z 30 LL |     u.f8(42) + CtxtFn::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9336318Z 
2020-04-02T01:24:16.9336709Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9337051Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9337289Z 33    |
2020-04-02T01:24:16.9337289Z 33    |
2020-04-02T01:24:16.9337541Z 34 LL |     u.f8(42) + OtherTrait::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9338052Z 
2020-04-02T01:24:16.9338440Z - help: disambiguate the method call for candidate #3
2020-04-02T01:24:16.9338779Z + help: disambiguate the associated function for candidate #3
2020-04-02T01:24:16.9339018Z 37    |
2020-04-02T01:24:16.9339018Z 37    |
2020-04-02T01:24:16.9339265Z 38 LL |     u.f8(42) + UnusedTrait::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9339773Z 
2020-04-02T01:24:16.9340079Z 64    |     --^^^^^^--
2020-04-02T01:24:16.9340267Z 65    |     | |
2020-04-02T01:24:16.9340534Z 66    |     | this is an associated function, not a method
2020-04-02T01:24:16.9340534Z 66    |     | this is an associated function, not a method
2020-04-02T01:24:16.9341167Z -    |     help: disambiguate the method call for the candidate: `ManyImplTrait::is_str(t)`
2020-04-02T01:24:16.9341705Z +    |     help: disambiguate the associated function for the candidate: `ManyImplTrait::is_str(t)`
2020-04-02T01:24:16.9342401Z 69    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-02T01:24:16.9342844Z 70 note: the candidate is defined in the trait `ManyImplTrait`
2020-04-02T01:24:16.9343070Z 
2020-04-02T01:24:16.9343168Z 
2020-04-02T01:24:16.9343168Z 
2020-04-02T01:24:16.9343376Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9344019Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/issue-7575.stderr
2020-04-02T01:24:16.9344620Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9345317Z To only update this specific test, also pass `--test-args span/issue-7575.rs`
2020-04-02T01:24:16.9345761Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9346000Z status: exit code: 1
2020-04-02T01:24:16.9346000Z status: exit code: 1
2020-04-02T01:24:16.9347891Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-7575.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/auxiliary"
2020-04-02T01:24:16.9349435Z ------------------------------------------
2020-04-02T01:24:16.9349619Z 
2020-04-02T01:24:16.9349978Z ------------------------------------------
2020-04-02T01:24:16.9350182Z stderr:
2020-04-02T01:24:16.9350182Z stderr:
2020-04-02T01:24:16.9350563Z ------------------------------------------
2020-04-02T01:24:16.9350896Z error[E0599]: no method named `f9` found for type `usize` in the current scope
2020-04-02T01:24:16.9351434Z   --> /checkout/src/test/ui/span/issue-7575.rs:62:18
2020-04-02T01:24:16.9351676Z    |
2020-04-02T01:24:16.9351875Z LL |     u.f8(42) + u.f9(342) + m.fff(42)
2020-04-02T01:24:16.9352181Z    |                  ^^ this is an associated function, not a method
2020-04-02T01:24:16.9352799Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-02T01:24:16.9352799Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-02T01:24:16.9353215Z note: candidate #1 is defined in the trait `CtxtFn`
2020-04-02T01:24:16.9353943Z    |
2020-04-02T01:24:16.9354299Z LL |     fn f9(_: usize) -> usize;
2020-04-02T01:24:16.9354567Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9354567Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9354848Z note: candidate #2 is defined in the trait `OtherTrait`
2020-04-02T01:24:16.9355577Z    |
2020-04-02T01:24:16.9355929Z LL |     fn f9(_: usize) -> usize;
2020-04-02T01:24:16.9356175Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9356469Z note: candidate #3 is defined in the trait `UnusedTrait`
2020-04-02T01:24:16.9356469Z note: candidate #3 is defined in the trait `UnusedTrait`
2020-04-02T01:24:16.9356960Z   --> /checkout/src/test/ui/span/issue-7575.rs:17:5
2020-04-02T01:24:16.9357182Z    |
2020-04-02T01:24:16.9357547Z LL |     fn f9(_: usize) -> usize;
2020-04-02T01:24:16.9357793Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-02T01:24:16.9358124Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-04-02T01:24:16.9358590Z    = note: the following traits define an item `f9`, perhaps you need to implement one of them:
2020-04-02T01:24:16.9358941Z            candidate #1: `CtxtFn`
2020-04-02T01:24:16.9359189Z            candidate #2: `OtherTrait`
2020-04-02T01:24:16.9359460Z            candidate #3: `UnusedTrait`
2020-04-02T01:24:16.9359975Z    |
2020-04-02T01:24:16.9359975Z    |
2020-04-02T01:24:16.9360218Z LL |     u.f8(42) + CtxtFn::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9360790Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9361031Z    |
2020-04-02T01:24:16.9361031Z    |
2020-04-02T01:24:16.9361266Z LL |     u.f8(42) + OtherTrait::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9361858Z help: disambiguate the associated function for candidate #3
2020-04-02T01:24:16.9362101Z    |
2020-04-02T01:24:16.9362101Z    |
2020-04-02T01:24:16.9362337Z LL |     u.f8(42) + UnusedTrait::f9(u, 342) + m.fff(42)
2020-04-02T01:24:16.9362825Z 
2020-04-02T01:24:16.9362825Z 
2020-04-02T01:24:16.9363145Z error[E0599]: no method named `fff` found for struct `Myisize` in the current scope
2020-04-02T01:24:16.9363992Z    |
2020-04-02T01:24:16.9363992Z    |
2020-04-02T01:24:16.9364164Z LL | struct Myisize(isize);
2020-04-02T01:24:16.9364614Z    | ---------------------- method `fff` not found for this
2020-04-02T01:24:16.9364856Z ...
2020-04-02T01:24:16.9365052Z LL |     u.f8(42) + u.f9(342) + m.fff(42)
2020-04-02T01:24:16.9365465Z    |                            --^^^
2020-04-02T01:24:16.9366037Z    |                            | this is an associated function, not a method
2020-04-02T01:24:16.9366037Z    |                            | this is an associated function, not a method
2020-04-02T01:24:16.9366658Z    |                            help: use associated function syntax instead: `Myisize::fff`
2020-04-02T01:24:16.9367331Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-02T01:24:16.9367766Z note: the candidate is defined in an impl for the type `Myisize`
2020-04-02T01:24:16.9368315Z   --> /checkout/src/test/ui/span/issue-7575.rs:39:5
2020-04-02T01:24:16.9368542Z    |
2020-04-02T01:24:16.9368542Z    |
2020-04-02T01:24:16.9368900Z LL |     fn fff(i: isize) -> isize {
2020-04-02T01:24:16.9369328Z 
2020-04-02T01:24:16.9369602Z error[E0599]: no method named `is_str` found for type parameter `T` in the current scope
2020-04-02T01:24:16.9370152Z   --> /checkout/src/test/ui/span/issue-7575.rs:70:7
2020-04-02T01:24:16.9370392Z    |
2020-04-02T01:24:16.9370392Z    |
2020-04-02T01:24:16.9370550Z LL |     t.is_str()
2020-04-02T01:24:16.9370878Z    |     --^^^^^^--
2020-04-02T01:24:16.9371068Z    |     | |
2020-04-02T01:24:16.9371305Z    |     | this is an associated function, not a method
2020-04-02T01:24:16.9371735Z    |     help: disambiguate the associated function for the candidate: `ManyImplTrait::is_str(t)`
2020-04-02T01:24:16.9372417Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2020-04-02T01:24:16.9372838Z note: the candidate is defined in the trait `ManyImplTrait`
2020-04-02T01:24:16.9373361Z   --> /checkout/src/test/ui/span/issue-7575.rs:45:5
2020-04-02T01:24:16.9373584Z    |
---
2020-04-02T01:24:16.9377731Z 16    |         ^^^^^^^^^^^^^
2020-04-02T01:24:16.9378186Z - help: disambiguate the method call for candidate #1
2020-04-02T01:24:16.9378513Z + help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9378751Z 18    |
2020-04-02T01:24:16.9378963Z 19 LL |     inner::A::foo(&t);
2020-04-02T01:24:16.9379343Z 
2020-04-02T01:24:16.9379734Z - help: disambiguate the method call for candidate #2
2020-04-02T01:24:16.9380076Z + help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9380315Z 22    |
2020-04-02T01:24:16.9380315Z 22    |
2020-04-02T01:24:16.9380510Z 23 LL |     inner::B::foo(&t);
2020-04-02T01:24:16.9380905Z 
2020-04-02T01:24:16.9381003Z 
2020-04-02T01:24:16.9381209Z The actual stderr differed from the expected stderr.
2020-04-02T01:24:16.9382041Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/trait-alias-ambiguous.stderr
2020-04-02T01:24:16.9382041Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/trait-alias-ambiguous.stderr
2020-04-02T01:24:16.9382697Z To update references, rerun the tests and pass the `--bless` flag
2020-04-02T01:24:16.9383290Z To only update this specific test, also pass `--test-args traits/trait-alias-ambiguous.rs`
2020-04-02T01:24:16.9383753Z error: 1 errors occurred comparing output.
2020-04-02T01:24:16.9383992Z status: exit code: 1
2020-04-02T01:24:16.9383992Z status: exit code: 1
2020-04-02T01:24:16.9385968Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary"
2020-04-02T01:24:16.9387576Z ------------------------------------------
2020-04-02T01:24:16.9387750Z 
2020-04-02T01:24:16.9388100Z ------------------------------------------
2020-04-02T01:24:16.9388318Z stderr:
2020-04-02T01:24:16.9388318Z stderr:
2020-04-02T01:24:16.9388679Z ------------------------------------------
2020-04-02T01:24:16.9388960Z error[E0034]: multiple applicable items in scope
2020-04-02T01:24:16.9389495Z   --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:21:7
2020-04-02T01:24:16.9389748Z    |
2020-04-02T01:24:16.9389938Z LL |     t.foo(); //~ ERROR E0034
2020-04-02T01:24:16.9390192Z    |       ^^^ multiple `foo` found
2020-04-02T01:24:16.9390375Z    |
2020-04-02T01:24:16.9390666Z note: candidate #1 is defined in an impl of the trait `inner::A` for the type `u8`
2020-04-02T01:24:16.9391531Z    |
2020-04-02T01:24:16.9391705Z LL |         fn foo(&self) {}
2020-04-02T01:24:16.9391932Z    |         ^^^^^^^^^^^^^
2020-04-02T01:24:16.9391932Z    |         ^^^^^^^^^^^^^
2020-04-02T01:24:16.9392264Z note: candidate #2 is defined in an impl of the trait `inner::B` for the type `u8`
2020-04-02T01:24:16.9393115Z    |
2020-04-02T01:24:16.9393290Z LL |         fn foo(&self) {}
2020-04-02T01:24:16.9393503Z    |         ^^^^^^^^^^^^^
2020-04-02T01:24:16.9393772Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9393772Z help: disambiguate the associated function for candidate #1
2020-04-02T01:24:16.9394015Z    |
2020-04-02T01:24:16.9394225Z LL |     inner::A::foo(&t); //~ ERROR E0034
2020-04-02T01:24:16.9394756Z help: disambiguate the associated function for candidate #2
2020-04-02T01:24:16.9394985Z    |
2020-04-02T01:24:16.9394985Z    |
2020-04-02T01:24:16.9395196Z LL |     inner::B::foo(&t); //~ ERROR E0034
2020-04-02T01:24:16.9395608Z 
2020-04-02T01:24:16.9395794Z error: aborting due to previous error
2020-04-02T01:24:16.9395961Z 
2020-04-02T01:24:16.9396405Z For more information about this error, try `rustc --explain E0034`.
---
2020-04-02T01:24:16.9405336Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-02T01:24:16.9405754Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-02T01:24:16.9405986Z 
2020-04-02T01:24:16.9406089Z 
2020-04-02T01:24:16.9409943Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-02T01:24:16.9412638Z 
2020-04-02T01:24:16.9412737Z 
2020-04-02T01:24:16.9413231Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-02T01:24:16.9413603Z Build completed unsuccessfully in 1:00:04
2020-04-02T01:24:16.9413603Z Build completed unsuccessfully in 1:00:04
2020-04-02T01:24:16.9413871Z == clock drift check ==
2020-04-02T01:24:16.9414120Z   local time: Thu Apr  2 01:24:16 UTC 2020
2020-04-02T01:24:16.9609082Z   network time: Thu, 02 Apr 2020 01:24:16 GMT
2020-04-02T01:24:17.4275046Z 
2020-04-02T01:24:17.4275046Z 
2020-04-02T01:24:17.4346668Z ##[error]Bash exited with code '1'.
2020-04-02T01:24:17.4359306Z ##[section]Finishing: Run build
2020-04-02T01:24:17.4406942Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70643/merge to s
2020-04-02T01:24:17.4412083Z Task         : Get sources
2020-04-02T01:24:17.4412429Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T01:24:17.4412765Z Version      : 1.0.0
2020-04-02T01:24:17.4412991Z Author       : Microsoft
2020-04-02T01:24:17.4412991Z Author       : Microsoft
2020-04-02T01:24:17.4413343Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T01:24:17.4413769Z ==============================================================================
2020-04-02T01:24:17.7553156Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T01:24:17.7608596Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70643/merge to s
2020-04-02T01:24:17.7701980Z Cleaning up task key
2020-04-02T01:24:17.7703355Z Start cleaning up orphan processes.
2020-04-02T01:24:17.7979951Z Terminate orphan process: pid (3950) (python)
2020-04-02T01:24:17.8150582Z ##[section]Finishing: Finalize Job

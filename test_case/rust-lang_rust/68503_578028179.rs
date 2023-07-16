plain
2020-01-24T07:50:45.0991788Z diff of stderr:
2020-01-24T07:50:45.0999770Z 
2020-01-24T07:50:45.1001286Z 2   --> $DIR/issue-7573.rs:21:9
2020-01-24T07:50:45.1001536Z 3    |
2020-01-24T07:50:45.1001733Z 4 LL |     let mut lines_to_use: Vec<&CrateId> = Vec::new();
2020-01-24T07:50:45.1002212Z -    |         ---------------- `lines_to_use` is declared here, outside of the closure body
2020-01-24T07:50:45.1002690Z +    |         ---------------- `lines_to_use` declared here, outside of the closure body
2020-01-24T07:50:45.1003180Z 6 LL |
2020-01-24T07:50:45.1003350Z 7 LL |     let push_id = |installed_id: &CrateId| {
2020-01-24T07:50:45.1004195Z 8    |                    ------------ `installed_id` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1004962Z 
2020-01-24T07:50:45.1005300Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1005300Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1005920Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-7573.nll/issue-7573.nll.stderr
2020-01-24T07:50:45.1006435Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1006891Z To only update this specific test, also pass `--test-args borrowck/issue-7573.rs`
2020-01-24T07:50:45.1007246Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1007429Z status: exit code: 1
2020-01-24T07:50:45.1007429Z status: exit code: 1
2020-01-24T07:50:45.1008949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-7573.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-7573.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-7573.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1009631Z ------------------------------------------
2020-01-24T07:50:45.1009794Z 
2020-01-24T07:50:45.1010111Z ------------------------------------------
2020-01-24T07:50:45.1010303Z stderr:
2020-01-24T07:50:45.1010303Z stderr:
2020-01-24T07:50:45.1010605Z ------------------------------------------
2020-01-24T07:50:45.1010802Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1011685Z   --> /checkout/src/test/ui/borrowck/issue-7573.rs:21:9
2020-01-24T07:50:45.1011890Z    |
2020-01-24T07:50:45.1012062Z LL |     let mut lines_to_use: Vec<&CrateId> = Vec::new();
2020-01-24T07:50:45.1012716Z    |         ---------------- `lines_to_use` declared here, outside of the closure body
2020-01-24T07:50:45.1013190Z LL |         //~^ NOTE cannot infer an appropriate lifetime
2020-01-24T07:50:45.1013391Z LL |     let push_id = |installed_id: &CrateId| {
2020-01-24T07:50:45.1013813Z    |                    ------------ `installed_id` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1014464Z ...
2020-01-24T07:50:45.1015005Z LL |         lines_to_use.push(installed_id);
2020-01-24T07:50:45.1015226Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `installed_id` escapes the closure body here
2020-01-24T07:50:45.1015745Z error: aborting due to previous error
2020-01-24T07:50:45.1016043Z 
2020-01-24T07:50:45.1016188Z 
2020-01-24T07:50:45.1016579Z ------------------------------------------
2020-01-24T07:50:45.1016579Z ------------------------------------------
2020-01-24T07:50:45.1016779Z 
2020-01-24T07:50:45.1016906Z 
2020-01-24T07:50:45.1017318Z ---- [ui (nll)] ui/borrowck/regions-escape-bound-fn-2.rs stdout ----
2020-01-24T07:50:45.1017517Z diff of stderr:
2020-01-24T07:50:45.1017666Z 
2020-01-24T07:50:45.1018033Z 2   --> $DIR/regions-escape-bound-fn-2.rs:8:18
2020-01-24T07:50:45.1018401Z 3    |
2020-01-24T07:50:45.1018549Z 4 LL |     let mut x = None;
2020-01-24T07:50:45.1019558Z -    |         ----- `x` is declared here, outside of the closure body
2020-01-24T07:50:45.1020150Z +    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1020342Z 6 LL |     with_int(|y| x = Some(y));
2020-01-24T07:50:45.1020688Z 7    |               -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1020979Z 
2020-01-24T07:50:45.1021084Z 
2020-01-24T07:50:45.1021237Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1021237Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1021649Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn-2.nll/regions-escape-bound-fn-2.nll.stderr
2020-01-24T07:50:45.1022174Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1022580Z To only update this specific test, also pass `--test-args borrowck/regions-escape-bound-fn-2.rs`
2020-01-24T07:50:45.1022874Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1023033Z status: exit code: 1
2020-01-24T07:50:45.1023033Z status: exit code: 1
2020-01-24T07:50:45.1024151Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/regions-escape-bound-fn-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn-2.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn-2.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1024844Z ------------------------------------------
2020-01-24T07:50:45.1025003Z 
2020-01-24T07:50:45.1025309Z ------------------------------------------
2020-01-24T07:50:45.1025489Z stderr:
2020-01-24T07:50:45.1025489Z stderr:
2020-01-24T07:50:45.1025814Z ------------------------------------------
2020-01-24T07:50:45.1026077Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1026458Z   --> /checkout/src/test/ui/borrowck/regions-escape-bound-fn-2.rs:8:18
2020-01-24T07:50:45.1026632Z    |
2020-01-24T07:50:45.1026959Z LL |     let mut x = None;
2020-01-24T07:50:45.1027299Z    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1027466Z LL |     with_int(|y| x = Some(y));
2020-01-24T07:50:45.1027813Z    |               -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1028153Z    |               `y` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1028295Z 
2020-01-24T07:50:45.1028509Z error: aborting due to previous error
2020-01-24T07:50:45.1028702Z 
---
2020-01-24T07:50:45.1030744Z diff of stderr:
2020-01-24T07:50:45.1030856Z 
2020-01-24T07:50:45.1031144Z 2   --> $DIR/regions-escape-bound-fn.rs:8:18
2020-01-24T07:50:45.1031334Z 3    |
2020-01-24T07:50:45.1031465Z 4 LL |     let mut x: Option<&isize> = None;
2020-01-24T07:50:45.1031753Z -    |         ----- `x` is declared here, outside of the closure body
2020-01-24T07:50:45.1032141Z +    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1032338Z 6 LL |     with_int(|y| x = Some(y));
2020-01-24T07:50:45.1032653Z 7    |               -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1032869Z 
2020-01-24T07:50:45.1033077Z 
2020-01-24T07:50:45.1033156Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1033156Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1033681Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn.nll/regions-escape-bound-fn.nll.stderr
2020-01-24T07:50:45.1033995Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1034436Z To only update this specific test, also pass `--test-args borrowck/regions-escape-bound-fn.rs`
2020-01-24T07:50:45.1034783Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1034864Z status: exit code: 1
2020-01-24T07:50:45.1034864Z status: exit code: 1
2020-01-24T07:50:45.1035841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/regions-escape-bound-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-bound-fn.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1036621Z ------------------------------------------
2020-01-24T07:50:45.1036790Z 
2020-01-24T07:50:45.1037132Z ------------------------------------------
2020-01-24T07:50:45.1037331Z stderr:
2020-01-24T07:50:45.1037331Z stderr:
2020-01-24T07:50:45.1037823Z ------------------------------------------
2020-01-24T07:50:45.1038026Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1038382Z   --> /checkout/src/test/ui/borrowck/regions-escape-bound-fn.rs:8:18
2020-01-24T07:50:45.1038580Z    |
2020-01-24T07:50:45.1038678Z LL |     let mut x: Option<&isize> = None;
2020-01-24T07:50:45.1038967Z    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1039056Z LL |     with_int(|y| x = Some(y));
2020-01-24T07:50:45.1039445Z    |               -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1039745Z    |               `y` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1039812Z 
2020-01-24T07:50:45.1039900Z error: aborting due to previous error
2020-01-24T07:50:45.1039939Z 
---
2020-01-24T07:50:45.1041148Z diff of stderr:
2020-01-24T07:50:45.1041286Z 
2020-01-24T07:50:45.1041658Z 2   --> $DIR/regions-escape-unboxed-closure.rs:6:23
2020-01-24T07:50:45.1041842Z 3    |
2020-01-24T07:50:45.1041987Z 4 LL |     let mut x: Option<&isize> = None;
2020-01-24T07:50:45.1042252Z -    |         ----- `x` is declared here, outside of the closure body
2020-01-24T07:50:45.1042647Z +    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1042857Z 6 LL |     with_int(&mut |y| x = Some(y));
2020-01-24T07:50:45.1043147Z 7    |                    -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1043404Z 
2020-01-24T07:50:45.1043456Z 
2020-01-24T07:50:45.1043534Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1043534Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1043922Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-unboxed-closure.nll/regions-escape-unboxed-closure.nll.stderr
2020-01-24T07:50:45.1044180Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1044628Z To only update this specific test, also pass `--test-args borrowck/regions-escape-unboxed-closure.rs`
2020-01-24T07:50:45.1044966Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1045043Z status: exit code: 1
2020-01-24T07:50:45.1045043Z status: exit code: 1
2020-01-24T07:50:45.1046026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/regions-escape-unboxed-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-unboxed-closure.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/regions-escape-unboxed-closure.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1046889Z ------------------------------------------
2020-01-24T07:50:45.1046936Z 
2020-01-24T07:50:45.1047151Z ------------------------------------------
2020-01-24T07:50:45.1047211Z stderr:
2020-01-24T07:50:45.1047211Z stderr:
2020-01-24T07:50:45.1047587Z ------------------------------------------
2020-01-24T07:50:45.1047784Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1048106Z   --> /checkout/src/test/ui/borrowck/regions-escape-unboxed-closure.rs:6:23
2020-01-24T07:50:45.1048198Z    |
2020-01-24T07:50:45.1048252Z LL |     let mut x: Option<&isize> = None;
2020-01-24T07:50:45.1048494Z    |         ----- `x` declared here, outside of the closure body
2020-01-24T07:50:45.1048562Z LL |     with_int(&mut |y| x = Some(y));
2020-01-24T07:50:45.1048806Z    |                    -  ^^^^^^^^^^^ `y` escapes the closure body here
2020-01-24T07:50:45.1048958Z    |                    `y` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1049170Z 
2020-01-24T07:50:45.1049231Z error: aborting due to previous error
2020-01-24T07:50:45.1049288Z 
---
2020-01-24T07:50:45.1050420Z diff of stderr:
2020-01-24T07:50:45.1050554Z 
2020-01-24T07:50:45.1050822Z 2   --> $DIR/expect-region-supply-region.rs:18:9
2020-01-24T07:50:45.1051160Z 3    |
2020-01-24T07:50:45.1051259Z 4 LL |     let mut f: Option<&u32> = None;
2020-01-24T07:50:45.1051581Z -    |         ----- `f` is declared here, outside of the closure body
2020-01-24T07:50:45.1052079Z +    |         ----- `f` declared here, outside of the closure body
2020-01-24T07:50:45.1052259Z 6 LL |     closure_expecting_bound(|x| {
2020-01-24T07:50:45.1052762Z 7    |                              - `x` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1052988Z 8 LL |         f = Some(x);
2020-01-24T07:50:45.1053280Z 12   --> $DIR/expect-region-supply-region.rs:28:9
2020-01-24T07:50:45.1053359Z 13    |
2020-01-24T07:50:45.1053359Z 13    |
2020-01-24T07:50:45.1053410Z 14 LL |     let mut f: Option<&u32> = None;
2020-01-24T07:50:45.1053661Z -    |         ----- `f` is declared here, outside of the closure body
2020-01-24T07:50:45.1053888Z +    |         ----- `f` declared here, outside of the closure body
2020-01-24T07:50:45.1053979Z 16 LL |     closure_expecting_bound(|x: &u32| {
2020-01-24T07:50:45.1054394Z 17    |                              - `x` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1054584Z 18 LL |         f = Some(x);
2020-01-24T07:50:45.1054745Z 
2020-01-24T07:50:45.1054839Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1054839Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1055238Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region.nll/expect-region-supply-region.nll.stderr
2020-01-24T07:50:45.1055674Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1056119Z To only update this specific test, also pass `--test-args closures/closure-expected-type/expect-region-supply-region.rs`
2020-01-24T07:50:45.1056578Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1056649Z status: exit code: 1
2020-01-24T07:50:45.1056649Z status: exit code: 1
2020-01-24T07:50:45.1057723Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1058493Z ------------------------------------------
2020-01-24T07:50:45.1058649Z 
2020-01-24T07:50:45.1059725Z ------------------------------------------
2020-01-24T07:50:45.1060169Z stderr:
2020-01-24T07:50:45.1060169Z stderr:
2020-01-24T07:50:45.1065357Z ------------------------------------------
2020-01-24T07:50:45.1065737Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1066329Z   --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region.rs:18:9
2020-01-24T07:50:45.1066436Z    |
2020-01-24T07:50:45.1066542Z LL |     let mut f: Option<&u32> = None;
2020-01-24T07:50:45.1067154Z    |         ----- `f` declared here, outside of the closure body
2020-01-24T07:50:45.1067819Z LL |     closure_expecting_bound(|x| {
2020-01-24T07:50:45.1068821Z    |                              - `x` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1069084Z LL |         f = Some(x); //~ ERROR borrowed data cannot be stored outside of its closure
2020-01-24T07:50:45.1069187Z    |         ^^^^^^^^^^^ `x` escapes the closure body here
2020-01-24T07:50:45.1069488Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1069963Z   --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region.rs:28:9
2020-01-24T07:50:45.1070370Z    |
2020-01-24T07:50:45.1070370Z    |
2020-01-24T07:50:45.1070675Z LL |     let mut f: Option<&u32> = None;
2020-01-24T07:50:45.1071166Z    |         ----- `f` declared here, outside of the closure body
2020-01-24T07:50:45.1071698Z LL |     closure_expecting_bound(|x: &u32| {
2020-01-24T07:50:45.1072413Z    |                              - `x` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1072764Z LL |         f = Some(x); //~ ERROR borrowed data cannot be stored outside of its closure
2020-01-24T07:50:45.1074921Z    |         ^^^^^^^^^^^ `x` escapes the closure body here
2020-01-24T07:50:45.1075295Z error: lifetime may not live long enough
2020-01-24T07:50:45.1076262Z   --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region.rs:37:30
2020-01-24T07:50:45.1076941Z    |
2020-01-24T07:50:45.1076941Z    |
2020-01-24T07:50:45.1077682Z LL | fn expect_bound_supply_named<'x>() {
2020-01-24T07:50:45.1078196Z    |                              -- lifetime `'x` defined here
2020-01-24T07:50:45.1078282Z ...
2020-01-24T07:50:45.1078766Z LL |     closure_expecting_bound(|x: &'x u32| {
2020-01-24T07:50:45.1079398Z    |                              ^  - let's call the lifetime of this reference `'1`
2020-01-24T07:50:45.1079700Z    |                              |
2020-01-24T07:50:45.1080235Z    |                              requires that `'1` must outlive `'x`
2020-01-24T07:50:45.1083064Z error: lifetime may not live long enough
2020-01-24T07:50:45.1083539Z   --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region.rs:37:30
2020-01-24T07:50:45.1083811Z    |
2020-01-24T07:50:45.1083811Z    |
2020-01-24T07:50:45.1084037Z LL | fn expect_bound_supply_named<'x>() {
2020-01-24T07:50:45.1084961Z    |                              -- lifetime `'x` defined here
2020-01-24T07:50:45.1085659Z ...
2020-01-24T07:50:45.1086351Z LL |     closure_expecting_bound(|x: &'x u32| {
2020-01-24T07:50:45.1087435Z    |                              ^ requires that `'x` must outlive `'static`
2020-01-24T07:50:45.1088954Z    = help: consider replacing `'x` with `'static`
2020-01-24T07:50:45.1089015Z 
2020-01-24T07:50:45.1089107Z error: aborting due to 4 previous errors
2020-01-24T07:50:45.1089156Z 
2020-01-24T07:50:45.1089156Z 
2020-01-24T07:50:45.1089193Z 
2020-01-24T07:50:45.1089728Z ------------------------------------------
2020-01-24T07:50:45.1089945Z 
2020-01-24T07:50:45.1090012Z 
2020-01-24T07:50:45.1090544Z ---- [ui (nll)] ui/regions/regions-bounded-method-type-parameters-trait-bound.rs stdout ----
2020-01-24T07:50:45.1090805Z diff of stderr:
2020-01-24T07:50:45.1091661Z 
2020-01-24T07:50:45.1092464Z 4 LL | fn caller2<'a,'b,F:Foo<'a>>(a: Inv<'a>, b: Inv<'b>, f: F) {
2020-01-24T07:50:45.1096825Z 5    |                             -           - `b` is a reference that is only valid in the function body
2020-01-24T07:50:45.1097056Z 6    |                             |
2020-01-24T07:50:45.1097487Z -    |                             `a` is declared here, outside of the function body
2020-01-24T07:50:45.1097603Z +    |                             `a` declared here, outside of the function body
2020-01-24T07:50:45.1098001Z 8 LL |     // Here the value provided for 'y is 'b, and hence 'b:'a does not hold.
2020-01-24T07:50:45.1098098Z 9 LL |     f.method(b);
2020-01-24T07:50:45.1098371Z 10    |     ^^^^^^^^^^^ `b` escapes the function body here
2020-01-24T07:50:45.1098482Z 
2020-01-24T07:50:45.1098567Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1098567Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1099240Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll/regions-bounded-method-type-parameters-trait-bound.nll.stderr
2020-01-24T07:50:45.1100153Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1101085Z To only update this specific test, also pass `--test-args regions/regions-bounded-method-type-parameters-trait-bound.rs`
2020-01-24T07:50:45.1101245Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1101494Z status: exit code: 1
2020-01-24T07:50:45.1101494Z status: exit code: 1
2020-01-24T07:50:45.1103460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1104190Z ------------------------------------------
2020-01-24T07:50:45.1104250Z 
2020-01-24T07:50:45.1104566Z ------------------------------------------
2020-01-24T07:50:45.1104651Z stderr:
2020-01-24T07:50:45.1104651Z stderr:
2020-01-24T07:50:45.1104958Z ------------------------------------------
2020-01-24T07:50:45.1105053Z error[E0521]: borrowed data escapes outside of function
2020-01-24T07:50:45.1105611Z   --> /checkout/src/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.rs:20:5
2020-01-24T07:50:45.1105710Z    |
2020-01-24T07:50:45.1106363Z LL | fn caller2<'a,'b,F:Foo<'a>>(a: Inv<'a>, b: Inv<'b>, f: F) {
2020-01-24T07:50:45.1106911Z    |                             -           - `b` is a reference that is only valid in the function body
2020-01-24T07:50:45.1107003Z    |                             |
2020-01-24T07:50:45.1107099Z    |                             `a` declared here, outside of the function body
2020-01-24T07:50:45.1107551Z LL |     // Here the value provided for 'y is 'b, and hence 'b:'a does not hold.
2020-01-24T07:50:45.1107684Z LL |     f.method(b); //~ ERROR lifetime mismatch [E0623]
2020-01-24T07:50:45.1107761Z    |     ^^^^^^^^^^^ `b` escapes the function body here
2020-01-24T07:50:45.1108154Z    = help: consider adding the following bound: `'b: 'a`
2020-01-24T07:50:45.1108227Z 
2020-01-24T07:50:45.1108286Z error: aborting due to previous error
2020-01-24T07:50:45.1108347Z 
---
2020-01-24T07:50:45.1109502Z diff of stderr:
2020-01-24T07:50:45.1109564Z 
2020-01-24T07:50:45.1110166Z 2   --> $DIR/regions-nested-fns.rs:10:9
2020-01-24T07:50:45.1110262Z 3    |
2020-01-24T07:50:45.1110321Z 4 LL |     let mut ay = &y;
2020-01-24T07:50:45.1110832Z -    |         ------ `ay` is declared here, outside of the closure body
2020-01-24T07:50:45.1111309Z +    |         ------ `ay` declared here, outside of the closure body
2020-01-24T07:50:45.1111423Z 6 LL | 
2020-01-24T07:50:45.1111722Z 7 LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
2020-01-24T07:50:45.1112289Z 8    |                                                           - `z` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1112426Z 
2020-01-24T07:50:45.1112494Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1112494Z The actual stderr differed from the expected stderr.
2020-01-24T07:50:45.1112910Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/regions-nested-fns.nll.stderr
2020-01-24T07:50:45.1113385Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T07:50:45.1114109Z To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`
2020-01-24T07:50:45.1114278Z error: 1 errors occurred comparing output.
2020-01-24T07:50:45.1114517Z status: exit code: 1
2020-01-24T07:50:45.1114517Z status: exit code: 1
2020-01-24T07:50:45.1116117Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/auxiliary" "-A" "unused"
2020-01-24T07:50:45.1117112Z ------------------------------------------
2020-01-24T07:50:45.1117169Z 
2020-01-24T07:50:45.1117639Z ------------------------------------------
2020-01-24T07:50:45.1117745Z stderr:
2020-01-24T07:50:45.1117745Z stderr:
2020-01-24T07:50:45.1118032Z ------------------------------------------
2020-01-24T07:50:45.1118118Z error[E0521]: borrowed data escapes outside of closure
2020-01-24T07:50:45.1118808Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:10:9
2020-01-24T07:50:45.1118889Z    |
2020-01-24T07:50:45.1118973Z LL |     let mut ay = &y; //~ ERROR E0495
2020-01-24T07:50:45.1119262Z    |         ------ `ay` declared here, outside of the closure body
2020-01-24T07:50:45.1119362Z LL | 
2020-01-24T07:50:45.1119634Z LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
2020-01-24T07:50:45.1120007Z    |                                                           - `z` is a reference that is only valid in the closure body
2020-01-24T07:50:45.1120134Z ...
2020-01-24T07:50:45.1120197Z LL |         ay = z;
2020-01-24T07:50:45.1120286Z    |         ^^^^^^ `z` escapes the closure body here
2020-01-24T07:50:45.1120584Z error[E0597]: `y` does not live long enough
2020-01-24T07:50:45.1120954Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:5:18
2020-01-24T07:50:45.1121067Z    |
2020-01-24T07:50:45.1121067Z    |
2020-01-24T07:50:45.1121126Z LL |     let mut ay = &y; //~ ERROR E0495
2020-01-24T07:50:45.1121219Z    |                  ^^ borrowed value does not live long enough
2020-01-24T07:50:45.1121289Z ...
2020-01-24T07:50:45.1121540Z LL |         if false { return ay; }
2020-01-24T07:50:45.1122275Z ...
2020-01-24T07:50:45.1122336Z LL | }
2020-01-24T07:50:45.1122795Z    | - `y` dropped here while still borrowed
2020-01-24T07:50:45.1122936Z 
2020-01-24T07:50:45.1122936Z 
2020-01-24T07:50:45.1123026Z error[E0597]: `y` does not live long enough
2020-01-24T07:50:45.1123339Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:9:15
2020-01-24T07:50:45.1123437Z    |
2020-01-24T07:50:45.1123710Z LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
2020-01-24T07:50:45.1124049Z    |                                                          --- value captured here
2020-01-24T07:50:45.1124154Z LL |         ay = x;
2020-01-24T07:50:45.1124219Z LL |         ay = &y;
2020-01-24T07:50:45.1124311Z    |               ^ borrowed value does not live long enough
2020-01-24T07:50:45.1124386Z ...
2020-01-24T07:50:45.1124463Z LL |         if false { return ay; }
2020-01-24T07:50:45.1124897Z ...
2020-01-24T07:50:45.1124953Z LL | }
2020-01-24T07:50:45.1125221Z    | - `y` dropped here while still borrowed
2020-01-24T07:50:45.1125281Z 
2020-01-24T07:50:45.1125281Z 
2020-01-24T07:50:45.1125364Z error: lifetime may not live long enough
2020-01-24T07:50:45.1125652Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:14:27
2020-01-24T07:50:45.1125751Z    |
2020-01-24T07:50:45.1125988Z LL | fn nested<'x>(x: &'x isize) {
2020-01-24T07:50:45.1126266Z    |           -- lifetime `'x` defined here
2020-01-24T07:50:45.1126337Z ...
2020-01-24T07:50:45.1126433Z LL |         if false { return x; } //~ ERROR E0312
2020-01-24T07:50:45.1126751Z    |                           ^ returning this value requires that `'x` must outlive `'static`
2020-01-24T07:50:45.1127136Z    = help: consider replacing `'x` with `'static`
2020-01-24T07:50:45.1127188Z 
2020-01-24T07:50:45.1127250Z error: aborting due to 4 previous errors
2020-01-24T07:50:45.1127314Z 
---
2020-01-24T07:50:45.1131627Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-24T07:50:45.1131770Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-24T07:50:45.1132012Z 
2020-01-24T07:50:45.1132050Z 
2020-01-24T07:50:45.1135461Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-01-24T07:50:45.1136294Z 
2020-01-24T07:50:45.1136340Z 
2020-01-24T07:50:45.1136440Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-24T07:50:45.1136529Z Build completed unsuccessfully in 1:33:25
2020-01-24T07:50:45.1136529Z Build completed unsuccessfully in 1:33:25
2020-01-24T07:50:45.1136630Z == clock drift check ==
2020-01-24T07:50:45.1136727Z   local time: Fri Jan 24 07:50:44 UTC 2020
2020-01-24T07:50:45.1136819Z   network time: Fri, 24 Jan 2020 07:50:44 GMT
2020-01-24T07:50:45.1136916Z == end clock drift check ==
2020-01-24T07:50:45.1506472Z 
2020-01-24T07:50:45.1670250Z ##[error]Bash exited with code '1'.
2020-01-24T07:50:45.1705868Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-24T07:50:45.1707618Z ==============================================================================
2020-01-24T07:50:45.1707694Z Task         : Get sources
2020-01-24T07:50:45.1707784Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

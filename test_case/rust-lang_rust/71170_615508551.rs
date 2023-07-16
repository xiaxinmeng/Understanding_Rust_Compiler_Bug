plain
2020-04-17T23:04:02.3460924Z ========================== Starting Command Output ===========================
2020-04-17T23:04:02.3465927Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/79699dba-cfa6-4d5a-890f-7b310ed2fc95.sh
2020-04-17T23:04:02.3466612Z 
2020-04-17T23:04:02.3471869Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T23:04:02.3491350Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-17T23:04:02.3495460Z Task         : Get sources
2020-04-17T23:04:02.3495782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T23:04:02.3496077Z Version      : 1.0.0
2020-04-17T23:04:02.3496280Z Author       : Microsoft
---
2020-04-17T23:04:03.3320308Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T23:04:03.3326218Z ##[command]git config gc.auto 0
2020-04-17T23:04:03.3330313Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T23:04:03.3334325Z ##[command]git config --get-all http.proxy
2020-04-17T23:04:03.3341500Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71170/merge:refs/remotes/pull/71170/merge
---
2020-04-17T23:06:21.4835706Z  ---> f58a2bb1e753
2020-04-17T23:06:21.4836588Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-17T23:06:21.4837233Z  ---> Using cache
2020-04-17T23:06:21.4837586Z  ---> d079cc6b6db8
2020-04-17T23:06:21.4842037Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-17T23:06:21.4843396Z  ---> 4183ca46ee56
2020-04-17T23:06:21.4843667Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-17T23:06:21.4844069Z  ---> Using cache
2020-04-17T23:06:21.4844423Z  ---> 69e7f8a2a2fb
---
2020-04-17T23:06:21.5333552Z Looks like docker image is the same as before, not uploading
2020-04-17T23:06:29.0231307Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T23:06:29.0555419Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T23:06:29.0589806Z == clock drift check ==
2020-04-17T23:06:29.0597553Z   local time: Fri Apr 17 23:06:29 UTC 2020
2020-04-17T23:06:29.1616997Z   network time: Fri, 17 Apr 2020 23:06:29 GMT
2020-04-17T23:06:29.1631043Z Starting sccache server...
2020-04-17T23:06:29.2509474Z configure: processing command line
2020-04-17T23:06:29.2510286Z configure: 
2020-04-17T23:06:29.2511604Z configure: rust.dist-src        := False
---
2020-04-17T23:12:05.8990008Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T23:12:07.4773666Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T23:12:09.1839137Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T23:12:10.7507345Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T23:12:20.1911918Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T23:12:22.8432226Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T23:12:27.7519743Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T23:12:32.4627496Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T23:12:42.6344929Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T23:32:38.0585394Z    Compiling autocfg v0.1.7
2020-04-17T23:32:38.8159553Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8160265Z    --> src/libcore/iter/adapters/mod.rs:167:22
2020-04-17T23:32:38.8160686Z     |
2020-04-17T23:32:38.8161344Z 167 |     move |acc, &elt| f(acc, elt)
2020-04-17T23:32:38.8162421Z 
2020-04-17T23:32:38.8181935Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8182603Z    --> src/libcore/iter/adapters/mod.rs:171:22
2020-04-17T23:32:38.8183044Z     |
2020-04-17T23:32:38.8183044Z     |
2020-04-17T23:32:38.8183547Z 171 |     move |acc, &elt| f(acc, elt)
2020-04-17T23:32:38.8184630Z 
2020-04-17T23:32:38.8221422Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8222050Z    --> src/libcore/iter/adapters/mod.rs:313:21
2020-04-17T23:32:38.8222468Z     |
2020-04-17T23:32:38.8222468Z     |
2020-04-17T23:32:38.8222954Z 313 |     move |acc, elt| f(acc, elt.clone())
2020-04-17T23:32:38.8223986Z 
2020-04-17T23:32:38.8230222Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8230824Z    --> src/libcore/iter/adapters/mod.rs:313:28
2020-04-17T23:32:38.8231250Z     |
2020-04-17T23:32:38.8231250Z     |
2020-04-17T23:32:38.8231749Z 313 |     move |acc, elt| f(acc, elt.clone())
2020-04-17T23:32:38.8232768Z 
2020-04-17T23:32:38.8307016Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8307613Z    --> src/libcore/iter/adapters/mod.rs:635:21
2020-04-17T23:32:38.8308025Z     |
2020-04-17T23:32:38.8308025Z     |
2020-04-17T23:32:38.8308513Z 635 |             move || iter.nth(step)
2020-04-17T23:32:38.8309672Z 
2020-04-17T23:32:38.8338554Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8339441Z    --> src/libcore/iter/adapters/mod.rs:695:21
2020-04-17T23:32:38.8339890Z     |
2020-04-17T23:32:38.8339890Z     |
2020-04-17T23:32:38.8340426Z 695 |             move || iter.nth_back(step)
2020-04-17T23:32:38.8341509Z 
2020-04-17T23:32:38.8369326Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8370098Z    --> src/libcore/iter/adapters/mod.rs:786:21
2020-04-17T23:32:38.8370631Z     |
2020-04-17T23:32:38.8370631Z     |
2020-04-17T23:32:38.8371284Z 786 |     move |acc, elt| g(acc, f(elt))
2020-04-17T23:32:38.8372730Z 
2020-04-17T23:32:38.8380497Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8381247Z    --> src/libcore/iter/adapters/mod.rs:786:28
2020-04-17T23:32:38.8381772Z     |
2020-04-17T23:32:38.8381772Z     |
2020-04-17T23:32:38.8382398Z 786 |     move |acc, elt| g(acc, f(elt))
2020-04-17T23:32:38.8383669Z 
2020-04-17T23:32:38.8417510Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8418329Z    --> src/libcore/iter/adapters/mod.rs:793:21
2020-04-17T23:32:38.8418877Z     |
2020-04-17T23:32:38.8418877Z     |
2020-04-17T23:32:38.8419519Z 793 |     move |acc, elt| g(acc, f(elt))
2020-04-17T23:32:38.8420796Z 
2020-04-17T23:32:38.8428008Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8428798Z    --> src/libcore/iter/adapters/mod.rs:793:28
2020-04-17T23:32:38.8429351Z     |
2020-04-17T23:32:38.8429351Z     |
2020-04-17T23:32:38.8429962Z 793 |     move |acc, elt| g(acc, f(elt))
2020-04-17T23:32:38.8431249Z 
2020-04-17T23:32:38.8481489Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8482312Z    --> src/libcore/iter/adapters/mod.rs:928:25
2020-04-17T23:32:38.8482881Z     |
2020-04-17T23:32:38.8482881Z     |
2020-04-17T23:32:38.8483599Z 928 |     move |acc, item| if predicate(&item) { fold(acc, item) } else { acc }
2020-04-17T23:32:38.8484984Z 
2020-04-17T23:32:38.8492250Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8493258Z    --> src/libcore/iter/adapters/mod.rs:928:44
2020-04-17T23:32:38.8493818Z     |
2020-04-17T23:32:38.8493818Z     |
2020-04-17T23:32:38.8494545Z 928 |     move |acc, item| if predicate(&item) { fold(acc, item) } else { acc }
2020-04-17T23:32:38.8496020Z 
2020-04-17T23:32:38.8541112Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8541928Z    --> src/libcore/iter/adapters/mod.rs:935:25
2020-04-17T23:32:38.8542499Z     |
2020-04-17T23:32:38.8542499Z     |
2020-04-17T23:32:38.8543322Z 935 |     move |acc, item| if predicate(&item) { fold(acc, item) } else { R::from_ok(acc) }
2020-04-17T23:32:38.8544994Z 
2020-04-17T23:32:38.8552143Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8552970Z    --> src/libcore/iter/adapters/mod.rs:935:44
2020-04-17T23:32:38.8553506Z     |
2020-04-17T23:32:38.8553506Z     |
2020-04-17T23:32:38.8554329Z 935 |     move |acc, item| if predicate(&item) { fold(acc, item) } else { R::from_ok(acc) }
2020-04-17T23:32:38.8555907Z 
2020-04-17T23:32:38.8565134Z error[E0382]: use of moved value
2020-04-17T23:32:38.8565908Z    --> src/libcore/iter/adapters/mod.rs:935:69
2020-04-17T23:32:38.8566449Z     |
2020-04-17T23:32:38.8566449Z     |
2020-04-17T23:32:38.8567277Z 935 |     move |acc, item| if predicate(&item) { fold(acc, item) } else { R::from_ok(acc) }
2020-04-17T23:32:38.8569585Z     |                                                                     |          | |
2020-04-17T23:32:38.8570846Z     |                                                                     |          | value moved here
2020-04-17T23:32:38.8570846Z     |                                                                     |          | value moved here
2020-04-17T23:32:38.8572224Z     |                                                                     |          move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.8573402Z     |                                                                     value used here after move
2020-04-17T23:32:38.8574629Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.8575141Z     |
2020-04-17T23:32:38.8575141Z     |
2020-04-17T23:32:38.8575864Z 931 | fn filter_try_fold<'a, T, Acc: Copy, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.8577201Z 
2020-04-17T23:32:38.8613095Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8614567Z    --> src/libcore/iter/adapters/mod.rs:971:22
2020-04-17T23:32:38.8615121Z     |
2020-04-17T23:32:38.8615121Z     |
2020-04-17T23:32:38.8615758Z 971 |             move |x| predicate(&x) as usize
2020-04-17T23:32:38.8617081Z 
2020-04-17T23:32:38.8655557Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8656390Z     --> src/libcore/iter/adapters/mod.rs:1059:28
2020-04-17T23:32:38.8656956Z      |
2020-04-17T23:32:38.8656956Z      |
2020-04-17T23:32:38.8657668Z 1059 |     move |acc, item| match f(item) {
2020-04-17T23:32:38.8659135Z 
2020-04-17T23:32:38.8659768Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8660482Z     --> src/libcore/iter/adapters/mod.rs:1060:20
2020-04-17T23:32:38.8661023Z      |
2020-04-17T23:32:38.8661023Z      |
2020-04-17T23:32:38.8661629Z 1060 |         Some(x) => fold(acc, x),
2020-04-17T23:32:38.8662978Z 
2020-04-17T23:32:38.8704893Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8705727Z     --> src/libcore/iter/adapters/mod.rs:1069:28
2020-04-17T23:32:38.8706477Z      |
2020-04-17T23:32:38.8706477Z      |
2020-04-17T23:32:38.8707105Z 1069 |     move |acc, item| match f(item) {
2020-04-17T23:32:38.8708410Z 
2020-04-17T23:32:38.8708974Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8709687Z     --> src/libcore/iter/adapters/mod.rs:1070:20
2020-04-17T23:32:38.8710247Z      |
2020-04-17T23:32:38.8710247Z      |
2020-04-17T23:32:38.8710850Z 1070 |         Some(x) => fold(acc, x),
2020-04-17T23:32:38.8712137Z 
2020-04-17T23:32:38.8720919Z error[E0382]: use of moved value
2020-04-17T23:32:38.8721664Z     --> src/libcore/iter/adapters/mod.rs:1071:17
2020-04-17T23:32:38.8722239Z      |
2020-04-17T23:32:38.8722239Z      |
2020-04-17T23:32:38.8722891Z 1071 |         None => R::from_ok(acc),
2020-04-17T23:32:38.8724717Z      |                 |          | |
2020-04-17T23:32:38.8725943Z      |                 |          | value moved here
2020-04-17T23:32:38.8725943Z      |                 |          | value moved here
2020-04-17T23:32:38.8727093Z      |                 |          move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.8728017Z      |                 value used here after move
2020-04-17T23:32:38.8729163Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.8729677Z      |
2020-04-17T23:32:38.8729677Z      |
2020-04-17T23:32:38.8730442Z 1065 | fn filter_map_try_fold<'a, T, B, Acc: Copy, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.8731609Z 
2020-04-17T23:32:38.8784093Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8784825Z     --> src/libcore/iter/adapters/mod.rs:1123:32
2020-04-17T23:32:38.8785278Z      |
2020-04-17T23:32:38.8785278Z      |
2020-04-17T23:32:38.8785819Z 1123 |             move |(), x| match f(x) {
2020-04-17T23:32:38.8787034Z 
2020-04-17T23:32:38.8821787Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8822520Z     --> src/libcore/iter/adapters/mod.rs:1231:27
2020-04-17T23:32:38.8823018Z      |
2020-04-17T23:32:38.8823018Z      |
2020-04-17T23:32:38.8823591Z 1231 |                 let acc = fold(acc, (*count, item));
2020-04-17T23:32:38.8824817Z 
2020-04-17T23:32:38.8825305Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8825935Z     --> src/libcore/iter/adapters/mod.rs:1233:17
2020-04-17T23:32:38.8826559Z      |
2020-04-17T23:32:38.8826559Z      |
2020-04-17T23:32:38.8827157Z 1233 |                 AddAssign::add_assign(count, 1);
2020-04-17T23:32:38.8828642Z 
2020-04-17T23:32:38.8872906Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8873677Z     --> src/libcore/iter/adapters/mod.rs:1252:27
2020-04-17T23:32:38.8874157Z      |
2020-04-17T23:32:38.8874157Z      |
2020-04-17T23:32:38.8874983Z 1252 |                 let acc = fold(acc, (count, item));
2020-04-17T23:32:38.8876215Z 
2020-04-17T23:32:38.8876732Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8877802Z     --> src/libcore/iter/adapters/mod.rs:1254:17
2020-04-17T23:32:38.8878271Z      |
2020-04-17T23:32:38.8878271Z      |
2020-04-17T23:32:38.8878928Z 1254 |                 AddAssign::add_assign(&mut count, 1);
2020-04-17T23:32:38.8880217Z 
2020-04-17T23:32:38.8907402Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8908116Z     --> src/libcore/iter/adapters/mod.rs:1301:17
2020-04-17T23:32:38.8908581Z      |
2020-04-17T23:32:38.8908581Z      |
2020-04-17T23:32:38.8909140Z 1301 |                 fold(acc, (count, item))
2020-04-17T23:32:38.8910288Z 
2020-04-17T23:32:38.8931391Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8932219Z     --> src/libcore/iter/adapters/mod.rs:1322:17
2020-04-17T23:32:38.8932711Z      |
2020-04-17T23:32:38.8932711Z      |
2020-04-17T23:32:38.8933305Z 1322 |                 fold(acc, (count, item))
2020-04-17T23:32:38.8934480Z 
2020-04-17T23:32:38.8975390Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.8976233Z     --> src/libcore/iter/adapters/mod.rs:1622:30
2020-04-17T23:32:38.8976709Z      |
2020-04-17T23:32:38.8976709Z      |
2020-04-17T23:32:38.8977238Z 1622 |                 if *flag || !pred(x) {
2020-04-17T23:32:38.8978331Z 
2020-04-17T23:32:38.9018267Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9018981Z     --> src/libcore/iter/adapters/mod.rs:1754:20
2020-04-17T23:32:38.9019477Z      |
2020-04-17T23:32:38.9019477Z      |
2020-04-17T23:32:38.9019988Z 1754 |                 if p(&x) {
2020-04-17T23:32:38.9021235Z 
2020-04-17T23:32:38.9032963Z error[E0382]: use of moved value
2020-04-17T23:32:38.9033763Z     --> src/libcore/iter/adapters/mod.rs:1755:21
2020-04-17T23:32:38.9034255Z      |
2020-04-17T23:32:38.9034255Z      |
2020-04-17T23:32:38.9034877Z 1755 |                     LoopState::from_try(fold(acc, x))
2020-04-17T23:32:38.9036620Z      |                     |                   |          |
2020-04-17T23:32:38.9037883Z      |                     |                   |          value moved here
2020-04-17T23:32:38.9038907Z      |                     |                   move occurs because value has type `R`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9039694Z      |                     value used here after move
2020-04-17T23:32:38.9039694Z      |                     value used here after move
2020-04-17T23:32:38.9040178Z      |
2020-04-17T23:32:38.9040640Z help: consider further restricting this bound
2020-04-17T23:32:38.9041063Z      |
2020-04-17T23:32:38.9041676Z 1748 |         fn check<'a, T, Acc, R: Try<Ok = Acc> + Copy>(
2020-04-17T23:32:38.9042650Z 
2020-04-17T23:32:38.9043129Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9043811Z     --> src/libcore/iter/adapters/mod.rs:1755:41
2020-04-17T23:32:38.9044278Z      |
2020-04-17T23:32:38.9044278Z      |
2020-04-17T23:32:38.9044885Z 1755 |                     LoopState::from_try(fold(acc, x))
2020-04-17T23:32:38.9046076Z 
2020-04-17T23:32:38.9046483Z error[E0382]: use of moved value
2020-04-17T23:32:38.9047020Z     --> src/libcore/iter/adapters/mod.rs:1758:38
2020-04-17T23:32:38.9047460Z      |
2020-04-17T23:32:38.9047460Z      |
2020-04-17T23:32:38.9048077Z 1758 |                     LoopState::Break(Try::from_ok(acc))
2020-04-17T23:32:38.9049756Z      |                                      |            | |
2020-04-17T23:32:38.9050685Z      |                                      |            | value moved here
2020-04-17T23:32:38.9050685Z      |                                      |            | value moved here
2020-04-17T23:32:38.9051701Z      |                                      |            move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9052585Z      |                                      value used here after move
2020-04-17T23:32:38.9053572Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.9054002Z      |
2020-04-17T23:32:38.9054002Z      |
2020-04-17T23:32:38.9054597Z 1748 |         fn check<'a, T, Acc: Copy, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.9055509Z 
2020-04-17T23:32:38.9101927Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9102686Z     --> src/libcore/iter/adapters/mod.rs:1998:25
2020-04-17T23:32:38.9103572Z      |
2020-04-17T23:32:38.9103572Z      |
2020-04-17T23:32:38.9104216Z 1998 |                 let r = fold(acc, x);
2020-04-17T23:32:38.9105362Z 
2020-04-17T23:32:38.9111963Z error[E0382]: use of moved value
2020-04-17T23:32:38.9113236Z     --> src/libcore/iter/adapters/mod.rs:1999:58
2020-04-17T23:32:38.9113692Z      |
2020-04-17T23:32:38.9113692Z      |
2020-04-17T23:32:38.9114368Z 1999 |                 if n == 0 { LoopState::Break(r) } else { LoopState::from_try(r) }
2020-04-17T23:32:38.9115428Z      |                                                          ^^^^^^^^^^^^^^^^^^^^-^
2020-04-17T23:32:38.9117704Z      |                                                          |                   value moved here
2020-04-17T23:32:38.9118851Z      |                                                          |                   move occurs because value has type `R`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9120463Z      |                                                          value used here after move
2020-04-17T23:32:38.9121053Z      |
2020-04-17T23:32:38.9121053Z      |
2020-04-17T23:32:38.9121571Z help: consider further restricting this bound
2020-04-17T23:32:38.9122757Z      |
2020-04-17T23:32:38.9123360Z 1992 |         fn check<T, Acc, R: Try<Ok = Acc> + Copy>(
2020-04-17T23:32:38.9124455Z 
2020-04-17T23:32:38.9167046Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9167754Z     --> src/libcore/iter/adapters/mod.rs:2097:25
2020-04-17T23:32:38.9168937Z      |
2020-04-17T23:32:38.9168937Z      |
2020-04-17T23:32:38.9169472Z 2097 |                 let r = fold(acc, x);
2020-04-17T23:32:38.9170586Z 
2020-04-17T23:32:38.9179446Z error[E0382]: use of moved value
2020-04-17T23:32:38.9180217Z     --> src/libcore/iter/adapters/mod.rs:2098:59
2020-04-17T23:32:38.9180713Z      |
2020-04-17T23:32:38.9180713Z      |
2020-04-17T23:32:38.9181467Z 2098 |                 if *n == 0 { LoopState::Break(r) } else { LoopState::from_try(r) }
2020-04-17T23:32:38.9182466Z      |                                                           ^^^^^^^^^^^^^^^^^^^^-^
2020-04-17T23:32:38.9184617Z      |                                                           |                   value moved here
2020-04-17T23:32:38.9186343Z      |                                                           |                   move occurs because value has type `R`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9187398Z      |                                                           value used here after move
2020-04-17T23:32:38.9187979Z      |
2020-04-17T23:32:38.9187979Z      |
2020-04-17T23:32:38.9188477Z help: consider further restricting this bound
2020-04-17T23:32:38.9189036Z      |
2020-04-17T23:32:38.9189654Z 2091 |         fn check<'a, T, Acc, R: Try<Ok = Acc> + Copy>(
2020-04-17T23:32:38.9190671Z 
2020-04-17T23:32:38.9263193Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9263927Z     --> src/libcore/iter/adapters/mod.rs:2231:33
2020-04-17T23:32:38.9264375Z      |
2020-04-17T23:32:38.9264375Z      |
2020-04-17T23:32:38.9264929Z 2231 |             move |acc, x| match f(state, x) {
2020-04-17T23:32:38.9266040Z 
2020-04-17T23:32:38.9266608Z error[E0382]: use of moved value
2020-04-17T23:32:38.9267166Z     --> src/libcore/iter/adapters/mod.rs:2232:42
2020-04-17T23:32:38.9267598Z      |
2020-04-17T23:32:38.9267598Z      |
2020-04-17T23:32:38.9292006Z 2232 |                 None => LoopState::Break(Try::from_ok(acc)),
2020-04-17T23:32:38.9293851Z      |                                          |            | |
2020-04-17T23:32:38.9294797Z      |                                          |            | value moved here
2020-04-17T23:32:38.9294797Z      |                                          |            | value moved here
2020-04-17T23:32:38.9295828Z      |                                          |            move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9296718Z      |                                          value used here after move
2020-04-17T23:32:38.9297700Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.9298130Z      |
2020-04-17T23:32:38.9298130Z      |
2020-04-17T23:32:38.9298743Z 2226 |         fn scan<'a, T, St, B, Acc: Copy, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.9299693Z 
2020-04-17T23:32:38.9300085Z error[E0382]: use of moved value
2020-04-17T23:32:38.9300831Z     --> src/libcore/iter/adapters/mod.rs:2233:28
2020-04-17T23:32:38.9301267Z      |
2020-04-17T23:32:38.9301267Z      |
2020-04-17T23:32:38.9301886Z 2233 |                 Some(x) => LoopState::from_try(fold(acc, x)),
2020-04-17T23:32:38.9303620Z      |                            |                   |          |
2020-04-17T23:32:38.9304560Z      |                            |                   |          value moved here
2020-04-17T23:32:38.9305602Z      |                            |                   move occurs because value has type `R`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9306674Z      |                            value used here after move
2020-04-17T23:32:38.9306674Z      |                            value used here after move
2020-04-17T23:32:38.9307208Z      |
2020-04-17T23:32:38.9307668Z help: consider further restricting this bound
2020-04-17T23:32:38.9308092Z      |
2020-04-17T23:32:38.9308717Z 2226 |         fn scan<'a, T, St, B, Acc, R: Try<Ok = Acc> + Copy>(
2020-04-17T23:32:38.9309711Z 
2020-04-17T23:32:38.9310189Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9310791Z     --> src/libcore/iter/adapters/mod.rs:2233:48
2020-04-17T23:32:38.9311222Z      |
2020-04-17T23:32:38.9311222Z      |
2020-04-17T23:32:38.9311853Z 2233 |                 Some(x) => LoopState::from_try(fold(acc, x)),
2020-04-17T23:32:38.9313088Z 
2020-04-17T23:32:38.9323872Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9324649Z     --> src/libcore/iter/adapters/mod.rs:2290:9
2020-04-17T23:32:38.9347359Z      |
2020-04-17T23:32:38.9347359Z      |
2020-04-17T23:32:38.9347901Z 2290 |         f(&item);
2020-04-17T23:32:38.9348744Z      |         ^^^^^^^^ use of possibly-uninitialized value
2020-04-17T23:32:38.9349161Z 
2020-04-17T23:32:38.9349662Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9350222Z     --> src/libcore/iter/adapters/mod.rs:2291:9
2020-04-17T23:32:38.9350650Z      |
2020-04-17T23:32:38.9351103Z 2291 |         fold(acc, item)
2020-04-17T23:32:38.9352057Z 
2020-04-17T23:32:38.9364135Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9365029Z     --> src/libcore/iter/adapters/mod.rs:2300:9
2020-04-17T23:32:38.9365485Z      |
2020-04-17T23:32:38.9365485Z      |
2020-04-17T23:32:38.9365960Z 2300 |         f(&item);
2020-04-17T23:32:38.9366667Z      |         ^^^^^^^^ use of possibly-uninitialized value
2020-04-17T23:32:38.9366967Z 
2020-04-17T23:32:38.9367443Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9368148Z     --> src/libcore/iter/adapters/mod.rs:2301:9
2020-04-17T23:32:38.9368558Z      |
2020-04-17T23:32:38.9369011Z 2301 |         fold(acc, item)
2020-04-17T23:32:38.9369976Z 
2020-04-17T23:32:38.9523514Z error[E0382]: use of moved value
2020-04-17T23:32:38.9524319Z    --> src/libcore/iter/adapters/flatten.rs:303:31
2020-04-17T23:32:38.9524798Z     |
2020-04-17T23:32:38.9524798Z     |
2020-04-17T23:32:38.9525294Z 303 |                 let mut mid = x.into_iter();
2020-04-17T23:32:38.9525953Z     |                               -^^^^^^^^^^^^
2020-04-17T23:32:38.9527131Z     |                               value moved here
2020-04-17T23:32:38.9527747Z     |                               value used here after move
2020-04-17T23:32:38.9528506Z     |                               move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9529052Z     |
2020-04-17T23:32:38.9529052Z     |
2020-04-17T23:32:38.9529491Z help: consider further restricting this bound
2020-04-17T23:32:38.9529891Z     |
2020-04-17T23:32:38.9530769Z 298 |         fn flatten<'a, T: IntoIterator + Copy, Acc, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.9531912Z 
2020-04-17T23:32:38.9532429Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9533073Z    --> src/libcore/iter/adapters/flatten.rs:304:25
2020-04-17T23:32:38.9533552Z     |
2020-04-17T23:32:38.9533552Z     |
2020-04-17T23:32:38.9534151Z 304 |                 let r = mid.try_fold(acc, &mut *fold);
2020-04-17T23:32:38.9535402Z 
2020-04-17T23:32:38.9535959Z error[E0382]: use of moved value
2020-04-17T23:32:38.9536527Z    --> src/libcore/iter/adapters/flatten.rs:304:25
2020-04-17T23:32:38.9536984Z     |
2020-04-17T23:32:38.9536984Z     |
2020-04-17T23:32:38.9537569Z 304 |                 let r = mid.try_fold(acc, &mut *fold);
2020-04-17T23:32:38.9539445Z     |                         |            | |
2020-04-17T23:32:38.9540317Z     |                         |            | value moved here
2020-04-17T23:32:38.9540317Z     |                         |            | value moved here
2020-04-17T23:32:38.9541307Z     |                         |            move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9542100Z     |                         value used here after move
2020-04-17T23:32:38.9543057Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.9543498Z     |
2020-04-17T23:32:38.9543498Z     |
2020-04-17T23:32:38.9544154Z 298 |         fn flatten<'a, T: IntoIterator, Acc: Copy, R: Try<Ok = Acc>>(
2020-04-17T23:32:38.9545214Z 
2020-04-17T23:32:38.9572125Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9572778Z    --> src/libcore/iter/adapters/flatten.rs:335:30
2020-04-17T23:32:38.9573205Z     |
2020-04-17T23:32:38.9573205Z     |
2020-04-17T23:32:38.9573752Z 335 |             move |acc, iter| iter.fold(acc, &mut *fold)
2020-04-17T23:32:38.9574852Z 
2020-04-17T23:32:38.9580857Z error[E0382]: use of moved value
2020-04-17T23:32:38.9581496Z    --> src/libcore/iter/adapters/flatten.rs:335:30
2020-04-17T23:32:38.9581982Z     |
2020-04-17T23:32:38.9581982Z     |
2020-04-17T23:32:38.9582540Z 335 |             move |acc, iter| iter.fold(acc, &mut *fold)
2020-04-17T23:32:38.9584020Z     |                              |  |
2020-04-17T23:32:38.9584757Z     |                              |  value moved here
2020-04-17T23:32:38.9585483Z     |                              value used here after move
2020-04-17T23:32:38.9586407Z     |                              move occurs because value has type `U`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9586407Z     |                              move occurs because value has type `U`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9587010Z     |
2020-04-17T23:32:38.9587493Z help: consider further restricting this bound
2020-04-17T23:32:38.9587922Z     |
2020-04-17T23:32:38.9588487Z 332 |         fn flatten<U: Iterator + Copy, Acc>(
2020-04-17T23:32:38.9589391Z 
2020-04-17T23:32:38.9629827Z error[E0382]: use of moved value
2020-04-17T23:32:38.9630757Z    --> src/libcore/iter/adapters/flatten.rs:335:30
2020-04-17T23:32:38.9631253Z     |
2020-04-17T23:32:38.9631253Z     |
2020-04-17T23:32:38.9631949Z 335 |             move |acc, iter| iter.fold(acc, &mut *fold)
2020-04-17T23:32:38.9633708Z     |                              |         | |
2020-04-17T23:32:38.9634541Z     |                              |         | value moved here
2020-04-17T23:32:38.9634541Z     |                              |         | value moved here
2020-04-17T23:32:38.9635453Z     |                              |         move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9636322Z     |                              value used here after move
2020-04-17T23:32:38.9637277Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.9637694Z     |
2020-04-17T23:32:38.9637694Z     |
2020-04-17T23:32:38.9638611Z 332 |         fn flatten<U: Iterator, Acc: Copy>(
2020-04-17T23:32:38.9639492Z 
2020-04-17T23:32:38.9656018Z error[E0382]: use of moved value
2020-04-17T23:32:38.9656817Z    --> src/libcore/iter/adapters/flatten.rs:383:31
2020-04-17T23:32:38.9657446Z     |
2020-04-17T23:32:38.9657446Z     |
2020-04-17T23:32:38.9657976Z 383 |                 let mut mid = x.into_iter();
2020-04-17T23:32:38.9658746Z     |                               -^^^^^^^^^^^^
2020-04-17T23:32:38.9659945Z     |                               value moved here
2020-04-17T23:32:38.9660575Z     |                               value used here after move
2020-04-17T23:32:38.9661329Z     |                               move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9661867Z     |
---
2020-04-17T23:32:38.9664255Z 
2020-04-17T23:32:38.9664709Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9665271Z    --> src/libcore/iter/adapters/flatten.rs:384:25
2020-04-17T23:32:38.9665681Z     |
2020-04-17T23:32:38.9666401Z 384 |                 let r = mid.try_rfold(acc, &mut *fold);
2020-04-17T23:32:38.9667708Z 
2020-04-17T23:32:38.9672400Z error[E0382]: use of moved value
2020-04-17T23:32:38.9672951Z    --> src/libcore/iter/adapters/flatten.rs:384:25
2020-04-17T23:32:38.9673442Z     |
2020-04-17T23:32:38.9673442Z     |
2020-04-17T23:32:38.9673970Z 384 |                 let r = mid.try_rfold(acc, &mut *fold);
2020-04-17T23:32:38.9675626Z     |                         |             | |
2020-04-17T23:32:38.9676515Z     |                         |             | value moved here
2020-04-17T23:32:38.9676515Z     |                         |             | value moved here
2020-04-17T23:32:38.9677661Z     |                         |             move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9678511Z     |                         value used here after move
2020-04-17T23:32:38.9679551Z help: consider further restricting type parameter `Acc`
2020-04-17T23:32:38.9679980Z     |
2020-04-17T23:32:38.9680549Z 380 |             T::IntoIter: DoubleEndedIterator, Acc: Copy
2020-04-17T23:32:38.9681231Z     |                                             ^^^^^^^^^^^
2020-04-17T23:32:38.9681231Z     |                                             ^^^^^^^^^^^
2020-04-17T23:32:38.9681489Z 
2020-04-17T23:32:38.9715727Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9716491Z    --> src/libcore/iter/adapters/flatten.rs:415:30
2020-04-17T23:32:38.9717008Z     |
2020-04-17T23:32:38.9717610Z 415 |             move |acc, iter| iter.rfold(acc, &mut *fold)
2020-04-17T23:32:38.9718909Z 
2020-04-17T23:32:38.9749027Z error[E0382]: use of moved value
2020-04-17T23:32:38.9749663Z    --> src/libcore/iter/adapters/flatten.rs:415:30
2020-04-17T23:32:38.9750115Z     |
2020-04-17T23:32:38.9750115Z     |
2020-04-17T23:32:38.9750641Z 415 |             move |acc, iter| iter.rfold(acc, &mut *fold)
2020-04-17T23:32:38.9752013Z     |                              |  |
2020-04-17T23:32:38.9752697Z     |                              |  value moved here
2020-04-17T23:32:38.9753368Z     |                              value used here after move
2020-04-17T23:32:38.9754109Z     |                              move occurs because value has type `U`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9754109Z     |                              move occurs because value has type `U`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9754832Z     |
2020-04-17T23:32:38.9755296Z help: consider further restricting this bound
2020-04-17T23:32:38.9755697Z     |
2020-04-17T23:32:38.9756248Z 412 |         fn flatten<U: DoubleEndedIterator + Copy, Acc>(
2020-04-17T23:32:38.9757149Z 
2020-04-17T23:32:38.9757518Z error[E0382]: use of moved value
2020-04-17T23:32:38.9758050Z    --> src/libcore/iter/adapters/flatten.rs:415:30
2020-04-17T23:32:38.9758458Z     |
2020-04-17T23:32:38.9758458Z     |
2020-04-17T23:32:38.9758969Z 415 |             move |acc, iter| iter.rfold(acc, &mut *fold)
2020-04-17T23:32:38.9760615Z     |                              |          | |
2020-04-17T23:32:38.9761454Z     |                              |          | value moved here
2020-04-17T23:32:38.9761454Z     |                              |          | value moved here
2020-04-17T23:32:38.9762368Z     |                              |          move occurs because value has type `Acc`, which does not implement the `Copy` trait
2020-04-17T23:32:38.9763144Z     |                              value used here after move
2020-04-17T23:32:38.9764040Z help: consider restricting type parameter `Acc`
2020-04-17T23:32:38.9764465Z     |
2020-04-17T23:32:38.9764465Z     |
2020-04-17T23:32:38.9765025Z 412 |         fn flatten<U: DoubleEndedIterator, Acc: Copy>(
2020-04-17T23:32:38.9765942Z 
2020-04-17T23:32:38.9910341Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9911003Z    --> src/libcore/iter/traits/double_ended.rs:231:30
2020-04-17T23:32:38.9911466Z     |
2020-04-17T23:32:38.9911466Z     |
2020-04-17T23:32:38.9912099Z 231 |             move |acc, x| Ok(f(acc, x))
2020-04-17T23:32:38.9913230Z 
2020-04-17T23:32:38.9931322Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9931959Z    --> src/libcore/iter/traits/double_ended.rs:291:20
2020-04-17T23:32:38.9932424Z     |
2020-04-17T23:32:38.9932424Z     |
2020-04-17T23:32:38.9933074Z 291 |                 if predicate(&x) { LoopState::Break(x) } else { LoopState::Continue(()) }
2020-04-17T23:32:38.9934202Z 
2020-04-17T23:32:38.9952943Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9953578Z    --> src/libcore/iter/traits/iterator.rs:655:29
2020-04-17T23:32:38.9954043Z     |
2020-04-17T23:32:38.9954043Z     |
2020-04-17T23:32:38.9954526Z 655 |             move |(), item| f(item)
2020-04-17T23:32:38.9955540Z 
2020-04-17T23:32:38.9994383Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9995199Z     --> src/libcore/iter/traits/iterator.rs:1702:20
2020-04-17T23:32:38.9995707Z      |
2020-04-17T23:32:38.9995707Z      |
2020-04-17T23:32:38.9996223Z 1702 |                 if f(&x) {
2020-04-17T23:32:38.9997315Z 
2020-04-17T23:32:38.9997889Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:38.9998525Z     --> src/libcore/iter/traits/iterator.rs:1703:21
2020-04-17T23:32:38.9998940Z      |
---
2020-04-17T23:32:39.0041145Z 
2020-04-17T23:32:39.0047033Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0047695Z     --> src/libcore/iter/traits/iterator.rs:1760:25
2020-04-17T23:32:39.0048118Z      |
2020-04-17T23:32:39.0048727Z 1760 |                 let p = predicate(&**x);
2020-04-17T23:32:39.0049855Z 
2020-04-17T23:32:39.0066996Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0067609Z     --> src/libcore/iter/traits/iterator.rs:1768:22
2020-04-17T23:32:39.0068083Z      |
2020-04-17T23:32:39.0068083Z      |
2020-04-17T23:32:39.0068595Z 1768 |             move |x| predicate(&**x)
2020-04-17T23:32:39.0069599Z 
2020-04-17T23:32:39.0088071Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0088733Z     --> src/libcore/iter/traits/iterator.rs:1920:26
2020-04-17T23:32:39.0089160Z      |
2020-04-17T23:32:39.0089160Z      |
2020-04-17T23:32:39.0089670Z 1920 |             move |(), x| f(x)
2020-04-17T23:32:39.0090646Z 
2020-04-17T23:32:39.0105227Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0105837Z     --> src/libcore/iter/traits/iterator.rs:2002:30
2020-04-17T23:32:39.0106472Z      |
2020-04-17T23:32:39.0106472Z      |
2020-04-17T23:32:39.0106999Z 2002 |             move |acc, x| Ok(f(acc, x))
2020-04-17T23:32:39.0108017Z 
2020-04-17T23:32:39.0145183Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0146035Z     --> src/libcore/iter/traits/iterator.rs:2092:20
2020-04-17T23:32:39.0146667Z      |
2020-04-17T23:32:39.0146667Z      |
2020-04-17T23:32:39.0147367Z 2092 |                 if f(x) { LoopState::Continue(()) } else { LoopState::Break(()) }
2020-04-17T23:32:39.0148608Z 
2020-04-17T23:32:39.0164382Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0165038Z     --> src/libcore/iter/traits/iterator.rs:2145:20
2020-04-17T23:32:39.0165490Z      |
2020-04-17T23:32:39.0165490Z      |
2020-04-17T23:32:39.0166131Z 2145 |                 if f(x) { LoopState::Break(()) } else { LoopState::Continue(()) }
2020-04-17T23:32:39.0167387Z 
2020-04-17T23:32:39.0187797Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0188412Z     --> src/libcore/iter/traits/iterator.rs:2208:20
2020-04-17T23:32:39.0188905Z      |
2020-04-17T23:32:39.0188905Z      |
2020-04-17T23:32:39.0189563Z 2208 |                 if predicate(&x) { LoopState::Break(x) } else { LoopState::Continue(()) }
2020-04-17T23:32:39.0190714Z 
2020-04-17T23:32:39.0207149Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0207763Z     --> src/libcore/iter/traits/iterator.rs:2239:32
2020-04-17T23:32:39.0208261Z      |
2020-04-17T23:32:39.0208261Z      |
2020-04-17T23:32:39.0208757Z 2239 |             move |(), x| match f(x) {
2020-04-17T23:32:39.0209779Z 
2020-04-17T23:32:39.0231539Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0232229Z     --> src/libcore/iter/traits/iterator.rs:2352:20
2020-04-17T23:32:39.0232657Z      |
2020-04-17T23:32:39.0232657Z      |
2020-04-17T23:32:39.0233357Z 2352 |                 if predicate(x) { LoopState::Break(i) } else { LoopState::Continue(Add::add(i, 1)) }
2020-04-17T23:32:39.0234517Z 
2020-04-17T23:32:39.0234959Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0235541Z     --> src/libcore/iter/traits/iterator.rs:2352:84
2020-04-17T23:32:39.0235956Z      |
2020-04-17T23:32:39.0235956Z      |
2020-04-17T23:32:39.0236632Z 2352 |                 if predicate(x) { LoopState::Break(i) } else { LoopState::Continue(Add::add(i, 1)) }
2020-04-17T23:32:39.0238160Z 
2020-04-17T23:32:39.0256992Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0257668Z     --> src/libcore/iter/traits/iterator.rs:2412:20
2020-04-17T23:32:39.0258104Z      |
2020-04-17T23:32:39.0258104Z      |
2020-04-17T23:32:39.0258778Z 2412 |                 if predicate(x) { LoopState::Break(i) } else { LoopState::Continue(i) }
2020-04-17T23:32:39.0259895Z 
2020-04-17T23:32:39.0280013Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0280984Z     --> src/libcore/iter/traits/iterator.rs:2499:23
2020-04-17T23:32:39.0281497Z      |
2020-04-17T23:32:39.0281497Z      |
2020-04-17T23:32:39.0282006Z 2499 |             move |x| (f(&x), x)
2020-04-17T23:32:39.0283208Z 
2020-04-17T23:32:39.0305619Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0306379Z     --> src/libcore/iter/traits/iterator.rs:2534:25
2020-04-17T23:32:39.0306859Z      |
2020-04-17T23:32:39.0306859Z      |
2020-04-17T23:32:39.0307428Z 2534 |             move |x, y| cmp::max_by(x, y, &mut compare)
2020-04-17T23:32:39.0308786Z 
2020-04-17T23:32:39.0316412Z error[E0382]: use of moved value
2020-04-17T23:32:39.0317511Z     --> src/libcore/iter/traits/iterator.rs:2534:25
2020-04-17T23:32:39.0320687Z      |
2020-04-17T23:32:39.0320687Z      |
2020-04-17T23:32:39.0322348Z 2534 |             move |x, y| cmp::max_by(x, y, &mut compare)
2020-04-17T23:32:39.0323168Z      |                         ^^^^^^^^^^^^-^^^^^^^^^^^^^^^^^^
2020-04-17T23:32:39.0324850Z      |                         |           value moved here
2020-04-17T23:32:39.0325824Z      |                         |           move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0326867Z      |                         value used here after move
2020-04-17T23:32:39.0327288Z      |
2020-04-17T23:32:39.0327288Z      |
2020-04-17T23:32:39.0327718Z help: consider restricting type parameter `T`
2020-04-17T23:32:39.0328131Z      |
2020-04-17T23:32:39.0328778Z 2533 |         fn fold<T: Copy>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
2020-04-17T23:32:39.0329801Z 
2020-04-17T23:32:39.0330207Z error[E0382]: use of moved value
2020-04-17T23:32:39.0335763Z     --> src/libcore/iter/traits/iterator.rs:2534:25
2020-04-17T23:32:39.0336375Z      |
2020-04-17T23:32:39.0336375Z      |
2020-04-17T23:32:39.0337274Z 2534 |             move |x, y| cmp::max_by(x, y, &mut compare)
2020-04-17T23:32:39.0340707Z      |                         ^^^^^^^^^^^^^^^-^^^^^^^^^^^^^^^
2020-04-17T23:32:39.0342340Z      |                         |              value moved here
2020-04-17T23:32:39.0343412Z      |                         |              move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0344289Z      |                         value used here after move
2020-04-17T23:32:39.0344798Z      |
2020-04-17T23:32:39.0344798Z      |
2020-04-17T23:32:39.0345233Z help: consider restricting type parameter `T`
2020-04-17T23:32:39.0345627Z      |
2020-04-17T23:32:39.0346572Z 2533 |         fn fold<T: Copy>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
2020-04-17T23:32:39.0347528Z 
2020-04-17T23:32:39.0372726Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0373425Z     --> src/libcore/iter/traits/iterator.rs:2563:23
2020-04-17T23:32:39.0373903Z      |
2020-04-17T23:32:39.0373903Z      |
2020-04-17T23:32:39.0374417Z 2563 |             move |x| (f(&x), x)
2020-04-17T23:32:39.0375559Z 
2020-04-17T23:32:39.0380109Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0380738Z     --> src/libcore/iter/traits/iterator.rs:2598:25
2020-04-17T23:32:39.0381182Z      |
2020-04-17T23:32:39.0381182Z      |
2020-04-17T23:32:39.0381746Z 2598 |             move |x, y| cmp::min_by(x, y, &mut compare)
2020-04-17T23:32:39.0382917Z 
2020-04-17T23:32:39.0387264Z error[E0382]: use of moved value
2020-04-17T23:32:39.0387852Z     --> src/libcore/iter/traits/iterator.rs:2598:25
2020-04-17T23:32:39.0388300Z      |
2020-04-17T23:32:39.0388300Z      |
2020-04-17T23:32:39.0388859Z 2598 |             move |x, y| cmp::min_by(x, y, &mut compare)
2020-04-17T23:32:39.0389622Z      |                         ^^^^^^^^^^^^-^^^^^^^^^^^^^^^^^^
2020-04-17T23:32:39.0391407Z      |                         |           value moved here
2020-04-17T23:32:39.0392360Z      |                         |           move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0393155Z      |                         value used here after move
2020-04-17T23:32:39.0393652Z      |
2020-04-17T23:32:39.0393652Z      |
2020-04-17T23:32:39.0394204Z help: consider restricting type parameter `T`
2020-04-17T23:32:39.0394603Z      |
2020-04-17T23:32:39.0395272Z 2597 |         fn fold<T: Copy>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
2020-04-17T23:32:39.0396155Z 
2020-04-17T23:32:39.0402194Z error[E0382]: use of moved value
2020-04-17T23:32:39.0402918Z     --> src/libcore/iter/traits/iterator.rs:2598:25
2020-04-17T23:32:39.0403392Z      |
2020-04-17T23:32:39.0403392Z      |
2020-04-17T23:32:39.0403995Z 2598 |             move |x, y| cmp::min_by(x, y, &mut compare)
2020-04-17T23:32:39.0404759Z      |                         ^^^^^^^^^^^^^^^-^^^^^^^^^^^^^^^
2020-04-17T23:32:39.0406205Z      |                         |              value moved here
2020-04-17T23:32:39.0407081Z      |                         |              move occurs because value has type `T`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0407858Z      |                         value used here after move
2020-04-17T23:32:39.0408297Z      |
2020-04-17T23:32:39.0408297Z      |
2020-04-17T23:32:39.0408753Z help: consider restricting type parameter `T`
2020-04-17T23:32:39.0409148Z      |
2020-04-17T23:32:39.0409793Z 2597 |         fn fold<T: Copy>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
2020-04-17T23:32:39.0410692Z 
2020-04-17T23:32:39.0434328Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0435114Z     --> src/libcore/iter/traits/iterator.rs:2670:17
2020-04-17T23:32:39.0435603Z      |
---
2020-04-17T23:32:39.0445224Z      |
2020-04-17T23:32:39.0446249Z 2670 |                 ts.extend(Some(t));
2020-04-17T23:32:39.0446906Z      |                 ^^^^^^^^^^-------^
2020-04-17T23:32:39.0447645Z      |                 |         |     |
2020-04-17T23:32:39.0448457Z      |                 |         |     value moved here
2020-04-17T23:32:39.0449431Z      |                 |         move occurs because value has type `option::Option<A>`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0450257Z      |                 value used here after move
2020-04-17T23:32:39.0450938Z error[E0381]: use of possibly-uninitialized variable: `_`
2020-04-17T23:32:39.0451726Z     --> src/libcore/iter/traits/iterator.rs:2671:17
2020-04-17T23:32:39.0452202Z      |
2020-04-17T23:32:39.0452202Z      |
2020-04-17T23:32:39.0452709Z 2671 |                 us.extend(Some(u));
2020-04-17T23:32:39.0453800Z 
2020-04-17T23:32:39.0459235Z error[E0382]: use of moved value
2020-04-17T23:32:39.0459823Z     --> src/libcore/iter/traits/iterator.rs:2671:17
2020-04-17T23:32:39.0460241Z      |
2020-04-17T23:32:39.0460241Z      |
2020-04-17T23:32:39.0460759Z 2671 |                 us.extend(Some(u));
2020-04-17T23:32:39.0462152Z      |                 |         |     |
2020-04-17T23:32:39.0463023Z      |                 |         |     value moved here
2020-04-17T23:32:39.0463023Z      |                 |         |     value moved here
2020-04-17T23:32:39.0463999Z      |                 |         move occurs because value has type `option::Option<B>`, which does not implement the `Copy` trait
2020-04-17T23:32:39.0464825Z      |                 value used here after move
2020-04-17T23:32:39.3342998Z    Compiling std v0.0.0 (/checkout/src/libstd)
2020-04-17T23:32:39.7248383Z    Compiling compiler_builtins v0.1.25
2020-04-17T23:32:41.4824751Z    Compiling backtrace-sys v0.1.35
2020-04-17T23:32:42.3989306Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
---
2020-04-17T23:32:47.9632755Z expected success, got: exit code: 101
2020-04-17T23:32:47.9644117Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-17T23:32:47.9644509Z Build completed unsuccessfully in 0:24:37
2020-04-17T23:32:47.9702373Z == clock drift check ==
2020-04-17T23:32:47.9728527Z   local time: Fri Apr 17 23:32:47 UTC 2020
2020-04-17T23:32:48.1713229Z   network time: Fri, 17 Apr 2020 23:32:48 GMT
2020-04-17T23:32:49.8420321Z 
2020-04-17T23:32:49.8420321Z 
2020-04-17T23:32:49.8509014Z ##[error]Bash exited with code '1'.
2020-04-17T23:32:49.8553099Z ##[section]Finishing: Run build
2020-04-17T23:32:49.8604879Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-17T23:32:49.8610463Z Task         : Get sources
2020-04-17T23:32:49.8610835Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T23:32:49.8611163Z Version      : 1.0.0
2020-04-17T23:32:49.8611394Z Author       : Microsoft
2020-04-17T23:32:49.8611394Z Author       : Microsoft
2020-04-17T23:32:49.8611782Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T23:32:49.8612200Z ==============================================================================
2020-04-17T23:32:50.2410696Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T23:32:50.2456922Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-17T23:32:50.2560462Z Cleaning up task key
2020-04-17T23:32:50.2561863Z Start cleaning up orphan processes.
2020-04-17T23:32:50.2774346Z Terminate orphan process: pid (3354) (python)
2020-04-17T23:32:50.3006117Z ##[section]Finishing: Finalize Job

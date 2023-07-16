plain
2020-04-19T15:36:23.0971458Z ========================== Starting Command Output ===========================
2020-04-19T15:36:23.0976809Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1abf3e9a-3271-4d06-ae56-d9166c025e45.sh
2020-04-19T15:36:23.0977253Z 
2020-04-19T15:36:23.0982334Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T15:36:23.0999864Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-19T15:36:23.1003019Z Task         : Get sources
2020-04-19T15:36:23.1003290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T15:36:23.1003554Z Version      : 1.0.0
2020-04-19T15:36:23.1003753Z Author       : Microsoft
---
2020-04-19T15:36:24.2708092Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T15:36:24.2717398Z ##[command]git config gc.auto 0
2020-04-19T15:36:24.2724415Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T15:36:24.2731431Z ##[command]git config --get-all http.proxy
2020-04-19T15:36:24.2742502Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71095/merge:refs/remotes/pull/71095/merge
---
2020-04-19T15:38:42.8414670Z  ---> 318032b5f0e2
2020-04-19T15:38:42.8415631Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T15:38:42.8420266Z  ---> Using cache
2020-04-19T15:38:42.8420615Z  ---> d44a858fd1ce
2020-04-19T15:38:42.8435295Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T15:38:42.8439611Z  ---> 58b910f50f5a
2020-04-19T15:38:42.8440021Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T15:38:42.8484393Z  ---> Using cache
2020-04-19T15:38:42.8485002Z  ---> ee7702aadba1
---
2020-04-19T15:38:42.8817241Z Looks like docker image is the same as before, not uploading
2020-04-19T15:38:50.8178747Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T15:38:50.8425176Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T15:38:50.8451594Z == clock drift check ==
2020-04-19T15:38:50.8474419Z   local time: Sun Apr 19 15:38:50 UTC 2020
2020-04-19T15:38:50.9072033Z   network time: Sun, 19 Apr 2020 15:38:50 GMT
2020-04-19T15:38:50.9095816Z Starting sccache server...
2020-04-19T15:38:50.9887511Z configure: processing command line
2020-04-19T15:38:50.9887711Z configure: 
2020-04-19T15:38:50.9888653Z configure: rust.dist-src        := False
---
2020-04-19T15:44:17.0477759Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T15:44:18.5778357Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T15:44:20.2150998Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T15:44:22.4184415Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T15:44:30.7834014Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T15:44:34.7383743Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T15:44:39.3902913Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T15:44:43.7399300Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T15:44:52.5337809Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T16:08:19.2762043Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T16:08:21.0240887Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T16:08:22.9344242Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T16:08:24.2130695Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T16:08:34.0869859Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T16:08:37.8836042Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T16:08:42.8778842Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T16:08:47.5821097Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T16:08:57.1977059Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T16:32:43.9278937Z .................................................................................................... 1700/9907
2020-04-19T16:32:47.9380599Z .................................................................................................... 1800/9907
2020-04-19T16:32:56.4620388Z ..................................................................................................i. 1900/9907
2020-04-19T16:33:04.0631632Z .................................................................................................... 2000/9907
2020-04-19T16:33:10.2705628Z ........................................................................................iiiii....... 2100/9907
2020-04-19T16:33:30.0602885Z .................................................................................................... 2300/9907
2020-04-19T16:33:32.2368406Z .................................................................................................... 2400/9907
2020-04-19T16:33:34.4726104Z .................................................................................................... 2500/9907
2020-04-19T16:33:40.4131973Z .................................................................................................... 2600/9907
---
2020-04-19T16:36:28.2187245Z ................................................................i...............i................... 5000/9907
2020-04-19T16:36:35.4116231Z .................................................................................................... 5100/9907
2020-04-19T16:36:42.4502655Z .................................................................................................... 5200/9907
2020-04-19T16:36:47.5483684Z ..........i......................................................................................... 5300/9907
2020-04-19T16:36:57.1446465Z i................................................................................................... 5400/9907
2020-04-19T16:37:01.8105935Z ii.ii........i...i.................................................................................. 5500/9907
2020-04-19T16:37:09.3773399Z ...............................................i.................................................... 5700/9907
2020-04-19T16:37:18.4103260Z ...............................................................................ii................... 5800/9907
2020-04-19T16:37:25.3058659Z ..................i................................................................................. 5900/9907
2020-04-19T16:37:30.5599033Z .................................................................................................... 6000/9907
2020-04-19T16:37:30.5599033Z .................................................................................................... 6000/9907
2020-04-19T16:37:41.0299747Z .................................................................................................... 6100/9907
2020-04-19T16:37:51.1973506Z ............ii...i..ii...........i.................................................................. 6200/9907
2020-04-19T16:38:06.3287331Z .................................................................................................... 6400/9907
2020-04-19T16:38:13.0050554Z .................................................................................................... 6500/9907
2020-04-19T16:38:13.0050554Z .................................................................................................... 6500/9907
2020-04-19T16:38:28.0756228Z ..........................................i..ii..................................................... 6600/9907
2020-04-19T16:38:49.4652083Z .................................................................................................... 6800/9907
2020-04-19T16:38:51.5900113Z ...........................................i........................................................ 6900/9907
2020-04-19T16:38:53.6912577Z .................................................................................................... 7000/9907
2020-04-19T16:38:55.8036187Z ...................................................................................i................ 7100/9907
---
2020-04-19T16:40:28.8107965Z .................................................................................................... 7800/9907
2020-04-19T16:40:33.2768441Z .................................................................................................... 7900/9907
2020-04-19T16:40:39.7335495Z .................................................................................................... 8000/9907
2020-04-19T16:40:45.5716097Z ..................................................i................................................. 8100/9907
2020-04-19T16:40:55.3412892Z ..................................................................................................ii 8200/9907
2020-04-19T16:41:00.6158925Z iiii.iiiii.i........................................................................................ 8300/9907
2020-04-19T16:41:13.9573428Z .................................................................................................... 8500/9907
2020-04-19T16:41:21.9921464Z .................................................................................................... 8600/9907
2020-04-19T16:41:35.6263912Z .................................................................................................... 8700/9907
2020-04-19T16:41:42.3295337Z .................................................................................................... 8800/9907
---
2020-04-19T16:43:29.7733444Z 
2020-04-19T16:43:29.7734657Z ---- [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs stdout ----
2020-04-19T16:43:29.7736109Z diff of stderr:
2020-04-19T16:43:29.7736427Z 
2020-04-19T16:43:29.7736859Z 32              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-04-19T16:43:29.7737290Z 33 
2020-04-19T16:43:29.7737754Z 34 error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7739575Z +   --> $DIR/alloc-types-no-impls-length-33.rs:21:23
2020-04-19T16:43:29.7739948Z 36    |
2020-04-19T16:43:29.7739948Z 36    |
2020-04-19T16:43:29.7740348Z 37 LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7741040Z 38    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7741552Z 
2020-04-19T16:43:29.7742087Z 47    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7742589Z 48 
2020-04-19T16:43:29.7743040Z 49 error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7744646Z +   --> $DIR/alloc-types-no-impls-length-33.rs:21:23
2020-04-19T16:43:29.7745045Z 51    |
2020-04-19T16:43:29.7745045Z 51    |
2020-04-19T16:43:29.7745424Z 52 LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7746094Z 53    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7746858Z 
2020-04-19T16:43:29.7747278Z 56              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-04-19T16:43:29.7747665Z 57 
2020-04-19T16:43:29.7748154Z 58 error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7749676Z +   --> $DIR/alloc-types-no-impls-length-33.rs:28:23
2020-04-19T16:43:29.7750055Z 60    |
2020-04-19T16:43:29.7750055Z 60    |
2020-04-19T16:43:29.7750430Z 61 LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7751114Z 62    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7751608Z 
2020-04-19T16:43:29.7752127Z 71    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7752674Z 72 
2020-04-19T16:43:29.7753141Z 73 error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7754832Z +   --> $DIR/alloc-types-no-impls-length-33.rs:28:23
2020-04-19T16:43:29.7755205Z 75    |
2020-04-19T16:43:29.7755205Z 75    |
2020-04-19T16:43:29.7755606Z 76 LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7756295Z 77    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7757047Z 
2020-04-19T16:43:29.7757353Z The actual stderr differed from the expected stderr.
2020-04-19T16:43:29.7757353Z The actual stderr differed from the expected stderr.
2020-04-19T16:43:29.7758257Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
2020-04-19T16:43:29.7759181Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T16:43:29.7760044Z To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-types-no-impls-length-33.rs`
2020-04-19T16:43:29.7762838Z error: 1 errors occurred comparing output.
2020-04-19T16:43:29.7763197Z status: exit code: 1
2020-04-19T16:43:29.7763197Z status: exit code: 1
2020-04-19T16:43:29.7765490Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary"
2020-04-19T16:43:29.7767514Z ------------------------------------------
2020-04-19T16:43:29.7767825Z 
2020-04-19T16:43:29.7768348Z ------------------------------------------
2020-04-19T16:43:29.7768708Z stderr:
2020-04-19T16:43:29.7768708Z stderr:
2020-04-19T16:43:29.7769286Z ------------------------------------------
2020-04-19T16:43:29.7770807Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-04-19T16:43:29.7771545Z   --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6:29
2020-04-19T16:43:29.7771828Z    |
2020-04-19T16:43:29.7772035Z LL |     let v: Vec<_> = [0; 33].into();
2020-04-19T16:43:29.7772434Z    |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
2020-04-19T16:43:29.7772763Z    |
2020-04-19T16:43:29.7773163Z    = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
2020-04-19T16:43:29.7776396Z    = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`
2020-04-19T16:43:29.7776757Z 
2020-04-19T16:43:29.7777247Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
2020-04-19T16:43:29.7778449Z    |
2020-04-19T16:43:29.7778449Z    |
2020-04-19T16:43:29.7778713Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7779273Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-19T16:43:29.7779957Z    = help: the following implementations were found:
2020-04-19T16:43:29.7779957Z    = help: the following implementations were found:
2020-04-19T16:43:29.7780589Z              <std::boxed::Box<(dyn std::error::Error + 'a)> as std::convert::From<E>>
2020-04-19T16:43:29.7781276Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-04-19T16:43:29.7782178Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-04-19T16:43:29.7783009Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-04-19T16:43:29.7783829Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-19T16:43:29.7783829Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-19T16:43:29.7784626Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-04-19T16:43:29.7785007Z 
2020-04-19T16:43:29.7785343Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
2020-04-19T16:43:29.7786354Z    |
2020-04-19T16:43:29.7786354Z    |
2020-04-19T16:43:29.7786595Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7787279Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-19T16:43:29.7787957Z    = help: the following implementations were found:
2020-04-19T16:43:29.7787957Z    = help: the following implementations were found:
2020-04-19T16:43:29.7788343Z              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-04-19T16:43:29.7788606Z 
2020-04-19T16:43:29.7788950Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7790087Z    |
2020-04-19T16:43:29.7790087Z    |
2020-04-19T16:43:29.7790329Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7790828Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7791458Z    = help: the following implementations were found:
2020-04-19T16:43:29.7791458Z    = help: the following implementations were found:
2020-04-19T16:43:29.7791773Z              <std::rc::Rc<T> as std::convert::From<T>>
2020-04-19T16:43:29.7792119Z              <std::rc::Rc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-19T16:43:29.7792493Z              <std::rc::Rc<[T]> as std::convert::From<&[T]>>
2020-04-19T16:43:29.7792848Z              <std::rc::Rc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-19T16:43:29.7793142Z            and 8 others
2020-04-19T16:43:29.7793543Z    = note: required because of the requirements on the impl of `std::convert::Into<std::rc::Rc<[i32; 33]>>` for `std::rc::Rc<[i32]>`
2020-04-19T16:43:29.7794258Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7794606Z 
2020-04-19T16:43:29.7794946Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7795957Z    |
2020-04-19T16:43:29.7795957Z    |
2020-04-19T16:43:29.7796209Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7796732Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-19T16:43:29.7797369Z    = help: the following implementations were found:
2020-04-19T16:43:29.7797369Z    = help: the following implementations were found:
2020-04-19T16:43:29.7797728Z              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-04-19T16:43:29.7797972Z 
2020-04-19T16:43:29.7798315Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7799369Z    |
2020-04-19T16:43:29.7799369Z    |
2020-04-19T16:43:29.7799624Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7800156Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7800802Z    = help: the following implementations were found:
2020-04-19T16:43:29.7800802Z    = help: the following implementations were found:
2020-04-19T16:43:29.7801252Z              <std::sync::Arc<T> as std::convert::From<T>>
2020-04-19T16:43:29.7801625Z              <std::sync::Arc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-19T16:43:29.7802020Z              <std::sync::Arc<[T]> as std::convert::From<&[T]>>
2020-04-19T16:43:29.7802404Z              <std::sync::Arc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-19T16:43:29.7802696Z            and 8 others
2020-04-19T16:43:29.7803142Z    = note: required because of the requirements on the impl of `std::convert::Into<std::sync::Arc<[i32; 33]>>` for `std::sync::Arc<[i32]>`
2020-04-19T16:43:29.7803809Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7804185Z 
2020-04-19T16:43:29.7804542Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-04-19T16:43:29.7805592Z    |
2020-04-19T16:43:29.7805592Z    |
2020-04-19T16:43:29.7805855Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-19T16:43:29.7806414Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-19T16:43:29.7807078Z    = help: the following implementations were found:
2020-04-19T16:43:29.7807078Z    = help: the following implementations were found:
2020-04-19T16:43:29.7807512Z              <std::sync::Arc<[T; _]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
2020-04-19T16:43:29.7807962Z error: aborting due to 7 previous errors
2020-04-19T16:43:29.7808121Z 
2020-04-19T16:43:29.7808550Z For more information about this error, try `rustc --explain E0277`.
2020-04-19T16:43:29.7808750Z 
---
2020-04-19T16:43:29.7811658Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T16:43:29.7812165Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T16:43:29.7812422Z 
2020-04-19T16:43:29.7812536Z 
2020-04-19T16:43:29.7818431Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T16:43:29.7821699Z 
2020-04-19T16:43:29.7821797Z 
2020-04-19T16:43:29.7822465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T16:43:29.7822819Z Build completed unsuccessfully in 1:03:02
2020-04-19T16:43:29.7822819Z Build completed unsuccessfully in 1:03:02
2020-04-19T16:43:29.7843725Z == clock drift check ==
2020-04-19T16:43:29.7864319Z   local time: Sun Apr 19 16:43:29 UTC 2020
2020-04-19T16:43:29.8572788Z   network time: Sun, 19 Apr 2020 16:43:29 GMT
2020-04-19T16:43:30.2560315Z 
2020-04-19T16:43:30.2560315Z 
2020-04-19T16:43:30.2637088Z ##[error]Bash exited with code '1'.
2020-04-19T16:43:30.2648895Z ##[section]Finishing: Run build
2020-04-19T16:43:30.2712195Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-19T16:43:30.2716478Z Task         : Get sources
2020-04-19T16:43:30.2716763Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T16:43:30.2717040Z Version      : 1.0.0
2020-04-19T16:43:30.2717232Z Author       : Microsoft
2020-04-19T16:43:30.2717232Z Author       : Microsoft
2020-04-19T16:43:30.2717534Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T16:43:30.2717902Z ==============================================================================
2020-04-19T16:43:30.6182647Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T16:43:30.6232767Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-19T16:43:30.6330780Z Cleaning up task key
2020-04-19T16:43:30.6331975Z Start cleaning up orphan processes.
2020-04-19T16:43:30.6534982Z Terminate orphan process: pid (5598) (python)
2020-04-19T16:43:30.6707257Z ##[section]Finishing: Finalize Job

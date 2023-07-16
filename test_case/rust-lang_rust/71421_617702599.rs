plain
2020-04-22T09:47:15.8768917Z ========================== Starting Command Output ===========================
2020-04-22T09:47:15.8771221Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/acae57c1-3c35-4258-94ae-8ee8751b19a1.sh
2020-04-22T09:47:15.8771461Z 
2020-04-22T09:47:15.8774960Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T09:47:15.8793058Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71421/merge to s
2020-04-22T09:47:15.8796084Z Task         : Get sources
2020-04-22T09:47:15.8796376Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T09:47:15.8796645Z Version      : 1.0.0
2020-04-22T09:47:15.8796828Z Author       : Microsoft
---
2020-04-22T09:47:16.8754829Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T09:47:16.8767593Z ##[command]git config gc.auto 0
2020-04-22T09:47:16.8771361Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T09:47:16.8774569Z ##[command]git config --get-all http.proxy
2020-04-22T09:47:16.8792272Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71421/merge:refs/remotes/pull/71421/merge
---
2020-04-22T09:50:00.7982960Z  ---> 318032b5f0e2
2020-04-22T09:50:00.7983639Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T09:50:00.7987295Z  ---> Using cache
2020-04-22T09:50:00.7987992Z  ---> d44a858fd1ce
2020-04-22T09:50:00.7989050Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T09:50:00.7999873Z  ---> 58b910f50f5a
2020-04-22T09:50:00.8000392Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T09:50:00.8004777Z  ---> Using cache
2020-04-22T09:50:00.8005457Z  ---> ee7702aadba1
---
2020-04-22T09:50:00.8507904Z Looks like docker image is the same as before, not uploading
2020-04-22T09:50:08.8598918Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T09:50:08.8836752Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T09:50:08.8861934Z == clock drift check ==
2020-04-22T09:50:08.8871893Z   local time: Wed Apr 22 09:50:08 UTC 2020
2020-04-22T09:50:09.1793483Z   network time: Wed, 22 Apr 2020 09:50:09 GMT
2020-04-22T09:50:09.1822546Z Starting sccache server...
2020-04-22T09:50:09.2645271Z configure: processing command line
2020-04-22T09:50:09.2646363Z configure: 
2020-04-22T09:50:09.2648064Z configure: rust.dist-src        := False
---
2020-04-22T09:55:01.2522638Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T09:55:02.6472197Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T09:55:04.1289510Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T09:55:05.6412267Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T09:55:13.2890067Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T09:55:16.0434934Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T09:55:20.0981411Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T09:55:23.9471427Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T09:55:32.0929446Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T10:16:25.9019288Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T10:16:27.5185288Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T10:16:29.1600085Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T10:16:29.2416030Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T10:16:39.2347566Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T10:16:41.1081254Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T10:16:45.4763752Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T10:16:49.6119258Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T10:16:59.7953056Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T10:38:10.5843273Z .................................................................................................... 1600/9912
2020-04-22T10:38:16.4196505Z .................................................................................................... 1700/9912
2020-04-22T10:38:20.4081859Z .................................................................................................... 1800/9912
2020-04-22T10:38:28.3270201Z .................................................................................................... 1900/9912
2020-04-22T10:38:35.4971194Z ..i................................................................................................. 2000/9912
2020-04-22T10:38:41.2501897Z ............................................................................................iiiii... 2100/9912
2020-04-22T10:38:59.5121376Z .................................................................................................... 2300/9912
2020-04-22T10:39:02.2015156Z .................................................................................................... 2400/9912
2020-04-22T10:39:04.3831229Z .................................................................................................... 2500/9912
2020-04-22T10:39:09.7770445Z .................................................................................................... 2600/9912
---
2020-04-22T10:42:01.1517952Z .................................................................................................... 5100/9912
2020-04-22T10:42:07.9240379Z .................................................................................................... 5200/9912
2020-04-22T10:42:12.7554404Z ...............i.................................................................................... 5300/9912
2020-04-22T10:42:22.1951491Z .....i.............................................................................................. 5400/9912
2020-04-22T10:42:27.2312814Z .....ii.ii........i...i............................................................................. 5500/9912
2020-04-22T10:42:34.7339711Z ....................................................i............................................... 5700/9912
2020-04-22T10:42:43.1492347Z ....................................................................................ii.............. 5800/9912
2020-04-22T10:42:49.9648214Z .......................i............................................................................ 5900/9912
2020-04-22T10:42:55.1600382Z .................................................................................................... 6000/9912
2020-04-22T10:42:55.1600382Z .................................................................................................... 6000/9912
2020-04-22T10:43:05.3898664Z .................................................................................................... 6100/9912
2020-04-22T10:43:14.9517850Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-22T10:43:29.1394164Z .................................................................................................... 6400/9912
2020-04-22T10:43:32.4500788Z .................................................................................................... 6500/9912
2020-04-22T10:43:32.4500788Z .................................................................................................... 6500/9912
2020-04-22T10:43:41.1981589Z ...............................................i..ii................................................ 6600/9912
2020-04-22T10:44:02.3170017Z .................................................................................................... 6800/9912
2020-04-22T10:44:04.5621122Z ................................................i................................................... 6900/9912
2020-04-22T10:44:06.5281652Z .................................................................................................... 7000/9912
2020-04-22T10:44:08.4677862Z ........................................................................................i........... 7100/9912
---
2020-04-22T10:45:36.8647932Z .................................................................................................... 7900/9912
2020-04-22T10:45:42.8540755Z .................................................................................................... 8000/9912
2020-04-22T10:45:48.1109717Z ......................................................i............................................. 8100/9912
2020-04-22T10:45:57.5004307Z .................................................................................................... 8200/9912
2020-04-22T10:46:02.6694906Z ...iiiiii.iiiii.i................................................................................... 8300/9912
2020-04-22T10:46:15.6897177Z .................................................................................................... 8500/9912
2020-04-22T10:46:23.4058813Z .................................................................................................... 8600/9912
2020-04-22T10:46:37.1589371Z .................................................................................................... 8700/9912
2020-04-22T10:46:43.9358268Z .................................................................................................... 8800/9912
---
2020-04-22T10:48:27.6064571Z 
2020-04-22T10:48:27.6066774Z ---- [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs stdout ----
2020-04-22T10:48:27.6067060Z diff of stderr:
2020-04-22T10:48:27.6082906Z 
2020-04-22T10:48:27.6083876Z 18              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-04-22T10:48:27.6084835Z 19              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-04-22T10:48:27.6086490Z 20              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-04-22T10:48:27.6087581Z +            and 17 others
2020-04-22T10:48:27.6093713Z 22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-22T10:48:27.6093713Z 22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-22T10:48:27.6094476Z 23    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-04-22T10:48:27.6094935Z 
2020-04-22T10:48:27.6095039Z 
2020-04-22T10:48:27.6095213Z The actual stderr differed from the expected stderr.
2020-04-22T10:48:27.6095213Z The actual stderr differed from the expected stderr.
2020-04-22T10:48:27.6096070Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
2020-04-22T10:48:27.6096704Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T10:48:27.6097296Z To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-types-no-impls-length-33.rs`
2020-04-22T10:48:27.6097726Z error: 1 errors occurred comparing output.
2020-04-22T10:48:27.6097927Z status: exit code: 1
2020-04-22T10:48:27.6097927Z status: exit code: 1
2020-04-22T10:48:27.6099780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary"
2020-04-22T10:48:27.6103675Z ------------------------------------------
2020-04-22T10:48:27.6103985Z 
2020-04-22T10:48:27.6104331Z ------------------------------------------
2020-04-22T10:48:27.6104507Z stderr:
2020-04-22T10:48:27.6104507Z stderr:
2020-04-22T10:48:27.6104855Z ------------------------------------------
2020-04-22T10:48:27.6105127Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-04-22T10:48:27.6105695Z   --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6:29
2020-04-22T10:48:27.6105969Z    |
2020-04-22T10:48:27.6106143Z LL |     let v: Vec<_> = [0; 33].into();
2020-04-22T10:48:27.6106503Z    |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
2020-04-22T10:48:27.6106815Z    |
2020-04-22T10:48:27.6107159Z    = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
2020-04-22T10:48:27.6107732Z    = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`
2020-04-22T10:48:27.6108036Z 
2020-04-22T10:48:27.6108346Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
2020-04-22T10:48:27.6109279Z    |
2020-04-22T10:48:27.6109279Z    |
2020-04-22T10:48:27.6109500Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6110011Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-22T10:48:27.6110612Z    = help: the following implementations were found:
2020-04-22T10:48:27.6110612Z    = help: the following implementations were found:
2020-04-22T10:48:27.6111180Z              <std::boxed::Box<(dyn std::error::Error + 'a)> as std::convert::From<E>>
2020-04-22T10:48:27.6111801Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-04-22T10:48:27.6112488Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-04-22T10:48:27.6113219Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-04-22T10:48:27.6113939Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-22T10:48:27.6113939Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-22T10:48:27.6114552Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-04-22T10:48:27.6114892Z 
2020-04-22T10:48:27.6115204Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
2020-04-22T10:48:27.6116133Z    |
2020-04-22T10:48:27.6116133Z    |
2020-04-22T10:48:27.6116357Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6116873Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-22T10:48:27.6117466Z    = help: the following implementations were found:
2020-04-22T10:48:27.6117466Z    = help: the following implementations were found:
2020-04-22T10:48:27.6117830Z              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-04-22T10:48:27.6118065Z 
2020-04-22T10:48:27.6118429Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-04-22T10:48:27.6119346Z    |
2020-04-22T10:48:27.6119346Z    |
2020-04-22T10:48:27.6119563Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6120107Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-22T10:48:27.6120683Z    = help: the following implementations were found:
2020-04-22T10:48:27.6120683Z    = help: the following implementations were found:
2020-04-22T10:48:27.6120989Z              <std::rc::Rc<T> as std::convert::From<T>>
2020-04-22T10:48:27.6121315Z              <std::rc::Rc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-22T10:48:27.6121645Z              <std::rc::Rc<[T]> as std::convert::From<&[T]>>
2020-04-22T10:48:27.6121993Z              <std::rc::Rc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-22T10:48:27.6122259Z            and 8 others
2020-04-22T10:48:27.6122629Z    = note: required because of the requirements on the impl of `std::convert::Into<std::rc::Rc<[i32; 33]>>` for `std::rc::Rc<[i32]>`
2020-04-22T10:48:27.6123218Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-04-22T10:48:27.6123538Z 
2020-04-22T10:48:27.6123840Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-04-22T10:48:27.6124769Z    |
2020-04-22T10:48:27.6124769Z    |
2020-04-22T10:48:27.6126030Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6126543Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-22T10:48:27.6127128Z    = help: the following implementations were found:
2020-04-22T10:48:27.6127128Z    = help: the following implementations were found:
2020-04-22T10:48:27.6127478Z              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-04-22T10:48:27.6127702Z 
2020-04-22T10:48:27.6128005Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-04-22T10:48:27.6128969Z    |
2020-04-22T10:48:27.6128969Z    |
2020-04-22T10:48:27.6129188Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6129696Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-22T10:48:27.6130281Z    = help: the following implementations were found:
2020-04-22T10:48:27.6130281Z    = help: the following implementations were found:
2020-04-22T10:48:27.6130594Z              <std::sync::Arc<T> as std::convert::From<T>>
2020-04-22T10:48:27.6130934Z              <std::sync::Arc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-22T10:48:27.6131276Z              <std::sync::Arc<[T]> as std::convert::From<&[T]>>
2020-04-22T10:48:27.6131631Z              <std::sync::Arc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-22T10:48:27.6131896Z            and 8 others
2020-04-22T10:48:27.6132278Z    = note: required because of the requirements on the impl of `std::convert::Into<std::sync::Arc<[i32; 33]>>` for `std::sync::Arc<[i32]>`
2020-04-22T10:48:27.6132888Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-04-22T10:48:27.6133217Z 
2020-04-22T10:48:27.6133524Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-04-22T10:48:27.6134529Z    |
2020-04-22T10:48:27.6134529Z    |
2020-04-22T10:48:27.6134750Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-22T10:48:27.6135259Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-22T10:48:27.6135847Z    = help: the following implementations were found:
2020-04-22T10:48:27.6135847Z    = help: the following implementations were found:
2020-04-22T10:48:27.6136259Z              <std::sync::Arc<[T; _]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
2020-04-22T10:48:27.6136657Z error: aborting due to 7 previous errors
2020-04-22T10:48:27.6136800Z 
2020-04-22T10:48:27.6137201Z For more information about this error, try `rustc --explain E0277`.
2020-04-22T10:48:27.6137378Z 
---
2020-04-22T10:48:27.6139829Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T10:48:27.6140183Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T10:48:27.6140378Z 
2020-04-22T10:48:27.6140461Z 
2020-04-22T10:48:27.6143572Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T10:48:27.6145706Z 
2020-04-22T10:48:27.6145791Z 
2020-04-22T10:48:27.6146277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T10:48:27.6146592Z Build completed unsuccessfully in 0:56:50
2020-04-22T10:48:27.6146592Z Build completed unsuccessfully in 0:56:50
2020-04-22T10:48:27.6158245Z == clock drift check ==
2020-04-22T10:48:27.6171924Z   local time: Wed Apr 22 10:48:27 UTC 2020
2020-04-22T10:48:27.9120286Z   network time: Wed, 22 Apr 2020 10:48:27 GMT
2020-04-22T10:48:28.4551641Z 
2020-04-22T10:48:28.4551641Z 
2020-04-22T10:48:28.4629738Z ##[error]Bash exited with code '1'.
2020-04-22T10:48:28.4642217Z ##[section]Finishing: Run build
2020-04-22T10:48:28.4679550Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71421/merge to s
2020-04-22T10:48:28.4684138Z Task         : Get sources
2020-04-22T10:48:28.4684421Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T10:48:28.4684699Z Version      : 1.0.0
2020-04-22T10:48:28.4685208Z Author       : Microsoft
2020-04-22T10:48:28.4685208Z Author       : Microsoft
2020-04-22T10:48:28.4685645Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T10:48:28.4685993Z ==============================================================================
2020-04-22T10:48:28.7862509Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T10:48:28.7907795Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71421/merge to s
2020-04-22T10:48:28.7988760Z Cleaning up task key
2020-04-22T10:48:28.7989761Z Start cleaning up orphan processes.
2020-04-22T10:48:28.8151841Z Terminate orphan process: pid (3783) (python)
2020-04-22T10:48:28.8358897Z ##[section]Finishing: Finalize Job

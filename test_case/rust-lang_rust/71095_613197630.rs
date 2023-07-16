plain
2020-04-14T01:52:23.5255490Z ========================== Starting Command Output ===========================
2020-04-14T01:52:23.5258283Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/956b826e-0b27-4433-b37c-d1c172e7392a.sh
2020-04-14T01:52:23.5258573Z 
2020-04-14T01:52:23.5262800Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T01:52:23.5283769Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-14T01:52:23.5287324Z Task         : Get sources
2020-04-14T01:52:23.5287673Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T01:52:23.5287991Z Version      : 1.0.0
2020-04-14T01:52:23.5288209Z Author       : Microsoft
---
2020-04-14T01:52:24.5285112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T01:52:24.5294449Z ##[command]git config gc.auto 0
2020-04-14T01:52:24.5303974Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T01:52:24.5309729Z ##[command]git config --get-all http.proxy
2020-04-14T01:52:24.5320127Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71095/merge:refs/remotes/pull/71095/merge
---
2020-04-14T01:54:45.0896259Z  ---> f58a2bb1e753
2020-04-14T01:54:45.0896965Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T01:54:45.0899197Z  ---> Using cache
2020-04-14T01:54:45.0899509Z  ---> d079cc6b6db8
2020-04-14T01:54:45.0900385Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T01:54:45.0903040Z  ---> 4183ca46ee56
2020-04-14T01:54:45.0903223Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T01:54:45.0905795Z  ---> Using cache
2020-04-14T01:54:45.0906065Z  ---> 69e7f8a2a2fb
---
2020-04-14T01:54:45.1297402Z Looks like docker image is the same as before, not uploading
2020-04-14T01:54:49.4784346Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:54:49.5024646Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:54:49.5051555Z == clock drift check ==
2020-04-14T01:54:49.5056845Z   local time: Tue Apr 14 01:54:49 UTC 2020
2020-04-14T01:54:49.8471443Z   network time: Tue, 14 Apr 2020 01:54:49 GMT
2020-04-14T01:54:49.8496704Z Starting sccache server...
2020-04-14T01:54:49.9300367Z configure: processing command line
2020-04-14T01:54:49.9301134Z configure: 
2020-04-14T01:54:49.9302190Z configure: rust.dist-src        := False
---
2020-04-14T01:59:31.3489871Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T01:59:32.6728405Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T01:59:34.0925023Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T01:59:34.8273998Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T01:59:42.8847766Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T01:59:44.6683457Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T01:59:48.4663955Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T01:59:52.0022812Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T02:00:00.9398709Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T02:18:50.3361165Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T02:18:51.8750720Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T02:18:53.6561135Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T02:18:55.0775232Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T02:19:04.0675678Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T02:19:06.3758186Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T02:19:10.8190741Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T02:19:15.4088240Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T02:19:24.9734752Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T02:41:18.2847229Z .................................................................................................... 1700/9891
2020-04-14T02:41:22.1066094Z .................................................................................................... 1800/9891
2020-04-14T02:41:29.5835333Z .................................................................................................... 1900/9891
2020-04-14T02:41:36.6680174Z ....i............................................................................................... 2000/9891
2020-04-14T02:41:42.3894414Z ..............................................................................................iiiii. 2100/9891
2020-04-14T02:42:00.3700827Z .................................................................................................... 2300/9891
2020-04-14T02:42:02.2192588Z .................................................................................................... 2400/9891
2020-04-14T02:42:04.1021095Z .................................................................................................... 2500/9891
2020-04-14T02:42:09.0822940Z .................................................................................................... 2600/9891
---
2020-04-14T02:44:49.3622894Z .................................................................................................... 5100/9891
2020-04-14T02:44:55.9042127Z .................................................................................................... 5200/9891
2020-04-14T02:45:00.5317366Z ..............i..................................................................................... 5300/9891
2020-04-14T02:45:09.0214079Z ....i............................................................................................... 5400/9891
2020-04-14T02:45:13.5561293Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-14T02:45:20.2194199Z ..................................................i................................................. 5700/9891
2020-04-14T02:45:29.1047939Z ......................................................................ii............................ 5800/9891
2020-04-14T02:45:34.9702389Z .........i.......................................................................................... 5900/9891
2020-04-14T02:45:40.0375368Z .................................................................................................... 6000/9891
2020-04-14T02:45:40.0375368Z .................................................................................................... 6000/9891
2020-04-14T02:45:48.9666803Z .................................................................................................... 6100/9891
2020-04-14T02:45:58.5213847Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-14T02:46:11.0911625Z .................................................................................................... 6400/9891
2020-04-14T02:46:14.0233247Z .................................................................................................... 6500/9891
2020-04-14T02:46:14.0233247Z .................................................................................................... 6500/9891
2020-04-14T02:46:24.7503265Z .................................i..ii.............................................................. 6600/9891
2020-04-14T02:46:43.6231655Z .................................................................................................... 6800/9891
2020-04-14T02:46:45.3577576Z .................................i.................................................................. 6900/9891
2020-04-14T02:46:47.1400238Z .................................................................................................... 7000/9891
2020-04-14T02:46:49.0312391Z ........................................................................i........................... 7100/9891
---
2020-04-14T02:48:14.4834593Z .................................................................................................... 7800/9891
2020-04-14T02:48:18.3065098Z .................................................................................................... 7900/9891
2020-04-14T02:48:24.0568917Z .................................................................................................... 8000/9891
2020-04-14T02:48:29.7944665Z ......................................i............................................................. 8100/9891
2020-04-14T02:48:37.8969212Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-14T02:48:51.5595531Z ................................i......i............................................................ 8400/9891
2020-04-14T02:48:54.5828391Z .................................................................................................... 8500/9891
2020-04-14T02:49:03.4966655Z .................................................................................................... 8600/9891
2020-04-14T02:49:14.7641047Z .................................................................................................... 8700/9891
---
2020-04-14T02:50:54.1855128Z 
2020-04-14T02:50:54.1856039Z ---- [ui] ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs stdout ----
2020-04-14T02:50:54.1856435Z diff of stderr:
2020-04-14T02:50:54.1856658Z 
2020-04-14T02:50:54.1857468Z 18              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-04-14T02:50:54.1858357Z 19              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-04-14T02:50:54.1859224Z 20              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-04-14T02:50:54.1860147Z +            and 17 others
2020-04-14T02:50:54.1860673Z 22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-14T02:50:54.1860673Z 22    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-14T02:50:54.1861419Z 23    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-04-14T02:50:54.1862095Z 
2020-04-14T02:50:54.1862279Z 
2020-04-14T02:50:54.1862565Z The actual stderr differed from the expected stderr.
2020-04-14T02:50:54.1862565Z The actual stderr differed from the expected stderr.
2020-04-14T02:50:54.1863728Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
2020-04-14T02:50:54.1866032Z To update references, rerun the tests and pass the `--bless` flag
2020-04-14T02:50:54.1867537Z To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-types-no-impls-length-33.rs`
2020-04-14T02:50:54.1868939Z error: 1 errors occurred comparing output.
2020-04-14T02:50:54.1869222Z status: exit code: 1
2020-04-14T02:50:54.1869222Z status: exit code: 1
2020-04-14T02:50:54.1872127Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary"
2020-04-14T02:50:54.1876523Z ------------------------------------------
2020-04-14T02:50:54.1876653Z 
2020-04-14T02:50:54.1876941Z ------------------------------------------
2020-04-14T02:50:54.1877095Z stderr:
2020-04-14T02:50:54.1877095Z stderr:
2020-04-14T02:50:54.1877402Z ------------------------------------------
2020-04-14T02:50:54.1877643Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-04-14T02:50:54.1878149Z   --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6:29
2020-04-14T02:50:54.1878388Z    |
2020-04-14T02:50:54.1878540Z LL |     let v: Vec<_> = [0; 33].into();
2020-04-14T02:50:54.1878859Z    |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
2020-04-14T02:50:54.1879133Z    |
2020-04-14T02:50:54.1879437Z    = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
2020-04-14T02:50:54.1879944Z    = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`
2020-04-14T02:50:54.1880211Z 
2020-04-14T02:50:54.1880487Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
2020-04-14T02:50:54.1881407Z    |
2020-04-14T02:50:54.1881407Z    |
2020-04-14T02:50:54.1881638Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1882141Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-14T02:50:54.1882670Z    = help: the following implementations were found:
2020-04-14T02:50:54.1882670Z    = help: the following implementations were found:
2020-04-14T02:50:54.1883352Z              <std::boxed::Box<(dyn std::error::Error + 'a)> as std::convert::From<E>>
2020-04-14T02:50:54.1883979Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-04-14T02:50:54.1884678Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-04-14T02:50:54.1885411Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-04-14T02:50:54.1886145Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-14T02:50:54.1886145Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-04-14T02:50:54.1886781Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-04-14T02:50:54.1887119Z 
2020-04-14T02:50:54.1887439Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
2020-04-14T02:50:54.1888384Z    |
2020-04-14T02:50:54.1888384Z    |
2020-04-14T02:50:54.1888609Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1889278Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-04-14T02:50:54.1889996Z    = help: the following implementations were found:
2020-04-14T02:50:54.1889996Z    = help: the following implementations were found:
2020-04-14T02:50:54.1890367Z              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-04-14T02:50:54.1890603Z 
2020-04-14T02:50:54.1892838Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-04-14T02:50:54.1894049Z    |
2020-04-14T02:50:54.1894049Z    |
2020-04-14T02:50:54.1894273Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1894778Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-14T02:50:54.1895360Z    = help: the following implementations were found:
2020-04-14T02:50:54.1895360Z    = help: the following implementations were found:
2020-04-14T02:50:54.1895761Z              <std::rc::Rc<T> as std::convert::From<T>>
2020-04-14T02:50:54.1896054Z              <std::rc::Rc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-14T02:50:54.1896348Z              <std::rc::Rc<[T]> as std::convert::From<&[T]>>
2020-04-14T02:50:54.1896661Z              <std::rc::Rc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-14T02:50:54.1896893Z            and 8 others
2020-04-14T02:50:54.1897221Z    = note: required because of the requirements on the impl of `std::convert::Into<std::rc::Rc<[i32; 33]>>` for `std::rc::Rc<[i32]>`
2020-04-14T02:50:54.1897747Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-04-14T02:50:54.1898028Z 
2020-04-14T02:50:54.1898295Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-04-14T02:50:54.1899114Z    |
2020-04-14T02:50:54.1899114Z    |
2020-04-14T02:50:54.1899394Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1899840Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-04-14T02:50:54.1900400Z    = help: the following implementations were found:
2020-04-14T02:50:54.1900400Z    = help: the following implementations were found:
2020-04-14T02:50:54.1900718Z              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-04-14T02:50:54.1900912Z 
2020-04-14T02:50:54.1901182Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-04-14T02:50:54.1902008Z    |
2020-04-14T02:50:54.1902008Z    |
2020-04-14T02:50:54.1902202Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1902654Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-14T02:50:54.1903174Z    = help: the following implementations were found:
2020-04-14T02:50:54.1903174Z    = help: the following implementations were found:
2020-04-14T02:50:54.1903454Z              <std::sync::Arc<T> as std::convert::From<T>>
2020-04-14T02:50:54.1903755Z              <std::sync::Arc<T> as std::convert::From<std::boxed::Box<T>>>
2020-04-14T02:50:54.1904057Z              <std::sync::Arc<[T]> as std::convert::From<&[T]>>
2020-04-14T02:50:54.1904375Z              <std::sync::Arc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-04-14T02:50:54.1904607Z            and 8 others
2020-04-14T02:50:54.1904947Z    = note: required because of the requirements on the impl of `std::convert::Into<std::sync::Arc<[i32; 33]>>` for `std::sync::Arc<[i32]>`
2020-04-14T02:50:54.1905494Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-04-14T02:50:54.1905955Z 
2020-04-14T02:50:54.1906276Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-04-14T02:50:54.1908278Z    |
2020-04-14T02:50:54.1908278Z    |
2020-04-14T02:50:54.1908528Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-04-14T02:50:54.1909091Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-04-14T02:50:54.1909740Z    = help: the following implementations were found:
2020-04-14T02:50:54.1909740Z    = help: the following implementations were found:
2020-04-14T02:50:54.1910134Z              <std::sync::Arc<[T; _]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
2020-04-14T02:50:54.1910560Z error: aborting due to 7 previous errors
2020-04-14T02:50:54.1910715Z 
2020-04-14T02:50:54.1911147Z For more information about this error, try `rustc --explain E0277`.
2020-04-14T02:50:54.1911343Z 
---
2020-04-14T02:50:54.1914843Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-14T02:50:54.1915238Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-14T02:50:54.1915447Z 
2020-04-14T02:50:54.1915537Z 
2020-04-14T02:50:54.1919082Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-14T02:50:54.1921472Z 
2020-04-14T02:50:54.1921566Z 
2020-04-14T02:50:54.1925229Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-14T02:50:54.1925584Z Build completed unsuccessfully in 0:54:32
2020-04-14T02:50:54.1925584Z Build completed unsuccessfully in 0:54:32
2020-04-14T02:50:54.1951479Z == clock drift check ==
2020-04-14T02:50:54.1970457Z   local time: Tue Apr 14 02:50:54 UTC 2020
2020-04-14T02:50:54.3640956Z   network time: Tue, 14 Apr 2020 02:50:54 GMT
2020-04-14T02:50:54.7691001Z 
2020-04-14T02:50:54.7691001Z 
2020-04-14T02:50:54.7759159Z ##[error]Bash exited with code '1'.
2020-04-14T02:50:54.7771721Z ##[section]Finishing: Run build
2020-04-14T02:50:54.7810427Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-14T02:50:54.7815350Z Task         : Get sources
2020-04-14T02:50:54.7815640Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T02:50:54.7815918Z Version      : 1.0.0
2020-04-14T02:50:54.7816112Z Author       : Microsoft
2020-04-14T02:50:54.7816112Z Author       : Microsoft
2020-04-14T02:50:54.7816418Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T02:50:54.7816768Z ==============================================================================
2020-04-14T02:50:55.0632784Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T02:50:55.0677368Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71095/merge to s
2020-04-14T02:50:55.0807720Z Cleaning up task key
2020-04-14T02:50:55.0808740Z Start cleaning up orphan processes.
2020-04-14T02:50:55.0993571Z Terminate orphan process: pid (3571) (python)
2020-04-14T02:50:55.1131863Z ##[section]Finishing: Finalize Job

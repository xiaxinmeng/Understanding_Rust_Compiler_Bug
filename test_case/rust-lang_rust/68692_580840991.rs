plain
2020-01-31T16:47:13.2283828Z ========================== Starting Command Output ===========================
2020-01-31T16:47:13.2285521Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8494ac8b-c2de-4661-ab10-4764e233a11e.sh
2020-01-31T16:47:13.2285563Z 
2020-01-31T16:47:13.2288786Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T16:47:13.2295616Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-01-31T16:47:13.2297247Z Task         : Get sources
2020-01-31T16:47:13.2297292Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T16:47:13.2297323Z Version      : 1.0.0
2020-01-31T16:47:13.2297355Z Author       : Microsoft
---
2020-01-31T16:47:14.3565966Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T16:47:14.3707271Z ##[command]git config gc.auto 0
2020-01-31T16:47:14.3798408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T16:47:14.3855920Z ##[command]git config --get-all http.proxy
2020-01-31T16:47:14.4006395Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68692/merge:refs/remotes/pull/68692/merge
---
2020-01-31T17:45:48.6895951Z .................................................................................................... 1700/9559
2020-01-31T17:45:53.6497826Z .................................................................................................... 1800/9559
2020-01-31T17:46:06.4700966Z .........................i.......................................................................... 1900/9559
2020-01-31T17:46:13.5974634Z .................................................................................................... 2000/9559
2020-01-31T17:46:28.8792940Z ...............iiiii................................................................................ 2100/9559
2020-01-31T17:46:39.1586341Z .................................................................................................... 2300/9559
2020-01-31T17:46:41.6018929Z .................................................................................................... 2400/9559
2020-01-31T17:46:46.7339329Z .................................................................................................... 2500/9559
2020-01-31T17:47:08.5064680Z .................................................................................................... 2600/9559
---
2020-01-31T17:49:47.1039331Z .................................................................................................... 4800/9559
2020-01-31T17:49:52.3004759Z ..........................................................i...............i......................... 4900/9559
2020-01-31T17:50:00.3243941Z .................................................................................................... 5000/9559
2020-01-31T17:50:08.4201517Z .................................................................................................... 5100/9559
2020-01-31T17:50:13.3735583Z .i.................................................................................................. 5200/9559
2020-01-31T17:50:24.6377701Z ..........................................................................ii.ii........i...i........ 5300/9559
2020-01-31T17:50:33.5199608Z ............i....................................................................................... 5500/9559
2020-01-31T17:50:43.8331599Z .................................................................................................... 5600/9559
2020-01-31T17:50:50.2928316Z .............................................................i...................................... 5700/9559
2020-01-31T17:50:57.6439659Z .................................................................................................... 5800/9559
2020-01-31T17:50:57.6439659Z .................................................................................................... 5800/9559
2020-01-31T17:51:05.8640614Z .................................................................................................... 5900/9559
2020-01-31T17:51:15.1456552Z ....................................................ii...i..ii...........i.......................... 6000/9559
2020-01-31T17:51:37.5272499Z .................................................................................................... 6200/9559
2020-01-31T17:51:45.2500072Z .................................................................................................... 6300/9559
2020-01-31T17:51:45.2500072Z .................................................................................................... 6300/9559
2020-01-31T17:51:53.5943226Z ................................................................................i..ii............... 6400/9559
2020-01-31T17:52:21.2384368Z .................................................................................................... 6600/9559
2020-01-31T17:52:26.9210346Z ........................................................i........................................... 6700/9559
2020-01-31T17:52:29.1152249Z .................................................................................................... 6800/9559
2020-01-31T17:52:31.4222298Z ........................................................i........................................... 6900/9559
---
2020-01-31T17:54:16.3607311Z .................................................................................................... 7600/9559
2020-01-31T17:54:21.8063566Z .................................................................................................... 7700/9559
2020-01-31T17:54:28.7183430Z .................................................................................................... 7800/9559
2020-01-31T17:54:39.7110149Z .................................................................................................... 7900/9559
2020-01-31T17:54:46.0561258Z ............iiiiiii.i............................................................................... 8000/9559
2020-01-31T17:55:00.9035793Z .................................................................................................... 8200/9559
2020-01-31T17:55:11.9162722Z .................................................................................................... 8300/9559
2020-01-31T17:55:25.7104227Z .................................................................................................... 8400/9559
2020-01-31T17:55:32.6473821Z .................................................................................................... 8500/9559
---
2020-01-31T17:57:31.5426133Z 
2020-01-31T17:57:31.5426186Z + error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-31T17:57:31.5426495Z +   --> $DIR/alloc-types-no-impls-length-33.rs:6:29
2020-01-31T17:57:31.5426548Z +    |
2020-01-31T17:57:31.5426595Z + LL |     let v: Vec<_> = [0; 33].into();
2020-01-31T17:57:31.5426674Z +    |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
2020-01-31T17:57:31.5426727Z +    |
2020-01-31T17:57:31.5426787Z +    = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
2020-01-31T17:57:31.5426873Z +    = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`
2020-01-31T17:57:31.5426936Z + 
2020-01-31T17:57:31.5427000Z 1 error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
2020-01-31T17:57:31.5427686Z +   --> $DIR/alloc-types-no-impls-length-33.rs:12:23
2020-01-31T17:57:31.5427731Z 3    |
2020-01-31T17:57:31.5427731Z 3    |
2020-01-31T17:57:31.5427792Z 4 LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5427854Z 5    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5427894Z 
2020-01-31T17:57:31.5427966Z 14    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5428016Z 15 
2020-01-31T17:57:31.5428073Z 16 error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
2020-01-31T17:57:31.5428575Z +   --> $DIR/alloc-types-no-impls-length-33.rs:12:23
2020-01-31T17:57:31.5428621Z 18    |
2020-01-31T17:57:31.5428621Z 18    |
2020-01-31T17:57:31.5428683Z 19 LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5428743Z 20    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5428781Z 
2020-01-31T17:57:31.5428847Z 23              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-01-31T17:57:31.5428891Z 24 
2020-01-31T17:57:31.5428942Z 25 error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5429412Z +   --> $DIR/alloc-types-no-impls-length-33.rs:19:23
2020-01-31T17:57:31.5429639Z 27    |
2020-01-31T17:57:31.5429639Z 27    |
2020-01-31T17:57:31.5429794Z 28 LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5429859Z 29    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5429896Z 
2020-01-31T17:57:31.5429951Z 38    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5430014Z 39 
2020-01-31T17:57:31.5430064Z 40 error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5430576Z +   --> $DIR/alloc-types-no-impls-length-33.rs:19:23
2020-01-31T17:57:31.5430620Z 42    |
2020-01-31T17:57:31.5430620Z 42    |
2020-01-31T17:57:31.5430665Z 43 LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5430756Z 44    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5430793Z 
2020-01-31T17:57:31.5430839Z 47              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-01-31T17:57:31.5430898Z 48 
2020-01-31T17:57:31.5430949Z 49 error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5431418Z +   --> $DIR/alloc-types-no-impls-length-33.rs:26:23
2020-01-31T17:57:31.5431463Z 51    |
2020-01-31T17:57:31.5431463Z 51    |
2020-01-31T17:57:31.5431508Z 52 LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5431582Z 53    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5431627Z 
2020-01-31T17:57:31.5431686Z 62    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5431735Z 63 
2020-01-31T17:57:31.5431802Z 64 error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5432256Z +   --> $DIR/alloc-types-no-impls-length-33.rs:26:23
2020-01-31T17:57:31.5432317Z 66    |
2020-01-31T17:57:31.5432317Z 66    |
2020-01-31T17:57:31.5432362Z 67 LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5432421Z 68    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5432521Z 70    = help: the following implementations were found:
2020-01-31T17:57:31.5432521Z 70    = help: the following implementations were found:
2020-01-31T17:57:31.5432580Z 71              <std::sync::Arc<[T; _]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
2020-01-31T17:57:31.5432866Z - error: aborting due to 6 previous errors
2020-01-31T17:57:31.5432915Z + error: aborting due to 7 previous errors
2020-01-31T17:57:31.5432972Z 74 
2020-01-31T17:57:31.5433215Z 75 For more information about this error, try `rustc --explain E0277`.
2020-01-31T17:57:31.5433215Z 75 For more information about this error, try `rustc --explain E0277`.
2020-01-31T17:57:31.5433261Z 76 
2020-01-31T17:57:31.5433286Z 
2020-01-31T17:57:31.5433311Z 
2020-01-31T17:57:31.5433369Z The actual stderr differed from the expected stderr.
2020-01-31T17:57:31.5433718Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/alloc-types-no-impls-length-33.stderr
2020-01-31T17:57:31.5434443Z To update references, rerun the tests and pass the `--bless` flag
2020-01-31T17:57:31.5434773Z To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-types-no-impls-length-33.rs`
2020-01-31T17:57:31.5434975Z error: 1 errors occurred comparing output.
2020-01-31T17:57:31.5435097Z status: exit code: 1
2020-01-31T17:57:31.5435097Z status: exit code: 1
2020-01-31T17:57:31.5436055Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33/auxiliary" "-A" "unused"
2020-01-31T17:57:31.5436405Z ------------------------------------------
2020-01-31T17:57:31.5436450Z 
2020-01-31T17:57:31.5436695Z ------------------------------------------
2020-01-31T17:57:31.5436742Z stderr:
2020-01-31T17:57:31.5436742Z stderr:
2020-01-31T17:57:31.5436971Z ------------------------------------------
2020-01-31T17:57:31.5437044Z error[E0277]: arrays only have std trait implementations for lengths 0..=32
2020-01-31T17:57:31.5437331Z   --> /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6:29
2020-01-31T17:57:31.5437386Z    |
2020-01-31T17:57:31.5437448Z LL |     let v: Vec<_> = [0; 33].into();
2020-01-31T17:57:31.5437506Z    |                             ^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[{integer}; 33]`
2020-01-31T17:57:31.5437558Z    |
2020-01-31T17:57:31.5437633Z    = note: required because of the requirements on the impl of `std::convert::From<[{integer}; 33]>` for `std::vec::Vec<{integer}>`
2020-01-31T17:57:31.5437698Z    = note: required because of the requirements on the impl of `std::convert::Into<std::vec::Vec<{integer}>>` for `[{integer}; 33]`
2020-01-31T17:57:31.5437744Z 
2020-01-31T17:57:31.5437820Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::From<std::boxed::Box<[i32]>>` is not satisfied
2020-01-31T17:57:31.5438389Z    |
2020-01-31T17:57:31.5438389Z    |
2020-01-31T17:57:31.5438433Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5438492Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5438603Z    = help: the following implementations were found:
2020-01-31T17:57:31.5438603Z    = help: the following implementations were found:
2020-01-31T17:57:31.5438859Z              <std::boxed::Box<(dyn std::error::Error + 'a)> as std::convert::From<E>>
2020-01-31T17:57:31.5439136Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<&str>>
2020-01-31T17:57:31.5439416Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::borrow::Cow<'a, str>>>
2020-01-31T17:57:31.5439710Z              <std::boxed::Box<(dyn std::error::Error + 'static)> as std::convert::From<std::string::String>>
2020-01-31T17:57:31.5439779Z            and 16 others
2020-01-31T17:57:31.5439834Z    = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<[i32; 33]>>` for `std::boxed::Box<[i32]>`
2020-01-31T17:57:31.5439913Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::boxed::Box<[i32]>>` for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5439949Z 
2020-01-31T17:57:31.5440000Z error[E0277]: the trait bound `std::boxed::Box<[i32; 33]>: std::convert::TryFrom<std::boxed::Box<[i32]>>` is not satisfied
2020-01-31T17:57:31.5440334Z    |
2020-01-31T17:57:31.5440334Z    |
2020-01-31T17:57:31.5440379Z LL |     let boxed_array = <Box<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5440584Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::boxed::Box<[i32]>>` is not implemented for `std::boxed::Box<[i32; 33]>`
2020-01-31T17:57:31.5440696Z    = help: the following implementations were found:
2020-01-31T17:57:31.5440696Z    = help: the following implementations were found:
2020-01-31T17:57:31.5440746Z              <std::boxed::Box<[T; _]> as std::convert::TryFrom<std::boxed::Box<[T]>>>
2020-01-31T17:57:31.5440793Z 
2020-01-31T17:57:31.5440842Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::From<std::rc::Rc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5441356Z    |
2020-01-31T17:57:31.5441356Z    |
2020-01-31T17:57:31.5441399Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5441456Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5441574Z    = help: the following implementations were found:
2020-01-31T17:57:31.5441574Z    = help: the following implementations were found:
2020-01-31T17:57:31.5441619Z              <std::rc::Rc<T> as std::convert::From<T>>
2020-01-31T17:57:31.5441681Z              <std::rc::Rc<T> as std::convert::From<std::boxed::Box<T>>>
2020-01-31T17:57:31.5441727Z              <std::rc::Rc<[T]> as std::convert::From<&[T]>>
2020-01-31T17:57:31.5441774Z              <std::rc::Rc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-01-31T17:57:31.5441832Z            and 8 others
2020-01-31T17:57:31.5441883Z    = note: required because of the requirements on the impl of `std::convert::Into<std::rc::Rc<[i32; 33]>>` for `std::rc::Rc<[i32]>`
2020-01-31T17:57:31.5441943Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::rc::Rc<[i32]>>` for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5441977Z 
2020-01-31T17:57:31.5442041Z error[E0277]: the trait bound `std::rc::Rc<[i32; 33]>: std::convert::TryFrom<std::rc::Rc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5442366Z    |
2020-01-31T17:57:31.5442366Z    |
2020-01-31T17:57:31.5442425Z LL |     let boxed_array = <Rc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5442481Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::rc::Rc<[i32]>>` is not implemented for `std::rc::Rc<[i32; 33]>`
2020-01-31T17:57:31.5442585Z    = help: the following implementations were found:
2020-01-31T17:57:31.5442585Z    = help: the following implementations were found:
2020-01-31T17:57:31.5442634Z              <std::rc::Rc<[T; _]> as std::convert::TryFrom<std::rc::Rc<[T]>>>
2020-01-31T17:57:31.5442663Z 
2020-01-31T17:57:31.5442726Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::From<std::sync::Arc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5443763Z    |
2020-01-31T17:57:31.5443763Z    |
2020-01-31T17:57:31.5443831Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5443889Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5444503Z    = help: the following implementations were found:
2020-01-31T17:57:31.5444503Z    = help: the following implementations were found:
2020-01-31T17:57:31.5444554Z              <std::sync::Arc<T> as std::convert::From<T>>
2020-01-31T17:57:31.5444606Z              <std::sync::Arc<T> as std::convert::From<std::boxed::Box<T>>>
2020-01-31T17:57:31.5444674Z              <std::sync::Arc<[T]> as std::convert::From<&[T]>>
2020-01-31T17:57:31.5444727Z              <std::sync::Arc<[T]> as std::convert::From<std::vec::Vec<T>>>
2020-01-31T17:57:31.5444775Z            and 8 others
2020-01-31T17:57:31.5444849Z    = note: required because of the requirements on the impl of `std::convert::Into<std::sync::Arc<[i32; 33]>>` for `std::sync::Arc<[i32]>`
2020-01-31T17:57:31.5445092Z    = note: required because of the requirements on the impl of `std::convert::TryFrom<std::sync::Arc<[i32]>>` for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5445134Z 
2020-01-31T17:57:31.5445188Z error[E0277]: the trait bound `std::sync::Arc<[i32; 33]>: std::convert::TryFrom<std::sync::Arc<[i32]>>` is not satisfied
2020-01-31T17:57:31.5445629Z    |
2020-01-31T17:57:31.5445629Z    |
2020-01-31T17:57:31.5445676Z LL |     let boxed_array = <Arc<[i32; 33]>>::try_from(boxed_slice);
2020-01-31T17:57:31.5445756Z    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::TryFrom<std::sync::Arc<[i32]>>` is not implemented for `std::sync::Arc<[i32; 33]>`
2020-01-31T17:57:31.5445856Z    = help: the following implementations were found:
2020-01-31T17:57:31.5445856Z    = help: the following implementations were found:
2020-01-31T17:57:31.5445925Z              <std::sync::Arc<[T; _]> as std::convert::TryFrom<std::sync::Arc<[T]>>>
2020-01-31T17:57:31.5446013Z error: aborting due to 7 previous errors
2020-01-31T17:57:31.5446051Z 
2020-01-31T17:57:31.5446323Z For more information about this error, try `rustc --explain E0277`.
2020-01-31T17:57:31.5446357Z 
---
2020-01-31T17:57:31.5456269Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-01-31T17:57:31.5456370Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-31T17:57:31.5470752Z 
2020-01-31T17:57:31.5471262Z 
2020-01-31T17:57:31.5473115Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-31T17:57:31.5473382Z 
2020-01-31T17:57:31.5473413Z 
2020-01-31T17:57:31.5486580Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-31T17:57:31.5486656Z Build completed unsuccessfully in 1:04:44
2020-01-31T17:57:31.5486656Z Build completed unsuccessfully in 1:04:44
2020-01-31T17:57:31.5549796Z == clock drift check ==
2020-01-31T17:57:31.5574304Z   local time: Fri Jan 31 17:57:31 UTC 2020
2020-01-31T17:57:31.8523110Z   network time: Fri, 31 Jan 2020 17:57:31 GMT
2020-01-31T17:57:31.8524880Z == end clock drift check ==
2020-01-31T17:57:32.2187630Z 
2020-01-31T17:57:32.2302734Z ##[error]Bash exited with code '1'.
2020-01-31T17:57:32.2318078Z ##[section]Finishing: Run build
2020-01-31T17:57:32.2346784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-01-31T17:57:32.2349235Z Task         : Get sources
2020-01-31T17:57:32.2349296Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T17:57:32.2349340Z Version      : 1.0.0
2020-01-31T17:57:32.2349379Z Author       : Microsoft
2020-01-31T17:57:32.2349379Z Author       : Microsoft
2020-01-31T17:57:32.2349441Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-31T17:57:32.2349487Z ==============================================================================
2020-01-31T17:57:32.6989508Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-31T17:57:32.7031117Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-01-31T17:57:32.7149908Z Cleaning up task key
2020-01-31T17:57:32.7150644Z Start cleaning up orphan processes.
2020-01-31T17:57:32.7294378Z Terminate orphan process: pid (3814) (python)
2020-01-31T17:57:32.7537746Z ##[section]Finishing: Finalize Job

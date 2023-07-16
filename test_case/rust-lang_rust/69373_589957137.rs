plain
2020-02-22T13:20:09.3100763Z ========================== Starting Command Output ===========================
2020-02-22T13:20:09.3104246Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3cf37151-c0ea-4fbe-8e34-753d22628c2e.sh
2020-02-22T13:20:09.3104582Z 
2020-02-22T13:20:09.3107926Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T13:20:09.3123541Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T13:20:09.3126792Z Task         : Get sources
2020-02-22T13:20:09.3127010Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T13:20:09.3127308Z Version      : 1.0.0
2020-02-22T13:20:09.3127454Z Author       : Microsoft
---
2020-02-22T13:20:12.2310592Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T13:20:12.2319094Z ##[command]git config gc.auto 0
2020-02-22T13:20:12.2323706Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T13:20:12.2330173Z ##[command]git config --get-all http.proxy
2020-02-22T13:20:12.2340757Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69373/merge:refs/remotes/pull/69373/merge
---
2020-02-22T13:25:53.4113888Z    Compiling libc v0.2.66
2020-02-22T13:25:54.8057326Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i8, [u8; 1]>` is not stable as `const fn`
2020-02-22T13:25:54.8058955Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:54.8059637Z      |
2020-02-22T13:25:54.8060623Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.8061591Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:54.8062598Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:54.8063657Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:54.8064926Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:54.8065629Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:54.8066134Z ...    |
2020-02-22T13:25:54.8066626Z 2372 | |     }
2020-02-22T13:25:54.8066626Z 2372 | |     }
2020-02-22T13:25:54.8067159Z 2373 | | }
2020-02-22T13:25:54.8067752Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:54.8068297Z ...
2020-02-22T13:25:54.8068913Z 2377 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T13:25:54.8069639Z 2378 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T13:25:54.8070768Z      |
2020-02-22T13:25:54.8070768Z      |
2020-02-22T13:25:54.8071393Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:54.8072030Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:54.8617903Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i16, [u8; 2]>` is not stable as `const fn`
2020-02-22T13:25:54.8618703Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:54.8619140Z      |
2020-02-22T13:25:54.8619650Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.8619650Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.8620457Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:54.8621439Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:54.8622307Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:54.8623504Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:54.8624197Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:54.8624698Z ...    |
2020-02-22T13:25:54.8625356Z 2372 | |     }
2020-02-22T13:25:54.8625356Z 2372 | |     }
2020-02-22T13:25:54.8626112Z 2373 | | }
2020-02-22T13:25:54.8626743Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:54.8627318Z ...
2020-02-22T13:25:54.8628078Z 2383 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T13:25:54.8629233Z 2384 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T13:25:54.8630470Z      |
2020-02-22T13:25:54.8630470Z      |
2020-02-22T13:25:54.8631095Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:54.8631928Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:54.9202818Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i32, [u8; 4]>` is not stable as `const fn`
2020-02-22T13:25:54.9203589Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:54.9204047Z      |
2020-02-22T13:25:54.9204608Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.9204608Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.9205539Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:54.9206729Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:54.9207688Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:54.9208987Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:54.9209761Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:54.9210293Z ...    |
2020-02-22T13:25:54.9210819Z 2372 | |     }
2020-02-22T13:25:54.9210819Z 2372 | |     }
2020-02-22T13:25:54.9211425Z 2373 | | }
2020-02-22T13:25:54.9212069Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:54.9212643Z ...
2020-02-22T13:25:54.9213344Z 2389 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T13:25:54.9214217Z 2390 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:54.9214997Z 2391 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T13:25:54.9216254Z      |
2020-02-22T13:25:54.9216254Z      |
2020-02-22T13:25:54.9217064Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:54.9217803Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:54.9779016Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i64, [u8; 8]>` is not stable as `const fn`
2020-02-22T13:25:54.9779736Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:54.9780157Z      |
2020-02-22T13:25:54.9780660Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.9780660Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:54.9781483Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:54.9782635Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:54.9783505Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:54.9784701Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:54.9785397Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:54.9785905Z ...    |
2020-02-22T13:25:54.9786400Z 2372 | |     }
2020-02-22T13:25:54.9786400Z 2372 | |     }
2020-02-22T13:25:54.9786948Z 2373 | | }
2020-02-22T13:25:54.9787543Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:54.9787948Z ...
2020-02-22T13:25:54.9788583Z 2396 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T13:25:54.9789386Z 2397 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T13:25:54.9790175Z 2398 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:54.9790960Z 2399 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T13:25:54.9792241Z      |
2020-02-22T13:25:54.9792241Z      |
2020-02-22T13:25:54.9792863Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:54.9793521Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.0375372Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i128, [u8; 16]>` is not stable as `const fn`
2020-02-22T13:25:55.0376435Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:55.0377260Z      |
2020-02-22T13:25:55.0378002Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.0378002Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.0378990Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.0380629Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.0382378Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.0384001Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.0385101Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.0385620Z ...    |
2020-02-22T13:25:55.0386130Z 2372 | |     }
2020-02-22T13:25:55.0386130Z 2372 | |     }
2020-02-22T13:25:55.0386716Z 2373 | | }
2020-02-22T13:25:55.0387322Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.0387750Z ...
2020-02-22T13:25:55.0388401Z 2404 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T13:25:55.0389386Z 2405 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T13:25:55.0390639Z 2406 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T13:25:55.0391758Z 2407 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T13:25:55.0392505Z ...    |
2020-02-22T13:25:55.0393388Z 2410 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T13:25:55.0394408Z 2411 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T13:25:55.0396174Z      |
2020-02-22T13:25:55.0396174Z      |
2020-02-22T13:25:55.0397022Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.0397906Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.0398263Z 
2020-02-22T13:25:55.0997787Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<isize, [u8; 8]>` is not stable as `const fn`
2020-02-22T13:25:55.0998957Z     --> src/libcore/num/mod.rs:2259:26
2020-02-22T13:25:55.1000727Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.1000727Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.1001763Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.1003016Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.1004248Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.1005851Z 2259 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.1006722Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.1007334Z ...    |
2020-02-22T13:25:55.1007960Z 2372 | |     }
2020-02-22T13:25:55.1007960Z 2372 | |     }
2020-02-22T13:25:55.1008619Z 2373 | | }
2020-02-22T13:25:55.1009347Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.1009850Z ...
2020-02-22T13:25:55.1010610Z 2434 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2020-02-22T13:25:55.1011641Z 2435 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2020-02-22T13:25:55.1012753Z 2436 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.1013717Z 2437 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T13:25:55.1014832Z 2438 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T13:25:55.1016657Z      |
2020-02-22T13:25:55.1016657Z      |
2020-02-22T13:25:55.1017497Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.1018718Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.1019219Z 
2020-02-22T13:25:55.1565286Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 1], i8>` is not stable as `const fn`
2020-02-22T13:25:55.1566357Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.1568434Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.1568434Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.1569751Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.1571420Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.1572515Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.1573192Z ...    |
2020-02-22T13:25:55.1576174Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.1578975Z ...    |
2020-02-22T13:25:55.1579894Z 2372 | |     }
2020-02-22T13:25:55.1580880Z 2373 | | }
2020-02-22T13:25:55.1581709Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.1581709Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.1582434Z ...
2020-02-22T13:25:55.1583299Z 2377 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T13:25:55.1584280Z 2378 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T13:25:55.1586407Z      |
2020-02-22T13:25:55.1586407Z      |
2020-02-22T13:25:55.1587685Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.1588570Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.1588930Z 
2020-02-22T13:25:55.2174359Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 2], i16>` is not stable as `const fn`
2020-02-22T13:25:55.2175201Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.2176271Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.2176271Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.2177297Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.2178484Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.2179641Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.2180526Z ...    |
2020-02-22T13:25:55.2181692Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.2183531Z ...    |
2020-02-22T13:25:55.2184141Z 2372 | |     }
2020-02-22T13:25:55.2184788Z 2373 | | }
2020-02-22T13:25:55.2185487Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.2185487Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.2186002Z ...
2020-02-22T13:25:55.2186772Z 2383 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T13:25:55.2187724Z 2384 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T13:25:55.2189584Z      |
2020-02-22T13:25:55.2189584Z      |
2020-02-22T13:25:55.2190358Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.2191189Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.2191524Z 
2020-02-22T13:25:55.2974322Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 4], i32>` is not stable as `const fn`
2020-02-22T13:25:55.2975220Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.2976330Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.2976330Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.2977347Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.2978531Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.2980513Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.2983140Z ...    |
2020-02-22T13:25:55.2984767Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.2986900Z ...    |
2020-02-22T13:25:55.2987780Z 2372 | |     }
2020-02-22T13:25:55.2988752Z 2373 | | }
2020-02-22T13:25:55.2989560Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.2989560Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.2990621Z ...
2020-02-22T13:25:55.2992080Z 2389 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T13:25:55.2993435Z 2390 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.2994458Z 2391 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T13:25:55.2996048Z      |
2020-02-22T13:25:55.2996048Z      |
2020-02-22T13:25:55.2996886Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.2997773Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.2998130Z 
2020-02-22T13:25:55.3669487Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], i64>` is not stable as `const fn`
2020-02-22T13:25:55.3670808Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.3671994Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.3671994Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.3673121Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.3674436Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.3675682Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.3676458Z ...    |
2020-02-22T13:25:55.3677297Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.3679054Z ...    |
2020-02-22T13:25:55.3679856Z 2372 | |     }
2020-02-22T13:25:55.3680529Z 2373 | | }
2020-02-22T13:25:55.3681256Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.3681256Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.3681782Z ...
2020-02-22T13:25:55.3682560Z 2396 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T13:25:55.3683586Z 2397 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T13:25:55.3684609Z 2398 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.3685563Z 2399 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T13:25:55.3687184Z      |
2020-02-22T13:25:55.3687184Z      |
2020-02-22T13:25:55.3688100Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.3688952Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.3689285Z 
2020-02-22T13:25:55.4429190Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 16], i128>` is not stable as `const fn`
2020-02-22T13:25:55.4430282Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.4431440Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.4431440Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.4432526Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.4434034Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.4435174Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.4435928Z ...    |
2020-02-22T13:25:55.4436728Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.4438290Z ...    |
2020-02-22T13:25:55.4438941Z 2372 | |     }
2020-02-22T13:25:55.4439637Z 2373 | | }
2020-02-22T13:25:55.4440487Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.4440487Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.4441011Z ...
2020-02-22T13:25:55.4441778Z 2404 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T13:25:55.4442769Z 2405 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T13:25:55.4443801Z 2406 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T13:25:55.4444897Z 2407 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T13:25:55.4445610Z ...    |
2020-02-22T13:25:55.4446371Z 2410 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T13:25:55.4447436Z 2411 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T13:25:55.4449090Z      |
2020-02-22T13:25:55.4449090Z      |
2020-02-22T13:25:55.4449997Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.4450890Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.4768414Z    Compiling autocfg v0.1.7
2020-02-22T13:25:55.4768414Z    Compiling autocfg v0.1.7
2020-02-22T13:25:55.5352992Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], isize>` is not stable as `const fn`
2020-02-22T13:25:55.5353962Z     --> src/libcore/num/mod.rs:2369:26
2020-02-22T13:25:55.5355174Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.5355174Z 256  | / macro_rules! int_impl {
2020-02-22T13:25:55.5356314Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T13:25:55.5357928Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.5359119Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.5359890Z ...    |
2020-02-22T13:25:55.5360818Z 2369 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:55.5362444Z ...    |
2020-02-22T13:25:55.5363145Z 2372 | |     }
2020-02-22T13:25:55.5363872Z 2373 | | }
2020-02-22T13:25:55.5364674Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.5364674Z      | |_- in this expansion of `int_impl!`
2020-02-22T13:25:55.5365227Z ...
2020-02-22T13:25:55.5366050Z 2434 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2020-02-22T13:25:55.5367180Z 2435 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2020-02-22T13:25:55.5368344Z 2436 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.5369364Z 2437 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T13:25:55.5370384Z 2438 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T13:25:55.5372167Z      |
2020-02-22T13:25:55.5372167Z      |
2020-02-22T13:25:55.5373400Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.5374309Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.5374818Z 
2020-02-22T13:25:55.6786961Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u8, [u8; 1]>` is not stable as `const fn`
2020-02-22T13:25:55.6787672Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.6788575Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.6788575Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.6789383Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.6790293Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.6791391Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.6792568Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.6793274Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.6793812Z ...    |
2020-02-22T13:25:55.6794292Z 4311 | |     }
2020-02-22T13:25:55.6794292Z 4311 | |     }
2020-02-22T13:25:55.6794841Z 4312 | | }
2020-02-22T13:25:55.6795796Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.6796511Z ...
2020-02-22T13:25:55.6797347Z 4316 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2020-02-22T13:25:55.6798282Z 4317 | |     "[0x12]", "", "" }
2020-02-22T13:25:55.6799779Z      |
2020-02-22T13:25:55.6799779Z      |
2020-02-22T13:25:55.6800637Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.6801511Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.6801898Z 
2020-02-22T13:25:55.7239306Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u16, [u8; 2]>` is not stable as `const fn`
2020-02-22T13:25:55.7240370Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.7242122Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.7242122Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.7243085Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.7244092Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.7245275Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.7246591Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.7247347Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.7247866Z ...    |
2020-02-22T13:25:55.7248390Z 4311 | |     }
2020-02-22T13:25:55.7248390Z 4311 | |     }
2020-02-22T13:25:55.7248988Z 4312 | | }
2020-02-22T13:25:55.7249609Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.7250222Z ...
2020-02-22T13:25:55.7250898Z 4814 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2020-02-22T13:25:55.7251678Z 4815 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T13:25:55.7253098Z      |
2020-02-22T13:25:55.7253098Z      |
2020-02-22T13:25:55.7253783Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.7254479Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.7254939Z 
2020-02-22T13:25:55.7989354Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u32, [u8; 4]>` is not stable as `const fn`
2020-02-22T13:25:55.7990742Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.7992349Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.7992349Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.7993597Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.7995042Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.7996458Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.7998415Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.7999553Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.8000391Z ...    |
2020-02-22T13:25:55.8001234Z 4311 | |     }
2020-02-22T13:25:55.8001234Z 4311 | |     }
2020-02-22T13:25:55.8002134Z 4312 | | }
2020-02-22T13:25:55.8003331Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.8004160Z ...
2020-02-22T13:25:55.8005178Z 4820 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2020-02-22T13:25:55.8006526Z 4821 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T13:25:55.8009566Z      |
2020-02-22T13:25:55.8009566Z      |
2020-02-22T13:25:55.8010491Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.8011584Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.8011945Z 
2020-02-22T13:25:55.8675945Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u64, [u8; 8]>` is not stable as `const fn`
2020-02-22T13:25:55.8679031Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.8680431Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.8680431Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.8681936Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.8683314Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.8686568Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.8688529Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.8689775Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.8690546Z ...    |
2020-02-22T13:25:55.8691121Z 4311 | |     }
2020-02-22T13:25:55.8691121Z 4311 | |     }
2020-02-22T13:25:55.8693047Z 4312 | | }
2020-02-22T13:25:55.8693774Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.8694265Z ...
2020-02-22T13:25:55.8695020Z 4826 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2020-02-22T13:25:55.8695989Z 4827 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2020-02-22T13:25:55.8696857Z 4828 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.8697907Z 4829 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T13:25:55.8698705Z 4830 | |     "", ""}
2020-02-22T13:25:55.8699953Z      |
2020-02-22T13:25:55.8699953Z      |
2020-02-22T13:25:55.8700833Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.8701591Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.8701891Z 
2020-02-22T13:25:55.9394818Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u128, [u8; 16]>` is not stable as `const fn`
2020-02-22T13:25:55.9395854Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.9397532Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.9397532Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.9398927Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.9400104Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.9401333Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.9402814Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.9403686Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.9404294Z ...    |
2020-02-22T13:25:55.9405119Z 4311 | |     }
2020-02-22T13:25:55.9405119Z 4311 | |     }
2020-02-22T13:25:55.9405808Z 4312 | | }
2020-02-22T13:25:55.9406551Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.9407092Z ...
2020-02-22T13:25:55.9407867Z 4835 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
2020-02-22T13:25:55.9409113Z 4836 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T13:25:55.9410118Z 4837 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T13:25:55.9411034Z 4838 | |     "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
2020-02-22T13:25:55.9411690Z ...    |
2020-02-22T13:25:55.9412687Z 4841 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
2020-02-22T13:25:55.9413614Z 4842 | |      "", ""}
2020-02-22T13:25:55.9414978Z      |
2020-02-22T13:25:55.9414978Z      |
2020-02-22T13:25:55.9415760Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.9416551Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.9416870Z 
2020-02-22T13:25:55.9952141Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<usize, [u8; 8]>` is not stable as `const fn`
2020-02-22T13:25:55.9953992Z     --> src/libcore/num/mod.rs:4198:26
2020-02-22T13:25:55.9955565Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.9955565Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:55.9956661Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:55.9957945Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:55.9959105Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:55.9961786Z 4198 | |                 unsafe { mem::transmute(self) }
2020-02-22T13:25:55.9962949Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T13:25:55.9963512Z ...    |
2020-02-22T13:25:55.9964052Z 4311 | |     }
2020-02-22T13:25:55.9964052Z 4311 | |     }
2020-02-22T13:25:55.9964639Z 4312 | | }
2020-02-22T13:25:55.9965288Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:55.9966055Z ...
2020-02-22T13:25:55.9966694Z 4863 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2020-02-22T13:25:55.9967732Z 4864 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2020-02-22T13:25:55.9968510Z 4865 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T13:25:55.9969280Z 4866 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T13:25:55.9970604Z 4867 | |     usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T13:25:55.9972307Z      |
2020-02-22T13:25:55.9972307Z      |
2020-02-22T13:25:55.9973687Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:55.9974478Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:55.9974770Z 
2020-02-22T13:25:56.0673236Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 1], u8>` is not stable as `const fn`
2020-02-22T13:25:56.0674059Z     --> src/libcore/num/mod.rs:4308:26
2020-02-22T13:25:56.0675582Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.0675582Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.0676879Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:56.0678420Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:56.0679782Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:56.0680607Z ...    |
2020-02-22T13:25:56.0681456Z 4308 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:56.0682945Z ...    |
2020-02-22T13:25:56.0683500Z 4311 | |     }
2020-02-22T13:25:56.0684106Z 4312 | | }
2020-02-22T13:25:56.0684106Z 4312 | | }
2020-02-22T13:25:56.0684746Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:56.0685208Z ...
2020-02-22T13:25:56.0686064Z 4316 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2020-02-22T13:25:56.0687224Z 4317 | |     "[0x12]", "", "" }
2020-02-22T13:25:56.0688901Z      |
2020-02-22T13:25:56.0688901Z      |
2020-02-22T13:25:56.0689858Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:56.0690820Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:56.0691295Z 
2020-02-22T13:25:56.1488346Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 2], u16>` is not stable as `const fn`
2020-02-22T13:25:56.1489607Z     --> src/libcore/num/mod.rs:4308:26
2020-02-22T13:25:56.1491546Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.1491546Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.1492919Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:56.1494604Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:56.1495632Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:56.1496781Z ...    |
2020-02-22T13:25:56.1497415Z 4308 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:56.1498650Z ...    |
2020-02-22T13:25:56.1499150Z 4311 | |     }
2020-02-22T13:25:56.1499822Z 4312 | | }
2020-02-22T13:25:56.1499822Z 4312 | | }
2020-02-22T13:25:56.1500614Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:56.1501045Z ...
2020-02-22T13:25:56.1501944Z 4814 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2020-02-22T13:25:56.1502935Z 4815 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T13:25:56.1504410Z      |
2020-02-22T13:25:56.1504410Z      |
2020-02-22T13:25:56.1505070Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:56.1505940Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:56.1506207Z 
2020-02-22T13:25:56.2109240Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 4], u32>` is not stable as `const fn`
2020-02-22T13:25:56.2110211Z     --> src/libcore/num/mod.rs:4308:26
2020-02-22T13:25:56.2112100Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.2112100Z 2441 | / macro_rules! uint_impl {
2020-02-22T13:25:56.2113241Z 2442 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T13:25:56.2114534Z 2443 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T13:25:56.2116247Z 2444 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T13:25:56.2117040Z ...    |
2020-02-22T13:25:56.2118203Z 4308 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T13:25:56.2119473Z ...    |
2020-02-22T13:25:56.2119986Z 4311 | |     }
2020-02-22T13:25:56.2120712Z 4312 | | }
2020-02-22T13:25:56.2120712Z 4312 | | }
2020-02-22T13:25:56.2121539Z      | |_- in this expansion of `uint_impl!`
2020-02-22T13:25:56.2121961Z ...
2020-02-22T13:25:56.2122610Z 4820 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2020-02-22T13:25:56.2123478Z 4821 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T13:25:56.2125219Z      |
2020-02-22T13:25:56.2125219Z      |
2020-02-22T13:25:56.2125867Z      = note: for more information, see issue ***/issues/57563
2020-02-22T13:25:56.2126528Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T13:25:56.2126795Z 
2020-02-22T13:25:56.2664087Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], u64>` is not stable as `const fn`
2020-02-22T13:25:56.2664795Z     --> src/libcore/num/mod.rs:4308:26
2020-02-22T13:25:56.2665683Z 2441 | / macro_rules! uint_impl {
---
2020-02-22T13:25:59.5680728Z   local time: Sat Feb 22 13:25:59 UTC 2020
2020-02-22T13:25:59.8581301Z   network time: Sat, 22 Feb 2020 13:25:59 GMT
2020-02-22T13:25:59.8587710Z == end clock drift check ==
2020-02-22T13:26:01.0653241Z 
2020-02-22T13:26:01.0739515Z ##[error]Bash exited with code '1'.
2020-02-22T13:26:01.0751562Z ##[section]Finishing: Run build
2020-02-22T13:26:01.0791768Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T13:26:01.0797684Z Task         : Get sources
2020-02-22T13:26:01.0797965Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T13:26:01.0798250Z Version      : 1.0.0
2020-02-22T13:26:01.0798435Z Author       : Microsoft
2020-02-22T13:26:01.0798435Z Author       : Microsoft
2020-02-22T13:26:01.0798720Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T13:26:01.0799294Z ==============================================================================
2020-02-22T13:26:01.3864012Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T13:26:01.3932606Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T13:26:01.4029651Z Cleaning up task key
2020-02-22T13:26:01.4031019Z Start cleaning up orphan processes.
2020-02-22T13:26:01.4318722Z Terminate orphan process: pid (5115) (python)
2020-02-22T13:26:01.4343820Z ##[section]Finishing: Finalize Job

plain
2020-02-22T12:49:34.1549738Z ========================== Starting Command Output ===========================
2020-02-22T12:49:34.1555863Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/050b4fc2-282e-46e7-bfb7-9d08593c1abe.sh
2020-02-22T12:49:34.1556135Z 
2020-02-22T12:49:34.1561303Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T12:49:34.1582268Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:49:34.1585795Z Task         : Get sources
2020-02-22T12:49:34.1586048Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:49:34.1586341Z Version      : 1.0.0
2020-02-22T12:49:34.1586511Z Author       : Microsoft
---
2020-02-22T12:49:36.3846067Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T12:49:36.4400280Z ##[command]git config gc.auto 0
2020-02-22T12:49:36.4411792Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T12:49:36.4416993Z ##[command]git config --get-all http.proxy
2020-02-22T12:49:36.4425601Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69373/merge:refs/remotes/pull/69373/merge
---
2020-02-22T12:54:28.8028055Z    Compiling libc v0.2.66
2020-02-22T12:54:30.2050863Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i8, [u8; 1]>` is not stable as `const fn`
2020-02-22T12:54:30.2051723Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.2052195Z      |
2020-02-22T12:54:30.2053104Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.2054655Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.2056331Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.2057783Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.2059505Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.2060316Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.2060881Z ...    |
2020-02-22T12:54:30.2061452Z 2370 | |     }
2020-02-22T12:54:30.2061452Z 2370 | |     }
2020-02-22T12:54:30.2062381Z 2371 | | }
2020-02-22T12:54:30.2063084Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.2063918Z ...
2020-02-22T12:54:30.2065074Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:54:30.2066125Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:54:30.2068533Z      |
2020-02-22T12:54:30.2068533Z      |
2020-02-22T12:54:30.2069416Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.2070134Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.2657357Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i16, [u8; 2]>` is not stable as `const fn`
2020-02-22T12:54:30.2658474Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.2659000Z      |
2020-02-22T12:54:30.2659625Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.2659625Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.2660686Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.2661937Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.2663067Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.2664562Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.2665480Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.2666097Z ...    |
2020-02-22T12:54:30.2666719Z 2370 | |     }
2020-02-22T12:54:30.2666719Z 2370 | |     }
2020-02-22T12:54:30.2667401Z 2371 | | }
2020-02-22T12:54:30.2668131Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.2668662Z ...
2020-02-22T12:54:30.2669459Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:54:30.2670461Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:54:30.2672017Z      |
2020-02-22T12:54:30.2672017Z      |
2020-02-22T12:54:30.2673106Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.2674014Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.3236407Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i32, [u8; 4]>` is not stable as `const fn`
2020-02-22T12:54:30.3237640Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.3238132Z      |
2020-02-22T12:54:30.3238756Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.3238756Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.3239829Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.3241067Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.3242194Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.3243701Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.3244603Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.3245220Z ...    |
2020-02-22T12:54:30.3246912Z 2370 | |     }
2020-02-22T12:54:30.3246912Z 2370 | |     }
2020-02-22T12:54:30.3247666Z 2371 | | }
2020-02-22T12:54:30.3248449Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.3249017Z ...
2020-02-22T12:54:30.3250053Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:54:30.3251117Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.3252064Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:54:30.3253601Z      |
2020-02-22T12:54:30.3253601Z      |
2020-02-22T12:54:30.3254406Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.3255244Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.3813423Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i64, [u8; 8]>` is not stable as `const fn`
2020-02-22T12:54:30.3814249Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.3814676Z      |
2020-02-22T12:54:30.3815387Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.3815387Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.3816389Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.3817472Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.3818588Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.3819908Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.3821083Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.3822029Z ...    |
2020-02-22T12:54:30.3822788Z 2370 | |     }
2020-02-22T12:54:30.3822788Z 2370 | |     }
2020-02-22T12:54:30.3824274Z 2371 | | }
2020-02-22T12:54:30.3825030Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.3825569Z ...
2020-02-22T12:54:30.3826689Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:54:30.3827947Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:54:30.3828829Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.3829644Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:54:30.3831287Z      |
2020-02-22T12:54:30.3831287Z      |
2020-02-22T12:54:30.3831968Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.3832700Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.4421087Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<i128, [u8; 16]>` is not stable as `const fn`
2020-02-22T12:54:30.4422087Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.4423033Z      |
2020-02-22T12:54:30.4423870Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.4423870Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.4425383Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.4427161Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.4429097Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.4431114Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.4432436Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.4434044Z ...    |
2020-02-22T12:54:30.4434933Z 2370 | |     }
2020-02-22T12:54:30.4434933Z 2370 | |     }
2020-02-22T12:54:30.4435976Z 2371 | | }
2020-02-22T12:54:30.4436927Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.4437528Z ...
2020-02-22T12:54:30.4438322Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:54:30.4439486Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:54:30.4440844Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:54:30.4441992Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:54:30.4442938Z ...    |
2020-02-22T12:54:30.4443950Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:54:30.4445120Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:54:30.4447220Z      |
2020-02-22T12:54:30.4447220Z      |
2020-02-22T12:54:30.4448262Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.4449322Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.4449719Z 
2020-02-22T12:54:30.5103494Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<isize, [u8; 8]>` is not stable as `const fn`
2020-02-22T12:54:30.5104678Z     --> src/libcore/num/mod.rs:2258:26
2020-02-22T12:54:30.5105965Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.5105965Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.5107173Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.5108890Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.5110172Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.5111916Z 2258 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:30.5113354Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:30.5114404Z ...    |
2020-02-22T12:54:30.5115059Z 2370 | |     }
2020-02-22T12:54:30.5115059Z 2370 | |     }
2020-02-22T12:54:30.5115768Z 2371 | | }
2020-02-22T12:54:30.5116900Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.5117525Z ...
2020-02-22T12:54:30.5118359Z 2432 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2020-02-22T12:54:30.5119439Z 2433 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:54:30.5121834Z 2434 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.5123157Z 2435 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T12:54:30.5124362Z 2436 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T12:54:30.5127461Z      |
2020-02-22T12:54:30.5127461Z      |
2020-02-22T12:54:30.5128497Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.5129385Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.5129944Z 
2020-02-22T12:54:30.5740817Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 1], i8>` is not stable as `const fn`
2020-02-22T12:54:30.5742023Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.5743495Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.5743495Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.5744717Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.5747090Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.5748604Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.5749944Z ...    |
2020-02-22T12:54:30.5751034Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.5753132Z ...    |
2020-02-22T12:54:30.5754192Z 2370 | |     }
2020-02-22T12:54:30.5755137Z 2371 | | }
2020-02-22T12:54:30.5755928Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.5755928Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.5756740Z ...
2020-02-22T12:54:30.5757598Z 2375 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2020-02-22T12:54:30.5758873Z 2376 | |     "[0x12]", "[0x12]", "", "" }
2020-02-22T12:54:30.5760638Z      |
2020-02-22T12:54:30.5760638Z      |
2020-02-22T12:54:30.5761497Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.5762557Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.5763267Z 
2020-02-22T12:54:30.6398646Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 2], i16>` is not stable as `const fn`
2020-02-22T12:54:30.6399820Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.6401754Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.6401754Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.6403297Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.6405385Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.6407131Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.6408334Z ...    |
2020-02-22T12:54:30.6409740Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.6411830Z ...    |
2020-02-22T12:54:30.6412918Z 2370 | |     }
2020-02-22T12:54:30.6413870Z 2371 | | }
2020-02-22T12:54:30.6414874Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.6414874Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.6415668Z ...
2020-02-22T12:54:30.6416972Z 2381 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2020-02-22T12:54:30.6418109Z 2382 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:54:30.6420061Z      |
2020-02-22T12:54:30.6420061Z      |
2020-02-22T12:54:30.6421092Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.6421990Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.6422430Z 
2020-02-22T12:54:30.7016914Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 4], i32>` is not stable as `const fn`
2020-02-22T12:54:30.7018020Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.7019485Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.7019485Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.7020691Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.7022072Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.7023323Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.7024189Z ...    |
2020-02-22T12:54:30.7025134Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.7026948Z ...    |
2020-02-22T12:54:30.7027698Z 2370 | |     }
2020-02-22T12:54:30.7028524Z 2371 | | }
2020-02-22T12:54:30.7029419Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.7029419Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.7030076Z ...
2020-02-22T12:54:30.7031037Z 2387 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2020-02-22T12:54:30.7032197Z 2388 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.7033381Z 2389 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:54:30.7035205Z      |
2020-02-22T12:54:30.7035205Z      |
2020-02-22T12:54:30.7036117Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.7037285Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.7037777Z 
2020-02-22T12:54:30.7683983Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], i64>` is not stable as `const fn`
2020-02-22T12:54:30.7684946Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.7686084Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.7686084Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.7687429Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.7688841Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.7689974Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.7690700Z ...    |
2020-02-22T12:54:30.7691477Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.7693025Z ...    |
2020-02-22T12:54:30.7693660Z 2370 | |     }
2020-02-22T12:54:30.7694339Z 2371 | | }
2020-02-22T12:54:30.7695076Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.7695076Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.7695611Z ...
2020-02-22T12:54:30.7696393Z 2394 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2020-02-22T12:54:30.7697666Z 2395 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:54:30.7699663Z 2396 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.7700783Z 2397 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2020-02-22T12:54:30.7702806Z      |
2020-02-22T12:54:30.7702806Z      |
2020-02-22T12:54:30.7703628Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.7704471Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.7704828Z 
2020-02-22T12:54:30.8649163Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 16], i128>` is not stable as `const fn`
2020-02-22T12:54:30.8650002Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.8651633Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.8651633Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.8652719Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.8654574Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.8656071Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.8656722Z ...    |
2020-02-22T12:54:30.8657448Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.8658830Z ...    |
2020-02-22T12:54:30.8659399Z 2370 | |     }
2020-02-22T12:54:30.8660012Z 2371 | | }
2020-02-22T12:54:30.8660682Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.8660682Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.8661168Z ...
2020-02-22T12:54:30.8661852Z 2402 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2020-02-22T12:54:30.8662737Z 2403 | |     170141183460469231731687303715884105727, "", "", 16,
2020-02-22T12:54:30.8663661Z 2404 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:54:30.8664598Z 2405 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:54:30.8665255Z ...    |
2020-02-22T12:54:30.8666108Z 2408 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2020-02-22T12:54:30.8667150Z 2409 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2020-02-22T12:54:30.8668615Z      |
2020-02-22T12:54:30.8668615Z      |
2020-02-22T12:54:30.8669517Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.8670230Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.8821580Z    Compiling autocfg v0.1.7
2020-02-22T12:54:30.8821580Z    Compiling autocfg v0.1.7
2020-02-22T12:54:30.9454901Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], isize>` is not stable as `const fn`
2020-02-22T12:54:30.9455802Z     --> src/libcore/num/mod.rs:2367:26
2020-02-22T12:54:30.9457162Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.9457162Z 256  | / macro_rules! int_impl {
2020-02-22T12:54:30.9458227Z 257  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2020-02-22T12:54:30.9459421Z 258  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:30.9460718Z 259  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:30.9461412Z ...    |
2020-02-22T12:54:30.9462179Z 2367 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:30.9463654Z ...    |
2020-02-22T12:54:30.9464263Z 2370 | |     }
2020-02-22T12:54:30.9464911Z 2371 | | }
2020-02-22T12:54:30.9465626Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.9465626Z      | |_- in this expansion of `int_impl!`
2020-02-22T12:54:30.9466194Z ...
2020-02-22T12:54:30.9466936Z 2432 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2020-02-22T12:54:30.9468504Z 2433 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2020-02-22T12:54:30.9469661Z 2434 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:30.9470575Z 2435 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T12:54:30.9471508Z 2436 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T12:54:30.9473333Z      |
2020-02-22T12:54:30.9473333Z      |
2020-02-22T12:54:30.9474247Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:30.9475026Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:30.9479376Z 
2020-02-22T12:54:31.0892271Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u8, [u8; 1]>` is not stable as `const fn`
2020-02-22T12:54:31.0893236Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.0894340Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.0894340Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.0895372Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.0896698Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.0898088Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.0899663Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.0900544Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.0901146Z ...    |
2020-02-22T12:54:31.0901740Z 4307 | |     }
2020-02-22T12:54:31.0901740Z 4307 | |     }
2020-02-22T12:54:31.0904306Z 4308 | | }
2020-02-22T12:54:31.0905109Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.0905660Z ...
2020-02-22T12:54:31.0906611Z 4312 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2020-02-22T12:54:31.0907566Z 4313 | |     "[0x12]", "", "" }
2020-02-22T12:54:31.0909055Z      |
2020-02-22T12:54:31.0909055Z      |
2020-02-22T12:54:31.0909931Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.0910809Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.0919337Z 
2020-02-22T12:54:31.1745569Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u16, [u8; 2]>` is not stable as `const fn`
2020-02-22T12:54:31.1746565Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.1747768Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.1747768Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.1748929Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.1750218Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.1751387Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.1753688Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.1754655Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.1755321Z ...    |
2020-02-22T12:54:31.1756167Z 4307 | |     }
2020-02-22T12:54:31.1756167Z 4307 | |     }
2020-02-22T12:54:31.1756952Z 4308 | | }
2020-02-22T12:54:31.1757771Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.1758326Z ...
2020-02-22T12:54:31.1759183Z 4810 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2020-02-22T12:54:31.1760345Z 4811 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:54:31.1761957Z      |
2020-02-22T12:54:31.1761957Z      |
2020-02-22T12:54:31.1762961Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.1764030Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.1769090Z 
2020-02-22T12:54:31.2368425Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u32, [u8; 4]>` is not stable as `const fn`
2020-02-22T12:54:31.2369290Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.2370510Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.2370510Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.2371455Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.2372519Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.2373496Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.2374874Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.2375694Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.2376259Z ...    |
2020-02-22T12:54:31.2376819Z 4307 | |     }
2020-02-22T12:54:31.2376819Z 4307 | |     }
2020-02-22T12:54:31.2377626Z 4308 | | }
2020-02-22T12:54:31.2378321Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.2378980Z ...
2020-02-22T12:54:31.2379757Z 4816 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2020-02-22T12:54:31.2380808Z 4817 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:54:31.2383224Z      |
2020-02-22T12:54:31.2383224Z      |
2020-02-22T12:54:31.2383970Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.2384731Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.2390025Z 
2020-02-22T12:54:31.3252465Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u64, [u8; 8]>` is not stable as `const fn`
2020-02-22T12:54:31.3253509Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.3254698Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.3254698Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.3255706Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.3257208Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.3258480Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.3260470Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.3261334Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.3261940Z ...    |
2020-02-22T12:54:31.3262555Z 4307 | |     }
2020-02-22T12:54:31.3262555Z 4307 | |     }
2020-02-22T12:54:31.3263206Z 4308 | | }
2020-02-22T12:54:31.3263915Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.3264444Z ...
2020-02-22T12:54:31.3265433Z 4822 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2020-02-22T12:54:31.3266798Z 4823 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2020-02-22T12:54:31.3268459Z 4824 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:31.3269373Z 4825 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T12:54:31.3270180Z 4826 | |     "", ""}
2020-02-22T12:54:31.3271503Z      |
2020-02-22T12:54:31.3271503Z      |
2020-02-22T12:54:31.3272298Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.3273252Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.3278008Z 
2020-02-22T12:54:31.3903527Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<u128, [u8; 16]>` is not stable as `const fn`
2020-02-22T12:54:31.3904396Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.3905466Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.3905466Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.3906834Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.3907967Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.3909179Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.3910585Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.3911431Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.3912175Z ...    |
2020-02-22T12:54:31.3913574Z 4307 | |     }
2020-02-22T12:54:31.3913574Z 4307 | |     }
2020-02-22T12:54:31.3914276Z 4308 | | }
2020-02-22T12:54:31.3914963Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.3915469Z ...
2020-02-22T12:54:31.3916191Z 4831 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
2020-02-22T12:54:31.3917300Z 4832 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2020-02-22T12:54:31.3918268Z 4833 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2020-02-22T12:54:31.3919128Z 4834 | |     "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
2020-02-22T12:54:31.3919738Z ...    |
2020-02-22T12:54:31.3920603Z 4837 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
2020-02-22T12:54:31.3921344Z 4838 | |      "", ""}
2020-02-22T12:54:31.3922581Z      |
2020-02-22T12:54:31.3922581Z      |
2020-02-22T12:54:31.3923324Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.3924080Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.3956408Z 
2020-02-22T12:54:31.4649560Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<usize, [u8; 8]>` is not stable as `const fn`
2020-02-22T12:54:31.4650424Z     --> src/libcore/num/mod.rs:4195:26
2020-02-22T12:54:31.4651454Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.4651454Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.4653611Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.4655128Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.4656594Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.4657941Z 4195 | |                 unsafe { mem::transmute(self) }
2020-02-22T12:54:31.4658718Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2020-02-22T12:54:31.4659281Z ...    |
2020-02-22T12:54:31.4659815Z 4307 | |     }
2020-02-22T12:54:31.4659815Z 4307 | |     }
2020-02-22T12:54:31.4660410Z 4308 | | }
2020-02-22T12:54:31.4661073Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.4661522Z ...
2020-02-22T12:54:31.4662247Z 4859 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2020-02-22T12:54:31.4663150Z 4860 | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2020-02-22T12:54:31.4663968Z 4861 | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2020-02-22T12:54:31.4664987Z 4862 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2020-02-22T12:54:31.4669795Z 4863 | |     usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2020-02-22T12:54:31.4673776Z      |
2020-02-22T12:54:31.4673776Z      |
2020-02-22T12:54:31.4674818Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.4676021Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.4716617Z 
2020-02-22T12:54:31.5366731Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 1], u8>` is not stable as `const fn`
2020-02-22T12:54:31.5368196Z     --> src/libcore/num/mod.rs:4304:26
2020-02-22T12:54:31.5369600Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.5369600Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.5370917Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.5372597Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.5373794Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.5375369Z ...    |
2020-02-22T12:54:31.5376300Z 4304 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:31.5378111Z ...    |
2020-02-22T12:54:31.5378867Z 4307 | |     }
2020-02-22T12:54:31.5379700Z 4308 | | }
2020-02-22T12:54:31.5379700Z 4308 | | }
2020-02-22T12:54:31.5380587Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.5381253Z ...
2020-02-22T12:54:31.5382181Z 4312 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2020-02-22T12:54:31.5383185Z 4313 | |     "[0x12]", "", "" }
2020-02-22T12:54:31.5384849Z      |
2020-02-22T12:54:31.5384849Z      |
2020-02-22T12:54:31.5385760Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.5386834Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.5390636Z 
2020-02-22T12:54:31.6227422Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 2], u16>` is not stable as `const fn`
2020-02-22T12:54:31.6228308Z     --> src/libcore/num/mod.rs:4304:26
2020-02-22T12:54:31.6229426Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.6229426Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.6230444Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.6231630Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.6232709Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.6233627Z ...    |
2020-02-22T12:54:31.6234427Z 4304 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:31.6236116Z ...    |
2020-02-22T12:54:31.6236996Z 4307 | |     }
2020-02-22T12:54:31.6237695Z 4308 | | }
2020-02-22T12:54:31.6237695Z 4308 | | }
2020-02-22T12:54:31.6238472Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.6239351Z ...
2020-02-22T12:54:31.6240369Z 4810 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2020-02-22T12:54:31.6241654Z 4811 | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2020-02-22T12:54:31.6243169Z      |
2020-02-22T12:54:31.6243169Z      |
2020-02-22T12:54:31.6243946Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.6244755Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.6248749Z 
2020-02-22T12:54:31.6990963Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 4], u32>` is not stable as `const fn`
2020-02-22T12:54:31.6991864Z     --> src/libcore/num/mod.rs:4304:26
2020-02-22T12:54:31.6993272Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.6993272Z 2439 | / macro_rules! uint_impl {
2020-02-22T12:54:31.6994337Z 2440 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2020-02-22T12:54:31.6995462Z 2441 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2020-02-22T12:54:31.6998995Z 2442 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2020-02-22T12:54:31.6999961Z ...    |
2020-02-22T12:54:31.7000802Z 4304 | |                 unsafe { mem::transmute(bytes) }
2020-02-22T12:54:31.7003099Z ...    |
2020-02-22T12:54:31.7003755Z 4307 | |     }
2020-02-22T12:54:31.7004500Z 4308 | | }
2020-02-22T12:54:31.7004500Z 4308 | | }
2020-02-22T12:54:31.7005290Z      | |_- in this expansion of `uint_impl!`
2020-02-22T12:54:31.7005851Z ...
2020-02-22T12:54:31.7006740Z 4816 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2020-02-22T12:54:31.7008074Z 4817 | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2020-02-22T12:54:31.7010483Z      |
2020-02-22T12:54:31.7010483Z      |
2020-02-22T12:54:31.7011329Z      = note: for more information, see issue ***/issues/57563
2020-02-22T12:54:31.7012292Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-22T12:54:31.7028212Z 
2020-02-22T12:54:31.7794952Z error[E0723]: can only call other `const fn` within a `const fn`, but `const intrinsics::transmute::<[u8; 8], u64>` is not stable as `const fn`
2020-02-22T12:54:31.7795764Z     --> src/libcore/num/mod.rs:4304:26
2020-02-22T12:54:31.7796804Z 2439 | / macro_rules! uint_impl {
---
2020-02-22T12:54:37.3910439Z   local time: Sat Feb 22 12:54:37 UTC 2020
2020-02-22T12:54:37.7872289Z   network time: Sat, 22 Feb 2020 12:54:37 GMT
2020-02-22T12:54:37.7877579Z == end clock drift check ==
2020-02-22T12:54:38.6125894Z 
2020-02-22T12:54:38.6161899Z ##[error]Bash exited with code '1'.
2020-02-22T12:54:38.6173131Z ##[section]Finishing: Run build
2020-02-22T12:54:38.6216705Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:54:38.6222738Z Task         : Get sources
2020-02-22T12:54:38.6223060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:54:38.6223360Z Version      : 1.0.0
2020-02-22T12:54:38.6223823Z Author       : Microsoft
2020-02-22T12:54:38.6223823Z Author       : Microsoft
2020-02-22T12:54:38.6224654Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T12:54:38.6225512Z ==============================================================================
2020-02-22T12:54:38.9693208Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T12:54:38.9733222Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69373/merge to s
2020-02-22T12:54:38.9824138Z Cleaning up task key
2020-02-22T12:54:38.9825238Z Start cleaning up orphan processes.
2020-02-22T12:54:39.0020078Z Terminate orphan process: pid (4889) (python)
2020-02-22T12:54:39.0225242Z ##[section]Finishing: Finalize Job
